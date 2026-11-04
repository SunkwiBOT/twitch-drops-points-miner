export const SPAM_MAX_COUNT = 25;

export const SPAM_MAX_DELAY_MS = 5_000;

export const SPAM_DELAY_STEP_MS = 50;

export const SPAM_POLL_MS = 2_500;

export const SPAM_POLL_FAILURES_SHOWN = 3;

export const SPAM_CLOCK_MS = 1_000;

export const SPAM_MESSAGE_MAX = 200;

// ! Counted again from each answer of the burst: a request waits for a free
// slot, then up to 20 s for Twitch. Silent longer, the message carrying the
// result was lost with the push channel.
export const SPAM_RESULT_WAIT_MS = 90_000;

export enum BalanceTracking {
  LIVE = "LIVE",
  PAUSED = "PAUSED",
  FAILING = "FAILING",
}

export enum RewardStatus {
  AVAILABLE = "AVAILABLE",
  COOLDOWN = "COOLDOWN",
  DISABLED = "DISABLED",
  PAUSED = "PAUSED",
  OUT_OF_STOCK = "OUT_OF_STOCK",
}

export const UNAVAILABLE_STATUSES: ReadonlySet<RewardStatus> = new Set([
  RewardStatus.DISABLED,
  RewardStatus.PAUSED,
  RewardStatus.OUT_OF_STOCK,
]);

export const BALANCE_DOTS: Record<BalanceTracking, string> = {
  [BalanceTracking.LIVE]: "bg-success-emphasis",
  [BalanceTracking.PAUSED]: "bg-muted-foreground",
  [BalanceTracking.FAILING]: "bg-destructive",
};

export const SPAM_SKELETON_CARDS = 6;
