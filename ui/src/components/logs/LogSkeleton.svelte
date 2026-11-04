<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { LOG_LINE_CLASSES, LOG_LIST_CLASSES } from "./logs.constants";

  type LogSkeletonPropsType = { rows: number };

  let { rows }: LogSkeletonPropsType = $props();
</script>

<div class={LOG_LIST_CLASSES} aria-hidden="true">
  {#each { length: rows }, index (index)}
    <div class={LOG_LINE_CLASSES}>
      <Skeleton class="my-0.5 h-3 w-16" />
      <Skeleton class="my-0.5 hidden h-3 w-10 md:block" />
      <Skeleton class="my-0.5 hidden h-3 md:block" />
      <Skeleton class="my-0.5 h-3 w-3/4" />
    </div>
  {/each}
</div>
