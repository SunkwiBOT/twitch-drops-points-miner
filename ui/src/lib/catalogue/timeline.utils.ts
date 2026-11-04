import { CampaignStatus } from "../bindings/CampaignStatus";
import type { CampaignType } from "../bindings/CampaignType";
import type { GameType } from "../bindings/GameType";
import { DAY_MS } from "../time/time.constants";
import { zoneOffsetMs } from "../time/time.utils";
import {
  BarKey,
  BarLabelPlacement,
  END_CHARACTER_PX,
  LABEL_PADDING_PX,
  LANE_GAP,
  MIN_BAR_PERCENT,
  NAME_CHARACTER_PX,
  PERCENT,
  PILL_CHARACTER_PX,
  PILL_EXTRA_PX,
  SINGLE_LANE_HEIGHT,
  SINGLE_LANE_PADDING,
  STACKED_LANE_HEIGHT,
  STACKED_LANE_PADDING,
  TICK_CHARACTER_PX,
  TICK_LABEL_PADDING_PX,
  ZOOM_FRAMES,
} from "../../components/catalogue/timeline.constants";
import type { TimelineZoom } from "../../components/catalogue/timeline.constants";

export const toWallClock = (ms: number, zone: string): number => ms + zoneOffsetMs(ms, zone);

// ! The second pass takes the offset in force at the instant, not at `wall` read
// as UTC: a time the clocks skip (daylight saving) gives the end of the jump.
export const fromWallClock = (wall: number, zone: string): number => {
  const time = wall - zoneOffsetMs(wall - zoneOffsetMs(wall, zone), zone);
  const shown = toWallClock(time, zone);

  return shown >= wall ? time : time + (wall - shown);
};

export const floorOnClock = (ms: number, step: number, zone: string): number =>
  fromWallClock(Math.floor(toWallClock(ms, zone) / step) * step, zone);

export const zoneMidnight = (ms: number, zone: string): number => floorOnClock(ms, DAY_MS, zone);

export const nextZoneMidnight = (ms: number, zone: string): number =>
  fromWallClock(Math.floor(toWallClock(ms, zone) / DAY_MS) * DAY_MS + DAY_MS, zone);

export type TimelineRangeType = {
  start: number;
  span: number;
  tick: number;
};

export const getTimelineRange = (
  zoom: TimelineZoom,
  now: number,
  zone: string,
): TimelineRangeType => {
  const frame = ZOOM_FRAMES[zoom];

  return {
    start: floorOnClock(now - frame.lead, frame.align, zone),
    span: frame.span,
    tick: frame.tick,
  };
};

export const toRangePercent = (range: TimelineRangeType, time: number): number =>
  ((time - range.start) / range.span) * PERCENT;

export type TimelineTickType = {
  time: number;
  percent: number;
  isMidnight: boolean;
  isFirstOfMonth: boolean;
};

export const getTimelineTicks = (range: TimelineRangeType, zone: string): TimelineTickType[] => {
  const end = range.start + range.span;
  const lastWall = toWallClock(end, zone);
  const ticks: TimelineTickType[] = [];
  let previous = Number.NEGATIVE_INFINITY;

  for (
    let wall = Math.ceil(toWallClock(range.start, zone) / range.tick) * range.tick;
    wall <= lastWall;
    wall += range.tick
  ) {
    const time = fromWallClock(wall, zone);
    const isMidnight = wall % DAY_MS === 0;
    const shows = isMidnight || toWallClock(time, zone) === wall;

    if (shows && time > previous && time >= range.start && time < end) {
      ticks.push({
        time,
        percent: toRangePercent(range, time),
        isMidnight,
        isFirstOfMonth: isMidnight && new Date(wall).getUTCDate() === 1,
      });
      previous = time;
    }
  }

  return ticks;
};

export const getTickLabelStep = (
  labels: readonly string[],
  range: TimelineRangeType,
  laneWidth: number,
): number => {
  const spacing = (range.tick / range.span) * laneWidth;
  const longest = Math.max(0, ...labels.map((label) => label.length));
  const needed = longest * TICK_CHARACTER_PX + TICK_LABEL_PADDING_PX;

  return spacing > 0 ? Math.max(1, Math.ceil(needed / spacing)) : 1;
};

export type BandType = { left: number; width: number };

export const getTodayBand = (range: TimelineRangeType, now: number, zone: string): BandType => {
  const left = Math.max(0, toRangePercent(range, zoneMidnight(now, zone)));
  const right = Math.min(PERCENT, toRangePercent(range, nextZoneMidnight(now, zone)));

  return { left, width: Math.max(0, right - left) };
};

export type BarGeometryType = {
  isCutLeft: boolean;
  isCutRight: boolean;
  left: number;
  width: number;
  progress: number;
};

export const getBarGeometry = (
  range: TimelineRangeType,
  start: number,
  end: number,
  now: number,
): BarGeometryType | null => {
  const rangeEnd = range.start + range.span;
  const visibleStart = Math.max(start, range.start);
  const visibleEnd = Math.min(end, rangeEnd);

  if (!(visibleEnd > visibleStart)) {
    return null;
  }

  const left = toRangePercent(range, visibleStart);
  const width = Math.max(toRangePercent(range, visibleEnd) - left, MIN_BAR_PERCENT);
  const run = ((now - visibleStart) / (visibleEnd - visibleStart)) * PERCENT;

  return {
    isCutLeft: start < range.start,
    isCutRight: end > rangeEnd,
    left,
    width,
    progress: Math.min(PERCENT, Math.max(0, run)),
  };
};

export const estimateLabelWidth = (name: string, endLabel: string, pill: string): number =>
  name.length * NAME_CHARACTER_PX +
  endLabel.length * END_CHARACTER_PX +
  (pill ? pill.length * PILL_CHARACTER_PX + PILL_EXTRA_PX : 0) +
  LABEL_PADDING_PX;

export const getLabelPlacement = (
  geometry: Pick<BarGeometryType, "left" | "width">,
  laneWidth: number,
  labelWidth: number,
): BarLabelPlacement => {
  const barPx = (geometry.width / PERCENT) * laneWidth;

  if (barPx >= labelWidth) {
    return BarLabelPlacement.INSIDE;
  }

  const roomRight = ((PERCENT - geometry.left - geometry.width) / PERCENT) * laneWidth;

  if (roomRight > labelWidth) {
    return BarLabelPlacement.RIGHT;
  }

  const roomLeft = (geometry.left / PERCENT) * laneWidth;

  if (roomLeft > labelWidth) {
    return BarLabelPlacement.LEFT;
  }

  return BarLabelPlacement.INSIDE;
};

export type TimelineItemType = { campaign: CampaignType; start: number; end: number };

export type TimelineRowType = {
  game: GameType;
  items: TimelineItemType[];
  rewards: number;
  firstEnd: number;
};

const byEnd = (a: TimelineItemType, b: TimelineItemType): number =>
  a.end - b.end || a.campaign.name.localeCompare(b.campaign.name);

export const toTimelineRows = (campaigns: readonly CampaignType[]): TimelineRowType[] => {
  const rows = new Map<string, TimelineRowType>();

  for (const campaign of campaigns) {
    const item: TimelineItemType = {
      campaign,
      start: Date.parse(campaign.starts_at),
      end: Date.parse(campaign.ends_at),
    };
    const row = rows.get(campaign.game.id);

    if (row) {
      row.items.push(item);
      row.rewards += campaign.drops.length;
      row.firstEnd = Math.min(row.firstEnd, item.end);
    } else {
      rows.set(campaign.game.id, {
        game: campaign.game,
        items: [item],
        rewards: campaign.drops.length,
        firstEnd: item.end,
      });
    }
  }

  const list = [...rows.values()];

  for (const row of list) {
    row.items.sort(byEnd);
  }

  return list.sort((a, b) => a.firstEnd - b.firstEnd || a.game.name.localeCompare(b.game.name));
};

export type LaneLayoutType = {
  height: number;
  laneHeight: number;
  padding: number;
  gap: number;
};

export const getLaneLayout = (campaignCount: number): LaneLayoutType => {
  const isSingle = campaignCount <= 1;
  const laneHeight = isSingle ? SINGLE_LANE_HEIGHT : STACKED_LANE_HEIGHT;
  const padding = isSingle ? SINGLE_LANE_PADDING : STACKED_LANE_PADDING;
  const lanes = Math.max(1, campaignCount);

  return {
    height: padding * 2 + lanes * laneHeight + (lanes - 1) * LANE_GAP,
    laneHeight,
    padding,
    gap: LANE_GAP,
  };
};

export const getLaneTop = (layout: LaneLayoutType, index: number): number =>
  layout.padding + index * (layout.laneHeight + layout.gap);

export const getNextBarIndex = (index: number, key: string, count: number): number | null => {
  if (index < 0 || count === 0) {
    return null;
  }

  let next = index;

  switch (key) {
    case BarKey.PREVIOUS:
      next = Math.max(0, index - 1);
      break;
    case BarKey.NEXT:
      next = Math.min(count - 1, index + 1);
      break;
    case BarKey.FIRST:
      next = 0;
      break;
    case BarKey.LAST:
      next = count - 1;
      break;
  }

  return next === index ? null : next;
};

export const runningCampaigns = (campaigns: CampaignType[], now: number): CampaignType[] =>
  campaigns.filter(
    (c) => c.status === CampaignStatus.ACTIVE && new Date(c.ends_at).getTime() > now,
  );
