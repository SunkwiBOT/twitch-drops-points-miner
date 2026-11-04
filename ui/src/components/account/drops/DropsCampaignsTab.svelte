<script lang="ts">
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import CampaignCard from "../../catalogue/CampaignCard.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import Panel from "../../ui/Panel.svelte";

  type DropsCampaignsTabPropsType = {
    account: AccountSnapshotType;
    campaigns: CampaignType[];
    onopen: (campaign: CampaignType) => void;
  };

  let { account, campaigns, onopen }: DropsCampaignsTabPropsType = $props();
</script>

<Panel title={t.drops_all_campaigns()}>
  {#if campaigns.length === 0}
    <EmptyState text={t.campaigns_empty()} />
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-4">
      {#each campaigns as c (c.id)}
        <CampaignCard
          campaign={c}
          ordered={account.settings.drops.campaign_order.includes(c.id)}
          {onopen}
        />
      {/each}
    </div>
  {/if}
</Panel>
