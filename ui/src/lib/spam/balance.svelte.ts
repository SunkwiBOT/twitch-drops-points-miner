import type { SpamBalanceType } from "../bindings/SpamBalanceType";
import { sessionEndedBy } from "../state/state.svelte";
import { BalanceTracking, SPAM_POLL_FAILURES_SHOWN, SPAM_POLL_MS } from "./spam.constants";

const watched = (): boolean => document.hasFocus() && document.visibilityState === "visible";

export class BalanceTracker {
  current = $state<SpamBalanceType | null>(null);

  #failures = $state(0);
  #focused = $state(watched());
  #reading = false;
  #loads = 0;
  #fetch: () => Promise<SpamBalanceType>;

  readonly tracking = $derived.by((): BalanceTracking => {
    if (!this.#focused) {
      return BalanceTracking.PAUSED;
    }

    return this.#failures >= SPAM_POLL_FAILURES_SHOWN
      ? BalanceTracking.FAILING
      : BalanceTracking.LIVE;
  });

  constructor(fetch: () => Promise<SpamBalanceType>, polling: () => boolean) {
    this.#fetch = fetch;

    $effect(() => {
      const sync = (): void => {
        this.#focused = watched();
      };

      window.addEventListener("focus", sync);
      window.addEventListener("blur", sync);
      document.addEventListener("visibilitychange", sync);

      return () => {
        window.removeEventListener("focus", sync);
        window.removeEventListener("blur", sync);
        document.removeEventListener("visibilitychange", sync);
      };
    });

    $effect(() => {
      if (!this.#focused || !polling()) {
        return;
      }

      const timer = setInterval(() => void this.read(), SPAM_POLL_MS);

      return () => clearInterval(timer);
    });
  }

  loaded(balance: SpamBalanceType): void {
    this.#loads += 1;
    this.current = balance;
    this.#failures = 0;
  }

  // ! A failed read is tried again at the next tick: a toast every 2.5 s would
  // bury the page.
  read = async (): Promise<void> => {
    if (this.#reading) {
      return;
    }

    this.#reading = true;

    const loads = this.#loads;

    try {
      const read = await this.#fetch();

      // ! A full reload landed meanwhile: its balance is newer than this read.
      if (loads !== this.#loads) {
        return;
      }

      this.#failures = 0;

      if (read.balance !== this.current?.balance || read.changed_at !== this.current.changed_at) {
        this.current = read;
      }
    } catch (e: any) {
      if (!sessionEndedBy(e)) {
        this.#failures += 1;
      }
    } finally {
      this.#reading = false;
    }
  };
}
