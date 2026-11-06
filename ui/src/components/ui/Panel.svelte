<script lang="ts">
  import type { Snippet } from "svelte";

  import {
    Card,
    CardAction,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { number } from "../../lib/format.utils";
  import Badge from "./Badge.svelte";
  import RichText from "./RichText.svelte";
  import { BadgeTone } from "./ui.constants";

  type PanelPropsType = {
    title: string;
    count?: number;
    description?: string;
    actions?: Snippet;
    children?: Snippet;
  };

  let { title, count, description, actions, children }: PanelPropsType = $props();
</script>

<Card class="gap-4 [--card-spacing:--spacing(5)] max-md:[--card-spacing:--spacing(3.5)]">
  <CardHeader class="flex flex-wrap items-center gap-x-3 gap-y-1.5">
    <CardTitle class="flex items-center gap-2 text-base font-semibold">
      {title}
      {#if count !== undefined}<Badge
          text={number(count)}
          tone={BadgeTone.NEUTRAL}
          dot={false}
        />{/if}
    </CardTitle>
    {#if actions}
      <CardAction
        class="ml-auto flex max-w-full min-w-0 flex-wrap items-center justify-end gap-2 self-center"
        >{@render actions()}</CardAction
      >
    {/if}
    {#if description}
      <CardDescription class="basis-full text-hint"><RichText text={description} /></CardDescription
      >
    {/if}
  </CardHeader>
  {#if children}<CardContent class="flex flex-col gap-3">{@render children()}</CardContent>{/if}
</Card>
