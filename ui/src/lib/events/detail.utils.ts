import { EventKind } from "../bindings/EventKind";
import type { MinerEventType } from "../bindings/MinerEventType";
import { number } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { reasonsLabel } from "../spam/spam.utils";

export type RedeemedRewardType = { url: string; color: string | undefined };

const burstDetail = (event: MinerEventType): string => {
  const { count = 0, sent = 0, cancelled = 0, refusals = [], failures = [] } = event.context;
  const refused = count - sent - cancelled;

  return [
    refused > 0 ? t.ev_rewards_refused({ refused, reason: reasonsLabel(refusals, failures) }) : "",
    cancelled > 0 ? t.spam_cancelled({ count: cancelled, shown: number(cancelled) }) : "",
  ]
    .filter(Boolean)
    .join(" · ");
};

export const eventDetail = (event: MinerEventType): string | undefined =>
  event.kind === EventKind.REWARD_REDEEMED ? burstDetail(event) : event.detail;

export const redeemedReward = (event: MinerEventType): RedeemedRewardType | null =>
  event.kind === EventKind.REWARD_REDEEMED && event.context.reward
    ? { url: event.context.reward, color: event.context.reward_color }
    : null;
