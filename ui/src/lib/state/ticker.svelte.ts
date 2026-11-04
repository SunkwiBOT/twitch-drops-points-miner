const TICK_MS = 30_000;

export const ticker = $state({ now: Date.now() });

// ! It lives as long as the app, so the interval is never cleared.
setInterval(() => {
  ticker.now = Date.now();
}, TICK_MS);
