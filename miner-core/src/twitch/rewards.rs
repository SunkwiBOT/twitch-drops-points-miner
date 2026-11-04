use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use super::operations::Operation;
use super::{text, values, TwitchClient};
use crate::error::{Error, Result};
use crate::spam::SpamReward;
use crate::util::random_hex;

pub const DEFAULT_REWARD_IMAGE: &str =
    "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardsContext {
    pub channel_id: String,
    pub balance: i64,
    pub rewards: Vec<SpamReward>,
}

impl TwitchClient {
    pub async fn custom_rewards(&self, login: &str) -> Result<Option<RewardsContext>> {
        let response = self
            .gql(Operation::CustomRewards, json!({ "channelLogin": login }))
            .await?;

        rewards_context(&response)
    }

    // ! `None` is a redemption Twitch took. It answers 200 even when it refuses
    // one: the reason is the mutation's `error.code` (NOT_ENOUGH_POINTS…).
    pub async fn redeem_reward(
        &self,
        channel_id: &str,
        reward: &SpamReward,
        text: Option<&str>,
        leaving: impl FnOnce() -> bool,
    ) -> Result<Option<String>> {
        let mut input = json!({
            "channelID": channel_id,
            "cost": reward.cost,
            "prompt": reward.prompt,
            "rewardID": reward.id,
            "title": reward.title,
            "transactionID": transaction_id(),
        });

        if let Some(text) = text {
            input["textInput"] = json!(text);
        }

        let response = self
            .gql_once(
                Operation::RedeemCustomReward,
                json!({ "input": input }),
                leaving,
            )
            .await?;

        redemption_answer(&response)
    }
}

// ! A login no channel has comes back as a null channel without any error.
pub fn rewards_context(response: &Value) -> Result<Option<RewardsContext>> {
    let channel = &response["data"]["community"]["channel"];
    let Some(channel_id) = text(&channel["id"]) else {
        return gql_error(Operation::CustomRewards, response).map_or(Ok(None), Err);
    };
    let mut rewards: Vec<SpamReward> = values(&channel["communityPointsSettings"]["customRewards"])
        .filter_map(|item| {
            let reward = custom_reward(item);

            if reward.is_none() {
                tracing::debug!("a custom reward without an id or a cost was left out");
            }

            reward
        })
        .collect();

    rewards.sort_by_key(|reward| reward.cost);

    let balance = match channel["self"]["communityPoints"]["balance"].as_i64() {
        Some(balance) => balance,
        // ! A channel without channel points has no balance and nothing to
        // redeem; rewards without the balance would all read as too dear.
        None if rewards.is_empty() => 0,
        None => {
            return Err(Error::InvalidResponse(
                "the channel's rewards came without the viewer's balance".into(),
            ))
        }
    };

    Ok(Some(RewardsContext {
        channel_id,
        balance,
        rewards,
    }))
}

fn custom_reward(item: &Value) -> Option<SpamReward> {
    let cooldown = &item["globalCooldownSetting"];

    Some(SpamReward {
        id: text(&item["id"])?,
        title: text(&item["title"]).unwrap_or_default(),
        prompt: text(&item["prompt"]),
        cost: item["cost"].as_i64().filter(|cost| *cost >= 0)?,
        background_color: text(&item["backgroundColor"]).filter(|color| hex_color(color)),
        image_url: image(&item["image"])
            .or_else(|| image(&item["defaultImage"]))
            .unwrap_or_else(|| DEFAULT_REWARD_IMAGE.to_owned()),
        is_enabled: item["isEnabled"].as_bool().unwrap_or(true),
        is_paused: item["isPaused"].as_bool().unwrap_or(false),
        is_in_stock: item["isInStock"].as_bool().unwrap_or(true),
        is_user_input_required: item["isUserInputRequired"].as_bool().unwrap_or(false),
        cooldown_expires_at: item["cooldownExpiresAt"]
            .as_str()
            .and_then(|at| at.parse::<DateTime<Utc>>().ok()),
        cooldown_seconds: cooldown["isEnabled"]
            .as_bool()
            .filter(|enabled| *enabled)
            .and_then(|_| cooldown["globalCooldownSeconds"].as_u64())
            .and_then(|seconds| u32::try_from(seconds).ok())
            .filter(|seconds| *seconds > 0),
    })
}

fn image(value: &Value) -> Option<String> {
    ["url4x", "url2x", "url"]
        .into_iter()
        .find_map(|size| text(&value[size]))
}

// ! The color is the streamer's own and goes into a style: only a plain hex
// color passes.
fn hex_color(color: &str) -> bool {
    color
        .strip_prefix('#')
        .is_some_and(|hex| matches!(hex.len(), 3 | 6) && hex.bytes().all(|b| b.is_ascii_hexdigit()))
}

fn gql_error(operation: Operation, response: &Value) -> Option<Error> {
    values(&response["errors"]).next().map(|error| Error::Gql {
        operation: operation.name(),
        code: text(&error["message"]).unwrap_or_default(),
    })
}

pub fn redemption_answer(response: &Value) -> Result<Option<String>> {
    if let Some(error) = gql_error(Operation::RedeemCustomReward, response) {
        return Err(error);
    }

    let Some(result) = response["data"]
        .as_object()
        .and_then(|data| data.values().next())
        .filter(|result| result.is_object())
    else {
        return Err(Error::InvalidResponse(
            "RedeemCustomReward answered without a result".to_owned(),
        ));
    };

    Ok(text(&result["error"]["code"]))
}

fn transaction_id() -> String {
    let hex = random_hex(16);

    format!(
        "{}-{}-4{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[13..16],
        &hex[16..20],
        &hex[20..32]
    )
}
