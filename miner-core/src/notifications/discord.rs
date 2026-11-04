use std::collections::HashSet;
use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest::multipart::{Form, Part};
use reqwest::{Response, StatusCode};
use serde_json::{json, Value};
use url::Url;

use super::{
    account_name, category_url, cdn, channel_name, channel_url, clip, color_for, label_for,
    retry_after_header, seconds, sent_by, test_picture, Answer, Notice, Shared,
};
use crate::config::DiscordSettings;
use crate::error::{Error, Result};
use crate::events::MinerEvent;
use crate::i18n;
use crate::requests::RequestKind;

type AttachmentType = (&'static str, &'static [u8]);

pub const EMBEDS_MAX: usize = 10;
const MESSAGE_TEXT_MAX: usize = 6000;
const TITLE_MAX: usize = 256;
const WEBHOOKS_PATH: &str = "/api/webhooks";
const RATE_LIMIT_REMAINING: &str = "x-ratelimit-remaining";
const RATE_LIMIT_RESET_AFTER: &str = "x-ratelimit-reset-after";

const MARKDOWN: [char; 12] = ['\\', '*', '_', '~', '`', '|', '>', '<', '[', ']', '(', ')'];

const MARKDOWN_LINE_START: [char; 2] = ['#', '-'];

pub(super) async fn send(
    shared: &Shared,
    settings: &DiscordSettings,
    notices: &[Notice],
) -> Result<Answer> {
    let url = settings.webhook_url.trim();

    if !shared
        .endpoints
        .discord
        .iter()
        .any(|prefix| url.starts_with(prefix))
    {
        return Err(Error::Notification(
            "Discord webhook URL must start with https://discord.com/api/webhooks/".into(),
        ));
    }

    let mut attachment = None;
    let payload = payload(notices, &mut attachment);
    let request = match attachment {
        None => shared.client.post(url).json(&payload),
        Some((name, bytes)) => {
            let part = Part::bytes(bytes)
                .file_name(name)
                .mime_str("image/png")
                .map_err(|e| Error::Notification(e.to_string()))?;
            let form = Form::new()
                .text("payload_json", payload.to_string())
                .part("files[0]", part);

            shared.client.post(url).multipart(form)
        }
    };
    let response = shared
        .requests
        .start(RequestKind::Discord, target(url))
        .account(common_account(notices))
        .send(request)
        .await?;

    Ok(answer(response).await)
}

fn target(url: &str) -> String {
    let host = Url::parse(url)
        .ok()
        .and_then(|url| url.host_str().map(str::to_owned))
        .unwrap_or_default();

    format!("{host}{WEBHOOKS_PATH}")
}

pub(super) fn common_account(notices: &[Notice]) -> Option<&str> {
    let first = notices.first()?.event.account_id.as_str();

    notices
        .iter()
        .all(|notice| notice.event.account_id == first)
        .then_some(first)
        .filter(|id| !id.is_empty())
}

// ! A message the webhook's bucket used up must wait for the bucket to fill
// again, or the next one is refused with 429.
fn pause(headers: &HeaderMap) -> Option<Duration> {
    let text = |name: &str| headers.get(name).and_then(|value| value.to_str().ok());

    if text(RATE_LIMIT_REMAINING)?.trim() != "0" {
        return None;
    }

    seconds(text(RATE_LIMIT_RESET_AFTER)?)
}

async fn answer(response: Response) -> Answer {
    let status = response.status();
    let pause = pause(response.headers());

    if status != StatusCode::TOO_MANY_REQUESTS {
        return Answer {
            status,
            retry_after: None,
            pause,
        };
    }

    let header = retry_after_header(&response);
    let body = response.json::<Value>().await.ok();
    let retry_after = body
        .as_ref()
        .and_then(|body| body["retry_after"].as_f64())
        .and_then(|after| seconds(&after.to_string()))
        .or(header);

    Answer {
        status,
        retry_after,
        pause,
    }
}

// ! Names and chat messages come from other people: no mention may ping
// anyone, whatever the text holds.
pub(super) fn payload(notices: &[Notice], attachment: &mut Option<AttachmentType>) -> Value {
    let mut embeds: Vec<Value> = notices
        .iter()
        .map(|notice| embed(&notice.event, notice.language, attachment))
        .collect();

    if notices.len() > 1 {
        distinguish(&mut embeds, notices);
    }

    json!({
        "username": "Twitch Miner",
        "embeds": embeds,
        "allowed_mentions": {"parse": []},
    })
}

// ! Discord folds the embeds of one message that share an address, and folds
// look-alike ones too: in a grouped message every embed gets its own title
// (with its account) and an address no other embed has.
fn distinguish(embeds: &mut [Value], notices: &[Notice]) {
    let mut titles = HashSet::new();
    let mut addresses = HashSet::new();

    for (embed, notice) in embeds.iter_mut().zip(notices) {
        let event = &notice.event;
        let base = match account_name(event) {
            Some(account) => format!("{} · {account}", event.title),
            None => event.title.clone(),
        };
        let mut copy = 1;
        let mut title = markdown(&base, TITLE_MAX);

        while !titles.insert(title.clone()) {
            copy += 1;

            let suffix = format!(" ({copy})");

            title = format!(
                "{}{suffix}",
                markdown(&base, TITLE_MAX - suffix.encode_utf16().count())
            );
        }

        embed["title"] = json!(title);

        let shared_address = embed["url"]
            .as_str()
            .is_some_and(|url| !addresses.insert(url.to_owned()));

        if shared_address {
            if let Some(fields) = embed.as_object_mut() {
                fields.remove("url");
            }
        }
    }
}

fn units(value: &Value) -> usize {
    value.as_str().map_or(0, |text| text.encode_utf16().count())
}

fn message_text(payload: &Value) -> usize {
    payload["embeds"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|embed| {
            let fields: usize = embed["fields"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|field| units(&field["name"]) + units(&field["value"]))
                .sum();

            units(&embed["title"])
                + units(&embed["description"])
                + units(&embed["footer"]["text"])
                + units(&embed["author"]["name"])
                + fields
        })
        .sum()
}

pub(super) fn fits(notices: &[Notice]) -> bool {
    notices.len() <= EMBEDS_MAX && message_text(&payload(notices, &mut None)) <= MESSAGE_TEXT_MAX
}

fn picture_url(url: Option<&str>, attachment: &mut Option<AttachmentType>) -> Option<String> {
    let url = url?;

    if let Some((name, bytes)) = test_picture(url) {
        *attachment = Some((name, bytes));

        return Some(format!("attachment://{name}"));
    }

    cdn(Some(url)).map(str::to_owned)
}

// ! Leading spaces keep a line's start: Discord still reads ` - item` as a list.
fn marked(text: &str) -> impl Iterator<Item = (char, bool)> + '_ {
    text.chars().scan(true, |line_start, ch| {
        let special = MARKDOWN.contains(&ch) || (*line_start && MARKDOWN_LINE_START.contains(&ch));

        *line_start = ch == '\n' || (*line_start && ch.is_whitespace());

        Some((ch, special))
    })
}

// ! Escapes and clips in one pass, so a cut never separates an escape from its
// character.
fn markdown(text: &str, max: usize) -> String {
    let size = |(ch, special): (char, bool)| ch.len_utf16() + usize::from(special);
    let fits = marked(text).map(size).sum::<usize>() <= max;
    let room = if fits { max } else { max.saturating_sub(1) };
    let mut escaped = String::with_capacity(text.len());
    let mut used = 0;

    for (ch, special) in marked(text) {
        if used + size((ch, special)) > room {
            escaped.push('…');

            break;
        }

        if special {
            escaped.push('\\');
        }

        escaped.push(ch);
        used += size((ch, special));
    }

    escaped
}

fn embed(event: &MinerEvent, language: &str, attachment: &mut Option<AttachmentType>) -> Value {
    let context = &event.context;
    let mut embed = json!({
        "title": markdown(&event.title, TITLE_MAX),
        "color": color_for(event.kind),
        "timestamp": event.at.to_rfc3339(),
    });

    if let Some(name) = account_name(event) {
        embed["footer"] = json!({"text": clip(name, 2048)});

        if let Some(avatar) = picture_url(context.account_avatar.as_deref(), attachment) {
            embed["footer"]["icon_url"] = json!(avatar);
        }
    }

    let mut fields = vec![json!({
        "name": i18n::text(language, "notify_kind", &[]),
        "value": clip(&label_for(language, event.kind), 1024),
        "inline": true,
    })];

    if let Some(name) = channel_name(context) {
        let name = markdown(name, 256);
        let value = match channel_url(context) {
            Some(url) => format!("[{name}]({url})"),
            None => name,
        };

        fields.push(json!({
            "name": i18n::text(language, "notify_streamer", &[]),
            "value": value,
            "inline": true,
        }));
    }

    fields.push(json!({
        "name": i18n::text(language, "notify_sent_by", &[]),
        "value": format!("[{}]({})", sent_by(), crate::REPOSITORY_URL),
        "inline": false,
    }));

    embed["fields"] = json!(fields);

    if let Some(url) = channel_url(context) {
        embed["url"] = json!(url);
    }

    if let Some(detail) = &event.detail {
        embed["description"] = json!(markdown(detail, 4096));
    }

    if let Some(game) = context.game.as_deref().filter(|game| !game.is_empty()) {
        embed["author"] = json!({"name": clip(game, 256)});

        if let Some(url) = category_url(context) {
            embed["author"]["url"] = json!(url);
        }

        if let Some(art) = picture_url(context.art.as_deref(), attachment) {
            embed["author"]["icon_url"] = json!(art);
        }
    }

    let thumbnail = picture_url(context.avatar.as_deref(), attachment)
        .or_else(|| picture_url(context.art.as_deref(), attachment));

    if let Some(thumbnail) = thumbnail {
        embed["thumbnail"] = json!({"url": thumbnail});
    }

    if let Some(reward) = picture_url(context.reward.as_deref(), attachment) {
        embed["image"] = json!({"url": reward});
    }

    embed
}
