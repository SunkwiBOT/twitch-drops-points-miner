import { ClockFormat } from "../bindings/ClockFormat";
import { DateFormat } from "../bindings/DateFormat";
import { locale, t } from "../i18n/i18n.svelte";
import { DAY_MS, HOUR_MS, MINUTE_MS } from "./time.constants";
import { timePreferences } from "./time.svelte";

type PartsType = { year: string; month: string; day: string };

const toMs = (value: string | number): number =>
  typeof value === "number" ? value : new Date(value).getTime();

const hour12 = (format: ClockFormat): boolean | undefined => {
  switch (format) {
    case ClockFormat.H12:
      return true;
    case ClockFormat.H24:
      return false;
    case ClockFormat.AUTO:
      return undefined;
  }
};

const clockOptions = (format: ClockFormat): Intl.DateTimeFormatOptions => ({
  timeZone: timePreferences.zone,
  hour: "2-digit",
  minute: "2-digit",
  hour12: hour12(format),
});

const dateParts = (
  ms: number,
  year: NonNullable<Intl.DateTimeFormatOptions["year"]>,
): PartsType => {
  const parts = new Intl.DateTimeFormat("en-US", {
    timeZone: timePreferences.zone,
    year,
    month: "2-digit",
    day: "2-digit",
  }).formatToParts(ms);
  const part = (type: Intl.DateTimeFormatPartTypes): string =>
    parts.find((item) => item.type === type)?.value ?? "";

  return { year: part("year"), month: part("month"), day: part("day") };
};

export const formatDateAs = (
  value: string | number,
  short: boolean,
  format: DateFormat,
): string => {
  const ms = toMs(value);
  const { year, month, day } = dateParts(ms, short ? "2-digit" : "numeric");

  switch (format) {
    case DateFormat.DAY_MONTH_YEAR:
      return `${day}/${month}/${year}`;
    case DateFormat.MONTH_DAY_YEAR:
      return `${month}/${day}/${year}`;
    case DateFormat.YEAR_MONTH_DAY:
      return `${year}-${month}-${day}`;
    case DateFormat.AUTO:
      return new Intl.DateTimeFormat(locale.current, {
        timeZone: timePreferences.zone,
        year: short ? "2-digit" : "numeric",
        month: "2-digit",
        day: "2-digit",
      }).format(ms);
  }
};

export const formatDate = (value: string | number, short: boolean): string =>
  formatDateAs(value, short, timePreferences.dateFormat);

export const formatClockAs = (value: string | number, format: ClockFormat): string =>
  new Intl.DateTimeFormat(locale.current, clockOptions(format)).format(toMs(value));

export const formatClock = (value: string | number): string =>
  formatClockAs(value, timePreferences.clockFormat);

export const formatClockWithSeconds = (value: string | number): string =>
  new Intl.DateTimeFormat(locale.current, {
    ...clockOptions(timePreferences.clockFormat),
    second: "2-digit",
  }).format(toMs(value));

export const formatShortDateTime = (value: string | number): string =>
  `${formatDate(value, true)} ${formatClock(value)}`;

export const formatWeekdayDay = (ms: number): string =>
  new Intl.DateTimeFormat(locale.current, {
    timeZone: timePreferences.zone,
    weekday: "short",
    day: "numeric",
  }).format(ms);

export const formatDayMonth = (ms: number): string =>
  new Intl.DateTimeFormat(locale.current, {
    timeZone: timePreferences.zone,
    day: "numeric",
    month: "short",
  }).format(ms);

export const formatWeekdayDayMonth = (ms: number): string =>
  new Intl.DateTimeFormat(locale.current, {
    timeZone: timePreferences.zone,
    weekday: "short",
    day: "numeric",
    month: "short",
  }).format(ms);

export const zoneOffsetMs = (ms: number, zone: string): number => {
  const parts = new Intl.DateTimeFormat("en-US", {
    timeZone: zone,
    year: "numeric",
    month: "numeric",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
    hourCycle: "h23",
  }).formatToParts(ms);
  const value = (type: Intl.DateTimeFormatPartTypes): number =>
    Number(parts.find((item) => item.type === type)?.value ?? 0);
  const wallClock = Date.UTC(
    value("year"),
    value("month") - 1,
    value("day"),
    value("hour"),
    value("minute"),
  );

  return wallClock - Math.floor(ms / MINUTE_MS) * MINUTE_MS;
};

export const formatZoneOffset = (zone: string, now: number): string => {
  const minutes = Math.round(zoneOffsetMs(now, zone) / MINUTE_MS);

  if (minutes === 0) {
    return "UTC";
  }

  const sign = minutes > 0 ? "+" : "−";
  const hours = Math.floor(Math.abs(minutes) / 60);
  const rest = Math.abs(minutes) % 60;

  return `UTC${sign}${hours}${rest ? `:${String(rest).padStart(2, "0")}` : ""}`;
};

export const listTimeZones = (): string[] => Intl.supportedValuesOf("timeZone");

export const relativeTime = (iso: string | undefined, now: number): string => {
  if (!iso) {
    return "";
  }

  const diff = new Date(iso).getTime() - now;
  const abs = Math.abs(diff);
  const minutes = Math.round(abs / MINUTE_MS);
  const hours = Math.round(abs / HOUR_MS);
  const days = Math.round(abs / DAY_MS);

  if (minutes < 1) {
    return t.time_now();
  }

  if (diff < 0) {
    if (minutes < 60) {
      return t.time_minutes_ago({ count: minutes });
    }

    return hours < 48 ? t.time_hours_ago({ count: hours }) : t.time_days_ago({ count: days });
  }

  if (minutes < 60) {
    return t.time_in_minutes({ count: minutes });
  }

  return hours < 48 ? t.time_in_hours({ count: hours }) : t.time_in_days({ count: days });
};
