<script lang="ts">
  import type { Snippet } from "svelte";

  import { number } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Icon from "./Icon.svelte";
  import { BalanceAlign, IconName } from "./ui.constants";

  type PointsBalancePropsType = {
    balance: number;
    earned: number;
    streak?: number;
    align: BalanceAlign;
    children?: Snippet;
  };

  let { balance, earned, streak, align, children }: PointsBalancePropsType = $props();

  const end = $derived(align === BalanceAlign.END);
</script>

<span
  class={[
    "flex shrink-0 flex-col gap-0.5",
    end ? "min-w-22 items-end text-right" : "items-start text-left",
  ]}
>
  <span class="inline-flex items-center gap-1 font-semibold tabular-nums"
    ><span class="text-link"><Icon name={IconName.CHANNEL_POINTS} size={14} /></span>{number(
      balance,
    )}</span
  >
  {#if earned > 0 || streak !== undefined}
    <span class="text-xs text-muted-foreground tabular-nums">
      {#if earned > 0}<span class="inline-flex items-center gap-1 text-success-foreground"
          ><Icon name={IconName.CHANNEL_POINTS} size={12} />+{number(earned)}</span
        >{/if}
      {#if earned > 0 && streak !== undefined}·{/if}
      {#if streak !== undefined}<span
          class="inline-flex items-center gap-0.5 font-semibold text-foreground"
          ><span aria-hidden="true">{number(streak)}</span><Icon
            name={IconName.WATCH_STREAK}
            size={13}
            class="-translate-y-px"
          /><span class="sr-only">{t.streamer_streak({ count: number(streak) })}</span></span
        >{/if}
    </span>
  {/if}
  {#if children}<span
      class={["flex flex-col text-xs text-muted-foreground", end ? "items-end" : "items-start"]}
      >{@render children()}</span
    >{/if}
</span>
