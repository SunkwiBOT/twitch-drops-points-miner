<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import Badge from "../ui/Badge.svelte";
  import { BadgeTone, IconName } from "../ui/ui.constants";

  type LinkBadgePropsType = { linked: boolean | null | undefined };

  let { linked }: LinkBadgePropsType = $props();
</script>

{#if linked === true}
  <Badge
    text={t.dashboard_linked()}
    tone={BadgeTone.SUCCESS}
    dot={false}
    icon={IconName.LINK}
    hint={t.badge_hint_linked()}
  />
{:else if linked === false}
  <Badge
    text={t.dashboard_not_linked()}
    tone={BadgeTone.WARNING}
    dot={false}
    icon={IconName.ALERT}
    hint={t.badge_hint_not_linked()}
  />
{/if}
