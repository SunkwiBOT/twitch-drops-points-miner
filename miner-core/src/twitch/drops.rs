use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use super::operations::Operation;
use super::{note_left_out, text, values, TwitchClient};
use crate::domain::{Benefit, BenefitKind, Campaign, CampaignStatus, ChannelRef, Game, TimedDrop};
use crate::error::{Error, Result};

pub struct Inventory {
    pub campaigns: Vec<Campaign>,
    pub awards: HashMap<String, DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimOutcome {
    Claimed,
    AlreadyClaimed,
    Refused(String),
}

impl TwitchClient {
    pub async fn inventory(&self) -> Result<Inventory> {
        let response = self.gql(Operation::Inventory, Value::Null).await?;
        let inventory = response
            .pointer("/data/currentUser/inventory")
            .filter(|v| v.is_object())
            .ok_or_else(|| Error::InvalidResponse("currentUser.inventory missing".into()))?;
        let awards = values(&inventory["gameEventDrops"])
            .filter_map(|v| {
                Some((
                    text(&v["id"])?,
                    text(&v["lastAwardedAt"])?.parse::<DateTime<Utc>>().ok()?,
                ))
            })
            .collect();
        let records: Vec<&Value> = values(&inventory["dropCampaignsInProgress"]).collect();
        let campaigns: Vec<Campaign> = records
            .iter()
            .filter_map(|record| parse_campaign(record, true))
            .collect();

        note_left_out("inventory campaigns", records.len(), campaigns.len());

        Ok(Inventory { campaigns, awards })
    }

    pub async fn claim_drop(&self, instance_id: &str) -> Result<ClaimOutcome> {
        let response = self
            .gql(
                Operation::ClaimDrop,
                json!({"input": {"dropInstanceID": instance_id}}),
            )
            .await?;

        Ok(
            match response["data"]["claimDropRewards"]["status"].as_str() {
                Some("ELIGIBLE_FOR_ALL") => ClaimOutcome::Claimed,
                Some("DROP_INSTANCE_ALREADY_CLAIMED") => ClaimOutcome::AlreadyClaimed,
                Some(other) => ClaimOutcome::Refused(other.to_owned()),
                None => ClaimOutcome::Refused("unknown".to_owned()),
            },
        )
    }
}

fn parse_time(value: &Value) -> Option<DateTime<Utc>> {
    text(value)?.parse().ok()
}

fn parse_status(value: &Value) -> CampaignStatus {
    match value.as_str().unwrap_or("ACTIVE") {
        "UPCOMING" => CampaignStatus::Upcoming,
        "EXPIRED" => CampaignStatus::Expired,
        "DISABLED" => CampaignStatus::Disabled,
        _ => CampaignStatus::Active,
    }
}

pub fn parse_campaign(record: &Value, from_inventory: bool) -> Option<Campaign> {
    let id = text(&record["id"])?;
    let game_node = &record["game"];
    let game = Game {
        id: text(&game_node["id"]).unwrap_or_default(),
        name: text(&game_node["displayName"]).or_else(|| text(&game_node["name"]))?,
        slug: text(&game_node["slug"]),
        box_art_url: text(&game_node["boxArtURL"]),
    };

    // ! An allow list Twitch switched off ("isEnabled": false) limits nothing.
    let allow_enabled = record["allow"]["isEnabled"].as_bool().unwrap_or(true);
    let allowed_channels = values(&record["allow"]["channels"])
        .filter(|_| allow_enabled)
        .filter_map(|c| {
            Some(ChannelRef {
                id: text(&c["id"])?,
                login: text(&c["name"])
                    .or_else(|| text(&c["login"]))
                    .unwrap_or_default(),
                display_name: text(&c["displayName"]).unwrap_or_default(),
            })
        })
        .collect();

    let drops = values(&record["timeBasedDrops"])
        .filter_map(|d| {
            let me = &d["self"];

            Some(TimedDrop {
                id: text(&d["id"])?,
                name: text(&d["name"]).unwrap_or_default(),
                starts_at: parse_time(&d["startAt"])?,
                ends_at: parse_time(&d["endAt"])?,
                required_minutes: d["requiredMinutesWatched"].as_u64().unwrap_or(0) as u32,
                required_subs: d["requiredSubs"].as_u64().unwrap_or(0) as u32,
                current_minutes: me["currentMinutesWatched"].as_u64().unwrap_or(0) as u32,
                instance_id: text(&me["dropInstanceID"]),
                claimed: me["isClaimed"].as_bool().unwrap_or(false),
                preconditions_met: me["hasPreconditionsMet"].as_bool(),
                precondition_drop_ids: values(&d["preconditionDrops"])
                    .filter_map(|p| text(&p["id"]))
                    .collect(),
                benefits: values(&d["benefitEdges"])
                    .filter_map(|edge| {
                        let benefit = &edge["benefit"];

                        Some(Benefit {
                            id: text(&benefit["id"])?,
                            name: text(&benefit["name"]).unwrap_or_default(),
                            kind: BenefitKind::from_twitch(benefit["distributionType"].as_str()),
                            image_url: text(&benefit["imageAssetURL"]),
                        })
                    })
                    .collect(),
            })
        })
        .collect();

    Some(Campaign {
        id,
        name: text(&record["name"]).unwrap_or_default(),
        game,
        status: parse_status(&record["status"]),
        linked: record["self"]["isAccountConnected"].as_bool(),
        link_url: text(&record["accountLinkURL"]),
        image_url: text(&record["imageURL"]),
        starts_at: parse_time(&record["startAt"])?,
        ends_at: parse_time(&record["endAt"])?,
        allowed_channels,
        drops,
        in_inventory: from_inventory,
    })
}
