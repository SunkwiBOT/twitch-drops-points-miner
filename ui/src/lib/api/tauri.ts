// ! Command names are kept by hand in TauriCommand: a renamed `#[tauri::command]`
// fails only at run time.

import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { listen as tauriListen } from "@tauri-apps/api/event";

import type { ChannelBannersResponseType } from "../bindings/ChannelBannersResponseType";
import type { SearchResponseType } from "../bindings/SearchResponseType";
import type { DesktopEvent } from "../bindings/DesktopEvent";
import type { AppInfoResponseType } from "../bindings/AppInfoResponseType";
import type { AutostartResponseType } from "../bindings/AutostartResponseType";
import type { BurstProgressType } from "../bindings/BurstProgressType";
import type { CatalogRefreshResponseType } from "../bindings/CatalogRefreshResponseType";
import type { ChannelResponseType } from "../bindings/ChannelResponseType";
import type { ChannelsLookupResponseType } from "../bindings/ChannelsLookupResponseType";
import type { CloseAction } from "../bindings/CloseAction";
import type { CommandType } from "../bindings/CommandType";
import type { FollowsResponseType } from "../bindings/FollowsResponseType";
import type { HistoryQueryType } from "../bindings/HistoryQueryType";
import type { HistoryResponseType } from "../bindings/HistoryResponseType";
import type { LoginFlowResponseType } from "../bindings/LoginFlowResponseType";
import type { LogsQueryType } from "../bindings/LogsQueryType";
import type { LogsReportResponseType } from "../bindings/LogsReportResponseType";
import type { LogsResponseType } from "../bindings/LogsResponseType";
import type { MinerEventType } from "../bindings/MinerEventType";
import type { NotificationTestResponseType } from "../bindings/NotificationTestResponseType";
import type { ProxyTestResponseType } from "../bindings/ProxyTestResponseType";
import type { RedeemRequestType } from "../bindings/RedeemRequestType";
import type { RedeemResponseType } from "../bindings/RedeemResponseType";
import type { RequestsQueryType } from "../bindings/RequestsQueryType";
import type { RequestsResponseType } from "../bindings/RequestsResponseType";
import type { SnapshotResponseType } from "../bindings/SnapshotResponseType";
import type { SnapshotType } from "../bindings/SnapshotType";
import type { SpamBalanceResponseType } from "../bindings/SpamBalanceResponseType";
import type { SpamChannelResponseType } from "../bindings/SpamChannelResponseType";
import type { StartupProblemResponseType } from "../bindings/StartupProblemResponseType";
import { toApiError } from "../errors/errors.utils";
import type { TauriCommand, UpdateEvent } from "./api.constants";

export type UnsubscribeType = () => void;

// ! The desktop shell exports no bindings: these three mirror `UpdateInfoDto`,
// `FailedUpdateDto` and `UpdateProgressDto` in src-tauri/src/updates.rs and
// change with them.
export type UpdateInfoType = {
  version: string;
  current_version: string;
  notes: string | null;
};

export type FailedUpdateType = { version: string };

export type UpdateProgressType = {
  downloaded: number;
  total: number | null;
};

type TauriCommandsType = {
  [TauriCommand.SNAPSHOT]: { args: undefined; result: SnapshotResponseType };
  [TauriCommand.COMMAND]: { args: { command: CommandType }; result: null };
  [TauriCommand.REFRESH_CATALOG]: { args: undefined; result: CatalogRefreshResponseType };
  [TauriCommand.START_LOGIN]: { args: undefined; result: LoginFlowResponseType };
  [TauriCommand.LOOKUP_CHANNEL]: { args: { login: string }; result: ChannelResponseType };
  [TauriCommand.LOOKUP_CHANNELS]: {
    args: { logins: string[] };
    result: ChannelsLookupResponseType;
  };
  [TauriCommand.SEARCH]: { args: { query: string }; result: SearchResponseType };
  [TauriCommand.CHANNEL_BANNERS]: {
    args: { logins: string[] };
    result: ChannelBannersResponseType;
  };
  [TauriCommand.FOLLOWS]: { args: { accountId: string }; result: FollowsResponseType };
  [TauriCommand.SPAM_CHANNEL]: {
    args: { accountId: string; login: string };
    result: SpamChannelResponseType;
  };
  [TauriCommand.SPAM_BALANCE]: {
    args: { accountId: string; login: string };
    result: SpamBalanceResponseType;
  };
  [TauriCommand.REDEEM]: { args: { request: RedeemRequestType }; result: RedeemResponseType };
  [TauriCommand.TEST_NOTIFICATIONS]: { args: undefined; result: NotificationTestResponseType };
  [TauriCommand.TEST_PROXY]: { args: { url: string }; result: ProxyTestResponseType };
  [TauriCommand.HISTORY]: { args: { query: HistoryQueryType }; result: HistoryResponseType };
  [TauriCommand.LOGS]: { args: { query: LogsQueryType }; result: LogsResponseType };
  [TauriCommand.LOGS_REPORT]: { args: { query: LogsQueryType }; result: LogsReportResponseType };
  [TauriCommand.REQUESTS]: { args: { query: RequestsQueryType }; result: RequestsResponseType };
  [TauriCommand.REQUESTS_REPORT]: {
    args: { query: RequestsQueryType };
    result: LogsReportResponseType;
  };
  [TauriCommand.CLOSE_WINDOW]: { args: { action: CloseAction }; result: null };
  [TauriCommand.APP_INFO]: { args: undefined; result: AppInfoResponseType };
  [TauriCommand.STARTUP_PROBLEM]: { args: undefined; result: StartupProblemResponseType };
  [TauriCommand.QUIT]: { args: undefined; result: null };
  [TauriCommand.OPEN_LOGS_DIRECTORY]: { args: undefined; result: null };
  [TauriCommand.AUTOSTART_STATUS]: { args: undefined; result: AutostartResponseType };
  [TauriCommand.SET_AUTOSTART]: { args: { enabled: boolean }; result: null };
  [TauriCommand.OPEN_DATA_DIRECTORY]: { args: undefined; result: null };
  [TauriCommand.CHECK_FOR_UPDATE]: { args: undefined; result: UpdateInfoType | null };
  [TauriCommand.FOUND_UPDATE]: { args: undefined; result: UpdateInfoType | null };
  [TauriCommand.TAKE_FAILED_UPDATE]: { args: undefined; result: FailedUpdateType | null };
  [TauriCommand.DOWNLOAD_UPDATE]: { args: undefined; result: string };
  [TauriCommand.APPLY_UPDATE]: { args: { version: string }; result: null };
};

type ArgumentsType<K extends TauriCommand> = TauriCommandsType[K]["args"] extends undefined
  ? []
  : [TauriCommandsType[K]["args"]];

type DesktopEventsType = {
  [DesktopEvent.SNAPSHOT]: SnapshotType;
  [DesktopEvent.MINER_EVENT]: MinerEventType;
  [DesktopEvent.CLOSE_REQUESTED]: null;
  [DesktopEvent.BURST_PROGRESS]: BurstProgressType;
};

type UpdateEventsType = {
  [UpdateEvent.PROGRESS]: UpdateProgressType;
  [UpdateEvent.AVAILABLE]: UpdateInfoType;
};

export const isTauri = (): boolean => "__TAURI_INTERNALS__" in window;

// ! Goes through the bundled API module, never a global: a script injected into
// the page cannot reach `invoke` from `window`.
export const invoke = async <K extends TauriCommand>(
  command: K,
  ...args: ArgumentsType<K>
): Promise<TauriCommandsType[K]["result"]> => {
  try {
    return await tauriInvoke<TauriCommandsType[K]["result"]>(command, args[0]);
  } catch (e: any) {
    throw toApiError(e);
  }
};

export const listen = <K extends DesktopEvent>(
  event: K,
  handler: (payload: DesktopEventsType[K]) => void,
): Promise<UnsubscribeType> =>
  tauriListen<DesktopEventsType[K]>(event, (message) => handler(message.payload));

export const listenUpdate = <K extends UpdateEvent>(
  event: K,
  handler: (payload: UpdateEventsType[K]) => void,
): Promise<UnsubscribeType> =>
  tauriListen<UpdateEventsType[K]>(event, (message) => handler(message.payload));
