<script lang="ts">
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { boundedInteger, boundNumberInput } from "../../../lib/input.utils";
  import { SPAM_MAX_COUNT } from "../../../lib/spam/spam.constants";
  import Icon from "../../ui/Icon.svelte";
  import Tip from "../../ui/Tip.svelte";
  import { IconName } from "../../ui/ui.constants";

  type QuantityStepperPropsType = {
    value: number;
    max: number;
    disabled: boolean;
    onchange: (value: number) => void;
  };

  let { value, max, disabled, onchange }: QuantityStepperPropsType = $props();

  const highest = $derived(Math.max(1, max));
  const limit = $derived(max < SPAM_MAX_COUNT ? t.spam_max_balance() : t.spam_max_count());

  const step = (by: number): void => onchange(boundedInteger(value + by, 1, highest));

  const BUTTON =
    "grid h-full w-8 place-items-center text-muted-foreground transition-colors outline-none hover:bg-muted hover:text-foreground focus-visible:z-10 focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:opacity-40 aria-disabled:opacity-40 aria-disabled:hover:bg-transparent aria-disabled:hover:text-muted-foreground";
</script>

<div
  class="inline-flex h-8 shrink-0 items-stretch overflow-hidden rounded-lg border border-input bg-background"
  role="group"
  aria-label={t.spam_quantity()}
>
  <Tip text={t.spam_decrease()}>
    {#snippet children(props)}
      <button
        {...props}
        type="button"
        class={BUTTON}
        aria-label={t.spam_decrease()}
        disabled={disabled || value <= 1}
        onclick={() => step(-1)}><Icon name={IconName.MINUS} size={14} /></button
      >
    {/snippet}
  </Tip>
  <input
    class="w-9 border-x border-input bg-transparent text-center text-hint font-semibold tabular-nums outline-none [appearance:textfield] focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:opacity-50 [&::-webkit-inner-spin-button]:appearance-none"
    type="number"
    inputmode="numeric"
    min="1"
    max={highest}
    aria-label={t.spam_quantity()}
    {value}
    {disabled}
    onchange={(event) => onchange(boundNumberInput(event.currentTarget, 1, highest))}
  />
  <Tip text={value >= max && max > 0 ? limit : t.spam_increase()}>
    {#snippet children(props)}
      <button
        {...props}
        type="button"
        class={BUTTON}
        aria-label={t.spam_increase()}
        aria-disabled={disabled || value >= max}
        onclick={() => {
          if (!disabled && value < max) {
            step(1);
          }
        }}><Icon name={IconName.PLUS} size={14} /></button
      >
    {/snippet}
  </Tip>
</div>
