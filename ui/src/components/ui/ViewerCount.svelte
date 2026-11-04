<script lang="ts">
  import { number } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Icon from "./Icon.svelte";
  import { IconName } from "./ui.constants";

  type ViewerCountPropsType = { count: number };

  let { count }: ViewerCountPropsType = $props();
</script>

<span class="inline-flex shrink-0 items-center gap-1 font-semibold text-live-emphasis tabular-nums">
  <Icon name={IconName.VIEWERS} size={14} />
  <span aria-hidden="true">{number(count)}</span>
  <span class="sr-only">{t.common_viewers({ count, shown: number(count) })}</span>
</span>
