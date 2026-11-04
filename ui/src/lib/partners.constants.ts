export const DROPS_API_NAME = "Twitch Drops API";

type PartnerType = { id: string; url: string; mark: string; name: string };

export const PARTNERS: readonly PartnerType[] = [
  {
    id: "drops",
    url: "https://drops.sunkwi.com",
    mark: "/partners/twitch-drops-api.svg",
    name: DROPS_API_NAME,
  },
  {
    id: "twitch",
    url: "https://twitch.sunkwi.com",
    mark: "/partners/twitch-alerts.svg",
    name: "Twitch Alerts 🔔",
  },
  {
    id: "kick",
    url: "https://kick.sunkwi.com",
    mark: "/partners/kick-alerts.svg",
    name: "Kick Alerts 🔔",
  },
];

export const ROTATION_MS = 30_000;
export const ROTATION_FADE_MS = 350;
export const REPO_URL = "https://github.com/SunkwiBOT/twitch-drops-points-miner";
export const ISSUES_URL = `${REPO_URL}/issues/new/choose`;
export const DISCORD_URL = "https://discord.gg/support-bots-891685895781113886";
export const DISCORD_NAME = "Discord";
export const GITHUB_NAME = "GitHub";
export const DROPS_WEB_URL = "https://drops.sunkwi.com";
export const TWITCH_CATEGORY_URL = "https://www.twitch.tv/directory/category";
export const TWITCH_SEARCH_URL = "https://www.twitch.tv/search";
export const TWITCH_CONNECTIONS_URL = "https://www.twitch.tv/settings/connections";
