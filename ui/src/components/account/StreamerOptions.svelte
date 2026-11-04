<script lang="ts">
  import { FILTER_OP_SYMBOLS, StreamerOption } from "../../lib/accounts/accounts.constants";
  import { FILTER_BY_LABELS, STRATEGY_LABELS } from "../../lib/accounts/options.utils";
  import { BetTiming } from "../../lib/bindings/BetTiming";
  import { ChatPresence } from "../../lib/bindings/ChatPresence";
  import { FilterBy } from "../../lib/bindings/FilterBy";
  import { FilterOp } from "../../lib/bindings/FilterOp";
  import type { PredictionSettingsType } from "../../lib/bindings/PredictionSettingsType";
  import type { StreamerSettingsType } from "../../lib/bindings/StreamerSettingsType";
  import { Strategy } from "../../lib/bindings/Strategy";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { boundNumberInput } from "../../lib/input.utils";
  import { Input } from "$lib/components/ui/input";
  import FormField from "../ui/FormField.svelte";
  import Select from "../ui/Select.svelte";
  import Setting from "../ui/Setting.svelte";
  import { MenuAlign } from "../ui/ui.constants";

  type StreamerOptionsPropsType = {
    options: StreamerSettingsType;
    mixed: StreamerOption[];
    onchange?: (option: StreamerOption) => void;
  };

  type NumberFieldType = keyof Pick<
    PredictionSettingsType,
    | "percentage"
    | "max_points"
    | "min_points"
    | "delay_seconds"
    | "timing_percent"
    | "percentage_gap"
  >;

  let { options = $bindable(), mixed, onchange }: StreamerOptionsPropsType = $props();

  const strategyOptions = $derived(
    Object.values(Strategy).map((strategy) => ({
      value: strategy,
      label: STRATEGY_LABELS[strategy](),
      description: strategy === Strategy.SMART ? t.strategy_smart_hint() : undefined,
    })),
  );

  const filterByOptions = $derived(
    Object.values(FilterBy).map((by) => ({ value: by, label: FILTER_BY_LABELS[by]() })),
  );

  const filterOpOptions = Object.values(FilterOp).map((op) => ({
    value: op,
    label: FILTER_OP_SYMBOLS[op],
  }));

  const chatOptions = $derived([
    { value: ChatPresence.ONLINE, label: t.chat_online(), description: t.chat_online_hint() },
    { value: ChatPresence.ALWAYS, label: t.chat_always(), description: t.chat_always_hint() },
    { value: ChatPresence.OFFLINE, label: t.chat_offline(), description: t.chat_offline_hint() },
    { value: ChatPresence.NEVER, label: t.chat_never(), description: t.chat_never_hint() },
  ]);

  const timingOptions = $derived([
    {
      value: BetTiming.AT_OPEN,
      label: t.bet_timing_at_open(),
      description: t.bet_timing_at_open_hint(),
    },
    {
      value: BetTiming.BEFORE_CLOSE,
      label: t.bet_timing_before_close(),
      description: t.bet_timing_before_close_hint(),
    },
    {
      value: BetTiming.PERCENT,
      label: t.bet_timing_percent(),
      description: t.bet_timing_percent_hint(),
    },
  ]);

  const isMixed = (option: StreamerOption): boolean => mixed.includes(option);

  const markOf = (option: StreamerOption): string | undefined =>
    isMixed(option) ? t.streamer_option_mixed() : undefined;

  const shown = <T,>(option: StreamerOption, value: T): T | null =>
    isMixed(option) ? null : value;

  const timing = $derived(shown(StreamerOption.TIMING, options.predictions.timing));

  const change = (option: StreamerOption, apply: () => void): void => {
    apply();
    onchange?.(option);
  };

  const setNumber = (
    option: StreamerOption,
    field: NumberFieldType,
    input: HTMLInputElement,
    min: number,
    max: number,
  ): void => {
    if (input.value.trim() === "" && isMixed(option)) {
      return;
    }

    const value = boundNumberInput(input, min, max);

    change(option, () => {
      options.predictions[field] = value;
    });
  };
</script>

{#snippet numberField(
  id: string,
  label: string,
  option: StreamerOption,
  field: NumberFieldType,
  min: number,
  max: number | undefined,
  hint: string | undefined,
)}
  <FormField {id} {label} {hint}>
    <Input
      {id}
      type="number"
      {min}
      {max}
      value={shown(option, options.predictions[field]) ?? ""}
      placeholder={markOf(option)}
      onchange={(event) =>
        setNumber(option, field, event.currentTarget, min, max ?? Number.MAX_SAFE_INTEGER)}
    />
  </FormField>
{/snippet}

<div class="grid gap-x-8 gap-y-4 md:grid-cols-2">
  <div class="flex flex-col">
    <Setting
      title={t.streamer_claim_bonus()}
      description={t.streamer_claim_bonus_hint()}
      checked={shown(StreamerOption.CLAIM_BONUS, options.claim_bonus) ?? false}
      disabled={false}
      mark={markOf(StreamerOption.CLAIM_BONUS)}
      onchange={(value) => change(StreamerOption.CLAIM_BONUS, () => (options.claim_bonus = value))}
    />
    <Setting
      title={t.streamer_watch_streak()}
      description={t.streamer_watch_streak_hint()}
      checked={shown(StreamerOption.WATCH_STREAK, options.watch_streak) ?? false}
      disabled={false}
      mark={markOf(StreamerOption.WATCH_STREAK)}
      onchange={(value) =>
        change(StreamerOption.WATCH_STREAK, () => (options.watch_streak = value))}
    />
    <Setting
      title={t.streamer_follow_raids()}
      description={t.streamer_follow_raids_hint()}
      checked={shown(StreamerOption.FOLLOW_RAIDS, options.follow_raids) ?? false}
      disabled={false}
      mark={markOf(StreamerOption.FOLLOW_RAIDS)}
      onchange={(value) =>
        change(StreamerOption.FOLLOW_RAIDS, () => (options.follow_raids = value))}
    />
    <Setting
      title={t.streamer_claim_moments()}
      description={t.streamer_claim_moments_hint()}
      checked={shown(StreamerOption.CLAIM_MOMENTS, options.claim_moments) ?? false}
      disabled={false}
      mark={markOf(StreamerOption.CLAIM_MOMENTS)}
      onchange={(value) =>
        change(StreamerOption.CLAIM_MOMENTS, () => (options.claim_moments = value))}
    />
    <Setting
      title={t.streamer_community_goals()}
      description={t.streamer_community_goals_hint()}
      checked={shown(StreamerOption.COMMUNITY_GOALS, options.community_goals) ?? false}
      disabled={false}
      mark={markOf(StreamerOption.COMMUNITY_GOALS)}
      onchange={(value) =>
        change(StreamerOption.COMMUNITY_GOALS, () => (options.community_goals = value))}
    />
    <div class="border-t pt-4">
      <FormField id="streamer-chat" label={t.streamer_chat()} hint={t.streamer_chat_hint()}>
        <Select
          id="streamer-chat"
          value={shown(StreamerOption.CHAT, options.chat)}
          options={chatOptions}
          label={t.streamer_chat()}
          placeholder={markOf(StreamerOption.CHAT)}
          align={MenuAlign.LEFT}
          onchange={(chat) => change(StreamerOption.CHAT, () => (options.chat = chat))}
        />
      </FormField>
    </div>
  </div>
  <div class="flex flex-col gap-4">
    <Setting
      title={t.streamer_predictions_enabled()}
      description={t.streamer_predictions_warning()}
      checked={shown(StreamerOption.PREDICTIONS, options.predictions.enabled) ?? false}
      disabled={false}
      mark={markOf(StreamerOption.PREDICTIONS)}
      onchange={(value) =>
        change(StreamerOption.PREDICTIONS, () => (options.predictions.enabled = value))}
    />
    {#if options.predictions.enabled || isMixed(StreamerOption.PREDICTIONS)}
      <FormField id="streamer-strategy" label={t.streamer_predictions_strategy()}>
        <Select
          id="streamer-strategy"
          value={shown(StreamerOption.STRATEGY, options.predictions.strategy)}
          options={strategyOptions}
          label={t.streamer_predictions_strategy()}
          placeholder={markOf(StreamerOption.STRATEGY)}
          align={MenuAlign.LEFT}
          onchange={(strategy) =>
            change(StreamerOption.STRATEGY, () => (options.predictions.strategy = strategy))}
        />
      </FormField>
      <FormField
        id="streamer-timing"
        label={t.streamer_predictions_timing()}
        hint={timingOptions.find((option) => option.value === timing)?.description}
      >
        <Select
          id="streamer-timing"
          value={timing}
          options={timingOptions}
          label={t.streamer_predictions_timing()}
          placeholder={markOf(StreamerOption.TIMING)}
          align={MenuAlign.LEFT}
          onchange={(value) =>
            change(StreamerOption.TIMING, () => (options.predictions.timing = value))}
        />
      </FormField>
      <div class="grid grid-cols-2 gap-3">
        {@render numberField(
          "streamer-p1",
          t.streamer_predictions_percentage(),
          StreamerOption.PERCENTAGE,
          "percentage",
          1,
          100,
          undefined,
        )}
        {@render numberField(
          "streamer-p2",
          t.streamer_predictions_max(),
          StreamerOption.MAX_POINTS,
          "max_points",
          1,
          250000,
          undefined,
        )}
        <div class={timing === BetTiming.AT_OPEN ? "col-span-2" : undefined}>
          {@render numberField(
            "streamer-p3",
            t.streamer_predictions_min(),
            StreamerOption.MIN_POINTS,
            "min_points",
            1,
            undefined,
            undefined,
          )}
        </div>
        {#if timing === BetTiming.BEFORE_CLOSE || timing === null}
          {@render numberField(
            "streamer-p4",
            t.streamer_predictions_delay(),
            StreamerOption.DELAY_SECONDS,
            "delay_seconds",
            0,
            300,
            undefined,
          )}
        {/if}
        {#if timing === BetTiming.PERCENT || timing === null}
          <div class={timing === null ? "col-span-2" : undefined}>
            {@render numberField(
              "streamer-p5",
              t.streamer_predictions_timing_percent(),
              StreamerOption.TIMING_PERCENT,
              "timing_percent",
              1,
              99,
              undefined,
            )}
          </div>
        {/if}
      </div>
      {#if options.predictions.strategy === Strategy.SMART || isMixed(StreamerOption.STRATEGY)}
        {@render numberField(
          "streamer-gap",
          t.streamer_predictions_gap(),
          StreamerOption.PERCENTAGE_GAP,
          "percentage_gap",
          0,
          100,
          t.streamer_predictions_gap_hint(),
        )}
      {/if}
      <div class="flex flex-col">
        <Setting
          title={t.streamer_predictions_stealth()}
          description={t.streamer_predictions_stealth_hint()}
          checked={shown(StreamerOption.STEALTH, options.predictions.stealth) ?? false}
          disabled={false}
          mark={markOf(StreamerOption.STEALTH)}
          onchange={(value) =>
            change(StreamerOption.STEALTH, () => (options.predictions.stealth = value))}
        />
        <Setting
          title={t.streamer_predictions_filter()}
          description={t.streamer_predictions_filter_hint()}
          checked={shown(StreamerOption.FILTER, Boolean(options.predictions.filter)) ?? false}
          disabled={false}
          mark={markOf(StreamerOption.FILTER)}
          onchange={(value) =>
            change(
              StreamerOption.FILTER,
              () =>
                (options.predictions.filter = value
                  ? { by: FilterBy.ODDS, op: FilterOp.GTE, value: 1.5 }
                  : undefined),
            )}
        />
      </div>
      {#if options.predictions.filter && !isMixed(StreamerOption.FILTER)}
        {@const filter = options.predictions.filter}
        <div class="grid grid-cols-[2fr_1fr_1fr] gap-3">
          <FormField id="streamer-filter-by" label={t.streamer_predictions_filter_by()}>
            <Select
              id="streamer-filter-by"
              value={shown(StreamerOption.FILTER_BY, filter.by)}
              options={filterByOptions}
              label={t.streamer_predictions_filter_by()}
              placeholder={markOf(StreamerOption.FILTER_BY)}
              align={MenuAlign.LEFT}
              onchange={(by) => change(StreamerOption.FILTER_BY, () => (filter.by = by))}
            />
          </FormField>
          <FormField id="streamer-filter-op" label={t.streamer_predictions_filter_op()}>
            <Select
              id="streamer-filter-op"
              value={shown(StreamerOption.FILTER_OP, filter.op)}
              options={filterOpOptions}
              label={t.streamer_predictions_filter_op()}
              placeholder={markOf(StreamerOption.FILTER_OP)}
              align={MenuAlign.LEFT}
              onchange={(op) => change(StreamerOption.FILTER_OP, () => (filter.op = op))}
            />
          </FormField>
          <FormField id="streamer-filter-value" label={t.streamer_predictions_filter_value()}>
            <Input
              id="streamer-filter-value"
              type="number"
              step="any"
              value={shown(StreamerOption.FILTER_VALUE, filter.value) ?? ""}
              placeholder={markOf(StreamerOption.FILTER_VALUE)}
              onchange={(event) => {
                const raw = event.currentTarget.value;

                if (raw.trim() === "" && isMixed(StreamerOption.FILTER_VALUE)) {
                  return;
                }

                const value = Number(raw) || 0;

                event.currentTarget.value = String(value);
                change(StreamerOption.FILTER_VALUE, () => (filter.value = value));
              }}
            />
          </FormField>
        </div>
      {/if}
    {/if}
  </div>
</div>
