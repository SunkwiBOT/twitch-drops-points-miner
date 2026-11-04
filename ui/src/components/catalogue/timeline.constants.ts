import { DAY_MS, HOUR_MS } from "../../lib/time/time.constants";

export enum TimelineZoom {
  DAY = "DAY",
  THREE_DAYS = "THREE_DAYS",
  TWO_WEEKS = "TWO_WEEKS",
}

export const TIMELINE_ZOOMS: readonly TimelineZoom[] = [
  TimelineZoom.DAY,
  TimelineZoom.THREE_DAYS,
  TimelineZoom.TWO_WEEKS,
];

export type ZoomFrameType = { lead: number; align: number; span: number; tick: number };

export const ZOOM_FRAMES: Record<TimelineZoom, ZoomFrameType> = {
  [TimelineZoom.DAY]: { lead: 2 * HOUR_MS, align: HOUR_MS, span: DAY_MS, tick: 2 * HOUR_MS },
  [TimelineZoom.THREE_DAYS]: {
    lead: 6 * HOUR_MS,
    align: 3 * HOUR_MS,
    span: 3 * DAY_MS,
    tick: 6 * HOUR_MS,
  },
  [TimelineZoom.TWO_WEEKS]: {
    lead: 9 * HOUR_MS,
    align: HOUR_MS,
    span: 14 * DAY_MS + 9 * HOUR_MS,
    tick: DAY_MS,
  },
};

export enum BarLabelPlacement {
  INSIDE = "INSIDE",
  LEFT = "LEFT",
  RIGHT = "RIGHT",
}

export enum BarKey {
  PREVIOUS = "ArrowLeft",
  NEXT = "ArrowRight",
  FIRST = "Home",
  LAST = "End",
}

export const TIMELINE_PAGE_SIZE = 36;

export const DEFAULT_LANE_WIDTH = 752;

export const TIMELINE_URGENT_MS = 6 * HOUR_MS;

export const PERCENT = 100;

export const MIN_BAR_PERCENT = 0.4;

export const NAME_CHARACTER_PX = 7;
export const END_CHARACTER_PX = 6.2;
export const PILL_CHARACTER_PX = 6.2;
export const PILL_EXTRA_PX = 16;
export const LABEL_PADDING_PX = 30;

export const TICK_CHARACTER_PX = 7;
export const TICK_LABEL_PADDING_PX = 12;

export const SINGLE_LANE_HEIGHT = 28;
export const STACKED_LANE_HEIGHT = 22;
export const LANE_GAP = 4;
export const SINGLE_LANE_PADDING = 8;
export const STACKED_LANE_PADDING = 7;

export const LABEL_ROOM_PX = 64;
