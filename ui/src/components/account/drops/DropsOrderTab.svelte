<script lang="ts">
  import { DropsList } from "../../../lib/accounts/accounts.constants";
  import {
    QUEUE_BLOCK_HINTS,
    QUEUE_BLOCK_LABELS,
    QUEUE_REASON_HINTS,
    QUEUE_REASON_LABELS,
  } from "../../../lib/accounts/accounts.utils";
  import { miningOrder, reorderBy } from "../../../lib/accounts/changes.utils";
  import {
    campaignOf,
    catalogueGame,
    editDropsList,
    ignoredCampaign,
    movePriorityGame,
    queueProgress,
    reorderQueue,
    resetQueue,
    runningGames,
    setIgnored,
  } from "../../../lib/accounts/drops.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import type { SnapshotType } from "../../../lib/bindings/SnapshotType";
  import { campaignEnd, categoryUrl } from "../../../lib/catalogue/campaign.utils";
  import { BoxArtSize } from "../../../lib/format.constants";
  import { boxArt, formatPercent } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { ticker } from "../../../lib/state/ticker.svelte";
  import { escapeEmphasis, strong } from "../../../lib/text.utils";
  import TextButton from "../../ui/TextButton.svelte";
  import LinkBadge from "../../catalogue/LinkBadge.svelte";
  import Badge from "../../ui/Badge.svelte";
  import ConfirmDialog from "../../ui/ConfirmDialog.svelte";
  import DetailLine from "../../ui/DetailLine.svelte";
  import ExternalLink from "../../ui/ExternalLink.svelte";
  import IconButton from "../../ui/IconButton.svelte";
  import Panel from "../../ui/Panel.svelte";
  import ProgressLine from "../../ui/ProgressLine.svelte";
  import SortableList from "../../ui/SortableList.svelte";
  import Thumb from "../../ui/Thumb.svelte";
  import TiledArt from "../../ui/TiledArt.svelte";
  import {
    BadgeTone,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    LinkLook,
    LinkMark,
    LinkSize,
    ThumbShape,
    ThumbSize,
  } from "../../ui/ui.constants";
  import GameDialog from "../GameDialog.svelte";

  type DropsOrderTabPropsType = {
    account: AccountSnapshotType;
    snapshot: SnapshotType;
    onopen: (campaign: CampaignType) => void;
  };

  let { account, snapshot, onopen }: DropsOrderTabPropsType = $props();

  const drops = $derived(account.settings.drops);

  const queue = $derived(
    reorderBy(account.queue, (item) => item.campaign_id, miningOrder(account.settings, account)),
  );

  const mining = (id: string): boolean =>
    account.slots.some((slot) => slot.drop?.campaign_id === id);

  const running = $derived(runningGames(snapshot, ticker.now));

  let adding = $state(false);
  let removing = $state<string | null>(null);
</script>

<div class="grid items-start gap-5 xl:grid-cols-[minmax(0,2fr)_minmax(0,3fr)]">
  <Panel title={t.drops_priority_games()} description={t.account_drops_priority_hint()}>
    {#snippet actions()}
      <TextButton
        label={t.drops_add_game()}
        icon={IconName.PLUS}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (adding = true)}
      />
    {/snippet}
    <SortableList
      items={drops.priority_games}
      key={(game) => game}
      empty={t.drops_priority_empty()}
      upLabel={t.common_up()}
      downLabel={t.common_down()}
      onreorder={(list, moved) => movePriorityGame(account, list, moved)}
    >
      {#snippet background(game)}
        {@const art = boxArt(catalogueGame(snapshot, game)?.box_art_url, BoxArtSize.ROW)}
        {#if art}<TiledArt url={art} shape={ThumbShape.BOX_ART} grey={false} />{/if}
      {/snippet}
      {#snippet row(game)}
        <span class="flex min-w-0 flex-1">
          <ExternalLink
            url={categoryUrl(game, catalogueGame(snapshot, game)?.slug)}
            text={game}
            mark={LinkMark.END}
            look={LinkLook.HOVER}
            size={LinkSize.MD}
          />
        </span>
      {/snippet}
      {#snippet actions(game)}
        <IconButton
          icon={IconName.TRASH}
          label={t.confirm_remove()}
          busy={false}
          disabled={false}
          variant={ButtonVariant.DANGER_GHOST}
          size={ButtonSize.SM}
          onclick={() => (removing = game)}
        />
      {/snippet}
    </SortableList>
  </Panel>
  <Panel title={t.queue_title()} description={t.queue_hint()}>
    {#snippet actions()}
      {#if drops.campaign_order.length > 0}
        <TextButton
          label={t.queue_reset()}
          icon={IconName.REFRESH}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => resetQueue(account)}
        />
      {/if}
    {/snippet}
    <SortableList
      items={queue}
      key={(item) => item.campaign_id}
      empty={t.queue_empty()}
      upLabel={t.common_up()}
      downLabel={t.common_down()}
      onreorder={(items, moved) => reorderQueue(account, snapshot, items, moved)}
    >
      {#snippet row(item)}
        {@const c = campaignOf(account, snapshot, item.campaign_id)}
        {@const done = queueProgress(account, snapshot, item.campaign_id)}
        <Thumb
          src={boxArt(c?.game.box_art_url, BoxArtSize.ROW)}
          round={false}
          contain={false}
          size={ThumbSize.XL}
          shape={ThumbShape.BOX_ART}
          fallback={IconName.GIFT}
        />
        <button
          class="flex min-w-0 flex-1 flex-col gap-0.5 text-left"
          type="button"
          onclick={() => c && onopen(c)}
        >
          <span class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
            <span class="truncate font-semibold">{c?.game.name ?? item.campaign_id}</span>
            <span class="inline-flex max-w-full flex-wrap items-center gap-1.5">
              {#if mining(item.campaign_id)}<Badge
                  text={t.queue_mining()}
                  tone={BadgeTone.SUCCESS}
                  dot={true}
                  hint={t.badge_hint_queue_mining()}
                />{:else if item.blocked}<Badge
                  text={QUEUE_BLOCK_LABELS[item.blocked]()}
                  tone={BadgeTone.WARNING}
                  dot={false}
                  hint={QUEUE_BLOCK_HINTS[item.blocked]()}
                />{:else}<Badge
                  text={QUEUE_REASON_LABELS[item.reason]()}
                  tone={BadgeTone.MUTED}
                  dot={false}
                  hint={QUEUE_REASON_HINTS[item.reason]()}
                />{/if}
              {#if item.linked === false}<LinkBadge linked={false} />{/if}
            </span>
          </span>
          <DetailLine
            parts={[
              escapeEmphasis(c?.name ?? ""),
              t.dashboard_remaining({ minutes: strong(item.remaining_minutes) }),
              campaignEnd(item.ends_at, ticker.now),
            ]}
          />
          <span class="mt-1 block">
            <ProgressLine
              current={done}
              required={100}
              label={formatPercent(done)}
              stacked={false}
            />
          </span>
        </button>
      {/snippet}
      {#snippet actions(item)}
        <IconButton
          icon={IconName.EYE_OFF}
          label={t.queue_ignore()}
          busy={false}
          disabled={false}
          variant={ButtonVariant.GHOST}
          size={ButtonSize.SM}
          onclick={() =>
            setIgnored(account, ignoredCampaign(account, snapshot, item.campaign_id), true)}
        />
      {/snippet}
    </SortableList>
  </Panel>
</div>

<ConfirmDialog
  open={removing !== null}
  title={t.confirm_remove_game_title()}
  text={t.confirm_remove_game_text({ name: strong(removing ?? "") })}
  confirmLabel={t.confirm_remove()}
  busyLabel={t.common_removing()}
  busy={false}
  onconfirm={() => {
    const game = removing;
    removing = null;

    if (game) {
      editDropsList(
        account,
        DropsList.PRIORITY_GAMES,
        drops.priority_games.filter((g) => g !== game),
      );
    }
  }}
  oncancel={() => (removing = null)}
/>

<GameDialog
  open={adding}
  catalogue={running}
  taken={drops.priority_games}
  onadd={(game) => {
    adding = false;
    editDropsList(account, DropsList.PRIORITY_GAMES, [...drops.priority_games, game]);
  }}
  onclose={() => (adding = false)}
/>
