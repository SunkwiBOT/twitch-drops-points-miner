import { AccountStatus } from "../bindings/AccountStatus";
import { Mode } from "../bindings/Mode";
import { PointsReason } from "../bindings/PointsReason";
import { QueueBlock } from "../bindings/QueueBlock";
import { QueueReason } from "../bindings/QueueReason";
import { BadgeTone, IconName } from "../../components/ui/ui.constants";
import { t } from "../i18n/i18n.svelte";
import { PROXY_CREDENTIALS_MASK } from "./accounts.constants";

export const STATUS_LABELS: Record<AccountStatus, () => string> = {
  [AccountStatus.STARTING]: () => t.status_starting(),
  [AccountStatus.MINING]: () => t.status_mining(),
  [AccountStatus.IDLE]: () => t.status_idle(),
  [AccountStatus.PAUSED]: () => t.status_paused(),
  [AccountStatus.LOGIN_REQUIRED]: () => t.status_login_required(),
};

export const MODE_LABELS: Record<Mode, () => string> = {
  [Mode.AUTO]: () => t.mode_auto_title(),
  [Mode.DROPS]: () => t.mode_drops_title(),
  [Mode.POINTS]: () => t.mode_points_title(),
};

export const STATUS_TONES: Record<AccountStatus, BadgeTone> = {
  [AccountStatus.STARTING]: BadgeTone.INFO,
  [AccountStatus.MINING]: BadgeTone.SUCCESS,
  [AccountStatus.IDLE]: BadgeTone.WARNING,
  [AccountStatus.PAUSED]: BadgeTone.MUTED,
  [AccountStatus.LOGIN_REQUIRED]: BadgeTone.DANGER,
};

export const STATUS_HINTS: Record<AccountStatus, (() => string) | null> = {
  [AccountStatus.STARTING]: null,
  [AccountStatus.MINING]: () => t.badge_hint_mining(),
  [AccountStatus.IDLE]: null,
  [AccountStatus.PAUSED]: () => t.badge_hint_paused(),
  [AccountStatus.LOGIN_REQUIRED]: () => t.badge_hint_login_required(),
};

export const POINTS_REASON_LABELS: Record<PointsReason, () => string> = {
  [PointsReason.STREAK_BONUS]: () => t.points_reason_streak_bonus(),
  [PointsReason.STREAK_AT_RISK]: () => t.points_reason_streak_at_risk(),
  [PointsReason.SUBSCRIBED]: () => t.points_reason_subscribed(),
  [PointsReason.LIST_ORDER]: () => t.points_reason_list_order(),
};

export const POINTS_REASON_HINTS: Record<PointsReason, () => string> = {
  [PointsReason.STREAK_BONUS]: () => t.points_reason_streak_bonus_hint(),
  [PointsReason.STREAK_AT_RISK]: () => t.points_reason_streak_at_risk_hint(),
  [PointsReason.SUBSCRIBED]: () => t.points_reason_subscribed_hint(),
  [PointsReason.LIST_ORDER]: () => t.points_reason_list_order_hint(),
};

export const POINTS_REASON_ICONS: Record<PointsReason, IconName> = {
  [PointsReason.STREAK_BONUS]: IconName.WATCH_STREAK,
  [PointsReason.STREAK_AT_RISK]: IconName.WATCH_STREAK,
  [PointsReason.SUBSCRIBED]: IconName.STAR,
  [PointsReason.LIST_ORDER]: IconName.LIST_ORDERED,
};

export const cleanLogin = (value: string): string => {
  const text = value.trim().toLowerCase();
  const at = text.indexOf("twitch.tv/");

  if (at < 0) {
    return text.replace(/^@+/, "").split("?")[0].replace(/\/+$/, "");
  }

  const parts = text
    .slice(at + "twitch.tv/".length)
    .split(/[/?#]/)
    .filter(Boolean);
  const first = parts.at(0);

  return (first === "popout" || first === "moderator" ? parts.at(1) : first) ?? "";
};

// ! Same rules as `util::normalize_login` in the core.
export const normalizeLogin = (input: string): string | null => {
  const login = cleanLogin(input);

  return /^[a-z0-9_]{1,25}$/.test(login) ? login : null;
};

export const splitEntries = (value: string): string[] =>
  value
    .split(/[\s,;]+/)
    .map((entry) => entry.trim())
    .filter(Boolean);

export const QUEUE_REASON_LABELS: Record<QueueReason, () => string> = {
  [QueueReason.ORDERED]: () => t.queue_reason_pinned(),
  [QueueReason.PRIORITY]: () => t.dashboard_queue_reason_priority(),
  [QueueReason.ENDING_SOON]: () => t.dashboard_queue_reason_ending_soon(),
  [QueueReason.LOWEST_REMAINING]: () => t.dashboard_queue_reason_lowest_remaining(),
};

export const QUEUE_REASON_HINTS: Record<QueueReason, () => string> = {
  [QueueReason.ORDERED]: () => t.badge_hint_reason_ordered(),
  [QueueReason.PRIORITY]: () => t.badge_hint_reason_priority(),
  [QueueReason.ENDING_SOON]: () => t.badge_hint_reason_ending_soon(),
  [QueueReason.LOWEST_REMAINING]: () => t.badge_hint_reason_lowest_remaining(),
};

export const QUEUE_BLOCK_LABELS: Record<QueueBlock, () => string> = {
  [QueueBlock.NO_LIVE_CHANNEL]: () => t.dashboard_queue_blocked_no_live_channel(),
  [QueueBlock.ONE_CAMPAIGN_AT_A_TIME]: () => t.dashboard_queue_blocked_one_campaign_at_a_time(),
  [QueueBlock.NO_DROP_AVAILABLE]: () => t.dashboard_queue_blocked_no_drop_available(),
};

export const QUEUE_BLOCK_HINTS: Record<QueueBlock, () => string> = {
  [QueueBlock.NO_LIVE_CHANNEL]: () => t.badge_hint_blocked_no_live(),
  [QueueBlock.ONE_CAMPAIGN_AT_A_TIME]: () => t.badge_hint_blocked_one_at_a_time(),
  [QueueBlock.NO_DROP_AVAILABLE]: () => t.badge_hint_blocked_no_drop(),
};

// ! Shown as the hint of an account's proxy, the default proxy keeps its user and
// password masked: a field's hint is never hidden like its text.
export const maskProxyCredentials = (proxy: string): string =>
  proxy.replace(
    /^([a-z][a-z0-9+.-]*:\/\/)?([^/]*)@/i,
    (_match, scheme: string | undefined, credentials: string) =>
      `${scheme ?? ""}${credentials
        .split(":")
        .slice(0, 2)
        .map(() => PROXY_CREDENTIALS_MASK)
        .join(":")}@`,
  );
