import { locale } from "./i18n/i18n.svelte";
import { type BoxArtSize, DurationUnit } from "./format.constants";
import { MINUTE_MS, SECOND_MS } from "./time/time.constants";

export const number = (value: number): string =>
  new Intl.NumberFormat(locale.current).format(value);

export const percent = (current: number, required: number): number =>
  required <= 0 ? 0 : Math.min(100, Math.round((current / required) * 100));

const unit = (value: number, name: DurationUnit): string =>
  new Intl.NumberFormat(locale.current, {
    style: "unit",
    unit: name,
    unitDisplay: "short",
  }).format(value);

export const minutes = (count: number): string =>
  new Intl.NumberFormat(locale.current, {
    style: "unit",
    unit: DurationUnit.MINUTE,
    unitDisplay: "long",
  }).format(count);

export const formatPercent = (value: number): string =>
  new Intl.NumberFormat(locale.current, { style: "percent", maximumFractionDigits: 0 }).format(
    value / 100,
  );

export const shortDuration = (ms: number): string => {
  const minutes = Math.max(0, Math.round(ms / MINUTE_MS));

  if (minutes < 60) {
    return unit(minutes, DurationUnit.MINUTE);
  }

  const hours = Math.floor(minutes / 60);

  if (hours < 48) {
    return minutes % 60
      ? `${unit(hours, DurationUnit.HOUR)} ${unit(minutes % 60, DurationUnit.MINUTE)}`
      : unit(hours, DurationUnit.HOUR);
  }

  return unit(Math.floor(hours / 24), DurationUnit.DAY);
};

export const elapsed = (ms: number): string => {
  if (ms < SECOND_MS) {
    return unit(ms, DurationUnit.MILLISECOND);
  }

  if (ms >= MINUTE_MS) {
    return shortDuration(ms);
  }

  return new Intl.NumberFormat(locale.current, {
    style: "unit",
    unit: DurationUnit.SECOND,
    unitDisplay: "short",
    maximumFractionDigits: 1,
  }).format(ms / SECOND_MS);
};

export const boxArt = (url: string | undefined, size: BoxArtSize): string | null =>
  url ? url.replace(/-\d+x\d+(\.\w+)$/, `-${size}$1`) : null;

export const maskId = (id: string): string =>
  id.length <= 6 ? id : `${id.slice(0, 4)}…${id.slice(-3)}`;

export const mmss = (ms: number): string => {
  const total = Math.max(0, Math.round(ms / 1000));

  return `${Math.floor(total / 60)}:${String(total % 60).padStart(2, "0")}`;
};
