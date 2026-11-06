// ! Only the "no connection" screen is kept on the device: everything else always comes from
// the network, so the dashboard never shows old data and nothing private is stored.
const CACHE = "offline";
const OFFLINE_PAGE = "/offline.html";
const KEPT = [OFFLINE_PAGE, "/offline.js", "/offline-texts.js", "/app-icon.svg", "/favicon.png"];

// ! 502-504: a reverse proxy answering for a server that is down.
const PROXY_DOWN = new Set([502, 503, 504]);

const keep = () => caches.open(CACHE).then((cache) => cache.addAll(KEPT));

self.addEventListener("install", (event) => {
  event.waitUntil(keep().then(() => self.skipWaiting()));
});

self.addEventListener("activate", (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener("fetch", (event) => {
  const { request } = event;

  if (request.method !== "GET") {
    return;
  }

  if (request.mode === "navigate") {
    const offline = () => caches.match(OFFLINE_PAGE);

    event.respondWith(
      fetch(request)
        .then((response) => {
          if (PROXY_DOWN.has(response.status)) {
            return offline().then((page) => page ?? response);
          }

          // A new build may have changed the screen: kept up to date while online.
          event.waitUntil(keep().catch(() => {}));

          return response;
        })
        .catch(() => offline()),
    );

    return;
  }

  const path = new URL(request.url).pathname;

  if (KEPT.includes(path)) {
    event.respondWith(fetch(request).catch(() => caches.match(path)));
  }
});
