<script lang="ts">
  import { cn } from "$lib/utils";
  import AvatarImage from "./AvatarImage.svelte";
  import { AVATAR_CLASSES, AvatarSize } from "./ui.constants";

  type AvatarPropsType = { url: string | undefined; name: string; size: AvatarSize };

  let { url, name, size }: AvatarPropsType = $props();

  const classes = $derived(
    cn(
      "inline-flex shrink-0 items-center justify-center overflow-hidden rounded-full bg-muted font-semibold text-muted-foreground uppercase",
      AVATAR_CLASSES[size],
    ),
  );
</script>

<AvatarImage {url} {name} class={classes} />
