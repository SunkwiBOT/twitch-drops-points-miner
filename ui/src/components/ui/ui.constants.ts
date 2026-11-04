export enum IconName {
  DASHBOARD = "DASHBOARD",
  USERS = "USERS",
  GIFT = "GIFT",
  TV = "TV",
  HISTORY = "HISTORY",
  SETTINGS = "SETTINGS",
  PLUS = "PLUS",
  MINUS = "MINUS",
  INFO = "INFO",
  X = "X",
  REFRESH = "REFRESH",
  TRASH = "TRASH",
  UP = "UP",
  DOWN = "DOWN",
  EXTERNAL = "EXTERNAL",
  CHECK = "CHECK",
  COPY = "COPY",
  PLAY = "PLAY",
  PAUSE = "PAUSE",
  CHANNEL_POINTS = "CHANNEL_POINTS",
  DROP = "DROP",
  LINK = "LINK",
  ALERT = "ALERT",
  LOCK = "LOCK",
  SUN = "SUN",
  MOON = "MOON",
  GLOBE = "GLOBE",
  SEARCH = "SEARCH",
  BELL = "BELL",
  LOGOUT = "LOGOUT",
  FILE = "FILE",
  PIN = "PIN",
  HOME = "HOME",
  LANGUAGE = "LANGUAGE",
  UNLOCK = "UNLOCK",
  TV2 = "TV2",
  CLOCK = "CLOCK",
  ARROW_UP = "ARROW_UP",
  ARROW_DOWN = "ARROW_DOWN",
  GRIP = "GRIP",
  LOADER = "LOADER",
  VIEWERS = "VIEWERS",
  AT = "AT",
  PACKAGE = "PACKAGE",
  TROPHY = "TROPHY",
  TARGET = "TARGET",
  ZAP = "ZAP",
  STAR = "STAR",
  FLAME = "FLAME",
  RADIO = "RADIO",
  EYE_OFF = "EYE_OFF",
  EYE = "EYE",
  LOG_IN = "LOG_IN",
  CALENDAR = "CALENDAR",
  GRID = "GRID",
  EDIT = "EDIT",
  DISCORD = "DISCORD",
  TELEGRAM = "TELEGRAM",
  WEBHOOK = "WEBHOOK",
  IMPORT = "IMPORT",
  LIST_ORDERED = "LIST_ORDERED",
  VERIFIED = "VERIFIED",
  WATCH_STREAK = "WATCH_STREAK",
  GITHUB = "GITHUB",
}

export enum ThumbSize {
  SM = "SM",
  MD = "MD",
  LG = "LG",
  XL = "XL",
}

export enum LinkLook {
  HOVER = "HOVER",
  ALWAYS = "ALWAYS",
}

export enum LinkSize {
  MD = "MD",
  LG = "LG",
}

export const LINK_SIZES: Record<LinkSize, { text: string; icon: number; fit: string }> = {
  [LinkSize.MD]: { text: "", icon: 13, fit: "truncate" },
  [LinkSize.LG]: { text: "text-lg", icon: 16, fit: "break-all" },
};

export enum LinkMark {
  NONE = "NONE",
  START = "START",
  END = "END",
}

export enum ThumbShape {
  SQUARE = "SQUARE",
  BOX_ART = "BOX_ART",
}

export const THUMB_PIXELS: Record<ThumbSize, number> = {
  [ThumbSize.SM]: 28,
  [ThumbSize.MD]: 36,
  [ThumbSize.LG]: 48,
  [ThumbSize.XL]: 64,
};

export enum AvatarSize {
  XS = "XS",
  SM = "SM",
  MD = "MD",
  LG = "LG",
  XL = "XL",
}

export const AVATAR_CLASSES: Record<AvatarSize, string> = {
  [AvatarSize.XS]: "size-5 text-initial-xs",
  [AvatarSize.SM]: "size-6.5 text-initial-sm",
  [AvatarSize.MD]: "size-8 text-xs",
  [AvatarSize.LG]: "size-11 text-base",
  [AvatarSize.XL]: "size-14 text-lg",
};

export enum ButtonVariant {
  DEFAULT = "DEFAULT",
  PRIMARY = "PRIMARY",
  DANGER = "DANGER",
  OUTLINE = "OUTLINE",
  SUCCESS = "SUCCESS",
  GHOST = "GHOST",
  DANGER_GHOST = "DANGER_GHOST",
}

export const BUTTON_VARIANTS: Record<
  ButtonVariant,
  "default" | "outline" | "ghost" | "destructive"
> = {
  [ButtonVariant.DEFAULT]: "outline",
  [ButtonVariant.PRIMARY]: "default",
  [ButtonVariant.DANGER]: "destructive",
  [ButtonVariant.OUTLINE]: "outline",
  [ButtonVariant.SUCCESS]: "outline",
  [ButtonVariant.GHOST]: "ghost",
  [ButtonVariant.DANGER_GHOST]: "ghost",
};

export const BUTTON_VARIANT_CLASSES: Record<ButtonVariant, string> = {
  [ButtonVariant.DEFAULT]: "",
  [ButtonVariant.PRIMARY]: "",
  [ButtonVariant.DANGER]: "",
  [ButtonVariant.OUTLINE]: "",
  [ButtonVariant.SUCCESS]:
    "border-success-emphasis/40 bg-success text-success-foreground hover:bg-success/70 hover:text-success-foreground",
  [ButtonVariant.GHOST]: "text-muted-foreground",
  [ButtonVariant.DANGER_GHOST]:
    "text-destructive hover:bg-destructive/10 hover:text-destructive dark:hover:bg-destructive/20",
};

export enum ButtonSize {
  MD = "MD",
  SM = "SM",
}

export const BUTTON_SIZES: Record<ButtonSize, "default" | "sm"> = {
  [ButtonSize.MD]: "default",
  [ButtonSize.SM]: "sm",
};

export const ICON_BUTTON_SIZES: Record<ButtonSize, "icon" | "icon-sm"> = {
  [ButtonSize.MD]: "icon",
  [ButtonSize.SM]: "icon-sm",
};

export enum BadgeTone {
  NEUTRAL = "NEUTRAL",
  MUTED = "MUTED",
  PRIMARY = "PRIMARY",
  ACCENT = "ACCENT",
  SUCCESS = "SUCCESS",
  WARNING = "WARNING",
  INFO = "INFO",
  DANGER = "DANGER",
  LIVE = "LIVE",
}

export enum BadgeVariant {
  TONE = "TONE",
  CHIP = "CHIP",
}

export const BADGE_VARIANTS: Record<BadgeVariant, "outline" | "secondary"> = {
  [BadgeVariant.TONE]: "outline",
  [BadgeVariant.CHIP]: "secondary",
};

export const BADGE_TONE_CLASSES: Record<BadgeTone, string> = {
  [BadgeTone.NEUTRAL]: "border-transparent bg-foreground/10 text-foreground",
  [BadgeTone.MUTED]: "border-border bg-transparent text-muted-foreground",
  [BadgeTone.PRIMARY]: "bg-primary text-primary-foreground",
  [BadgeTone.ACCENT]: "border-primary/40 bg-primary/15 text-link",
  [BadgeTone.SUCCESS]: "border-success-emphasis/30 bg-success text-success-foreground",
  [BadgeTone.WARNING]: "border-warning-emphasis/30 bg-warning text-warning-foreground",
  [BadgeTone.INFO]: "border-info-emphasis/30 bg-info text-info-foreground",
  [BadgeTone.DANGER]: "border-destructive/30 bg-destructive/15 text-destructive-foreground",
  [BadgeTone.LIVE]: "bg-live font-semibold tracking-wide text-live-foreground uppercase",
};

export const DOT_TONE_CLASSES: Record<BadgeTone, string> = {
  [BadgeTone.NEUTRAL]: "bg-muted-foreground",
  [BadgeTone.MUTED]: "bg-muted-foreground",
  [BadgeTone.PRIMARY]: "bg-primary",
  [BadgeTone.ACCENT]: "bg-primary",
  [BadgeTone.SUCCESS]: "bg-success-emphasis ring-3 ring-success",
  [BadgeTone.WARNING]: "bg-warning-emphasis",
  [BadgeTone.INFO]: "bg-info-emphasis",
  [BadgeTone.DANGER]: "bg-destructive",
  [BadgeTone.LIVE]: "bg-live-emphasis",
};

export enum MenuAlign {
  LEFT = "LEFT",
  RIGHT = "RIGHT",
}

export const COPIED_FEEDBACK_MS = 1_800;

export enum ButtonType {
  BUTTON = "button",
  SUBMIT = "submit",
}

export const TOOLTIP_DELAY_MS = 400;

export enum CopyLayout {
  ICON = "ICON",
  LABELLED = "LABELLED",
}

export enum MetaLayout {
  ROW = "ROW",
  STACKED = "STACKED",
  INLINE = "INLINE",
}

export const SELECT_SEARCH_FROM = 5;

export enum NoticeTone {
  INFO = "INFO",
  WARNING = "WARNING",
  DANGER = "DANGER",
}

export const NOTICE_TONE_CLASSES: Record<NoticeTone, string> = {
  [NoticeTone.INFO]: "border-info-emphasis/40 bg-info text-info-foreground",
  [NoticeTone.WARNING]: "border-warning-emphasis/40 bg-warning text-warning-foreground",
  [NoticeTone.DANGER]: "border-destructive/40 bg-destructive/10 text-destructive-foreground",
};

export const NOTICE_TONE_ICONS: Record<NoticeTone, IconName> = {
  [NoticeTone.INFO]: IconName.BELL,
  [NoticeTone.WARNING]: IconName.ALERT,
  [NoticeTone.DANGER]: IconName.ALERT,
};

export const MENU_EDGE_GAP = 8;

export const MENU_ITEM_CLASSES =
  "cursor-pointer gap-2.5 rounded-md px-2 py-1.5 data-selected:bg-accent data-selected:text-accent-foreground data-[checked=true]:bg-selected data-[checked=true]:font-medium data-[checked=true]:text-selected-foreground [&>svg:last-child]:hidden data-[checked=true]:[&>svg:last-child]:block";

export const MULTI_ITEM_CLASSES =
  "cursor-pointer gap-2.5 rounded-md px-2 py-1.5 data-selected:bg-accent data-selected:text-accent-foreground [&>svg:last-child]:hidden";

export const MULTI_ALL_VALUE = "__all__";

export const MULTI_PICTURES_MAX = 3;

export const ART_TILES: Record<
  ThumbShape,
  { width: number; height: number; stepX: number; stepY: number }
> = {
  [ThumbShape.BOX_ART]: { width: 27, height: 36, stepX: 30, stepY: 39 },
  [ThumbShape.SQUARE]: { width: 30, height: 30, stepX: 33, stepY: 33 },
};

// ! Never translated, as on Twitch, so it always fits the picture; screen readers
// hear the translated "live" instead.
export const LIVE_TAG = "LIVE";

export const LIVE_TAG_CLASSES: Partial<Record<AvatarSize, string>> = {
  [AvatarSize.SM]: "px-0.5 text-live-tag-sm/3",
  [AvatarSize.MD]: "px-1 text-live-tag/3.5",
  [AvatarSize.LG]: "px-1 text-live-tag/3.5",
  [AvatarSize.XL]: "px-1 text-live-tag/3.5",
};

const FIELD_BUTTON_LOOK =
  "grid cursor-pointer place-items-center text-muted-foreground transition-colors hover:text-foreground focus-visible:text-foreground focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none";

export const FIELD_BUTTON_CLASSES = `absolute inset-y-0 right-0 w-8 rounded-r-lg ${FIELD_BUTTON_LOOK}`;

export const FIELD_CLEAR_BUTTON_CLASSES = `absolute top-1 right-8 size-6 rounded-md ${FIELD_BUTTON_LOOK}`;

export const SEARCH_FIELD_PADDING = "pr-15";

export enum KeyboardKey {
  ENTER = "Enter",
  ESCAPE = "Escape",
  ARROW_UP = "ArrowUp",
  ARROW_DOWN = "ArrowDown",
}

export const CLEAR_SEARCH_KEY = KeyboardKey.ESCAPE;
