<script lang="ts">
  import { cn } from "$lib/utils";

  type AvatarImagePropsType = { url: string | undefined | null; name: string; class: string };

  let { url, name, class: className }: AvatarImagePropsType = $props();

  let failed = $state(false);

  $effect(() => {
    void url;
    failed = false;
  });
</script>

{#if url && !failed}
  <img
    class={cn(className, "object-cover")}
    src={url}
    alt=""
    loading="lazy"
    onerror={() => (failed = true)}
  />
{:else}
  <span class={className} aria-hidden="true">{name.slice(0, 1)}</span>
{/if}
