<script lang="ts">
  import { SlotPurposeKind } from "../../lib/bindings/SlotPurposeKind";
  import { STATUS_HINTS, STATUS_LABELS, STATUS_TONES } from "../../lib/accounts/accounts.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Badge from "./Badge.svelte";
  import { BadgeTone, IconName } from "./ui.constants";

  type AccountStatusBadgesPropsType = { account: AccountSnapshotType };

  let { account }: AccountStatusBadgesPropsType = $props();

  const drops = $derived(account.slots.some((slot) => slot.purpose.kind === SlotPurposeKind.DROP));
  const points = $derived(
    account.slots.some((slot) => slot.purpose.kind === SlotPurposeKind.POINTS),
  );
</script>

{#if account.status === AccountStatus.MINING && (drops || points)}
  {#if drops}<Badge
      text={t.badge_mining_drops()}
      tone={BadgeTone.ACCENT}
      dot={false}
      icon={IconName.GIFT}
      hint={t.badge_hint_mining_drops()}
    />{/if}
  {#if points}<Badge
      text={t.badge_mining_points()}
      tone={BadgeTone.SUCCESS}
      dot={false}
      icon={IconName.CHANNEL_POINTS}
      hint={t.badge_hint_mining_points()}
    />{/if}
{:else}
  <Badge
    text={STATUS_LABELS[account.status]()}
    tone={STATUS_TONES[account.status]}
    dot={true}
    hint={STATUS_HINTS[account.status]?.()}
  />
{/if}
