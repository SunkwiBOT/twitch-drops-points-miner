<script lang="ts">
  import type { Snippet } from "svelte";

  type MetaListPropsType = { children: Snippet };

  let { children }: MetaListPropsType = $props();
</script>

<dl class="m-0 grid min-w-0 grid-cols-[auto_minmax(0,1fr)] gap-x-6">{@render children()}</dl>
