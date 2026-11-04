<script lang="ts">
  import { SlotPurposeKind } from "../../lib/bindings/SlotPurposeKind";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Badge from "./Badge.svelte";
  import { BadgeTone, IconName } from "./ui.constants";

  type MiningBadgePropsType = { kind: SlotPurposeKind };

  let { kind }: MiningBadgePropsType = $props();
</script>

{#if kind === SlotPurposeKind.DROP}
  <Badge
    text={t.badge_mining_drops()}
    tone={BadgeTone.ACCENT}
    dot={false}
    icon={IconName.GIFT}
    hint={t.badge_hint_mining_drops()}
  />
{:else}
  <Badge
    text={t.badge_mining_points()}
    tone={BadgeTone.SUCCESS}
    dot={false}
    icon={IconName.CHANNEL_POINTS}
    hint={t.badge_hint_mining_points()}
  />
{/if}
