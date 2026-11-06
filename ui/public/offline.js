// The "no connection" screen the service worker shows when the dashboard cannot be reached:
// in the language and theme the dashboard last used (offline-texts.js comes from messages/).
const TEXTS = self.OFFLINE_TEXTS ?? {};

const recall = (key) => {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
};

const language = (() => {
  const shown = recall("tdpm.language");

  if (shown && TEXTS[shown]) {
    return shown;
  }

  for (const tag of navigator.languages ?? [navigator.language]) {
    if (TEXTS[tag]) {
      return tag;
    }

    const base = tag.split("-")[0];
    const near = Object.keys(TEXTS).find((code) => code.split("-")[0] === base);

    if (near) {
      return near;
    }
  }

  return "en";
})();

const [title, text, retry] = TEXTS[language] ?? TEXTS.en ?? [];

if (title) {
  document.documentElement.lang = language;
  document.title = `${title} · Twitch Miner`;
  document.getElementById("title").textContent = title;
  document.getElementById("text").textContent = text;
  document.getElementById("retry").textContent = retry;
}

try {
  const { theme } = JSON.parse(recall("tdpm.preferences") ?? "{}");

  if (theme === "DARK" || theme === "LIGHT") {
    document.documentElement.dataset.theme = theme;
  }
} catch {
  // The system's light or dark stays.
}

document.getElementById("retry").addEventListener("click", () => location.reload());
