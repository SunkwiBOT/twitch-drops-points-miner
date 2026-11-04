import { ClockFormat } from "../bindings/ClockFormat";
import { DateFormat } from "../bindings/DateFormat";
import type { GlobalSettingsType } from "../bindings/GlobalSettingsType";
import { AUTO_TIME_ZONE } from "./time.constants";

type TimePreferencesType = { zone: string; dateFormat: DateFormat; clockFormat: ClockFormat };

export const deviceTimeZone = (): string => Intl.DateTimeFormat().resolvedOptions().timeZone;

const isKnownZone = (zone: string): boolean => {
  try {
    new Intl.DateTimeFormat("en", { timeZone: zone });

    return true;
  } catch (e: any) {
    return false;
  }
};

export const timePreferences = $state<TimePreferencesType>({
  zone: deviceTimeZone(),
  dateFormat: DateFormat.AUTO,
  clockFormat: ClockFormat.AUTO,
});

export const applyTimePreferences = (settings: GlobalSettingsType): void => {
  const chosen = settings.time_zone;

  timePreferences.zone =
    chosen !== AUTO_TIME_ZONE && isKnownZone(chosen) ? chosen : deviceTimeZone();
  timePreferences.dateFormat = settings.date_format;
  timePreferences.clockFormat = settings.clock_format;
};
