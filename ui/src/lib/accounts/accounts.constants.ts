import { FilterOp } from "../bindings/FilterOp";

export enum CopySection {
  DROPS = "DROPS",
  POINTS = "POINTS",
}

export enum ImportSource {
  SEARCH = "SEARCH",
  PASTE = "PASTE",
  ACCOUNT = "ACCOUNT",
  FOLLOWS = "FOLLOWS",
}

export const IMPORT_BATCH = 20;

export const IMPORT_STAGGER_MS = 35;

export enum ImportEntryStatus {
  PENDING = "PENDING",
  FOUND = "FOUND",
  ALREADY = "ALREADY",
  MISSING = "MISSING",
  UNCHECKED = "UNCHECKED",
  INVALID = "INVALID",
}

export enum SetupStep {
  START = "START",
  MODE = "MODE",
  DROPS = "DROPS",
  STREAMERS = "STREAMERS",
  SUMMARY = "SUMMARY",
}

export enum SetupSource {
  DEFAULTS = "DEFAULTS",
  CUSTOM = "CUSTOM",
}

export const FILTER_OP_SYMBOLS: Record<FilterOp, string> = {
  [FilterOp.GT]: ">",
  [FilterOp.GTE]: "≥",
  [FilterOp.LT]: "<",
  [FilterOp.LTE]: "≤",
};

export const PROXY_CREDENTIALS_MASK = "•••";

export const GROUP_CHIPS_MAX = 16;

export enum StreamerOption {
  CLAIM_BONUS = "CLAIM_BONUS",
  WATCH_STREAK = "WATCH_STREAK",
  FOLLOW_RAIDS = "FOLLOW_RAIDS",
  CLAIM_MOMENTS = "CLAIM_MOMENTS",
  COMMUNITY_GOALS = "COMMUNITY_GOALS",
  CHAT = "CHAT",
  PREDICTIONS = "PREDICTIONS",
  STRATEGY = "STRATEGY",
  TIMING = "TIMING",
  PERCENTAGE = "PERCENTAGE",
  MAX_POINTS = "MAX_POINTS",
  MIN_POINTS = "MIN_POINTS",
  DELAY_SECONDS = "DELAY_SECONDS",
  TIMING_PERCENT = "TIMING_PERCENT",
  PERCENTAGE_GAP = "PERCENTAGE_GAP",
  STEALTH = "STEALTH",
  FILTER = "FILTER",
  FILTER_BY = "FILTER_BY",
  FILTER_OP = "FILTER_OP",
  FILTER_VALUE = "FILTER_VALUE",
}

export enum SetupEnd {
  START = "START",
  LATER = "LATER",
}

export enum DropsList {
  PRIORITY_GAMES = "priority_games",
  EXCLUDE_GAMES = "exclude_games",
  EXCLUDE_NAMES = "exclude_names",
}

export enum DropsTab {
  ORDER = "ORDER",
  SETTINGS = "SETTINGS",
  EXCLUSIONS = "EXCLUSIONS",
  CAMPAIGNS = "CAMPAIGNS",
}

export const DROPS_TAB_SLUGS: Record<DropsTab, string> = {
  [DropsTab.ORDER]: "order",
  [DropsTab.SETTINGS]: "settings",
  [DropsTab.EXCLUSIONS]: "exclusions",
  [DropsTab.CAMPAIGNS]: "campaigns",
};
