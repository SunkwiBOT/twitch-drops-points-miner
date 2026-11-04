<script lang="ts">
  import { Alert, AlertDescription } from "$lib/components/ui/alert";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import RichText from "./RichText.svelte";
  import { NOTICE_TONE_CLASSES, NOTICE_TONE_ICONS, NoticeTone } from "./ui.constants";

  type NoticePropsType = { tone: NoticeTone; text: string };

  let { tone, text }: NoticePropsType = $props();
</script>

<Alert class={cn("flex items-start gap-2.5", NOTICE_TONE_CLASSES[tone])}>
  <span class="mt-0.5 shrink-0"><Icon name={NOTICE_TONE_ICONS[tone]} size={16} /></span>
  <AlertDescription class="text-hint text-current"><RichText {text} /></AlertDescription>
</Alert>
