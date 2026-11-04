<script lang="ts">
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import AvatarImage from "./AvatarImage.svelte";
  import { AVATAR_CLASSES, AvatarSize, LIVE_TAG, LIVE_TAG_CLASSES } from "./ui.constants";

  type ChannelAvatarPropsType = {
    url: string | undefined | null;
    name: string;
    size: AvatarSize;
    live: boolean;
  };

  let { url, name, size, live }: ChannelAvatarPropsType = $props();

  const tagged = $derived(live && LIVE_TAG_CLASSES[size] !== undefined);

  const classes = $derived(
    cn(
      "inline-flex shrink-0 items-center justify-center overflow-hidden bg-muted font-semibold text-muted-foreground uppercase",
      AVATAR_CLASSES[size],
      size === AvatarSize.XS || size === AvatarSize.SM ? "rounded-md" : "rounded-lg",
    ),
  );
</script>

<span class="relative inline-flex shrink-0">
  <AvatarImage {url} {name} class={classes} />
  {#if tagged}
    <span
      class={cn(
        "absolute -bottom-1 left-1/2 -translate-x-1/2 rounded-sm bg-live font-bold tracking-wide text-live-foreground uppercase ring-2 ring-card",
        LIVE_TAG_CLASSES[size],
      )}
      aria-hidden="true">{LIVE_TAG}</span
    >
  {/if}
  {#if tagged}<span class="sr-only">{t.common_live()}</span>{/if}
</span>
