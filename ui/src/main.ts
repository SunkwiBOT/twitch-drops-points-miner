import { mount } from "svelte";

import "@fontsource-variable/inter";
import "./theme.css";

import App from "./components/layout/App.svelte";
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
