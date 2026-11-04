<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { untrack } from "svelte";

  import AccountSaveBar from "./AccountSaveBar.svelte";
  import { campaignEnd } from "../../lib/catalogue/campaign.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { TimedDropType } from "../../lib/bindings/TimedDropType";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { AccountField, ToastKind } from "../../lib/state/state.constants";
  import { editsOf } from "../../lib/state/edits.svelte";
  import { run, toast } from "../../lib/state/state.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { escapeEmphasis, strong } from "../../lib/text.utils";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import LinkBadge from "../catalogue/LinkBadge.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import Badge from "../ui/Badge.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import List from "../ui/List.svelte";
  import ListRow from "../ui/ListRow.svelte";
  import Notice from "../ui/Notice.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Panel from "../ui/Panel.svelte";
  import ProgressLine from "../ui/ProgressLine.svelte";
  import Setting from "../ui/Setting.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import {
    BadgeTone,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    NoticeTone,
    ThumbShape,
    ThumbSize,
  } from "../ui/ui.constants";

  type AccountInventoryPropsType = { account: AccountSnapshotType };

  type RewardType = { campaign: CampaignType; drop: TimedDropType };

  let { account }: AccountInventoryPropsType = $props();

  const rewards = $derived<RewardType[]>(
    account.campaigns.flatMap((campaign) => campaign.drops.map((drop) => ({ campaign, drop }))),
  );

  let claimedHere = $state<string[]>([]);

  const ready = $derived(
    rewards.filter(
      ({ drop }) => !drop.claimed && drop.instance_id && !claimedHere.includes(drop.instance_id),
    ),
  );

  // ! Twitch takes a claim on a game the account has not linked and gives
  // nothing: those drops wait for the link.
  const claimable = $derived(ready.filter(({ campaign }) => campaign.linked !== false));
  const waitingLink = $derived(ready.filter(({ campaign }) => campaign.linked === false));

  let open = $state<CampaignType | null>(null);

  const inProgress = $derived(
    account.campaigns
      .filter((campaign) => campaign.in_inventory)
      .map((campaign) => ({
        campaign,
        drops: campaign.drops.filter((drop) => !drop.claimed && !drop.instance_id),
      }))
      .filter((group) => group.drops.length > 0),
  );

  const claimed = $derived(rewards.filter(({ drop }) => drop.claimed));

  const edits = editsOf(untrack(() => account.id));

  const setAutoClaim = (value: boolean): void => {
    edits.set(AccountField.AUTO_CLAIM, (settings) => (settings.drops.auto_claim = value));
  };

  let claiming = $state<string | null>(null);
  let claimingAll = $state(false);

  const claim = async (instanceId: string): Promise<void> => {
    claiming = instanceId;

    if (
      await run({
        command: CommandName.CLAIM_DROP,
        account_id: account.id,
        drop_instance_id: instanceId,
      })
    ) {
      claimedHere = [...claimedHere, instanceId];
    }

    claiming = null;
  };

  const claimAll = async (): Promise<void> => {
    claimingAll = true;
    let done = 0;

    for (const { drop } of claimable) {
      if (drop.instance_id) {
        const instanceId = drop.instance_id;
        const claimedOne = await run({
          command: CommandName.CLAIM_DROP,
          account_id: account.id,
          drop_instance_id: instanceId,
        });

        if (claimedOne) {
          claimedHere = [...claimedHere, instanceId];
          done += 1;
        }
      }
    }

    claimingAll = false;

    if (done > 0) {
      toast(ToastKind.SUCCESS, t.inventory_claimed_all({ count: strong(done) }), {
        image: { url: account.avatar_url, name: account.display_name, channel: null },
        title: account.display_name,
      });
    }
  };
</script>

{#snippet reward(item: RewardType, size: ThumbSize)}
  <Thumb
    src={item.drop.benefits.at(0)?.image_url}
    round={false}
    contain={true}
    {size}
    shape={ThumbShape.SQUARE}
    fallback={IconName.GIFT}
  />
{/snippet}

<PageBody>
  <PageHeader intro={t.inventory_hint()} />
  {#if !account.settings.enabled && account.campaigns.length === 0}
    <Notice tone={NoticeTone.INFO} text={t.inventory_paused()} />
  {/if}

  <Panel title={t.inventory_claimable()} count={claimable.length}>
    {#snippet actions()}
      {#if claimable.length > 1}
        <ActionButton
          icon={IconName.GIFT}
          label={t.inventory_claim_all()}
          busyLabel={t.campaign_claiming()}
          busy={claimingAll}
          disabled={claiming !== null}
          variant={ButtonVariant.PRIMARY}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          onclick={claimAll}
        />
      {/if}
    {/snippet}
    <Setting
      title={t.account_drops_auto_claim()}
      description={t.drops_auto_claim_hint()}
      checked={edits.applied(account.settings).drops.auto_claim}
      disabled={false}
      onchange={setAutoClaim}
    />
    {#if claimable.length === 0}
      <EmptyState text={t.inventory_empty_claimable()} />
    {:else}
      <List>
        {#each claimable as item (item.drop.id)}
          {@const instanceId = item.drop.instance_id ?? ""}
          <ListRow
            title={item.drop.name}
            subtitle={`${escapeEmphasis(item.campaign.game.name)} · ${escapeEmphasis(item.campaign.name)}`}
            onopen={() => (open = item.campaign)}
            muted={false}
          >
            {#snippet lead()}{@render reward(item, ThumbSize.MD)}{/snippet}
            {#snippet trailing()}
              <ActionButton
                icon={IconName.GIFT}
                label={t.reward_claim()}
                busyLabel={t.campaign_claiming()}
                busy={claiming === instanceId}
                disabled={claimingAll || (claiming !== null && claiming !== instanceId)}
                variant={ButtonVariant.PRIMARY}
                size={ButtonSize.SM}
                type={ButtonType.BUTTON}
                onclick={() => claim(instanceId)}
              />
            {/snippet}
          </ListRow>
        {/each}
      </List>
    {/if}
  </Panel>

  {#if waitingLink.length > 0}
    <Panel
      title={t.inventory_waiting_link()}
      count={waitingLink.length}
      description={t.badge_hint_not_linked()}
    >
      <List>
        {#each waitingLink as item (item.drop.id)}
          <ListRow
            title={item.drop.name}
            subtitle={`${escapeEmphasis(item.campaign.game.name)} · ${escapeEmphasis(item.campaign.name)}`}
            onopen={() => (open = item.campaign)}
            muted={false}
          >
            {#snippet lead()}{@render reward(item, ThumbSize.MD)}{/snippet}
            {#snippet trailing()}
              {#if item.campaign.link_url}
                <ExternalButton
                  url={item.campaign.link_url}
                  label={t.campaign_link_game()}
                  variant={ButtonVariant.OUTLINE}
                  size={ButtonSize.SM}
                  icon={IconName.LINK}
                />
              {:else}
                <LinkBadge linked={false} />
              {/if}
            {/snippet}
          </ListRow>
        {/each}
      </List>
    </Panel>
  {/if}

  <Panel title={t.inventory_in_progress()} count={inProgress.length}>
    {#if inProgress.length === 0}
      <EmptyState text={t.inventory_empty_progress()} />
    {:else}
      <List>
        {#each inProgress as group (group.campaign.id)}
          <ListRow
            title={group.campaign.game.name}
            details={[
              escapeEmphasis(group.campaign.name),
              campaignEnd(group.campaign.ends_at, ticker.now),
            ]}
            onopen={() => (open = group.campaign)}
            muted={false}
          >
            {#snippet lead()}
              <Thumb
                src={boxArt(group.campaign.game.box_art_url, BoxArtSize.ROW)}
                round={false}
                contain={false}
                size={ThumbSize.MD}
                shape={ThumbShape.BOX_ART}
                fallback={IconName.GIFT}
              />
            {/snippet}
            {#snippet below()}
              <span class="flex flex-col gap-2">
                {#each group.drops as drop (drop.id)}
                  <span class="flex items-center gap-2.5">
                    {@render reward({ campaign: group.campaign, drop }, ThumbSize.SM)}
                    <span class="flex min-w-0 flex-1 flex-col gap-1">
                      <span class="flex min-w-0 items-center gap-2">
                        <span class="truncate text-xs font-semibold">{drop.name}</span>
                        {#if drop.required_subs > 0}<Badge
                            text={t.reward_sub_only()}
                            tone={BadgeTone.WARNING}
                            dot={false}
                            hint={t.badge_hint_sub_only()}
                          />{/if}
                      </span>
                      {#if drop.required_subs === 0}
                        <ProgressLine
                          current={drop.current_minutes}
                          required={drop.required_minutes}
                          stacked={false}
                        />
                      {/if}
                    </span>
                  </span>
                {/each}
              </span>
            {/snippet}
          </ListRow>
        {/each}
      </List>
    {/if}
  </Panel>

  <Panel title={t.inventory_claimed()} count={claimed.length}>
    {#if claimed.length === 0}
      <EmptyState text={t.inventory_empty_claimed()} />
    {:else}
      <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-2.5">
        {#each claimed as item (item.drop.id)}
          <button
            class="flex items-center gap-2.5 rounded-lg border p-2 text-left transition-colors hover:bg-accent"
            type="button"
            onclick={() => (open = item.campaign)}
          >
            {@render reward(item, ThumbSize.MD)}
            <span class="flex min-w-0 flex-col">
              <span class="truncate text-xs font-semibold">{item.drop.name}</span>
              <span class="truncate text-xs text-muted-foreground">{item.campaign.game.name}</span>
            </span>
          </button>
        {/each}
      </div>
    {/if}
  </Panel>
  <AccountSaveBar {account} />
</PageBody>

<CampaignDialog campaign={open} contextAccountId={account.id} onclose={() => (open = null)} />
