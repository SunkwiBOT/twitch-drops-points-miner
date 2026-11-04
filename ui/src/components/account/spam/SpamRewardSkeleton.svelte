<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { SPAM_SKELETON_CARDS } from "../../../lib/spam/spam.constants";
</script>

<div class="flex flex-col gap-3" aria-hidden="true">
  <Skeleton class="h-5 w-32" />
  <div class="grid grid-cols-[repeat(auto-fill,minmax(11rem,1fr))] gap-2.5">
    {#each { length: SPAM_SKELETON_CARDS }, index (index)}
      <div class="flex flex-col overflow-hidden rounded-md ring-1 ring-border">
        <Skeleton class="h-20 rounded-none" />
        <div class="flex flex-col gap-2 p-2.5">
          <Skeleton class="h-4 w-3/4" />
          <div class="flex h-8 items-center justify-between gap-2">
            <Skeleton class="h-4 w-12" />
            <Skeleton class="h-8 w-26 rounded-lg" />
          </div>
          <Skeleton class="h-9 w-full rounded-lg" />
        </div>
      </div>
    {/each}
  </div>
</div>
