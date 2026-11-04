import { ServerMessageKind } from "../bindings/ServerMessageKind";
import { openUrl as tauriOpenUrl } from "@tauri-apps/plugin-opener";

import type { AppInfoType } from "../bindings/AppInfoType";
import type { AuthResponseType } from "../bindings/AuthResponseType";
import type { AuthStatusType } from "../bindings/AuthStatusType";
import type { CatalogRefreshResponseType } from "../bindings/CatalogRefreshResponseType";
import type { CatalogRefreshType } from "../bindings/CatalogRefreshType";
import type { ChannelQueryType } from "../bindings/ChannelQueryType";
import type { ChannelResponseType } from "../bindings/ChannelResponseType";
import type { ChannelsLookupRequestType } from "../bindings/ChannelsLookupRequestType";
import type { ChannelsLookupResponseType } from "../bindings/ChannelsLookupResponseType";
import type { ChannelType } from "../bindings/ChannelType";
import type { CloseAction } from "../bindings/CloseAction";
import type { CommandType } from "../bindings/CommandType";
import type { DashboardLoginRequestType } from "../bindings/DashboardLoginRequestType";
import { DesktopEvent } from "../bindings/DesktopEvent";
import type { FollowsQueryType } from "../bindings/FollowsQueryType";
import type { FollowsResponseType } from "../bindings/FollowsResponseType";
import type { HistoryQueryType } from "../bindings/HistoryQueryType";
import type { HistoryResponseType } from "../bindings/HistoryResponseType";
import type { LoginFlowResponseType } from "../bindings/LoginFlowResponseType";
import type { LoginFlowViewType } from "../bindings/LoginFlowViewType";
import type { LogsQueryType } from "../bindings/LogsQueryType";
import type { LogsReportResponseType } from "../bindings/LogsReportResponseType";
import type { LogsResponseType } from "../bindings/LogsResponseType";
import type { MinerEventType } from "../bindings/MinerEventType";
import type { NotificationTestResponseType } from "../bindings/NotificationTestResponseType";
import type { NotificationTestType } from "../bindings/NotificationTestType";
import type { ProxyTestRequestType } from "../bindings/ProxyTestRequestType";
import type { ProxyTestResponseType } from "../bindings/ProxyTestResponseType";
import type { RequestsQueryType } from "../bindings/RequestsQueryType";
import type { RequestsResponseType } from "../bindings/RequestsResponseType";
import type { SearchQueryType } from "../bindings/SearchQueryType";
import type { SearchResponseType } from "../bindings/SearchResponseType";
import type { ChannelBannersRequestType } from "../bindings/ChannelBannersRequestType";
import type { ChannelBannersResponseType } from "../bindings/ChannelBannersResponseType";
import { RoutesApi } from "../bindings/RoutesApi";
import type { ServerMessageType } from "../bindings/ServerMessageType";
import type { SnapshotResponseType } from "../bindings/SnapshotResponseType";
import type { SnapshotType } from "../bindings/SnapshotType";
import type { StartupProblemType } from "../bindings/StartupProblemType";
import type { UpdateCheckResponseType } from "../bindings/UpdateCheckResponseType";
import {
  HttpMethod,
  RECONNECT_DELAY_MAX_MS,
  RECONNECT_DELAY_MIN_MS,
  TauriCommand,
  UpdateEvent,
} from "./api.constants";
import { http } from "./http";
import { invoke, isTauri, listen, listenUpdate } from "./tauri";
import type {
  FailedUpdateType,
  UnsubscribeType,
  UpdateInfoType,
  UpdateProgressType,
} from "./tauri";

type ListenersType = {
  onSnapshot: (snapshot: SnapshotType) => void;
  onEvent: (event: MinerEventType) => void;
  onConnection: (connected: boolean) => void;
};

export const fetchSnapshot = async (): Promise<SnapshotType> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.SNAPSHOT)).snapshot;
  }

  const response = await http<SnapshotResponseType>(RoutesApi.SNAPSHOT, {
    method: HttpMethod.GET,
  });

  return response.data.snapshot;
};

export const sendCommand = async (command: CommandType): Promise<void> => {
  if (isTauri()) {
    await invoke(TauriCommand.COMMAND, { command });

    return;
  }

  await http<null>(RoutesApi.COMMAND, { method: HttpMethod.POST, data: command });
};

export const requestCatalogRefresh = async (): Promise<CatalogRefreshType> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.REFRESH_CATALOG)).refresh;
  }

  const response = await http<CatalogRefreshResponseType>(RoutesApi.CATALOG_REFRESH, {
    method: HttpMethod.POST,
  });

  return response.data.refresh;
};

export const startLogin = async (): Promise<LoginFlowViewType> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.START_LOGIN)).login;
  }

  const response = await http<LoginFlowResponseType>(RoutesApi.LOGIN_FLOW, {
    method: HttpMethod.POST,
  });

  return response.data.login;
};

export const lookupChannel = async (login: string): Promise<ChannelType> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.LOOKUP_CHANNEL, { login })).channel;
  }

  const response = await http<ChannelResponseType>(RoutesApi.CHANNEL, {
    method: HttpMethod.GET,
    params: { login } satisfies ChannelQueryType,
  });

  return response.data.channel;
};

export const fetchBanners = async (logins: string[]): Promise<Record<string, string>> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.CHANNEL_BANNERS, { logins })).banners;
  }

  const response = await http<ChannelBannersResponseType>(RoutesApi.CHANNELS_BANNERS, {
    method: HttpMethod.POST,
    data: { logins } satisfies ChannelBannersRequestType,
  });

  return response.data.banners;
};

export const searchTwitch = async (query: string): Promise<SearchResponseType> => {
  if (isTauri()) {
    return invoke(TauriCommand.SEARCH, { query });
  }

  const response = await http<SearchResponseType>(RoutesApi.SEARCH, {
    method: HttpMethod.GET,
    params: { query } satisfies SearchQueryType,
  });

  return response.data;
};

export const lookupChannels = async (logins: string[]): Promise<ChannelsLookupResponseType> => {
  if (isTauri()) {
    return invoke(TauriCommand.LOOKUP_CHANNELS, { logins });
  }

  const response = await http<ChannelsLookupResponseType>(RoutesApi.CHANNELS_LOOKUP, {
    method: HttpMethod.POST,
    data: { logins } satisfies ChannelsLookupRequestType,
  });

  return response.data;
};

export const fetchFollows = async (accountId: string): Promise<ChannelType[]> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.FOLLOWS, { accountId })).channels;
  }

  const response = await http<FollowsResponseType>(RoutesApi.FOLLOWS, {
    method: HttpMethod.GET,
    params: { account_id: accountId } satisfies FollowsQueryType,
  });

  return response.data.channels;
};

export const testNotifications = async (): Promise<NotificationTestType[]> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.TEST_NOTIFICATIONS)).targets;
  }

  const response = await http<NotificationTestResponseType>(RoutesApi.NOTIFICATIONS_TEST, {
    method: HttpMethod.POST,
  });

  return response.data.targets;
};

export const testProxy = async (url: string): Promise<number> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.TEST_PROXY, { url })).latency_ms;
  }

  const response = await http<ProxyTestResponseType>(RoutesApi.PROXY_TEST, {
    method: HttpMethod.POST,
    data: { url } satisfies ProxyTestRequestType,
  });

  return response.data.latency_ms;
};

export const fetchHistory = async (query: HistoryQueryType): Promise<HistoryResponseType> => {
  if (isTauri()) {
    return invoke(TauriCommand.HISTORY, { query });
  }

  const response = await http<HistoryResponseType>(RoutesApi.HISTORY, {
    method: HttpMethod.GET,
    params: query,
  });

  return response.data;
};

export const fetchLogs = async (query: LogsQueryType): Promise<LogsResponseType> => {
  if (isTauri()) {
    return invoke(TauriCommand.LOGS, { query });
  }

  const response = await http<LogsResponseType>(RoutesApi.LOGS, {
    method: HttpMethod.GET,
    params: query,
  });

  return response.data;
};

export const fetchLogsReport = async (query: LogsQueryType): Promise<string> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.LOGS_REPORT, { query })).report;
  }

  const response = await http<LogsReportResponseType>(RoutesApi.LOGS_REPORT, {
    method: HttpMethod.GET,
    params: query,
  });

  return response.data.report;
};

export const fetchRequests = async (query: RequestsQueryType): Promise<RequestsResponseType> => {
  if (isTauri()) {
    return invoke(TauriCommand.REQUESTS, { query });
  }

  const response = await http<RequestsResponseType>(RoutesApi.REQUESTS, {
    method: HttpMethod.GET,
    params: query,
  });

  return response.data;
};

export const fetchRequestsReport = async (query: RequestsQueryType): Promise<string> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.REQUESTS_REPORT, { query })).report;
  }

  const response = await http<LogsReportResponseType>(RoutesApi.REQUESTS_REPORT, {
    method: HttpMethod.GET,
    params: query,
  });

  return response.data.report;
};

const isServerMessage = (value: unknown): value is ServerMessageType =>
  typeof value === "object" &&
  value !== null &&
  "type" in value &&
  ((value.type === ServerMessageKind.SNAPSHOT && "snapshot" in value) ||
    (value.type === ServerMessageKind.EVENT && "event" in value));

const parseServerMessage = (raw: unknown): ServerMessageType | null => {
  try {
    const value: unknown = JSON.parse(String(raw));

    return isServerMessage(value) ? value : null;
  } catch (e: any) {
    console.warn("dropped a malformed dashboard frame", e);

    return null;
  }
};

const subscribeDesktop = async (listeners: ListenersType): Promise<UnsubscribeType> => {
  const stopSnapshot = await listen(DesktopEvent.SNAPSHOT, listeners.onSnapshot);
  const stopEvent = await listen(DesktopEvent.MINER_EVENT, listeners.onEvent);

  listeners.onConnection(true);

  return () => {
    stopSnapshot();
    stopEvent();
  };
};

const subscribeDashboard = (listeners: ListenersType): UnsubscribeType => {
  let socket: WebSocket | null = null;
  let retry: ReturnType<typeof setTimeout> | null = null;
  let closed = false;
  let delay = RECONNECT_DELAY_MIN_MS;

  const connect = (): void => {
    const protocol = location.protocol === "https:" ? "wss" : "ws";
    const current = new WebSocket(`${protocol}://${location.host}${RoutesApi.EVENTS}`);

    socket = current;
    current.onopen = () => {
      delay = RECONNECT_DELAY_MIN_MS;
      listeners.onConnection(true);
    };
    current.onmessage = (frame) => {
      const message = parseServerMessage(frame.data);

      if (message?.type === ServerMessageKind.SNAPSHOT) {
        listeners.onSnapshot(message.snapshot);
      } else if (message?.type === ServerMessageKind.EVENT) {
        listeners.onEvent(message.event);
      }
    };
    current.onclose = () => {
      listeners.onConnection(false);

      if (closed) {
        return;
      }

      retry = setTimeout(connect, delay);
      delay = Math.min(delay * 2, RECONNECT_DELAY_MAX_MS);
    };
    current.onerror = () => current.close();
  };

  connect();

  return () => {
    closed = true;

    if (retry) {
      clearTimeout(retry);
    }

    socket?.close();
  };
};

export const subscribe = async (listeners: ListenersType): Promise<UnsubscribeType> =>
  isTauri() ? subscribeDesktop(listeners) : subscribeDashboard(listeners);

export const fetchAppInfo = async (): Promise<AppInfoType | null> => {
  if (!isTauri()) {
    return null;
  }

  return (await invoke(TauriCommand.APP_INFO)).app;
};

export const fetchStartupProblem = async (): Promise<StartupProblemType | null> =>
  isTauri() ? ((await invoke(TauriCommand.STARTUP_PROBLEM)).problem ?? null) : null;

export const quitApp = async (): Promise<void> => {
  await invoke(TauriCommand.QUIT);
};

export const fetchAutostart = async (): Promise<boolean> =>
  isTauri() ? (await invoke(TauriCommand.AUTOSTART_STATUS)).enabled : false;

export const setAutostart = async (enabled: boolean): Promise<void> => {
  await invoke(TauriCommand.SET_AUTOSTART, { enabled });
};

export const closeWindow = async (action: CloseAction): Promise<void> => {
  await invoke(TauriCommand.CLOSE_WINDOW, { action });
};

export const onCloseRequested = async (callback: () => void): Promise<UnsubscribeType | null> =>
  isTauri() ? listen(DesktopEvent.CLOSE_REQUESTED, callback) : null;

export const openLogsDirectory = async (): Promise<void> => {
  await invoke(TauriCommand.OPEN_LOGS_DIRECTORY);
};

export const openDataDirectory = async (): Promise<void> => {
  await invoke(TauriCommand.OPEN_DATA_DIRECTORY);
};

export const checkForUpdate = async (): Promise<string | null> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.CHECK_FOR_UPDATE))?.version ?? null;
  }

  const response = await http<UpdateCheckResponseType>(RoutesApi.UPDATES_CHECK, {
    method: HttpMethod.POST,
  });

  return response.data.check.available ?? null;
};

export const fetchFoundUpdate = async (): Promise<UpdateInfoType | null> =>
  invoke(TauriCommand.FOUND_UPDATE);

export const takeFailedUpdate = async (): Promise<FailedUpdateType | null> =>
  invoke(TauriCommand.TAKE_FAILED_UPDATE);

export const onUpdateAvailable = (
  callback: (found: UpdateInfoType) => void,
): Promise<UnsubscribeType> => listenUpdate(UpdateEvent.AVAILABLE, callback);

export const downloadUpdate = async (): Promise<string> => invoke(TauriCommand.DOWNLOAD_UPDATE);

export const applyUpdate = async (version: string): Promise<void> => {
  await invoke(TauriCommand.APPLY_UPDATE, { version });
};

export const onUpdateProgress = (
  callback: (progress: UpdateProgressType) => void,
): Promise<UnsubscribeType> => listenUpdate(UpdateEvent.PROGRESS, callback);

export const openUrl = async (url: string): Promise<void> => {
  if (isTauri()) {
    await tauriOpenUrl(url);

    return;
  }

  window.open(url, "_blank", "noopener,noreferrer");
};

export const fetchAuthStatus = async (): Promise<AuthStatusType> => {
  if (isTauri()) {
    return { authorized: true, public: false };
  }

  const response = await http<AuthResponseType>(RoutesApi.AUTH, { method: HttpMethod.GET });

  return response.data.auth;
};

export const signIn = async (password: string): Promise<void> => {
  await http<null>(RoutesApi.AUTH, {
    method: HttpMethod.POST,
    data: { password } satisfies DashboardLoginRequestType,
  });
};

export const signOut = async (): Promise<void> => {
  await http<null>(RoutesApi.AUTH, { method: HttpMethod.DELETE });
};

export const reportActivity = async (): Promise<void> => {
  await http<null>(RoutesApi.ACTIVITY, { method: HttpMethod.POST });
};
