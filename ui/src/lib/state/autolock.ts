import { reportActivity } from "../api/client";
import type { AutoLock } from "../bindings/AutoLock";
import { MINUTE_MS } from "../time/time.constants";
import {
  ACTIVITY_REPORT_GAP_MS,
  AUTO_LOCK_MINUTES,
  LOCK_CHECK_DELAY_MS,
  LOCK_CHECK_TIMEOUT_MS,
} from "./state.constants";
import { checkSession, sessionEnded, sessionEndedBy } from "./state.svelte";

type WatchedType = { open: boolean; idleMs: number | null };

type TimerType = ReturnType<typeof setTimeout>;

let watched: WatchedType = { open: false, idleMs: null };
let reportedAt = Number.NEGATIVE_INFINITY;
let heardAt = performance.now();
let pendingReport: TimerType | null = null;
let lockTimer: TimerType | null = null;

const stopLockTimer = (): void => {
  if (lockTimer) {
    clearTimeout(lockTimer);
    lockTimer = null;
  }
};

const armLock = (from: number): void => {
  stopLockTimer();

  if (!watched.open || watched.idleMs === null) {
    return;
  }

  const wait = from + watched.idleMs + LOCK_CHECK_DELAY_MS - performance.now();

  lockTimer = setTimeout(() => void lockIfIdle(), Math.max(0, wait));
};

// ! Without the server's answer in LOCK_CHECK_TIMEOUT_MS, nothing says the session
// was in use: the page locks.
const lockIfIdle = async (): Promise<void> => {
  lockTimer = null;

  let timeout: TimerType | undefined;
  const answered = await Promise.race([
    checkSession(),
    new Promise<boolean>((resolve) => {
      timeout = setTimeout(() => resolve(false), LOCK_CHECK_TIMEOUT_MS);
    }),
  ]);

  clearTimeout(timeout);

  if (!answered) {
    sessionEnded();

    return;
  }

  armLock(performance.now());
};

const report = async (): Promise<void> => {
  pendingReport = null;
  reportedAt = performance.now();
  heardAt = reportedAt;
  armLock(heardAt);

  try {
    await reportActivity();
  } catch (e: any) {
    sessionEndedBy(e);
  }
};

// ! Activity within ACTIVITY_REPORT_GAP_MS is reported at its end, so the server
// never takes the user for idle earlier than they were.
export const noteActivity = (): void => {
  if (!watched.open || pendingReport) {
    return;
  }

  const wait = reportedAt + ACTIVITY_REPORT_GAP_MS - performance.now();

  if (wait <= 0) {
    void report();
  } else {
    pendingReport = setTimeout(() => void report(), wait);
  }
};

export const noteReturn = (): void => {
  if (!watched.open || document.visibilityState !== "visible") {
    return;
  }

  if (pendingReport) {
    clearTimeout(pendingReport);
  }

  void report();
};

export const watchSession = (open: boolean, setting: AutoLock | null): void => {
  const minutes = setting === null ? null : AUTO_LOCK_MINUTES[setting];
  const idleMs = minutes === null ? null : minutes * MINUTE_MS;

  if (open === watched.open && idleMs === watched.idleMs) {
    return;
  }

  if (open && !watched.open) {
    heardAt = performance.now();
  }

  watched = { open, idleMs };

  if (!open && pendingReport) {
    clearTimeout(pendingReport);
    pendingReport = null;
  }

  armLock(heardAt);
};
