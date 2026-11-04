use reqwest::StatusCode;
use serde_json::{json, Value};

use super::{category_url, cdn, label_for, picture, retry_after_header, Answer, Notice, Shared};
use crate::config::WebhookSettings;
use crate::error::{Error, Result};
use crate::events::MinerEvent;
use crate::requests::RequestKind;

// ! The user's own address can name them: the requests list shows no target.
const TARGET: &str = "";

pub(super) async fn send(
    shared: &Shared,
    settings: &WebhookSettings,
    notice: &Notice,
) -> Result<Answer> {
    let url = settings.url.trim();

    if !url.starts_with("https://") && !url.starts_with("http://") {
        return Err(Error::Notification("webhook URL must be http(s)".into()));
    }

    let event = &notice.event;
    let mut request = shared.client.post(url).json(&body(event, notice.language));

    if !settings.authorization.trim().is_empty() {
        request = request.header("Authorization", settings.authorization.trim());
    }

    let response = shared
        .requests
        .start(RequestKind::Webhook, TARGET)
        .account(Some(event.account_id.as_str()).filter(|id| !id.is_empty()))
        .send(request)
        .await?;
    let status = response.status();
    let retry_after = if status == StatusCode::TOO_MANY_REQUESTS {
        retry_after_header(&response)
    } else {
        None
    };

    Ok(Answer {
        status,
        retry_after,
        pause: None,
    })
}

fn body(event: &MinerEvent, language: &str) -> Value {
    json!({
        "app": {
            "name": crate::APP_NAME,
            "version": crate::VERSION,
            "url": crate::REPOSITORY_URL,
        },
        "event": event,
        "label": label_for(language, event.kind),
        "context": event.context,
        "image_url": cdn(picture(&event.context)),
        "category_url": category_url(&event.context),
    })
}
