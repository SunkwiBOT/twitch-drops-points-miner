import { BadgeTone, IconName } from "../../components/ui/ui.constants";
import type { BurstResultType } from "../bindings/BurstResultType";
import type { ErrorApiMessage } from "../bindings/ErrorApiMessage";
import type { SpamPointsSettingsType } from "../bindings/SpamPointsSettingsType";
import type { SpamRewardType } from "../bindings/SpamRewardType";
import { describeCode } from "../errors/errors.utils";
import { mmss, number, shortDuration } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { HOUR_MS, MINUTE_MS, SECOND_MS } from "../time/time.constants";
import { relativeTime } from "../time/time.utils";
import { BalanceTracking, RewardStatus, SPAM_MAX_COUNT } from "./spam.constants";

// ! Keyed on Twitch's own codes (RedeemCustomReward's `error.code`), as Twitch sends them.
const REFUSAL_LABELS: Partial<Record<string, () => string>> = {
  NOT_ENOUGH_POINTS: () => t.spam_reason_not_enough_points(),
  MAX_PER_STREAM: () => t.spam_reason_max_per_stream(),
  MAX_PER_USER_PER_STREAM: () => t.spam_reason_max_per_stream(),
  GLOBAL_COOLDOWN: () => t.spam_reason_cooldown(),
  DISABLED: () => t.spam_reason_disabled(),
  PAUSED: () => t.spam_reason_paused(),
  OUT_OF_STOCK: () => t.spam_reason_out_of_stock(),
  PROPERTIES_MISMATCH: () => t.spam_reason_text_required(),
};

const refusalLabel = (code: string): string =>
  REFUSAL_LABELS[code]?.() ?? t.spam_reason_unknown({ code });

export const reasonsLabel = (
  refusals: readonly string[],
  failures: readonly ErrorApiMessage[],
): string =>
  [...new Set([...refusals.map(refusalLabel), ...failures.map(describeCode)])].join(", ");

export const burstOutcome = (count: number, result: BurstResultType): string => {
  const { sent, cancelled } = result;
  const reasons = [
    reasonsLabel(result.refusals, result.failures),
    cancelled > 0 ? t.spam_cancelled({ count: cancelled, shown: number(cancelled) }) : "",
  ]
    .filter(Boolean)
    .join(", ");

  if (sent === count) {
    return t.spam_sent_all({ count, shown: number(count) });
  }

  return sent > 0
    ? t.spam_sent_partial({ sent: number(sent), count: number(count), reason: reasons })
    : t.spam_sent_none({ reason: reasons });
};

export const BALANCE_HINTS: Record<BalanceTracking, () => string> = {
  [BalanceTracking.LIVE]: () => t.spam_balance_live_hint(),
  [BalanceTracking.PAUSED]: () => t.spam_balance_paused_hint(),
  [BalanceTracking.FAILING]: () => t.spam_balance_failing_hint(),
};

export const rewardStatus = (reward: SpamRewardType, now: number): RewardStatus => {
  if (!reward.is_enabled) {
    return RewardStatus.DISABLED;
  }

  if (reward.is_paused) {
    return RewardStatus.PAUSED;
  }

  if (!reward.is_in_stock) {
    return RewardStatus.OUT_OF_STOCK;
  }

  return reward.cooldown_expires_at && Date.parse(reward.cooldown_expires_at) > now
    ? RewardStatus.COOLDOWN
    : RewardStatus.AVAILABLE;
};

// ! A reward both paused and out of stock needs both filters on.
export const shownUnavailable = (
  reward: SpamRewardType,
  settings: SpamPointsSettingsType,
): boolean =>
  (settings.show_disabled || (reward.is_enabled && !reward.is_paused)) &&
  (settings.show_out_of_stock || reward.is_in_stock);

export const UNAVAILABLE_BADGES: Partial<
  Record<RewardStatus, { label: () => string; hint: () => string; tone: BadgeTone; icon: IconName }>
> = {
  [RewardStatus.DISABLED]: {
    label: () => t.spam_reward_disabled(),
    hint: () => t.spam_hint_disabled(),
    tone: BadgeTone.MUTED,
    icon: IconName.EYE_OFF,
  },
  [RewardStatus.PAUSED]: {
    label: () => t.spam_reward_paused(),
    hint: () => t.spam_hint_paused(),
    tone: BadgeTone.MUTED,
    icon: IconName.PAUSE,
  },
  [RewardStatus.OUT_OF_STOCK]: {
    label: () => t.spam_reward_out_of_stock(),
    hint: () => t.spam_hint_out_of_stock(),
    tone: BadgeTone.WARNING,
    icon: IconName.PACKAGE,
  },
};

export const maxCount = (cost: number, balance: number): number =>
  cost <= 0 ? SPAM_MAX_COUNT : Math.min(SPAM_MAX_COUNT, Math.floor(balance / cost));

export const countdown = (ms: number): string => (ms < HOUR_MS ? mmss(ms) : shortDuration(ms));

export const changedAgo = (iso: string, now: number): string => {
  const ms = now - Date.parse(iso);

  return ms < MINUTE_MS
    ? t.spam_seconds_ago({ count: Math.max(1, Math.round(ms / SECOND_MS)) })
    : relativeTime(iso, now);
};

// ! Not `crypto.randomUUID()`: it needs a secure context, and a dashboard on the
// local network is plain HTTP.
export const newBurstId = (): string =>
  Array.from(crypto.getRandomValues(new Uint8Array(16)), (byte) =>
    byte.toString(16).padStart(2, "0"),
  ).join("");
