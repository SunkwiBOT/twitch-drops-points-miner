<script lang="ts">
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import { CopySection, DROPS_TAB_SLUGS, DropsTab } from "../../lib/accounts/accounts.constants";
  import { ignoredItems, ongoingCampaigns } from "../../lib/accounts/drops.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app, openTab, showTab } from "../../lib/state/state.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";
  import CopyFromDialog from "./CopyFromDialog.svelte";
  import DropsCampaignsTab from "./drops/DropsCampaignsTab.svelte";
  import DropsExclusionsTab from "./drops/DropsExclusionsTab.svelte";
  import DropsOrderTab from "./drops/DropsOrderTab.svelte";
  import DropsSettingsTab from "./drops/DropsSettingsTab.svelte";

  type AccountDropsPropsType = { account: AccountSnapshotType; snapshot: SnapshotType };

  let { account, snapshot }: AccountDropsPropsType = $props();

  const drops = $derived(account.settings.drops);
  const hasOthers = $derived((app.snapshot?.accounts.length ?? 0) > 1);
  const ignored = $derived(ignoredItems(account, snapshot));
  const ongoing = $derived(ongoingCampaigns(account, ticker.now));

  const tab = $derived(openTab(Object.values(DropsTab), DROPS_TAB_SLUGS, DropsTab.ORDER));
  let copying = $state(false);
  let open = $state<CampaignType | null>(null);

  const openCampaign = (campaign: CampaignType): void => {
    open = campaign;
  };
</script>

<PageBody>
  <div class="flex flex-wrap items-center justify-between gap-3">
    <Segmented
      value={tab}
      label={t.drops_tabs_label()}
      onchange={(next) => showTab(DROPS_TAB_SLUGS, next)}
      options={[
        { value: DropsTab.ORDER, label: t.queue_title(), icon: IconName.LIST_ORDERED },
        { value: DropsTab.SETTINGS, label: t.drops_tab_settings(), icon: IconName.SETTINGS },
        {
          value: DropsTab.EXCLUSIONS,
          label: t.drops_tab_exclusions(),
          icon: IconName.EYE_OFF,
          count: ignored.length + drops.exclude_games.length + drops.exclude_names.length,
        },
        {
          value: DropsTab.CAMPAIGNS,
          label: t.drops_tab_campaigns(),
          icon: IconName.GRID,
          count: ongoing.length,
        },
      ]}
    />
    {#if hasOthers}
      <TextButton
        label={t.copy_drops_from_account()}
        icon={IconName.COPY}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (copying = true)}
      />
    {/if}
  </div>

  {#if tab === DropsTab.ORDER}
    <DropsOrderTab {account} {snapshot} onopen={openCampaign} />
  {:else if tab === DropsTab.SETTINGS}
    <DropsSettingsTab {account} />
  {:else if tab === DropsTab.EXCLUSIONS}
    <DropsExclusionsTab {account} {snapshot} {ignored} onopen={openCampaign} />
  {:else}
    <DropsCampaignsTab {account} campaigns={ongoing} onopen={openCampaign} />
  {/if}
  <AccountSaveBar {account} />
</PageBody>

<CampaignDialog campaign={open} contextAccountId={account.id} onclose={() => (open = null)} />

<CopyFromDialog
  open={copying}
  {account}
  section={CopySection.DROPS}
  onclose={() => (copying = false)}
/>
