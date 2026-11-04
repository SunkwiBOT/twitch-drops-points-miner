import type { BetFilterType } from "../bindings/BetFilterType";
import { FilterBy } from "../bindings/FilterBy";
import type { PredictionSettingsType } from "../bindings/PredictionSettingsType";
import type { StreamerSettingsType } from "../bindings/StreamerSettingsType";
import { Strategy } from "../bindings/Strategy";
import { t } from "../i18n/i18n.svelte";
import { StreamerOption } from "./accounts.constants";

type OptionValueType = boolean | number | string | null;

type OptionAccessType = {
  read: (streamer: StreamerSettingsType) => OptionValueType;
  copy: (from: StreamerSettingsType, to: StreamerSettingsType) => void;
};

type StreamerFieldType = keyof Omit<StreamerSettingsType, "login" | "enabled" | "predictions">;
type BetFieldType = keyof Omit<PredictionSettingsType, "filter">;

const field = <K extends StreamerFieldType>(key: K): OptionAccessType => ({
  read: (streamer) => streamer[key],
  copy: (from, to) => {
    to[key] = from[key];
  },
});

const bet = <K extends BetFieldType>(key: K): OptionAccessType => ({
  read: (streamer) => streamer.predictions[key],
  copy: (from, to) => {
    to.predictions[key] = from.predictions[key];
  },
});

const condition = <K extends keyof BetFilterType>(key: K): OptionAccessType => ({
  read: (streamer) => streamer.predictions.filter?.[key] ?? null,
  copy: (from, to) => {
    const source = from.predictions.filter;
    const target = to.predictions.filter;

    if (source && target) {
      target[key] = source[key];
    }
  },
});

const OPTIONS: Record<StreamerOption, OptionAccessType> = {
  [StreamerOption.CLAIM_BONUS]: field("claim_bonus"),
  [StreamerOption.WATCH_STREAK]: field("watch_streak"),
  [StreamerOption.FOLLOW_RAIDS]: field("follow_raids"),
  [StreamerOption.CLAIM_MOMENTS]: field("claim_moments"),
  [StreamerOption.COMMUNITY_GOALS]: field("community_goals"),
  [StreamerOption.CHAT]: field("chat"),
  [StreamerOption.PREDICTIONS]: bet("enabled"),
  [StreamerOption.STRATEGY]: bet("strategy"),
  [StreamerOption.TIMING]: bet("timing"),
  [StreamerOption.PERCENTAGE]: bet("percentage"),
  [StreamerOption.MAX_POINTS]: bet("max_points"),
  [StreamerOption.MIN_POINTS]: bet("min_points"),
  [StreamerOption.DELAY_SECONDS]: bet("delay_seconds"),
  [StreamerOption.TIMING_PERCENT]: bet("timing_percent"),
  [StreamerOption.PERCENTAGE_GAP]: bet("percentage_gap"),
  [StreamerOption.STEALTH]: bet("stealth"),
  [StreamerOption.FILTER]: {
    read: (streamer) => Boolean(streamer.predictions.filter),
    copy: (from, to) => {
      const source = from.predictions.filter;

      to.predictions.filter = source ? { ...source } : undefined;
    },
  },
  [StreamerOption.FILTER_BY]: condition("by"),
  [StreamerOption.FILTER_OP]: condition("op"),
  [StreamerOption.FILTER_VALUE]: condition("value"),
};

const STREAMER_OPTION_LABELS: Record<StreamerOption, () => string> = {
  [StreamerOption.CLAIM_BONUS]: () => t.streamer_claim_bonus(),
  [StreamerOption.WATCH_STREAK]: () => t.streamer_watch_streak(),
  [StreamerOption.FOLLOW_RAIDS]: () => t.streamer_follow_raids(),
  [StreamerOption.CLAIM_MOMENTS]: () => t.streamer_claim_moments(),
  [StreamerOption.COMMUNITY_GOALS]: () => t.streamer_community_goals(),
  [StreamerOption.CHAT]: () => t.streamer_chat(),
  [StreamerOption.PREDICTIONS]: () => t.streamer_predictions_enabled(),
  [StreamerOption.STRATEGY]: () => t.streamer_predictions_strategy(),
  [StreamerOption.TIMING]: () => t.streamer_predictions_timing(),
  [StreamerOption.PERCENTAGE]: () => t.streamer_predictions_percentage(),
  [StreamerOption.MAX_POINTS]: () => t.streamer_predictions_max(),
  [StreamerOption.MIN_POINTS]: () => t.streamer_predictions_min(),
  [StreamerOption.DELAY_SECONDS]: () => t.streamer_predictions_delay(),
  [StreamerOption.TIMING_PERCENT]: () => t.streamer_predictions_timing_percent(),
  [StreamerOption.PERCENTAGE_GAP]: () => t.streamer_predictions_gap(),
  [StreamerOption.STEALTH]: () => t.streamer_predictions_stealth(),
  [StreamerOption.FILTER]: () => t.streamer_predictions_filter(),
  [StreamerOption.FILTER_BY]: () => t.streamer_predictions_filter(),
  [StreamerOption.FILTER_OP]: () => t.streamer_predictions_filter(),
  [StreamerOption.FILTER_VALUE]: () => t.streamer_predictions_filter(),
};

const PARTS: Partial<Record<StreamerOption, StreamerOption[]>> = {
  [StreamerOption.FILTER]: [
    StreamerOption.FILTER_BY,
    StreamerOption.FILTER_OP,
    StreamerOption.FILTER_VALUE,
  ],
};

export const STREAMER_OPTIONS = Object.values(StreamerOption);

export const withParts = (option: StreamerOption): StreamerOption[] => [
  option,
  ...(PARTS[option] ?? []),
];

export const mixedOptions = (streamers: StreamerSettingsType[]): StreamerOption[] =>
  STREAMER_OPTIONS.filter((option) => {
    const values = new Set(streamers.map((streamer) => OPTIONS[option].read(streamer)));

    return values.size > 1;
  });

export const differingOptions = (
  a: StreamerSettingsType,
  b: StreamerSettingsType,
  among: StreamerOption[],
): StreamerOption[] =>
  among.filter((option) => OPTIONS[option].read(a) !== OPTIONS[option].read(b));

export const sameOptions = (a: StreamerSettingsType, b: StreamerSettingsType): boolean =>
  differingOptions(a, b, STREAMER_OPTIONS).length === 0;

// ! In the enum's order, whatever the order given: a condition before its parts.
export const copyOptions = (
  from: StreamerSettingsType,
  to: StreamerSettingsType,
  options: StreamerOption[],
): void => {
  for (const option of STREAMER_OPTIONS.filter((item) => options.includes(item))) {
    OPTIONS[option].copy(from, to);
  }
};

export const optionLabels = (options: StreamerOption[]): string[] => [
  ...new Set(options.map((option) => STREAMER_OPTION_LABELS[option]())),
];

export const STRATEGY_LABELS: Record<Strategy, () => string> = {
  [Strategy.SMART]: () => t.strategy_smart(),
  [Strategy.MOST_VOTED]: () => t.strategy_most_voted(),
  [Strategy.HIGH_ODDS]: () => t.strategy_high_odds(),
  [Strategy.PERCENTAGE]: () => t.strategy_percentage(),
  [Strategy.SMART_MONEY]: () => t.strategy_smart_money(),
  [Strategy.NUMBER1]: () => t.strategy_number_1(),
  [Strategy.NUMBER2]: () => t.strategy_number_2(),
  [Strategy.NUMBER3]: () => t.strategy_number_3(),
  [Strategy.NUMBER4]: () => t.strategy_number_4(),
};

export const FILTER_BY_LABELS: Record<FilterBy, () => string> = {
  [FilterBy.ODDS]: () => t.filter_by_odds(),
  [FilterBy.ODDS_PERCENTAGE]: () => t.filter_by_odds_percentage(),
  [FilterBy.PERCENTAGE_USERS]: () => t.filter_by_percentage_users(),
  [FilterBy.TOTAL_USERS]: () => t.filter_by_total_users(),
  [FilterBy.TOTAL_POINTS]: () => t.filter_by_total_points(),
  [FilterBy.DECISION_USERS]: () => t.filter_by_decision_users(),
  [FilterBy.DECISION_POINTS]: () => t.filter_by_decision_points(),
};
