import { fetchBanners, lookupChannels } from "../api/client";
import type { ChannelType } from "../bindings/ChannelType";
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

const flush = async (): Promise<void> => {
  flushing = true;

  while (queued.length > 0) {
    const batch = queued.splice(0, CHANNELS_LOOKUP_MAX);

    try {
      rememberChannels((await lookupChannels(batch)).channels);
    } catch (e: any) {
      // ! Pictures are a nicety: the names show anyway.
    }
  }

  flushing = false;
};

export const ensureChannels = (logins: string[]): void => {
  const wanted = logins.filter((login) => !knownChannels[login] && !asked.has(login));

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
    try {
      Object.assign(
        knownBanners,
        await fetchBanners(logins.slice(from, from + CHANNELS_LOOKUP_MAX)),
      );
    } catch (e: any) {}
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
