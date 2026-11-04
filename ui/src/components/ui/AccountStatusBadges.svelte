<script lang="ts">
  import { SlotPurposeKind } from "../../lib/bindings/SlotPurposeKind";
  import { STATUS_HINTS, STATUS_LABELS, STATUS_TONES } from "../../lib/accounts/accounts.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import Badge from "./Badge.svelte";
  import MiningBadge from "./MiningBadge.svelte";

  type AccountStatusBadgesPropsType = { account: AccountSnapshotType };

  let { account }: AccountStatusBadgesPropsType = $props();

  const drops = $derived(account.slots.some((slot) => slot.purpose.kind === SlotPurposeKind.DROP));
  const points = $derived(
    account.slots.some((slot) => slot.purpose.kind === SlotPurposeKind.POINTS),
  );
</script>

{#if account.status === AccountStatus.MINING && (drops || points)}
  {#if drops}<MiningBadge kind={SlotPurposeKind.DROP} />{/if}
  {#if points}<MiningBadge kind={SlotPurposeKind.POINTS} />{/if}
{:else}
  <Badge
    text={STATUS_LABELS[account.status]()}
    tone={STATUS_TONES[account.status]}
    dot={true}
    hint={STATUS_HINTS[account.status]?.()}
  />
{/if}
