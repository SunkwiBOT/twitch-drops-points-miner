<script lang="ts">
  import { FILLED_ICON_VIEWBOXES, ICON_PATHS } from "./icons.constants";
  import type { IconName } from "./ui.constants";

  type IconPropsType = { name: IconName; size: number; class?: string };

  let { name, size, class: className }: IconPropsType = $props();
</script>

{#if FILLED_ICON_VIEWBOXES.has(name)}
  <svg
    class={className}
    width={size}
    height={size}
    viewBox={FILLED_ICON_VIEWBOXES.get(name)}
    fill="currentColor"
    aria-hidden="true"
  >
    <path d={ICON_PATHS[name]} fill-rule="evenodd" />
  </svg>
{:else}
  <svg
    class={className}
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d={ICON_PATHS[name]} />
  </svg>
{/if}
