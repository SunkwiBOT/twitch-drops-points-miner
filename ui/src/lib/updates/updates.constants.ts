export enum UpdateCheck {
  IDLE = "IDLE",
  CHECKING = "CHECKING",
  CURRENT = "CURRENT",
  AVAILABLE = "AVAILABLE",
  FAILED = "FAILED",
}

export enum UpdateStage {
  IDLE = "IDLE",
  DOWNLOADING = "DOWNLOADING",
  INSTALLING = "INSTALLING",
}

export const UPDATE_TOAST_KEY = "update";

export const IMAGE_UPDATE_COMMANDS: readonly string[] = [
  "docker compose pull",
  "docker compose up -d",
];

export const UPDATE_BADGE_CLASSES =
  "rounded-full border-info-emphasis/40 bg-info text-info-foreground hover:bg-info/80 hover:text-info-foreground";

export const MEGABYTE_UNIT = "megabyte";
export const BYTES_PER_MEGABYTE = 1_000_000;
