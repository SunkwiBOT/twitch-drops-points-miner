<script lang="ts">
  import type { Snippet } from "svelte";

  import Icon from "./Icon.svelte";
  import type { IconName } from "./ui.constants";

  type EmptyStatePropsType = {
    text: string;
    title?: string;
    icon?: IconName;
    action?: Snippet;
  };

  let { text, title, icon, action }: EmptyStatePropsType = $props();
</script>

<div
  class="flex flex-col items-center gap-2 rounded-lg border border-dashed px-5 py-6 text-center text-hint text-muted-foreground"
>
  {#if icon}<Icon name={icon} size={22} />{/if}
  {#if title}<h2 class="text-base font-semibold text-foreground">{title}</h2>{/if}
  <p class="max-w-[60ch]">{text}</p>
  {#if action}<div class="mt-1 flex flex-wrap justify-center gap-2">{@render action()}</div>{/if}
</div>
