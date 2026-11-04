<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { campaignEnd } from "../../lib/catalogue/campaign.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { ErrorApiMessage } from "../../lib/bindings/ErrorApiMessage";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeCode } from "../../lib/errors/errors.utils";
  import { POINTS_ROUTINE } from "../../lib/events/events.utils";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { escapeEmphasis, strong } from "../../lib/text.utils";
  import { Page } from "../../lib/state/state.constants";
  import { app, navigate, run, setDropIgnored } from "../../lib/state/state.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import AccountActivity from "../AccountActivity.svelte";
  import ActivityFeed from "../ActivityFeed.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import LinkBadge from "../catalogue/LinkBadge.svelte";
  import RewardTiles from "../catalogue/RewardTiles.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { Card } from "$lib/components/ui/card";
  import EmptyState from "../ui/EmptyState.svelte";
  import List from "../ui/List.svelte";
  import ListRow from "../ui/ListRow.svelte";
  import Notice from "../ui/Notice.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Panel from "../ui/Panel.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import {
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    NoticeTone,
    ThumbShape,
    ThumbSize,
  } from "../ui/ui.constants";

  type AccountOverviewPropsType = { account: AccountSnapshotType; snapshot: SnapshotType };

  const QUEUE_PREVIEW = 6;
  const RECENT_MAX = 12;

  let { account, snapshot }: AccountOverviewPropsType = $props();

  const current = $derived.by<CampaignType | null>(() => {
    const id = account.slots.find((slot) => slot.drop)?.drop?.campaign_id;

    return id ? (account.campaigns.find((c) => c.id === id) ?? null) : null;
  });

  const campaignOf = (id: string): CampaignType | undefined =>
    account.campaigns.find((c) => c.id === id);

  const events = $derived(
    [...app.liveEvents, ...snapshot.recent_events]
      .filter((e) => e.account_id === account.id && !POINTS_ROUTINE.has(e.kind))
      .filter((e, i, all) => all.findIndex((o) => o.at === e.at && o.title === e.title) === i)
      .slice(0, RECENT_MAX),
  );

  let open = $state<CampaignType | null>(null);

  const claim = (instanceId: string): Promise<boolean> =>
    run({ command: CommandName.CLAIM_DROP, account_id: account.id, drop_instance_id: instanceId });
</script>

<PageBody>
  {#if account.problem === ErrorApiMessage.BEACON_BLOCKED}
    <Notice tone={NoticeTone.WARNING} text={t.beacon_blocked()} />
  {:else if account.problem && account.status !== AccountStatus.LOGIN_REQUIRED}
    <Notice tone={NoticeTone.DANGER} text={describeCode(account.problem)} />
  {/if}

  <Card class="gap-0 py-0"><AccountActivity {account} /></Card>

  <div class="grid grid-cols-[repeat(auto-fit,minmax(min(420px,100%),1fr))] items-start gap-5">
    <Panel title={t.overview_queue()}>
      {#snippet actions()}
        <TextButton
          label={t.overview_manage_queue()}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => navigate(Page.ACCOUNT_DROPS)}
        />
      {/snippet}
      {#if account.queue.length === 0}
        <EmptyState text={t.queue_empty()} />
      {:else}
        <List>
          {#each account.queue.slice(0, QUEUE_PREVIEW) as q, i (q.campaign_id)}
            {@const c = campaignOf(q.campaign_id)}
            <ListRow
              title={c?.game.name ?? q.campaign_id}
              details={[
                escapeEmphasis(c?.name ?? ""),
                t.dashboard_remaining({ minutes: strong(q.remaining_minutes) }),
                campaignEnd(q.ends_at, ticker.now),
              ]}
              onopen={() => (open = c ?? null)}
              muted={false}
            >
              {#snippet lead()}
                <span class="w-4 shrink-0 text-right text-xs text-muted-foreground tabular-nums"
                  >{i + 1}</span
                >
                <Thumb
                  src={boxArt(c?.game.box_art_url, BoxArtSize.ROW)}
                  round={false}
                  contain={false}
                  size={ThumbSize.LG}
                  shape={ThumbShape.BOX_ART}
                  fallback={IconName.GIFT}
                />
              {/snippet}
              {#snippet trailing()}<LinkBadge linked={q.linked} />{/snippet}
            </ListRow>
          {/each}
        </List>
      {/if}
    </Panel>
    <Panel title={t.overview_rewards()}>
      {#snippet actions()}
        {#if current}
          <TextButton
            label={t.campaign_details()}
            variant={ButtonVariant.OUTLINE}
            size={ButtonSize.SM}
            type={ButtonType.BUTTON}
            disabled={false}
            onclick={() => (open = current)}
          />
        {/if}
      {/snippet}
      {#if current}
        <RewardTiles
          campaign={current}
          ignored={account.settings.drops.exclude_drop_ids}
          onclaim={claim}
          onignore={(drop, ignored) => setDropIgnored([account.id], drop, ignored)}
        />
      {:else}
        <EmptyState text={t.overview_no_campaign()} />
      {/if}
    </Panel>
  </div>

  <Panel title={t.overview_recent()}>
    {#snippet actions()}
      <TextButton
        label={t.nav_history()}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => navigate(Page.ACCOUNT_HISTORY)}
      />
    {/snippet}
    <ActivityFeed {events} showAccount={false} wide={false} empty={t.history_empty()} />
  </Panel>
</PageBody>

<CampaignDialog campaign={open} contextAccountId={account.id} onclose={() => (open = null)} />
