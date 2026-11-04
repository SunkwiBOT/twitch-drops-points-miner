const TICK_MS = 30_000;

export const ticker = $state({ now: Date.now() });

setInterval(() => {
  ticker.now = Date.now();
}, TICK_MS);
