use reqwest::multipart::{Form, Part};
use reqwest::{Response, StatusCode};
use serde_json::{json, Value};
use url::Url;

use super::{
    account_name, category_url, channel_name, channel_url, label_for, picture, retry_after_header,
    seconds, sent_by, test_picture, Answer, Notice, Shared,
};
use crate::config::TelegramSettings;
use crate::error::{Error, Result};
use crate::events::{EventContext, MinerEvent};
use crate::i18n;
use crate::requests::RequestKind;

const CAPTION_MAX: usize = 1024;
const MESSAGE_MAX: usize = 4096;

const SEND_PHOTO: &str = "sendPhoto";
const SEND_MESSAGE: &str = "sendMessage";

pub(super) async fn send(
    shared: &Shared,
    settings: &TelegramSettings,
    notice: &Notice,
) -> Result<Answer> {
    let event = &notice.event;
    let language = notice.language;
    let account = Some(event.account_id.as_str()).filter(|id| !id.is_empty());
    let token = settings.bot_token.trim();

    if token.is_empty() || !token.contains(':') {
        return Err(Error::Notification(
            "Telegram bot token looks invalid".into(),
        ));
    }

    let chat_id = settings.chat_id.trim();

    if let Some(photo) = picture(&event.context) {
        let caption = text(event, language, CAPTION_MAX);
        let request = shared
            .client
            .post(method(shared.endpoints.telegram, token, SEND_PHOTO));
        let request = match test_picture(photo) {
            Some((name, bytes)) => {
                let part = Part::bytes(bytes)
                    .file_name(name)
                    .mime_str("image/png")
                    .map_err(|e| Error::Notification(e.to_string()))?;

                request.multipart(
                    Form::new()
                        .text("chat_id", chat_id.to_owned())
                        .text("caption", caption)
                        .text("parse_mode", "HTML")
                        .part("photo", part),
                )
            }
            None => request.json(&json!({
                "chat_id": chat_id,
                "photo": photo,
                "caption": caption,
                "parse_mode": "HTML",
            })),
        };
        let response = shared
            .requests
            .start(
                RequestKind::Telegram,
                target(shared.endpoints.telegram, SEND_PHOTO),
            )
            .account(account)
            .expecting(&[StatusCode::BAD_REQUEST])
            .send(request)
            .await?;

        // ! A picture Telegram cannot fetch answers 400: the text still goes out.
        if response.status() != StatusCode::BAD_REQUEST {
            return Ok(answer(response).await);
        }
    }

    let body = json!({
        "chat_id": chat_id,
        "text": text(event, language, MESSAGE_MAX),
        "parse_mode": "HTML",
        "link_preview_options": {"is_disabled": true},
    });
    let response = shared
        .requests
        .start(
            RequestKind::Telegram,
            target(shared.endpoints.telegram, SEND_MESSAGE),
        )
        .account(account)
        .send(
            shared
                .client
                .post(method(shared.endpoints.telegram, token, SEND_MESSAGE))
                .json(&body),
        )
        .await?;

    Ok(answer(response).await)
}

// ! The Bot API URL holds the token: never log it.
fn method(base: &str, token: &str, name: &str) -> String {
    format!("{base}/bot{token}/{name}")
}

fn target(base: &str, name: &str) -> String {
    let host = Url::parse(base)
        .ok()
        .and_then(|url| url.host_str().map(str::to_owned))
        .unwrap_or_default();

    format!("{host}/{name}")
}

async fn answer(response: Response) -> Answer {
    let status = response.status();

    if status != StatusCode::TOO_MANY_REQUESTS {
        return Answer::of(status);
    }

    let header = retry_after_header(&response);
    let body = response.json::<Value>().await.ok();
    let retry_after = body
        .as_ref()
        .and_then(|body| body["parameters"]["retry_after"].as_f64())
        .and_then(|after| seconds(&after.to_string()))
        .or(header);

    Answer {
        status,
        retry_after,
        pause: None,
    }
}

fn text(event: &MinerEvent, language: &str, max: usize) -> String {
    let mut head = format!(
        "<b>{}</b>\n{}",
        escape_within(&event.title, 256),
        escape(&label_for(language, event.kind))
    );

    if let Some(name) = account_name(event) {
        head.push_str(&format!(" · {}", escape_within(name, 64)));
    }

    let place = place(&event.context, language);
    let signature = signature();
    let used =
        units(&head) + place.as_deref().map_or(0, |place| units(place) + 1) + units(&signature) + 1;
    let mut text = head;

    if let Some(detail) = event.detail.as_deref().filter(|d| !d.is_empty()) {
        let room = max.saturating_sub(used + 1);

        if room > 1 {
            text.push('\n');
            text.push_str(&escape_within(detail, room));
        }
    }

    if let Some(place) = place {
        text.push('\n');
        text.push_str(&place);
    }

    text.push('\n');
    text.push_str(&signature);

    text
}

fn signature() -> String {
    format!(
        "<a href=\"{}\">{}</a>",
        escape(crate::REPOSITORY_URL),
        escape(&sent_by())
    )
}

fn place(context: &EventContext, language: &str) -> Option<String> {
    let game = context
        .game
        .as_deref()
        .filter(|game| !game.is_empty())
        .map(|game| {
            let name = escape_within(game, 128);

            match category_url(context) {
                Some(url) => format!("<a href=\"{}\">{name}</a>", escape(&url)),
                None => name,
            }
        });
    let channel = channel_name(context).map(|name| {
        let name = escape_within(name, 64);
        let shown = match channel_url(context) {
            Some(url) => format!("<a href=\"{url}\">{name}</a>"),
            None => name,
        };

        i18n::text(language, "notify_streamer_line", &[("channel", &shown)])
    });
    let lines: Vec<String> = [game, channel].into_iter().flatten().collect();

    (!lines.is_empty()).then(|| lines.join("\n"))
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_within(text: &str, max: usize) -> String {
    let escaped = escape(text);

    if units(&escaped) <= max {
        return escaped;
    }

    let mut cut = String::new();
    let mut used = 0;
    let mut buffer = [0u8; 4];

    for ch in text.chars() {
        let piece = escape(ch.encode_utf8(&mut buffer));

        if used + units(&piece) + 1 > max {
            break;
        }

        used += units(&piece);
        cut.push_str(&piece);
    }

    cut.push('…');

    cut
}

fn units(text: &str) -> usize {
    text.encode_utf16().count()
}
