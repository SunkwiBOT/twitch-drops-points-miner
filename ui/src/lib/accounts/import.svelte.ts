import type { IconName } from "../../components/ui/ui.constants";
import type { ChannelType } from "../bindings/ChannelType";
import type { StreamerSettingsType } from "../bindings/StreamerSettingsType";

export type ImportActionType = {
  label: string;
  icon: IconName;
  busy: boolean;
  busyLabel: string;
  disabled: boolean;
  run: () => void;
};

export type ImportCandidateType = {
  login: string;
  channel: ChannelType | undefined;
};

export const newStreamer = (
  defaults: StreamerSettingsType,
  login: string,
): StreamerSettingsType => ({ ...$state.snapshot(defaults), login, enabled: true });

export const fromChannels = (channels: ChannelType[]): ImportCandidateType[] =>
  channels.map((channel) => ({ login: channel.login, channel }));
