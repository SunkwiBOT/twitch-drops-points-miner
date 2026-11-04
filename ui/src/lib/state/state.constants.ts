import { AutoLock } from "../bindings/AutoLock";

export enum Page {
  HOME = "HOME",
  CATALOGUE = "CATALOGUE",
  HISTORY = "HISTORY",
  LOGS = "LOGS",
  SETTINGS = "SETTINGS",
  ACCOUNT = "ACCOUNT",
  ACCOUNT_DROPS = "ACCOUNT_DROPS",
  ACCOUNT_POINTS = "ACCOUNT_POINTS",
  ACCOUNT_SPAM = "ACCOUNT_SPAM",
  ACCOUNT_INVENTORY = "ACCOUNT_INVENTORY",
  ACCOUNT_HISTORY = "ACCOUNT_HISTORY",
  ACCOUNT_SETTINGS = "ACCOUNT_SETTINGS",
}

export const ACCOUNT_PAGES: ReadonlySet<Page> = new Set([
  Page.ACCOUNT,
  Page.ACCOUNT_DROPS,
  Page.ACCOUNT_POINTS,
  Page.ACCOUNT_SPAM,
  Page.ACCOUNT_INVENTORY,
  Page.ACCOUNT_HISTORY,
  Page.ACCOUNT_SETTINGS,
]);

export const PAGE_PATHS: Record<Page, string> = {
  [Page.HOME]: "/",
  [Page.CATALOGUE]: "/catalogue",
  [Page.HISTORY]: "/history",
  [Page.LOGS]: "/logs",
  [Page.SETTINGS]: "/settings",
  [Page.ACCOUNT]: "",
  [Page.ACCOUNT_DROPS]: "/drops",
  [Page.ACCOUNT_POINTS]: "/points",
  [Page.ACCOUNT_SPAM]: "/spam-points",
  [Page.ACCOUNT_INVENTORY]: "/inventory",
  [Page.ACCOUNT_HISTORY]: "/history",
  [Page.ACCOUNT_SETTINGS]: "/settings",
};

export const ACCOUNT_PATH = "/account";

export enum HistoryMode {
  PUSH = "PUSH",
  REPLACE = "REPLACE",
}

export enum MouseButton {
  BACK = 3,
  FORWARD = 4,
}

export enum AccountField {
  MODE = "MODE",
  SLOTS = "SLOTS",
  PROXY = "PROXY",
  AUTO_CLAIM = "AUTO_CLAIM",
  SKIP_UNFINISHABLE = "SKIP_UNFINISHABLE",
  MINE_POINTS = "MINE_POINTS",
  POINTS_FOLLOWED_ONLY = "POINTS_FOLLOWED_ONLY",
  STREAMERS_FIRST = "STREAMERS_FIRST",
  SCOPE = "SCOPE",
  LINKED_FIRST = "LINKED_FIRST",
  ORDER = "ORDER",
  STREAK_PRIORITY = "STREAK_PRIORITY",
  SUBSCRIBER_PRIORITY = "SUBSCRIBER_PRIORITY",
  STREAMER_DEFAULTS = "STREAMER_DEFAULTS",
}

export enum ToastKind {
  INFO = "INFO",
  SUCCESS = "SUCCESS",
  ERROR = "ERROR",
}

export const TOAST_DURATION_MS: Record<ToastKind, number> = {
  [ToastKind.INFO]: 4_000,
  [ToastKind.SUCCESS]: 4_000,
  [ToastKind.ERROR]: 8_000,
};

export const TOAST_ACTION_DURATION_MS = 15_000;

export const RELOAD_WAIT_MS = 2_000;

export const LINK_TOAST_KEY = "link-opened";

export const BULK_TOAST_KEY = "bulk-offer";

export const LIVE_EVENTS_MAX = 200;

export const SETTINGS_SAVE_DELAY_MS = 400;
export const SETTINGS_SETTLE_MS = 2_000;

export const AUTO_LOCK_MINUTES: Record<AutoLock, number | null> = {
  [AutoLock.NEVER]: null,
  [AutoLock.MINUTES1]: 1,
  [AutoLock.MINUTES5]: 5,
  [AutoLock.MINUTES10]: 10,
  [AutoLock.MINUTES15]: 15,
  [AutoLock.MINUTES30]: 30,
};

export const ACTIVITY_REPORT_GAP_MS = 30_000;

export const LOCK_CHECK_DELAY_MS = 2_000;

export const LOCK_CHECK_TIMEOUT_MS = 5_000;
