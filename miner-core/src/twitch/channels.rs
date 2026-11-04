use std::collections::HashMap;

use serde_json::{json, Value};

use super::operations::{directory_variables, Operation};
use super::{note_left_out, text, values, TwitchClient, TwitchHttp};
use crate::domain::{Channel, Game};
use crate::error::{Error, Result};

pub const DIRECTORY_LIMIT: usize = 30;

const LOOKUP_BATCH: usize = 20;

impl TwitchClient {
    pub async fn channel(&self, login: &str) -> Result<Option<Channel>> {
        let response = self
            .gql(Operation::StreamInfo, json!({"channel": login}))
            .await?;

        read_one(login, &response)
    }

    pub async fn channels(&self, logins: &[String]) -> Result<Vec<Channel>> {
        let mut out = Vec::with_capacity(logins.len());

        for chunk in logins.chunks(LOOKUP_BATCH) {
            let response = self
                .gql_batch(
                    Operation::StreamInfo,
                    chunk.iter().map(|login| json!({"channel": login})),
                )
                .await?;
            let items = response
                .as_array()
                .ok_or_else(|| Error::InvalidResponse("batch must be an array".into()))?;
            let read = read_batch(chunk, items);

            if !read.unchecked.is_empty() {
                tracing::debug!(
                    count = read.unchecked.len(),
                    "channels left unread in a batch"
                );
            }

            out.extend(read.channels);
        }

        Ok(out)
    }

    // ! Asked with the account's token: without it, Twitch leaves out the
    // campaigns linked to an account.
    pub async fn available_drops(
        &self,
        channel_ids: &[String],
    ) -> Result<HashMap<String, Vec<String>>> {
        let mut found = HashMap::new();

        for chunk in channel_ids.chunks(LOOKUP_BATCH) {
            let response = self
                .gql_batch(
                    Operation::AvailableDrops,
                    chunk.iter().map(|id| json!({"channelID": id})),
                )
                .await?;
            let items = response
                .as_array()
                .ok_or_else(|| Error::InvalidResponse("batch must be an array".into()))?;

            let before = found.len();

            found.extend(items.iter().filter_map(parse_available_drops));
            note_left_out("available drops", items.len(), found.len() - before);
        }

        Ok(found)
    }

    // ! Twitch leaves names and pictures out of the follow list: they are looked
    // up in batches.
    pub async fn followed_channels(&self, max: usize) -> Result<Vec<Channel>> {
        let mut found = self.follows(max).await?;
        let bare: Vec<String> = found
            .iter()
            .filter(|channel| channel.avatar_url.is_none())
            .map(|channel| channel.login.clone())
            .collect();

        if !bare.is_empty() {
            let mut looked: HashMap<String, Channel> = self
                .channels(&bare)
                .await?
                .into_iter()
                .map(|channel| (channel.login.clone(), channel))
                .collect();

            for channel in &mut found {
                if let Some(full) = looked.remove(&channel.login) {
                    *channel = full;
                }
            }
        }

        Ok(found)
    }

    pub async fn follows(&self, max: usize) -> Result<Vec<Channel>> {
        let mut found: Vec<Channel> = Vec::new();
        let mut cursor = String::new();

        loop {
            let response = self
                .gql(Operation::ChannelFollows, json!({"cursor": cursor}))
                .await?;
            let follows = &response["data"]["user"]["follows"];

            if follows.is_null() {
                return Err(Error::InvalidResponse("follows".into()));
            }

            let mut last = None;

            let mut edges = 0;
            let before = found.len();

            for edge in values(&follows["edges"]) {
                edges += 1;

                if let Some(channel) = parse_follow(&edge["node"]) {
                    found.push(channel);
                }

                last = text(&edge["cursor"]);
            }

            note_left_out("follows", edges, found.len() - before);

            let more = follows["pageInfo"]["hasNextPage"]
                .as_bool()
                .unwrap_or(false);

            match last {
                Some(next) if more && found.len() < max => cursor = next,
                _ => break,
            }
        }

        found.truncate(max);

        Ok(found)
    }

    pub async fn game_directory(&self, game: &Game) -> Result<Vec<Channel>> {
        let response = self
            .gql(
                Operation::GameDirectory,
                directory_variables(&game.slug(), DIRECTORY_LIMIT),
            )
            .await?;
        let game_node = &response["data"]["game"];

        if game_node.is_null() {
            return Err(Error::InvalidResponse(
                "the directory has no such game".into(),
            ));
        }

        let resolved = Game {
            id: text(&game_node["id"]).unwrap_or_else(|| game.id.clone()),
            name: text(&game_node["displayName"])
                .or_else(|| text(&game_node["name"]))
                .unwrap_or_else(|| game.name.clone()),
            slug: text(&game_node["slug"]).or_else(|| game.slug.clone()),
            box_art_url: game.box_art_url.clone(),
        };

        let edges: Vec<&Value> = values(&game_node["streams"]["edges"]).collect();
        let channels: Vec<Channel> = edges
            .iter()
            .filter_map(|edge| parse_directory_node(&edge["node"], &resolved))
            .collect();

        note_left_out("directory channels", edges.len(), channels.len());

        Ok(channels)
    }
}

pub async fn lookup_public(http: &TwitchHttp, login: &str) -> Result<Option<Channel>> {
    let body = http
        .post_gql(
            None,
            Operation::StreamInfo.name(),
            None,
            &Operation::StreamInfo.request(json!({"channel": login})),
        )
        .await?;

    read_one(login, &body)
}

fn read_one(login: &str, body: &Value) -> Result<Option<Channel>> {
    let read = read_batch(&[login.to_owned()], std::slice::from_ref(body));

    if !read.unchecked.is_empty() {
        return Err(Error::InvalidResponse(
            "Twitch did not read the channel".into(),
        ));
    }

    Ok(read.channels.into_iter().next())
}

#[derive(Debug, Default, PartialEq)]
pub struct BatchRead {
    pub channels: Vec<Channel>,
    pub unchecked: Vec<String>,
}

// ! One item of a batch answer can carry an error of its own, with `user` null:
// that channel was not read, which says nothing about whether it exists.
pub(crate) fn read_batch(logins: &[String], items: &[Value]) -> BatchRead {
    let mut read = BatchRead::default();

    for (index, login) in logins.iter().enumerate() {
        let Some(item) = items.get(index) else {
            read.unchecked.push(login.clone());
            continue;
        };
        let user = &item["data"]["user"];
        let failed = item["errors"]
            .as_array()
            .is_some_and(|errors| !errors.is_empty())
            || item["data"].is_null();

        if !user.is_null() {
            read.channels.push(parse_user(user, login));
        } else if failed {
            read.unchecked.push(login.clone());
        }
    }

    read
}

pub async fn lookup_public_many(http: &TwitchHttp, logins: &[String]) -> Result<BatchRead> {
    let mut out = BatchRead::default();

    for chunk in logins.chunks(LOOKUP_BATCH) {
        let batch: Vec<Value> = chunk
            .iter()
            .map(|login| Operation::StreamInfo.request(json!({"channel": login})))
            .collect();
        let body = http
            .post_gql(
                None,
                Operation::StreamInfo.name(),
                None,
                &Value::Array(batch),
            )
            .await?;
        let items = body
            .as_array()
            .ok_or_else(|| Error::InvalidResponse("batch must be an array".into()))?;
        let read = read_batch(chunk, items);

        out.channels.extend(read.channels);
        out.unchecked.extend(read.unchecked);
    }

    Ok(out)
}

fn parse_available_drops(item: &Value) -> Option<(String, Vec<String>)> {
    let channel = &item["data"]["channel"];
    let id = text(&channel["id"])?;
    let campaigns = values(&channel["viewerDropCampaigns"])
        .filter_map(|campaign| text(&campaign["id"]))
        .collect();

    Some((id, campaigns))
}

fn parse_follow(node: &Value) -> Option<Channel> {
    let login = text(&node["login"])?;

    Some(Channel {
        id: text(&node["id"]).unwrap_or_default(),
        display_name: text(&node["displayName"]).unwrap_or_else(|| login.clone()),
        avatar_url: text(&node["profileImageURL"]),
        login,
        ..Channel::default()
    })
}

fn parse_game(value: &Value) -> Option<Game> {
    let id = text(&value["id"])?;

    Some(Game {
        id,
        name: text(&value["displayName"]).or_else(|| text(&value["name"]))?,
        slug: text(&value["slug"]),
        box_art_url: text(&value["boxArtURL"]),
    })
}

fn parse_user(user: &Value, login: &str) -> Channel {
    let stream = &user["stream"];
    let live = text(&stream["id"]).is_some();

    Channel {
        id: text(&user["id"]).unwrap_or_default(),
        login: text(&user["login"]).unwrap_or_else(|| login.to_owned()),
        display_name: text(&user["displayName"]).unwrap_or_else(|| login.to_owned()),
        avatar_url: text(&user["profileImageURL"]),
        live,
        broadcast_id: text(&stream["id"]),
        viewers: stream["viewersCount"].as_u64(),
        game: if live {
            parse_game(&stream["game"]).or_else(|| parse_game(&user["broadcastSettings"]["game"]))
        } else {
            None
        },
        drops_enabled: false,
    }
}

fn parse_directory_node(node: &Value, game: &Game) -> Option<Channel> {
    let broadcaster = &node["broadcaster"];
    let login = text(&broadcaster["login"])?;

    Some(Channel {
        id: text(&broadcaster["id"])?,
        display_name: text(&broadcaster["displayName"]).unwrap_or_else(|| login.clone()),
        login,
        avatar_url: text(&broadcaster["profileImageURL"]),
        live: true,
        broadcast_id: text(&node["id"]),
        viewers: node["viewersCount"].as_u64(),
        game: Some(parse_game(&node["game"]).unwrap_or_else(|| game.clone())),
        drops_enabled: true,
    })
}

// ! One aliased root field per login, and Twitch refuses a query with more
// than 15 of them.
const BANNER_BATCH: usize = 15;

const BANNERS_QUERY: &str = "ProfileBanners";

// ! Logins must already be valid (`normalize_login`): they are written into
// the query.
pub async fn banners(
    http: &TwitchHttp,
    logins: &[String],
) -> Result<Vec<(String, Option<String>)>> {
    let mut out = Vec::new();
    let mut failure = None;

    for chunk in logins.chunks(BANNER_BATCH) {
        match banner_batch(http, chunk).await {
            Ok(found) => out.extend(chunk.iter().map(|login| {
                let banner = found
                    .iter()
                    .find(|(known, _)| known == login)
                    .map(|(_, url)| url.clone());

                (login.clone(), banner)
            })),
            Err(error) => {
                tracing::debug!(%error, logins = chunk.len(), "banners not read");
                failure = Some(error);
            }
        }
    }

    match failure {
        Some(error) if out.is_empty() => Err(error),
        _ => Ok(out),
    }
}

async fn banner_batch(http: &TwitchHttp, chunk: &[String]) -> Result<Vec<(String, String)>> {
    let fields: String = chunk
        .iter()
        .enumerate()
        .map(|(index, login)| {
            format!("u{index}: user(login: \"{login}\") {{ login bannerImageURL }} ")
        })
        .collect();

    let body = http
        .post_gql(
            None,
            BANNERS_QUERY,
            None,
            &json!({ "query": format!("query {{ {fields}}}") }),
        )
        .await?;

    Ok(parse_banners(&body, chunk.len()))
}

pub(crate) fn parse_banners(body: &Value, count: usize) -> Vec<(String, String)> {
    (0..count)
        .filter_map(|index| {
            let user = &body["data"][format!("u{index}")];

            Some((text(&user["login"])?, text(&user["bannerImageURL"])?))
        })
        .collect()
}
