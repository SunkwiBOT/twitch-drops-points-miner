<script lang="ts">
  import { cn } from "$lib/utils";

  type TimeLeftTagPropsType = { text: string; urgent: boolean };

  let { text, urgent }: TimeLeftTagPropsType = $props();
</script>

<span
  class={cn(
    "inline-flex h-4.5 shrink-0 items-center rounded-md px-1.5 text-xs font-bold",
    urgent ? "bg-live text-live-foreground" : "bg-background/85 text-foreground",
  )}>{text}</span
>
