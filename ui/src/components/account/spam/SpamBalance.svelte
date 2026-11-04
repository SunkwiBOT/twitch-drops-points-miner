<script lang="ts">
  import { number } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { BALANCE_DOTS, BalanceTracking } from "../../../lib/spam/spam.constants";
  import { BALANCE_HINTS, changedAgo } from "../../../lib/spam/spam.utils";
  import Icon from "../../ui/Icon.svelte";
  import Tip from "../../ui/Tip.svelte";
  import { IconName } from "../../ui/ui.constants";

  type SpamBalancePropsType = {
    balance: number;
    changedAt: string;
    tracking: BalanceTracking;
    now: number;
  };

  let { balance, changedAt, tracking, now }: SpamBalancePropsType = $props();

  const live = $derived(tracking === BalanceTracking.LIVE);

  const status = $derived.by(() => {
    switch (tracking) {
      case BalanceTracking.LIVE:
        return t.spam_balance_changed({ time: changedAgo(changedAt, now) });
      case BalanceTracking.PAUSED:
        return t.spam_balance_paused();
      case BalanceTracking.FAILING:
        return t.spam_balance_failing();
    }
  });
</script>

<span class="flex shrink-0 flex-col items-end gap-0.5 text-right">
  <span
    class="inline-flex items-center gap-1.5 text-xl leading-tight font-semibold tabular-nums"
    role="img"
    aria-label={t.spam_balance({ points: number(balance) })}
    ><span class="text-link"><Icon name={IconName.CHANNEL_POINTS} size={18} /></span>{number(
      balance,
    )}</span
  >
  <Tip text={BALANCE_HINTS[tracking]()}>
    {#snippet children(props)}
      <button
        {...props}
        type="button"
        class="inline-flex cursor-help items-center gap-1.5 text-xs text-muted-foreground"
      >
        <span class="relative inline-flex size-1.5 shrink-0" aria-hidden="true">
          {#if live}<span
              class="absolute inset-0 animate-[ping_2s_cubic-bezier(0,0,0.2,1)_infinite] rounded-full bg-success-emphasis opacity-60"
            ></span>{/if}
          <span class={["relative inline-flex size-1.5 rounded-full", BALANCE_DOTS[tracking]]}
          ></span>
        </span>
        {status}
      </button>
    {/snippet}
  </Tip>
</span>
