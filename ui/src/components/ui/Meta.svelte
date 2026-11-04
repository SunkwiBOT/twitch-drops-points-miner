<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Icon from "./Icon.svelte";
  import { IconName, MetaLayout } from "./ui.constants";

  type MetaPropsType = {
    label: string;
    value: string;
    layout: MetaLayout;
    mono: boolean;
    icon?: IconName;
    after?: Snippet;
  };

  let { label, value, layout, mono, icon, after }: MetaPropsType = $props();
</script>

<div
  class={cn(
    "min-w-0",
    layout === MetaLayout.ROW && "col-span-2 grid grid-cols-subgrid items-center py-1",
    layout === MetaLayout.STACKED && "flex flex-col gap-0.5",
    layout === MetaLayout.INLINE && "inline-flex items-baseline gap-1.5",
  )}
>
  <dt class="flex shrink-0 items-center gap-1.5 text-muted-foreground">
    {#if icon}<Icon name={icon} size={14} />{/if}{t.common_label({ label })}
  </dt>
  <dd class="m-0 flex min-w-0 items-center gap-1">
    <span
      class={cn(
        "truncate font-semibold text-foreground",
        mono && "font-mono text-hint",
        layout === MetaLayout.STACKED && "text-base",
      )}>{value}</span
    >
    {#if after}{@render after()}{/if}
  </dd>
</div>
