<script lang="ts">
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import { IconName, THUMB_PIXELS, ThumbShape, type ThumbSize } from "./ui.constants";

  type ThumbPropsType = {
    src: string | null | undefined;
    round: boolean;
    contain: boolean;
    size: ThumbSize;
    shape: ThumbShape;
    fallback: IconName;
  };

  let { src, round, contain, size, shape, fallback }: ThumbPropsType = $props();

  const pixels = $derived(THUMB_PIXELS[size]);
  const width = $derived(shape === ThumbShape.BOX_ART ? Math.round((pixels * 3) / 4) : pixels);

  let failed = $state(false);

  $effect(() => {
    void src;
    failed = false;
  });
</script>

{#if src && !failed}
  <img
    class={cn(
      "block shrink-0 bg-muted object-cover",
      round ? "rounded-full" : "rounded-md",
      contain && "object-contain p-0.75",
    )}
    style:width="{width}px"
    style:height="{pixels}px"
    {src}
    alt=""
    loading="lazy"
    onerror={() => (failed = true)}
  />
{:else}
  <span
    class={cn(
      "flex shrink-0 items-center justify-center bg-muted text-muted-foreground",
      round ? "rounded-full" : "rounded-md",
    )}
    style:width="{width}px"
    style:height="{pixels}px"
  >
    <Icon name={fallback} size={Math.round(width / 2)} />
  </span>
{/if}
