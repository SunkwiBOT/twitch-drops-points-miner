<script lang="ts">
  import { cn } from "$lib/utils";
  import { AVATAR_CLASSES, AvatarSize } from "./ui.constants";

  type AvatarPropsType = { url: string | undefined; name: string; size: AvatarSize };

  let { url, name, size }: AvatarPropsType = $props();

  let failed = $state(false);

  $effect(() => {
    void url;
    failed = false;
  });

  const classes = $derived(
    cn(
      "inline-flex shrink-0 items-center justify-center overflow-hidden rounded-full bg-muted font-semibold text-muted-foreground uppercase",
      AVATAR_CLASSES[size],
    ),
  );
</script>

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
