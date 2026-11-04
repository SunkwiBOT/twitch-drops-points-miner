import { t } from "../i18n/i18n.svelte";
import { TWITCH_CATEGORY_URL, TWITCH_SEARCH_URL } from "../partners.constants";
import { strong } from "../text.utils";
import { relativeTime } from "../time/time.utils";

export const campaignEnd = (endsAt: string, now: number): string => {
  const time = strong(relativeTime(endsAt, now));

  return new Date(endsAt).getTime() > now
    ? t.dashboard_ends_in({ time })
    : t.dashboard_ended_ago({ time });
};

// ! Same rule as `util::game_slug` in the core.
const gameSlug = (name: string): string =>
  name
    .replace(/['\u2019]/g, "")
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");

export const categoryUrl = (name: string, slug: string | undefined): string => {
  const path = slug || gameSlug(name);

  return path
    ? `${TWITCH_CATEGORY_URL}/${path}`
    : `${TWITCH_SEARCH_URL}?term=${encodeURIComponent(name)}`;
};
