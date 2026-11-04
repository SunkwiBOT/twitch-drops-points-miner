<script lang="ts">
  import { ClockFormat } from "../../lib/bindings/ClockFormat";
  import { DateFormat } from "../../lib/bindings/DateFormat";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import { HistoryRetention } from "../../lib/bindings/HistoryRetention";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { AUTO_TIME_ZONE } from "../../lib/time/time.constants";
  import { deviceTimeZone } from "../../lib/time/time.svelte";
  import {
    formatClockAs,
    formatDateAs,
    formatZoneOffset,
    listTimeZones,
  } from "../../lib/time/time.utils";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import Select from "../ui/Select.svelte";
  import { MenuAlign } from "../ui/ui.constants";
  import { SAMPLE_DATE } from "./settings.constants";

  type SettingsTimePropsType = {
    draft: GlobalSettingsType;
  };

  let { draft = $bindable() }: SettingsTimePropsType = $props();

  const sampleDate = (format: DateFormat): string => formatDateAs(SAMPLE_DATE, false, format);
  const sampleClock = (format: ClockFormat): string => formatClockAs(SAMPLE_DATE, format);

  const zones = $derived([
    {
      value: AUTO_TIME_ZONE,
      label: t.settings_time_zone_auto({ zone: deviceTimeZone() }),
    },
    ...listTimeZones().map((zone) => ({
      value: zone,
      label: zone.replaceAll("_", " "),
      meta: formatZoneOffset(zone, ticker.now),
    })),
  ]);

  const dateFormats = $derived([
    { value: DateFormat.AUTO, label: t.date_format_auto({ example: sampleDate(DateFormat.AUTO) }) },
    { value: DateFormat.DAY_MONTH_YEAR, label: sampleDate(DateFormat.DAY_MONTH_YEAR) },
    { value: DateFormat.MONTH_DAY_YEAR, label: sampleDate(DateFormat.MONTH_DAY_YEAR) },
    { value: DateFormat.YEAR_MONTH_DAY, label: sampleDate(DateFormat.YEAR_MONTH_DAY) },
  ]);

  const clockFormats = $derived([
    { value: ClockFormat.AUTO, label: t.clock_auto({ example: sampleClock(ClockFormat.AUTO) }) },
    { value: ClockFormat.H24, label: t.clock_24({ example: sampleClock(ClockFormat.H24) }) },
    { value: ClockFormat.H12, label: t.clock_12({ example: sampleClock(ClockFormat.H12) }) },
  ]);

  const retentions = $derived([
    { value: HistoryRetention.DAYS30, label: t.retention_30() },
    { value: HistoryRetention.DAYS90, label: t.retention_90() },
    { value: HistoryRetention.YEAR1, label: t.retention_365() },
    { value: HistoryRetention.UNLIMITED, label: t.retention_unlimited() },
  ]);
</script>

<Panel title={t.settings_time()}>
  <div class="grid grid-cols-[repeat(auto-fit,minmax(min(200px,100%),1fr))] gap-3">
    <FormField id="set-zone" label={t.settings_time_zone()}>
      <Select
        id="set-zone"
        value={draft.time_zone}
        options={zones}
        label={t.settings_time_zone()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.time_zone = value)}
      />
    </FormField>
    <FormField id="set-date" label={t.settings_date_format()}>
      <Select
        id="set-date"
        value={draft.date_format}
        options={dateFormats}
        label={t.settings_date_format()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.date_format = value)}
      />
    </FormField>
    <FormField id="set-clock" label={t.settings_clock_format()}>
      <Select
        id="set-clock"
        value={draft.clock_format}
        options={clockFormats}
        label={t.settings_clock_format()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.clock_format = value)}
      />
    </FormField>
    <FormField id="set-retention" label={t.settings_history_retention()}>
      <Select
        id="set-retention"
        value={draft.history_retention}
        options={retentions}
        label={t.settings_history_retention()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.history_retention = value)}
      />
    </FormField>
  </div>
</Panel>
