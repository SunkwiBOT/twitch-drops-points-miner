import { redeemRewards, sendCommand } from "../api/client";
import type { BurstProgressType } from "../bindings/BurstProgressType";
import type { BurstResultType } from "../bindings/BurstResultType";
import { CommandName } from "../bindings/CommandName";
import type { RedeemRequestType } from "../bindings/RedeemRequestType";
import { SPAM_RESULT_WAIT_MS } from "./spam.constants";
import { newBurstId } from "./spam.utils";

type RunningBurstType = { id: string; accountId: string; total: number };

type BurstOrderType = Omit<RedeemRequestType, "burst_id">;

export const runningBursts: Partial<Record<string, RunningBurstType>> = $state({});

export const burstProgress: Partial<Record<string, number>> = $state({});

type WaitingType = { end: (summary: BurstResultType) => void; restart: () => void };

const waiting = new Map<string, WaitingType>();

export const burstKey = (accountId: string, login: string, rewardId: string): string =>
  `${accountId}/${login}/${rewardId}`;

export const noteBurstProgress = (progress: BurstProgressType): void => {
  if (!(progress.burst_id in burstProgress)) {
    return;
  }

  burstProgress[progress.burst_id] = progress.done;

  const wait = waiting.get(progress.burst_id);

  if (progress.summary) {
    wait?.end(progress.summary);
  } else {
    wait?.restart();
  }
};

const cancelBurst = (id: string): void => {
  // ! Also sent as the page goes away: a refusal (app or dashboard locked) has
  // nowhere to show, and locking the app stops the bursts on its own.
  sendCommand({ command: CommandName.CANCEL_BURST, burst_id: id }).catch(() => undefined);
};

// ! Watched before it is sent: its result can arrive before the answer that
// accepts it. `null` is a result that never came: no word of the burst for a
// while, and the requests it has not sent yet are cancelled.
export const redeemBurst = async (
  order: BurstOrderType,
  delayMs: number,
): Promise<BurstResultType | null> => {
  const key = burstKey(order.account_id, order.login, order.reward_id);
  const id = newBurstId();
  let timer: ReturnType<typeof setTimeout> | undefined;
  const ended = new Promise<BurstResultType | null>((resolve) => {
    const restart = (): void => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        cancelBurst(id);
        resolve(null);
      }, SPAM_RESULT_WAIT_MS + delayMs);
    };

    waiting.set(id, { end: resolve, restart });
    restart();
  });

  runningBursts[key] = { id, accountId: order.account_id, total: order.count };
  burstProgress[id] = 0;

  try {
    await redeemRewards({ ...order, burst_id: id });

    return await ended;
  } finally {
    clearTimeout(timer);
    waiting.delete(id);
    delete runningBursts[key];
    delete burstProgress[id];
  }
};

export const cancelBursts = (accountId: string): void => {
  for (const burst of Object.values(runningBursts)) {
    if (burst?.accountId === accountId) {
      cancelBurst(burst.id);
    }
  }
};
