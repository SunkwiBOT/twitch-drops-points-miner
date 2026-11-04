// ! When Twitch rotates a hash the error is `PersistedQueryNotFound`: update
// it here.

use serde_json::{json, Value};

pub const FOLLOWS_PAGE: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Inventory,
    GameDirectory,
    StreamInfo,
    PlaybackAccessToken,
    ClaimDrop,
    ChannelPointsContext,
    ClaimCommunityPoints,
    ClaimMoment,
    JoinRaid,
    MakePrediction,
    RewardList,
    ContributeCommunityGoal,
    UserPointsContribution,
    ChannelFollows,
    SearchSuggestions,
    AvailableDrops,
    CustomRewards,
    RedeemCustomReward,
}

impl Operation {
    pub fn name(self) -> &'static str {
        match self {
            Self::Inventory => "Inventory",
            Self::GameDirectory => "DirectoryPage_Game",
            Self::StreamInfo => "VideoPlayerStreamInfoOverlayChannel",
            Self::PlaybackAccessToken => "PlaybackAccessToken",
            Self::ClaimDrop => "DropsPage_ClaimDropRewards",
            Self::ChannelPointsContext => "ChannelPointsContext",
            Self::ClaimCommunityPoints => "ClaimCommunityPoints",
            Self::ClaimMoment => "CommunityMomentCallout_Claim",
            Self::JoinRaid => "JoinRaid",
            Self::MakePrediction => "MakePrediction",
            Self::RewardList => "RewardList",
            Self::ContributeCommunityGoal => "ContributeCommunityPointsCommunityGoal",
            Self::UserPointsContribution => "UserPointsContribution",
            Self::ChannelFollows => "ChannelFollows",
            Self::SearchSuggestions => "SearchTray_SearchSuggestions",
            Self::AvailableDrops => "DropsHighlightService_AvailableDrops",
            // ! The web player's version of the same query, the one tested
            // for the custom rewards; the mining keeps its own.
            Self::CustomRewards => "ChannelPointsContext",
            Self::RedeemCustomReward => "RedeemCustomReward",
        }
    }

    pub fn hash(self) -> &'static str {
        match self {
            Self::Inventory => "d86775d0ef16a63a33ad52e80eaff963b2d5b72fada7c991504a57496e1d8e4b",
            Self::GameDirectory => {
                "cb5dc816e139dcb8a118f14b4b677d59abc224a4b016c4bc2bb00a47fe0ddec4"
            }
            Self::StreamInfo => "198492e0857f6aedead9665c81c5a06d67b25b58034649687124083ff288597d",
            Self::PlaybackAccessToken => {
                "ed230aa1e33e07eebb8928504583da78a5173989fadfb1ac94be06a04f3cdbe9"
            }
            Self::ClaimDrop => "a455deea71bdc9015b78eb49f4acfbce8baa7ccbedd28e549bb025bd0f751930",
            Self::ChannelPointsContext => {
                "7fe050e3761eb2cf258d70ee1a21cbd76fa8cf3d7e7b12fc437e7029d446b5e3"
            }
            Self::ClaimCommunityPoints => {
                "46aaeebe02c99afdf4fc97c7c0cba964124bf6b0af229395f1f6d1feed05b3d0"
            }
            Self::ClaimMoment => "e2d67415aead910f7f9ceb45a77b750a1e1d9622c936d832328a0689e054db62",
            Self::JoinRaid => "c6a332a86d1087fbbb1a8623aa01bd1313d2386e7c63be60fdb2d1901f01a4ae",
            Self::MakePrediction => {
                "b44682ecc88358817009f20e69d75081b1e58825bb40aa53d5dbadcc17c881d8"
            }
            Self::RewardList => "0b1471876d7647993731b9e3c6a13bf304c67fb31d07f06a945d42286ee377c4",
            Self::UserPointsContribution => {
                "23ff2c2d60708379131178742327ead913b93b1bd6f665517a6d9085b73f661f"
            }
            Self::ContributeCommunityGoal => {
                "5774f0ea5d89587d73021a2e03c3c44777d903840c608754a1be519f51e37bb6"
            }
            Self::ChannelFollows => {
                "eecf815273d3d949e5cf0085cc5084cd8a1b5b7b6f7990cf43cb0beadf546907"
            }
            Self::SearchSuggestions => {
                "1d2cd6ae289d7baa682ef4437ab010c8ea42749ebb81c052f87a8a857ea93378"
            }
            Self::AvailableDrops => {
                "9a62a09bce5b53e26e64a671e530bc599cb6aab1e5ba3cbd5d85966d3940716f"
            }
            Self::CustomRewards => {
                "374314de591e69925fce3ddc2bcf085796f56ebb8cad67a0daa3165c03adc345"
            }
            Self::RedeemCustomReward => {
                "d56249a7adb4978898ea3412e196688d4ac3cea1c0c2dfd65561d229ea5dcc42"
            }
        }
    }

    pub fn request(self, mut variables: Value) -> Value {
        if !variables.is_object() {
            variables = json!({});
        }

        match self {
            Self::Inventory => variables["fetchRewardCampaigns"] = json!(false),
            Self::PlaybackAccessToken => {
                variables["isLive"] = json!(true);
                variables["isVod"] = json!(false);
                variables["vodID"] = json!("");
                variables["platform"] = json!("web");
                variables["playerType"] = json!("site");
            }
            Self::RewardList => {
                variables["shouldIncludeAllSuspendedStreaks"] = json!(false);
            }
            Self::ChannelFollows => {
                variables["limit"] = json!(FOLLOWS_PAGE);
                variables["order"] = json!("ASC");
            }
            Self::CustomRewards => {
                variables["includeGoalTypes"] = json!(["CREATOR", "BOOST"]);
            }
            _ => {}
        }

        json!({
            "operationName": self.name(),
            "variables": variables,
            "extensions": {"persistedQuery": {"version": 1, "sha256Hash": self.hash()}}
        })
    }
}

pub fn directory_variables(slug: &str, limit: usize) -> Value {
    json!({
        "limit": limit, "slug": slug, "imageWidth": 50, "includeCostreaming": false,
        "options": {
            "broadcasterLanguages": [], "freeformTags": null, "includeRestricted": ["SUB_ONLY_LIVE"],
            "recommendationsContext": {"platform": "web"}, "sort": "RELEVANCE",
            "systemFilters": ["DROPS_ENABLED"], "tags": [], "requestID": "JIRA-VXP-2397"
        },
        "sortTypeIsRecency": false
    })
}
