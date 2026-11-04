<script lang="ts">
  import type { Snippet } from "svelte";

  import { DialogFooter } from "$lib/components/ui/dialog";

  type DialogActionsPropsType = { children: Snippet };

  let { children }: DialogActionsPropsType = $props();
</script>

<DialogFooter class="sticky -bottom-5 z-10 -mx-5 -mb-5 bg-popover px-5 py-3.5">
  {@render children()}
</DialogFooter>
