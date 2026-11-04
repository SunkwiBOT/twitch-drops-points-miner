export enum HttpMethod {
  GET = "GET",
  POST = "POST",
  DELETE = "DELETE",
}

export enum TauriCommand {
  SNAPSHOT = "snapshot",
  COMMAND = "command",
  REFRESH_CATALOG = "refresh_catalog",
  START_LOGIN = "start_login",
  LOOKUP_CHANNEL = "lookup_channel",
  LOOKUP_CHANNELS = "lookup_channels",
  SEARCH = "search",
  CHANNEL_BANNERS = "channel_banners",
  FOLLOWS = "follows",
  SPAM_CHANNEL = "spam_channel",
  SPAM_BALANCE = "spam_balance",
  REDEEM = "redeem",
  TEST_NOTIFICATIONS = "test_notifications",
  TEST_PROXY = "test_proxy",
  HISTORY = "history",
  LOGS = "logs",
  LOGS_REPORT = "logs_report",
  REQUESTS = "requests",
  REQUESTS_REPORT = "requests_report",
  CLOSE_WINDOW = "close_window",
  APP_INFO = "app_info",
  STARTUP_PROBLEM = "startup_problem",
  QUIT = "quit",
  OPEN_LOGS_DIRECTORY = "open_logs_directory",
  AUTOSTART_STATUS = "autostart_status",
  SET_AUTOSTART = "set_autostart",
  OPEN_DATA_DIRECTORY = "open_data_directory",
  CHECK_FOR_UPDATE = "check_for_update",
  FOUND_UPDATE = "found_update",
  TAKE_FAILED_UPDATE = "take_failed_update",
  DOWNLOAD_UPDATE = "download_update",
  APPLY_UPDATE = "apply_update",
}

export enum UpdateEvent {
  PROGRESS = "UPDATE_PROGRESS",
  AVAILABLE = "UPDATE_AVAILABLE",
}

export const REQUEST_TIMEOUT_MS = 30_000;
export const RECONNECT_DELAY_MIN_MS = 1_000;
export const RECONNECT_DELAY_MAX_MS = 15_000;
