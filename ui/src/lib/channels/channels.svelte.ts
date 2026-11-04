import { fetchBanners, lookupChannels } from "../api/client";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { ChannelType } from "../bindings/ChannelType";
import type { SearchChannelType } from "../bindings/SearchChannelType";
import { app } from "../state/state.svelte";
import { CHANNELS_LOOKUP_MAX } from "./channels.constants";

export const knownChannels: Partial<Record<string, ChannelType>> = $state({});

const asked = new Set<string>();
let queued: string[] = [];
let flushing = false;

export const rememberChannels = (channels: ChannelType[]): void => {
  for (const channel of channels) {
    knownChannels[channel.login] = channel;
  }
};

export const accountChannel = (
  account: AccountSnapshotType,
  login: string,
): ChannelType | undefined =>
  account.streamers.find((s) => s.login === login)?.channel ?? knownChannels[login];

export const searchedChannel = (found: SearchChannelType): ChannelType => ({
  id: "",
  login: found.login,
  display_name: found.display_name,
  avatar_url: found.avatar_url,
  live: found.live,
  drops_enabled: false,
});

const flush = async (): Promise<void> => {
  flushing = true;

  while (queued.length > 0) {
    const batch = queued.splice(0, CHANNELS_LOOKUP_MAX);

    try {
      rememberChannels((await lookupChannels(batch)).channels);
    } catch (e: any) {
      // ! Pictures are a nicety: the names show anyway.
      for (const login of batch) {
        asked.delete(login);
      }
    }
  }

  flushing = false;
};

// ! A channel known only from a search (no id) is read in full: its game, its
// viewers.
export const ensureChannels = (logins: string[]): void => {
  const wanted = logins.filter((login) => !knownChannels[login]?.id && !asked.has(login));

  if (wanted.length === 0) {
    return;
  }

  for (const login of wanted) {
    asked.add(login);
  }

  queued.push(...wanted);

  if (!flushing) {
    queueMicrotask(() => void flush());
  }
};

export const isLive = (login: string | null | undefined): boolean => {
  if (!login) {
    return false;
  }

  let reported: boolean | undefined;

  for (const account of app.snapshot?.accounts ?? []) {
    const seen = [
      ...account.slots.map((slot) => slot.channel),
      ...account.streamers.map((view) => view.channel),
    ].find((channel) => channel?.login === login);

    if (seen?.live) {
      return true;
    }

    reported ??= seen?.live;
  }

  return reported ?? knownChannels[login]?.live === true;
};

export const knownBanners: Partial<Record<string, string>> = $state({});
const bannersAsked = new Set<string>();

const fetchBannerBatches = async (logins: string[]): Promise<void> => {
  for (let from = 0; from < logins.length; from += CHANNELS_LOOKUP_MAX) {
    const batch = logins.slice(from, from + CHANNELS_LOOKUP_MAX);

    try {
      Object.assign(knownBanners, await fetchBanners(batch));
    } catch (e: any) {
      for (const login of batch) {
        bannersAsked.delete(login);
      }
    }
  }
};

export const ensureBanners = (logins: string[]): void => {
  const wanted = logins.filter((login) => !bannersAsked.has(login));

  if (wanted.length === 0) {
    return;
  }

  for (const login of wanted) {
    bannersAsked.add(login);
  }

  void fetchBannerBatches(wanted);
};
