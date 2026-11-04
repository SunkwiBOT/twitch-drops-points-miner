<script lang="ts">
  import type { Snippet } from "svelte";

  import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";

  type TipPropsType = {
    text: string;
    children: Snippet<[Record<string, unknown>]>;
  };

  let { text, children }: TipPropsType = $props();
</script>

<Tooltip ignoreNonKeyboardFocus={true}>
  <TooltipTrigger>
    {#snippet child({ props })}{@render children(props)}{/snippet}
  </TooltipTrigger>
  <TooltipContent>{text}</TooltipContent>
</Tooltip>
