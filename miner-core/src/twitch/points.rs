use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use super::operations::Operation;
use super::{text, values, TwitchClient};
use crate::domain::WatchStreak;
use crate::error::{Error, Result};
use crate::util::random_hex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointsContext {
    pub channel_id: String,
    pub enabled: bool,
    pub balance: i64,
    pub available_claim_id: Option<String>,
    pub subscribed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub amount_needed: i64,
    pub points_contributed: i64,
    pub per_stream_user_maximum: i64,
    pub status: String,
    pub in_stock: bool,
}

impl Goal {
    pub fn open(&self) -> bool {
        self.status == "STARTED" && self.in_stock
    }
}

impl TwitchClient {
    pub async fn points_context(&self, login: &str) -> Result<Option<PointsContext>> {
        let response = self
            .gql(
                Operation::ChannelPointsContext,
                json!({"channelLogin": login}),
            )
            .await?;
        let channel = &response["data"]["community"]["channel"];

        if channel.is_null() {
            return Ok(None);
        }

        let points = &channel["self"]["communityPoints"];

        Ok(Some(PointsContext {
            channel_id: text(&channel["id"]).unwrap_or_default(),
            enabled: channel["communityPointsSettings"]["isEnabled"]
                .as_bool()
                .unwrap_or(true),
            balance: points["balance"].as_i64().unwrap_or(0),
            available_claim_id: text(&points["availableClaim"]["id"]),
            subscribed: subscribed(&points["activeMultipliers"]),
        }))
    }

    pub async fn community_goals(&self, login: &str) -> Result<Vec<Goal>> {
        let response = self
            .gql(
                Operation::ChannelPointsContext,
                json!({"channelLogin": login}),
            )
            .await?;

        Ok(
            values(&response["data"]["community"]["channel"]["communityPointsSettings"]["goals"])
                .filter_map(|g| {
                    Some(Goal {
                        id: text(&g["id"])?,
                        title: text(&g["title"]).unwrap_or_default(),
                        amount_needed: g["amountNeeded"]
                            .as_i64()
                            .or_else(|| g["goalAmount"].as_i64())
                            .unwrap_or(0),
                        points_contributed: g["pointsContributed"].as_i64().unwrap_or(0),
                        per_stream_user_maximum: g["perStreamUserMaximumContribution"]
                            .as_i64()
                            .unwrap_or(0),
                        status: text(&g["status"]).unwrap_or_default(),
                        in_stock: g["isInStock"].as_bool().unwrap_or(true),
                    })
                })
                .collect(),
        )
    }

    pub async fn goal_contributions(&self, login: &str) -> Result<HashMap<String, i64>> {
        let response = self
            .gql(
                Operation::UserPointsContribution,
                json!({"channelLogin": login}),
            )
            .await?;

        Ok(values(
            &response["data"]["user"]["channel"]["self"]["communityPoints"]["goalContributions"],
        )
        .filter_map(|c| {
            Some((
                text(&c["goal"]["id"])?,
                c["userPointsContributedThisStream"].as_i64().unwrap_or(0),
            ))
        })
        .collect())
    }

    pub async fn claim_bonus(&self, channel_id: &str, claim_id: &str) -> Result<i64> {
        let operation = Operation::ClaimCommunityPoints;
        let response = self
            .gql(
                operation,
                json!({"input": {"channelID": channel_id, "claimID": claim_id}}),
            )
            .await?;
        let claim = &response["data"]["claimCommunityPoints"];

        refused(operation, &claim["error"])?;

        Ok(claim["currentPoints"].as_i64().unwrap_or(0))
    }

    pub async fn claim_moment(&self, moment_id: &str) -> Result<()> {
        let operation = Operation::ClaimMoment;
        let response = self
            .gql(operation, json!({"input": {"momentID": moment_id}}))
            .await?;

        refused(
            operation,
            &response["data"]["claimCommunityMoment"]["error"],
        )
    }

    pub async fn join_raid(&self, raid_id: &str) -> Result<()> {
        self.gql(Operation::JoinRaid, json!({"input": {"raidID": raid_id}}))
            .await?;

        Ok(())
    }

    pub async fn make_prediction(
        &self,
        event_id: &str,
        outcome_id: &str,
        points: i64,
    ) -> Result<()> {
        let operation = Operation::MakePrediction;
        let response = self
            .gql(
                operation,
                json!({"input": {
                    "eventID": event_id, "outcomeID": outcome_id, "points": points,
                    "transactionID": random_hex(16)
                }}),
            )
            .await?;

        refused(operation, &response["data"]["makePrediction"]["error"])
    }

    pub async fn contribute_goal(
        &self,
        channel_id: &str,
        goal_id: &str,
        amount: i64,
    ) -> Result<()> {
        let operation = Operation::ContributeCommunityGoal;
        let response = self
            .gql(
                operation,
                json!({"input": {
                    "amount": amount, "channelID": channel_id, "goalID": goal_id,
                    "transactionID": random_hex(16)
                }}),
            )
            .await?;

        refused(
            operation,
            &response["data"]["contributeCommunityPointsCommunityGoal"]["error"],
        )
    }

    pub async fn watch_streak(&self, login: &str) -> Result<Option<WatchStreak>> {
        let response = self
            .gql(Operation::RewardList, json!({"channelLogin": login}))
            .await?;
        let milestone = &response["data"]["channel"]["self"]["watchStreakMilestone"];

        if milestone.is_null() {
            return Ok(None);
        }

        Ok(Some(parse_streak(milestone)))
    }
}

// ! A mutation answers its refusal in the data, with status 200.
fn refused(operation: Operation, error: &Value) -> Result<()> {
    match text(&error["code"]) {
        Some(code) => Err(Error::Gql {
            operation: operation.name(),
            code,
        }),
        None => Ok(()),
    }
}

// ! Twitch lists a multiplier above 1 for each subscription tier (×1.2 to ×2).
fn subscribed(multipliers: &Value) -> bool {
    values(multipliers).any(|multiplier| {
        multiplier["factor"]
            .as_f64()
            .is_some_and(|factor| factor > 1.0)
    })
}

fn parse_streak(milestone: &Value) -> WatchStreak {
    WatchStreak {
        count: milestone["currentStreak"]
            .as_u64()
            .or_else(|| milestone["streakCount"].as_u64())
            .unwrap_or(0) as u32,
        missed_streams: milestone["missedStreams"]
            .as_array()
            .map(|a| a.len() as u32)
            .or_else(|| milestone["missedStreamsCount"].as_u64().map(|v| v as u32))
            .unwrap_or(0),
        expires_at: text(&milestone["expiresAt"]).and_then(|v| v.parse::<DateTime<Utc>>().ok()),
    }
}
