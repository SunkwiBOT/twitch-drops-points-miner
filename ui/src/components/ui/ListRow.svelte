<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import DetailLine from "./DetailLine.svelte";
  import RichText from "./RichText.svelte";

  type ListRowPropsType = {
    title: string;
    subtitle?: string;
    details?: string[];
    lead?: Snippet;
    badges?: Snippet;
    below?: Snippet;
    trailing?: Snippet;
    onopen?: () => void;
    muted: boolean;
  };

  let { title, subtitle, details, lead, badges, below, trailing, onopen, muted }: ListRowPropsType =
    $props();
</script>

{#snippet body()}
  {#if lead}{@render lead()}{/if}
  <span class="flex min-w-0 flex-1 flex-col gap-0.5">
    <span class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
      <span class="truncate font-semibold">{title}</span>
      {#if badges}{@render badges()}{/if}
    </span>
    {#if subtitle}<span class="truncate text-xs text-muted-foreground"
        ><RichText text={subtitle} /></span
      >{/if}
    {#if details}<DetailLine parts={details} />{/if}
    {#if below}<span class="mt-1 block">{@render below()}</span>{/if}
  </span>
{/snippet}

<div
  class={cn(
    "flex min-h-13 gap-3 border-b px-3 py-2.5 transition-colors last:border-b-0",
    below ? "items-start" : "items-center",
    onopen && "hover:bg-accent",
  )}
  role="listitem"
>
  <span
    class={cn(
      "flex min-w-0 flex-1 gap-3",
      below ? "items-start" : "items-center",
      muted && "opacity-55 grayscale",
    )}
  >
    {#if onopen}
      <button
        class={cn("flex min-w-0 flex-1 gap-3 text-left", below ? "items-start" : "items-center")}
        type="button"
        onclick={onopen}>{@render body()}</button
      >
    {:else}
      {@render body()}
    {/if}
  </span>
  {#if trailing}<span class="flex shrink-0 items-center gap-1.5">{@render trailing()}</span>{/if}
</div>
