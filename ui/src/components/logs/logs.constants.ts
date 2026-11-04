import { LogLevel } from "../../lib/bindings/LogLevel";

export enum LogsTab {
  LINES = "LINES",
  REQUESTS = "REQUESTS",
}

export const LOGS_PANEL_ID = "logs-panel";

export const LOGS_TAB_SLUGS: Record<LogsTab, string> = {
  [LogsTab.LINES]: "logs",
  [LogsTab.REQUESTS]: "requests",
};

export const LOGS_PAGE_SIZE = 100;
export const LOGS_REFRESH_MS = 5_000;
export const LOGS_SKELETON_ROWS = 12;
export const LOGS_MORE_SKELETON_ROWS = 2;

export const LOG_LEVELS_KEPT: readonly LogLevel[] = [LogLevel.ERROR, LogLevel.WARN, LogLevel.INFO];

export const LOG_LEVEL_CLASSES: Record<LogLevel, string> = {
  [LogLevel.ERROR]: "text-destructive",
  [LogLevel.WARN]: "text-warning-foreground",
  [LogLevel.INFO]: "text-success-foreground",
  [LogLevel.DEBUG]: "text-muted-foreground",
  [LogLevel.TRACE]: "text-muted-foreground",
};

export const LOG_LIST_CLASSES =
  "grid grid-cols-[max-content_minmax(0,1fr)] gap-x-3 font-mono text-xs leading-normal md:grid-cols-[max-content_max-content_7rem_minmax(0,1fr)]";
export const LOG_LINE_CLASSES =
  "col-span-full grid grid-cols-subgrid border-b py-1 break-words whitespace-pre-wrap last:border-b-0";
