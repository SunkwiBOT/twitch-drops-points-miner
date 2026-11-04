<script lang="ts">
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { AVATAR_CLASSES, AvatarSize, LIVE_TAG } from "./ui.constants";

  type ChannelAvatarPropsType = {
    url: string | undefined | null;
    name: string;
    size: AvatarSize;
    live: boolean;
  };

  const TAG_CLASSES: Partial<Record<AvatarSize, string>> = {
    [AvatarSize.SM]: "px-0.5 text-live-tag-sm/3",
    [AvatarSize.MD]: "px-1 text-live-tag/3.5",
    [AvatarSize.LG]: "px-1 text-live-tag/3.5",
    [AvatarSize.XL]: "px-1 text-live-tag/3.5",
  };

  let { url, name, size, live }: ChannelAvatarPropsType = $props();

  let failed = $state(false);

  $effect(() => {
    void url;
    failed = false;
  });

  const tagged = $derived(live && TAG_CLASSES[size] !== undefined);

  const classes = $derived(
    cn(
      "inline-flex shrink-0 items-center justify-center overflow-hidden bg-muted font-semibold text-muted-foreground uppercase",
      AVATAR_CLASSES[size],
      size === AvatarSize.XS || size === AvatarSize.SM ? "rounded-md" : "rounded-lg",
    ),
  );
</script>

<span class="relative inline-flex shrink-0">
  {#if url && !failed}
    <img
      class={cn(classes, "object-cover")}
      src={url}
      alt=""
      loading="lazy"
      onerror={() => (failed = true)}
    />
  {:else}
    <span class={classes} aria-hidden="true">{name.slice(0, 1)}</span>
  {/if}
  {#if tagged}
    <span
      class={cn(
        "absolute -bottom-1 left-1/2 -translate-x-1/2 rounded-sm bg-live font-bold tracking-wide text-live-foreground uppercase ring-2 ring-card",
        TAG_CLASSES[size],
      )}
      aria-hidden="true">{LIVE_TAG}</span
    >
  {/if}
  {#if tagged}<span class="sr-only">{t.common_live()}</span>{/if}
</span>
