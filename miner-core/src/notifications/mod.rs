#[cfg(feature = "notify-rust")]
pub mod desktop;
mod discord;
pub mod images;
mod queue;
mod telegram;
mod webhook;

use std::sync::{Arc, OnceLock, RwLock};
use std::time::Duration;

use reqwest::header::RETRY_AFTER;
use reqwest::{Response, StatusCode};

use crate::api::NotificationTarget;
use crate::config::{NotificationEvents, NotificationSettings};
use crate::error::{Error, Result};
use crate::events::{EventContext, EventKind, MinerEvent};
use crate::i18n;
use crate::requests::RequestLog;
use crate::util::{read_locked, write_locked};
use queue::Queues;

const TEST_PICTURE: (&str, &[u8]) = (
    "logo.png",
    include_bytes!("../../assets/notify-test/logo.png"),
);

const TEST_SCHEME: &str = "test://";

const CATEGORY_URL: &str = "https://www.twitch.tv/directory/category/";
const SEARCH_URL: &str = "https://www.twitch.tv/search?term=";

const TIMEOUT: Duration = Duration::from_secs(15);
// ! Under the interface's own request timeout (30 s).
const TEST_WAIT: Duration = Duration::from_secs(20);

pub(crate) struct Endpoints {
    pub discord: &'static [&'static str],
    pub telegram: &'static str,
}

const OFFICIAL: Endpoints = Endpoints {
    discord: &[
        "https://discord.com/api/webhooks/",
        "https://discordapp.com/api/webhooks/",
    ],
    telegram: "https://api.telegram.org",
};

#[derive(Debug, Clone)]
pub(crate) struct Notice {
    pub event: MinerEvent,
    pub language: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Answer {
    pub status: StatusCode,
    pub retry_after: Option<Duration>,
    pub pause: Option<Duration>,
}

impl Answer {
    pub fn of(status: StatusCode) -> Self {
        Self {
            status,
            ..Self::default()
        }
    }
}

pub(crate) fn seconds(text: &str) -> Option<Duration> {
    text.trim()
        .parse::<f64>()
        .ok()
        .filter(|seconds| seconds.is_finite() && *seconds >= 0.0)
        .map(Duration::from_secs_f64)
}

pub(crate) fn retry_after_header(response: &Response) -> Option<Duration> {
    response
        .headers()
        .get(RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(seconds)
}

pub fn wanted(events: &NotificationEvents, kind: EventKind) -> bool {
    match kind {
        EventKind::DropClaimed | EventKind::CampaignFinished => events.drop_claimed,
        EventKind::DropProgress => events.drop_progress,
        EventKind::BonusClaimed => events.bonus_claimed,
        EventKind::PredictionPlaced => events.prediction_placed,
        EventKind::PredictionResult => events.prediction_result,
        EventKind::StreamOnline => events.stream_online,
        EventKind::StreamOffline => events.stream_offline,
        EventKind::RaidJoined => events.raid,
        EventKind::MomentClaimed => events.moment,
        EventKind::ChatMention => events.chat_mention,
        EventKind::SessionExpired => events.session_expired,
        EventKind::Error => events.errors,
        EventKind::UpdateAvailable => events.update_available,
        EventKind::Info
        | EventKind::WatchStarted
        | EventKind::WatchStopped
        | EventKind::PointsEarned
        | EventKind::PointsSpent
        | EventKind::StreakRecovered
        | EventKind::GoalContributed
        | EventKind::LoginCompleted
        | EventKind::RewardRedeemed => false,
    }
}

pub(crate) struct Shared {
    client: reqwest::Client,
    settings: RwLock<NotificationSettings>,
    language: RwLock<String>,
    requests: RequestLog,
    endpoints: &'static Endpoints,
    queues: OnceLock<Queues>,
}

impl Shared {
    fn settings(&self) -> NotificationSettings {
        read_locked(&self.settings).clone()
    }

    pub(crate) fn enabled(&self, target: NotificationTarget) -> bool {
        let settings = read_locked(&self.settings);

        match target {
            NotificationTarget::Discord => {
                settings.discord.enabled && !settings.discord.webhook_url.trim().is_empty()
            }
            NotificationTarget::Telegram => {
                settings.telegram.enabled && !settings.telegram.bot_token.trim().is_empty()
            }
            NotificationTarget::Webhook => {
                settings.webhook.enabled && !settings.webhook.url.trim().is_empty()
            }
        }
    }

    pub(crate) async fn send(
        &self,
        target: NotificationTarget,
        notices: &[Notice],
    ) -> Result<Answer> {
        let settings = self.settings();

        match target {
            NotificationTarget::Discord => discord::send(self, &settings.discord, notices).await,
            NotificationTarget::Telegram => {
                telegram::send(self, &settings.telegram, &notices[0]).await
            }
            NotificationTarget::Webhook => {
                webhook::send(self, &settings.webhook, &notices[0]).await
            }
        }
    }
}

#[derive(Clone)]
pub struct Notifier {
    shared: Arc<Shared>,
}

impl Notifier {
    pub fn new(
        settings: NotificationSettings,
        language: &str,
        requests: RequestLog,
    ) -> Result<Self> {
        Self::with_endpoints(settings, language, requests, &OFFICIAL)
    }

    pub(crate) fn with_endpoints(
        settings: NotificationSettings,
        language: &str,
        requests: RequestLog,
        endpoints: &'static Endpoints,
    ) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(TIMEOUT)
            .user_agent(format!("twitch-drops-points-miner/{}", crate::VERSION))
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(Self {
            shared: Arc::new(Shared {
                client,
                settings: RwLock::new(settings),
                language: RwLock::new(language.to_owned()),
                requests,
                endpoints,
                queues: OnceLock::new(),
            }),
        })
    }

    pub fn update(&self, settings: NotificationSettings, language: &str) {
        *write_locked(&self.shared.settings) = settings;
        *write_locked(&self.shared.language) = language.to_owned();
    }

    fn language(&self) -> &'static str {
        i18n::resolve(&read_locked(&self.shared.language))
    }

    fn queues(&self) -> &Queues {
        self.shared
            .queues
            .get_or_init(|| Queues::start(&self.shared))
    }

    pub fn notify(&self, event: MinerEvent) {
        let wanted = wanted(&read_locked(&self.shared.settings).events, event.kind);

        if !wanted {
            return;
        }

        let language = self.language();
        let notice = Notice {
            event: redacted(localized(event, language)),
            language,
        };
        let queues = self.queues();

        for target in queue::TARGETS {
            if self.shared.enabled(target) {
                queues.push(target, notice.clone());
            }
        }
    }

    // ! A test waits its turn in the target's queue, so it never takes the 429
    // of a burst being sent.
    pub async fn test(&self) -> Vec<(NotificationTarget, Result<()>)> {
        let settings = self.shared.settings();
        let language = self.language();
        let notice = Notice {
            event: sample_in(language),
            language,
        };
        let queues = self.queues();
        let deadline = tokio::time::Instant::now() + TEST_WAIT;
        let waiting: Vec<_> = queue::TARGETS
            .into_iter()
            .filter(|target| match target {
                NotificationTarget::Discord => settings.discord.enabled,
                NotificationTarget::Telegram => settings.telegram.enabled,
                NotificationTarget::Webhook => settings.webhook.enabled,
            })
            .map(|target| (target, queues.test(target, notice.clone())))
            .collect();
        let mut results = Vec::new();

        for (target, answer) in waiting {
            let outcome = match tokio::time::timeout_at(deadline, answer).await {
                Ok(Ok(outcome)) => outcome,
                Ok(Err(_)) => Err(Error::Notification(format!(
                    "the {} queue is full",
                    queue::name(target)
                ))),
                Err(_) => Err(Error::Notification(format!(
                    "{} is still sending earlier notifications",
                    queue::name(target)
                ))),
            };

            results.push((target, outcome));
        }

        results
    }

    pub fn shutdown(&self) {
        let Some(queues) = self.shared.queues.get() else {
            return;
        };

        let dropped = queues.shutdown();

        if dropped > 0 {
            tracing::warn!(
                "{dropped} {} dropped at shutdown",
                if dropped == 1 {
                    "notification"
                } else {
                    "notifications"
                }
            );
        }
    }
}

pub fn sample_in(language: &str) -> MinerEvent {
    let name = |key: &str| i18n::text(language, key, &[]);
    let logo = format!("{TEST_SCHEME}{}", TEST_PICTURE.0);
    let event = MinerEvent::new("", "miner", EventKind::DropClaimed, "")
        .detail(i18n::text(language, "notify_test_detail", &[]))
        .context(EventContext {
            account_name: Some(name("notify_test_account")),
            account_avatar: Some(logo.clone()),
            channel_name: Some(name("notify_test_streamer")),
            avatar: Some(logo.clone()),
            campaign_name: Some(name("notify_test_campaign")),
            game: Some(name("notify_test_game")),
            art: Some(logo.clone()),
            drop_name: Some(name("notify_test_drop")),
            reward: Some(logo),
            ..EventContext::default()
        });
    let title = i18n::text(
        language,
        "notify_test_title",
        &[("text", &i18n::event_text(language, &event))],
    );

    MinerEvent { title, ..event }
}

pub fn sample() -> MinerEvent {
    sample_in(i18n::FALLBACK)
}

pub fn localized(mut event: MinerEvent, language: &str) -> MinerEvent {
    event.title = i18n::event_text(language, &event);

    event
}

pub fn picture(context: &EventContext) -> Option<&str> {
    shown(context.reward.as_deref())
        .or_else(|| shown(context.avatar.as_deref()))
        .or_else(|| shown(context.art.as_deref()))
}

pub(crate) fn cdn(url: Option<&str>) -> Option<&str> {
    url.filter(|url| images::is_twitch_cdn(url))
}

pub(crate) fn shown(url: Option<&str>) -> Option<&str> {
    url.filter(|url| images::is_twitch_cdn(url) || test_picture(url).is_some())
}

pub(crate) fn test_picture(url: &str) -> Option<(&'static str, &'static [u8])> {
    (url.strip_prefix(TEST_SCHEME)? == TEST_PICTURE.0).then_some(TEST_PICTURE)
}

pub(crate) fn sent_by() -> String {
    format!("{} • v{}", crate::APP_NAME, crate::VERSION)
}

pub(crate) fn category_url(context: &EventContext) -> Option<String> {
    if context
        .art
        .as_deref()
        .is_some_and(|art| test_picture(art).is_some())
    {
        return None;
    }

    if let Some(slug) = context.game_slug.as_deref().filter(|slug| !slug.is_empty()) {
        return Some(format!("{CATEGORY_URL}{slug}"));
    }

    let game = context.game.as_deref().filter(|game| !game.is_empty())?;
    let slug = crate::util::game_slug(game);

    Some(if slug.is_empty() {
        let term: String = url::form_urlencoded::byte_serialize(game.as_bytes()).collect();

        format!("{SEARCH_URL}{term}")
    } else {
        format!("{CATEGORY_URL}{slug}")
    })
}

pub(crate) fn channel_url(context: &EventContext) -> Option<String> {
    let login = crate::util::normalize_login(context.channel.as_deref()?)?;

    Some(format!("https://www.twitch.tv/{login}"))
}

pub(crate) fn channel_name(context: &EventContext) -> Option<&str> {
    context
        .channel_name
        .as_deref()
        .or(context.channel.as_deref())
        .filter(|name| !name.is_empty())
}

pub(crate) fn redacted(mut event: MinerEvent) -> MinerEvent {
    event.title = crate::logs::redact(&event.title);
    event.detail = event.detail.as_deref().map(crate::logs::redact);

    event
}

// ! Discord and Telegram count lengths in UTF-16 units.
pub(crate) fn clip(text: &str, max: usize) -> String {
    if text.encode_utf16().count() <= max {
        return text.to_owned();
    }

    let mut clipped = String::new();
    let mut used = 0;

    for ch in text.chars() {
        if used + ch.len_utf16() + 1 > max {
            break;
        }

        used += ch.len_utf16();
        clipped.push(ch);
    }

    clipped.push('…');

    clipped
}

pub(crate) fn color_for(kind: EventKind) -> u32 {
    match kind {
        EventKind::DropClaimed | EventKind::CampaignFinished => 0x9146ff,
        EventKind::BonusClaimed | EventKind::PointsEarned => 0x00f593,
        EventKind::PredictionPlaced | EventKind::ChatMention => 0x2b9bff,
        EventKind::PredictionResult => 0xffb86b,
        EventKind::Error | EventKind::SessionExpired => 0xeb0400,
        EventKind::StreamOnline => 0x00f593,
        EventKind::StreamOffline => 0x53535f,
        _ => 0xadadb8,
    }
}

pub(crate) fn label_for(language: &str, kind: EventKind) -> String {
    i18n::kind_label(language, kind)
}

pub(crate) fn account_name(event: &MinerEvent) -> Option<&str> {
    event
        .context
        .account_name
        .as_deref()
        .or(Some(event.account_login.as_str()))
        .filter(|name| !name.is_empty())
}
