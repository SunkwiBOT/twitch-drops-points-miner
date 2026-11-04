<script lang="ts">
  import type { Snippet } from "svelte";

  type ListPropsType = { children: Snippet };

  let { children }: ListPropsType = $props();
</script>

<div class="flex flex-col overflow-hidden rounded-lg border" role="list">
  {@render children()}
</div>
