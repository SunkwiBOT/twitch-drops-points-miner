use std::path::{Path, PathBuf};

use miner_core::api::{
    ApiRoute, AppInfoResponse, AuthResponse, AutostartResponse, CatalogRefreshResponse,
    ChannelBannersRequest, ChannelBannersResponse, ChannelQuery, ChannelResponse,
    ChannelsLookupRequest, ChannelsLookupResponse, Command, CommandName, DashboardLoginRequest,
    DesktopEvent, FollowsQuery, FollowsResponse, HistoryQuery, HistoryResponse, LoginFlowResponse,
    LogsQuery, LogsReportResponse, LogsResponse, NotificationTestResponse, ProxyTestRequest,
    ProxyTestResponse, RequestsQuery, RequestsResponse, SearchQuery, SearchResponse, ServerMessage,
    ServerMessageKind, SnapshotResponse, StartupProblemResponse, UpdateCheckResponse,
};
use miner_core::error::{ApiError, ErrorCode};
use miner_core::scheduler::SlotPurposeKind;
use regex::Regex;
use ts_rs::{Config, TS};

fn config(out_dir: &Path) -> Config {
    Config::new().with_out_dir(out_dir).with_large_int("number")
}

fn member_name(value: &str) -> String {
    let trimmed = value.strip_prefix("/api/").unwrap_or(value);

    trimmed
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_uppercase)
        .collect::<Vec<_>>()
        .join("_")
}

fn rename_enum_members(source: &str) -> String {
    let member = Regex::new(r#""([^"]+)" = ("[^"]+")"#).expect("valid pattern");

    source
        .lines()
        .map(|line| {
            if line.starts_with("export enum ") {
                member
                    .replace_all(line, |c: &regex::Captures| {
                        format!("{} = {}", member_name(&c[1]), &c[2])
                    })
                    .into_owned()
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn export<T: TS + 'static>(cfg: &Config) {
    T::export_all(cfg).expect("bindings written");
}

fn main() {
    let dir = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("usage: export-bindings <output directory>"),
    );

    if dir.exists() {
        for entry in std::fs::read_dir(&dir).expect("readable output directory") {
            let path = entry.expect("directory entry").path();

            if path.extension().is_some_and(|e| e == "ts") {
                std::fs::remove_file(path).expect("stale binding removed");
            }
        }
    }

    let cfg = config(&dir);

    export::<SnapshotResponse>(&cfg);
    export::<CatalogRefreshResponse>(&cfg);
    export::<UpdateCheckResponse>(&cfg);
    export::<Command>(&cfg);
    export::<HistoryResponse>(&cfg);
    export::<LogsResponse>(&cfg);
    export::<LogsReportResponse>(&cfg);
    export::<LoginFlowResponse>(&cfg);
    export::<ChannelResponse>(&cfg);
    export::<ChannelsLookupRequest>(&cfg);
    export::<ChannelsLookupResponse>(&cfg);
    export::<FollowsQuery>(&cfg);
    export::<FollowsResponse>(&cfg);
    export::<NotificationTestResponse>(&cfg);
    export::<ProxyTestResponse>(&cfg);
    export::<AuthResponse>(&cfg);
    export::<AppInfoResponse>(&cfg);
    export::<AutostartResponse>(&cfg);
    export::<StartupProblemResponse>(&cfg);
    export::<HistoryQuery>(&cfg);
    export::<LogsQuery>(&cfg);
    export::<RequestsQuery>(&cfg);
    export::<RequestsResponse>(&cfg);
    export::<ChannelQuery>(&cfg);
    export::<SearchQuery>(&cfg);
    export::<ChannelBannersRequest>(&cfg);
    export::<ChannelBannersResponse>(&cfg);
    export::<SearchResponse>(&cfg);
    export::<ProxyTestRequest>(&cfg);
    export::<DashboardLoginRequest>(&cfg);
    export::<ServerMessage>(&cfg);
    export::<ServerMessageKind>(&cfg);
    export::<CommandName>(&cfg);
    export::<SlotPurposeKind>(&cfg);
    export::<DesktopEvent>(&cfg);
    export::<ApiRoute>(&cfg);
    export::<ApiError>(&cfg);
    export::<ErrorCode>(&cfg);

    for entry in std::fs::read_dir(&dir).expect("readable output directory") {
        let path = entry.expect("directory entry").path();
        let source = std::fs::read_to_string(&path).expect("binding readable");

        if source.contains("export enum ") {
            std::fs::write(&path, rename_enum_members(&source) + "\n").expect("binding written");
        }
    }
}
