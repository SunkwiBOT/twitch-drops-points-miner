import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { StreamerSettingsType } from "../bindings/StreamerSettingsType";
import type { StreamerOption } from "./accounts.constants";
import { moveLike, reorderBy, type PlaceType } from "./changes.utils";
import { copyOptions } from "./options.utils";

export const setStreamersOptions = (
  settings: AccountSettingsType,
  logins: ReadonlySet<string>,
  values: StreamerSettingsType,
  options: StreamerOption[],
): void => {
  for (const streamer of settings.points.streamers) {
    if (logins.has(streamer.login)) {
      copyOptions(values, streamer, options);
    }
  }
};

export const setStreamerDefaults = (
  settings: AccountSettingsType,
  values: StreamerSettingsType,
  options: StreamerOption[],
): void => {
  copyOptions(values, settings.points.streamer_defaults, options);
};

export const setStreamerEnabled = (
  settings: AccountSettingsType,
  login: string,
  enabled: boolean,
): void => {
  const streamer = settings.points.streamers.find((item) => item.login === login);

  if (streamer) {
    streamer.enabled = enabled;
  }
};

export const removeStreamer = (settings: AccountSettingsType, login: string): void => {
  settings.points.streamers = settings.points.streamers.filter((item) => item.login !== login);
};

export const moveStreamer = (
  settings: AccountSettingsType,
  login: string,
  place: PlaceType,
): void => {
  const streamers = settings.points.streamers;
  const order = moveLike(
    streamers.map((item) => item.login),
    login,
    place,
  );

  if (order) {
    settings.points.streamers = reorderBy(streamers, (item) => item.login, order);
  }
};
