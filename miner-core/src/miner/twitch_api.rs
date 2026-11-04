use std::collections::HashMap;
use std::net::IpAddr;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use url::Url;
use zeroize::Zeroizing;

use crate::domain::{Channel, Game, WatchStreak};
use crate::error::Result;
use crate::twitch::auth::{self, TokenSet, Validation};
use crate::twitch::chat::Chat;
use crate::twitch::drops::{ClaimOutcome, Inventory};
use crate::twitch::playback::Playback;
use crate::twitch::points::{Goal, PointsContext};
use crate::twitch::pubsub::{PubSub, PubSubEvent};
use crate::twitch::spade;
use crate::twitch::TwitchClient;

pub(crate) trait TwitchApi {
    async fn validate(&self) -> Result<Option<Validation>>;

    async fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenSet>;

    async fn inventory(&self) -> Result<Inventory>;

    async fn claim_drop(&self, instance_id: &str) -> Result<ClaimOutcome>;

    async fn game_directory(&self, game: &Game) -> Result<Vec<Channel>>;

    async fn channels(&self, logins: &[String]) -> Result<Vec<Channel>>;

    async fn follows(&self, max: usize) -> Result<Vec<Channel>>;

    async fn available_drops(&self, channel_ids: &[String])
        -> Result<HashMap<String, Vec<String>>>;

    async fn points_context(&self, login: &str) -> Result<Option<PointsContext>>;

    async fn watch_streak(&self, login: &str) -> Result<Option<WatchStreak>>;

    async fn claim_bonus(&self, channel_id: &str, claim_id: &str) -> Result<i64>;

    async fn claim_moment(&self, moment_id: &str) -> Result<()>;

    async fn join_raid(&self, raid_id: &str) -> Result<()>;

    async fn make_prediction(&self, event_id: &str, outcome_id: &str, points: i64) -> Result<()>;

    async fn community_goals(&self, login: &str) -> Result<Vec<Goal>>;

    async fn goal_contributions(&self, login: &str) -> Result<HashMap<String, i64>>;

    async fn contribute_goal(&self, channel_id: &str, goal_id: &str, amount: i64) -> Result<()>;

    async fn resolve_beacon(&self, login: &str) -> Result<Url>;

    async fn send_minute_watched(&self, beacon: &Url, channel: &Channel) -> Result<bool>;

    async fn beacon_blocked_by(&self, beacon: &Url) -> Option<IpAddr>;

    async fn poll_playback(&self, playback: &mut Playback) -> Result<usize>;

    fn start_pubsub(&self, events: mpsc::Sender<PubSubEvent>, cancel: CancellationToken) -> PubSub;

    fn start_chat(&self, cancel: CancellationToken) -> Chat;
}

// ! `TwitchClient::name(self)` reaches the inherent method, which a path to the
// type picks before the trait's: these calls do not recurse.
impl TwitchApi for TwitchClient {
    async fn validate(&self) -> Result<Option<Validation>> {
        let token = Zeroizing::new(self.token().await);

        auth::validate(&self.http, Some(&self.user_id), &token).await
    }

    async fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenSet> {
        let tokens = auth::refresh(&self.http, Some(&self.user_id), refresh_token).await?;

        self.set_token(tokens.access_token.clone()).await;

        Ok(tokens)
    }

    async fn inventory(&self) -> Result<Inventory> {
        TwitchClient::inventory(self).await
    }

    async fn claim_drop(&self, instance_id: &str) -> Result<ClaimOutcome> {
        TwitchClient::claim_drop(self, instance_id).await
    }

    async fn game_directory(&self, game: &Game) -> Result<Vec<Channel>> {
        TwitchClient::game_directory(self, game).await
    }

    async fn channels(&self, logins: &[String]) -> Result<Vec<Channel>> {
        TwitchClient::channels(self, logins).await
    }

    async fn follows(&self, max: usize) -> Result<Vec<Channel>> {
        TwitchClient::follows(self, max).await
    }

    async fn available_drops(
        &self,
        channel_ids: &[String],
    ) -> Result<HashMap<String, Vec<String>>> {
        TwitchClient::available_drops(self, channel_ids).await
    }

    async fn points_context(&self, login: &str) -> Result<Option<PointsContext>> {
        TwitchClient::points_context(self, login).await
    }

    async fn watch_streak(&self, login: &str) -> Result<Option<WatchStreak>> {
        TwitchClient::watch_streak(self, login).await
    }

    async fn claim_bonus(&self, channel_id: &str, claim_id: &str) -> Result<i64> {
        TwitchClient::claim_bonus(self, channel_id, claim_id).await
    }

    async fn claim_moment(&self, moment_id: &str) -> Result<()> {
        TwitchClient::claim_moment(self, moment_id).await
    }

    async fn join_raid(&self, raid_id: &str) -> Result<()> {
        TwitchClient::join_raid(self, raid_id).await
    }

    async fn make_prediction(&self, event_id: &str, outcome_id: &str, points: i64) -> Result<()> {
        TwitchClient::make_prediction(self, event_id, outcome_id, points).await
    }

    async fn community_goals(&self, login: &str) -> Result<Vec<Goal>> {
        TwitchClient::community_goals(self, login).await
    }

    async fn goal_contributions(&self, login: &str) -> Result<HashMap<String, i64>> {
        TwitchClient::goal_contributions(self, login).await
    }

    async fn contribute_goal(&self, channel_id: &str, goal_id: &str, amount: i64) -> Result<()> {
        TwitchClient::contribute_goal(self, channel_id, goal_id, amount).await
    }

    async fn resolve_beacon(&self, login: &str) -> Result<Url> {
        spade::resolve_beacon(self, login).await
    }

    async fn send_minute_watched(&self, beacon: &Url, channel: &Channel) -> Result<bool> {
        spade::send_minute_watched(self, beacon, channel).await
    }

    async fn beacon_blocked_by(&self, beacon: &Url) -> Option<IpAddr> {
        if self.http.proxied {
            return None;
        }

        spade::dns_block(beacon).await
    }

    async fn poll_playback(&self, playback: &mut Playback) -> Result<usize> {
        playback.poll(self).await
    }

    fn start_pubsub(&self, events: mpsc::Sender<PubSubEvent>, cancel: CancellationToken) -> PubSub {
        PubSub::start(self.clone(), events, cancel)
    }

    fn start_chat(&self, cancel: CancellationToken) -> Chat {
        Chat::start(self.clone(), cancel)
    }
}
