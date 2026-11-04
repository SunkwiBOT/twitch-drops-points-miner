<script lang="ts">
  import { Progress } from "$lib/components/ui/progress";
  import { cn } from "$lib/utils";
  import { percent } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import RichText from "./RichText.svelte";

  type ProgressLinePropsType = {
    current: number;
    required: number;
    label?: string;
    stacked: boolean;
  };

  let { current, required, label, stacked }: ProgressLinePropsType = $props();

  const text = $derived(
    label ?? t.campaign_progress({ current: strong(current), required: strong(required) }),
  );

  const spoken = $derived(
    label ?? t.campaign_progress({ current: String(current), required: String(required) }),
  );
</script>

<span class={cn("flex w-full gap-x-2 gap-y-1", stacked ? "flex-col items-center" : "items-center")}>
  <Progress
    value={percent(current, required)}
    class={cn("h-1.5", stacked ? "w-full" : "flex-1")}
    aria-label={spoken}
  />
  <span
    class={cn(
      "shrink-0 text-xs text-muted-foreground tabular-nums",
      !stacked && "min-w-18 text-right",
    )}><RichText {text} /></span
  >
</span>
