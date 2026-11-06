import { mount } from "svelte";

import "@fontsource-variable/inter";
import "./theme.css";

import App from "./components/layout/App.svelte";
import { isTauri } from "./lib/api/tauri";
import { prepareLanguage } from "./lib/i18n/i18n.svelte";
import { start } from "./lib/state/state.svelte";
import { followUpdates } from "./lib/updates/updates.svelte";

window.addEventListener("contextmenu", (event) => {
  const editable =
    event.target instanceof Element && event.target.closest("input, textarea, [contenteditable]");

  if (!editable && !window.getSelection()?.toString()) {
    event.preventDefault();
  }
});

// ! The web dashboard only, and only where a browser allows a service worker (HTTPS or
// localhost): it shows a "no connection" screen offline. The dashboard works the same without.
if (!isTauri() && import.meta.env.PROD && window.isSecureContext && "serviceWorker" in navigator) {
  navigator.serviceWorker
    .register("/sw.js", { updateViaCache: "none" })
    .catch((e: unknown) => console.warn("the offline screen could not be set up", e));
}

const target = document.getElementById("app");

if (target) {
  prepareLanguage()
    .then(() => {
      mount(App, { target });

      return start();
    })
    .then(followUpdates)
    .catch((e: unknown) => console.error("the interface could not start", e));
}
