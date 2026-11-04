<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { cn } from "$lib/utils";

  type FeedSkeletonPropsType = { rows: number; wide: boolean };

  let { rows, wide }: FeedSkeletonPropsType = $props();
</script>

<ul class="m-0 flex list-none flex-col p-0" aria-hidden="true">
  {#each { length: rows }, index (index)}
    <li
      class={cn(
        "grid items-center gap-3 border-b py-2.5 last:border-b-0",
        wide
          ? "grid-cols-[7.5rem_2.25rem_minmax(0,1fr)_auto]"
          : "grid-cols-[3rem_2.25rem_minmax(0,1fr)_auto]",
      )}
    >
      <Skeleton class="h-3 w-4/5" />
      <Skeleton class="size-9 rounded-md" />
      <span class="flex flex-col gap-1.5">
        <Skeleton class="h-3.5 w-3/4" />
        <Skeleton class="h-3 w-1/3" />
      </span>
      <Skeleton class="h-5 w-20 rounded-full" />
    </li>
  {/each}
</ul>
