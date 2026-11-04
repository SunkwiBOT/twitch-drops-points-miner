import { locale } from "../i18n/i18n.svelte";
import { BYTES_PER_MEGABYTE, MEGABYTE_UNIT } from "./updates.constants";

export const megabytes = (bytes: number): string =>
  new Intl.NumberFormat(locale.current, {
    style: "unit",
    unit: MEGABYTE_UNIT,
    maximumFractionDigits: 1,
  }).format(bytes / BYTES_PER_MEGABYTE);
