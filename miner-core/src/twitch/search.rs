use serde_json::{json, Value};

use super::operations::Operation;
use super::{text, values, TwitchHttp};
use crate::domain::{SearchChannel, SearchGame};
use crate::error::Result;

pub async fn suggestions(
    http: &TwitchHttp,
    fragment: &str,
) -> Result<(Vec<SearchGame>, Vec<SearchChannel>)> {
    let body = http
        .post_gql(
            None,
            Operation::SearchSuggestions.name(),
            None,
            &Operation::SearchSuggestions.request(json!({
                "requestID": crate::util::random_hex(16),
                "queryFragment": fragment,
                "withOfflineChannelContent": true,
            })),
        )
        .await?;

    Ok(parse(&body))
}

pub(crate) fn parse(body: &Value) -> (Vec<SearchGame>, Vec<SearchChannel>) {
    let mut games = Vec::new();
    let mut channels = Vec::new();

    for edge in values(&body["data"]["searchSuggestions"]["edges"]) {
        let node = &edge["node"];
        let content = &node["content"];
        let Some(name) = text(&node["text"]) else {
            continue;
        };

        match content["__typename"].as_str() {
            Some("SearchSuggestionCategory") => {
                let Some(id) = text(&content["game"]["id"]).or_else(|| text(&content["id"])) else {
                    continue;
                };

                games.push(SearchGame {
                    id,
                    name,
                    slug: text(&content["game"]["slug"]),
                    box_art_url: text(&content["boxArtURL"]),
                });
            }
            Some("SearchSuggestionChannel") => {
                let Some(login) = text(&content["login"]) else {
                    continue;
                };

                channels.push(SearchChannel {
                    login,
                    display_name: name,
                    avatar_url: text(&content["profileImageURL"])
                        .map(|url| url.replace("-50x50.", "-70x70.")),
                    live: content["isLive"].as_bool().unwrap_or(false),
                    verified: content["isVerified"].as_bool().unwrap_or(false),
                });
            }
            _ => {}
        }
    }

    (games, channels)
}
