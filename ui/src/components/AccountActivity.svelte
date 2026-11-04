<script lang="ts">
  import Badge from "./ui/Badge.svelte";
  import {
    POINTS_REASON_HINTS,
    POINTS_REASON_ICONS,
    POINTS_REASON_LABELS,
  } from "../lib/accounts/accounts.utils";
  import { SLOTS_MAX } from "../lib/accounts/slots.constants";
  import type { PointsReason } from "../lib/bindings/PointsReason";
  import ChannelAvatar from "./ui/ChannelAvatar.svelte";
  import { AccountStatus } from "../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../lib/bindings/CampaignType";
  import { Mode } from "../lib/bindings/Mode";
  import { SlotPurposeKind } from "../lib/bindings/SlotPurposeKind";
  import { channelUrl } from "../lib/events/events.utils";
  import { BoxArtSize } from "../lib/format.constants";
  import { boxArt, number } from "../lib/format.utils";
  import { t } from "../lib/i18n/i18n.svelte";
  import { strong } from "../lib/text.utils";
  import { Page } from "../lib/state/state.constants";
  import { navigate } from "../lib/state/state.svelte";
  import { ticker } from "../lib/state/ticker.svelte";
  import { formatClock, relativeTime } from "../lib/time/time.utils";
  import TextButton from "./ui/TextButton.svelte";
  import CampaignDialog from "./catalogue/CampaignDialog.svelte";
  import RichText from "./ui/RichText.svelte";
  import ExternalLink from "./ui/ExternalLink.svelte";
  import LinkTwin from "./ui/LinkTwin.svelte";
  import Icon from "./ui/Icon.svelte";
  import PointsBalance from "./ui/PointsBalance.svelte";
  import MiningBadge from "./ui/MiningBadge.svelte";
  import ProgressLine from "./ui/ProgressLine.svelte";
  import Thumb from "./ui/Thumb.svelte";
  import ViewerCount from "./ui/ViewerCount.svelte";
  import {
    AvatarSize,
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
  } from "./ui/ui.constants";
  import type { SlotViewType } from "../lib/bindings/SlotViewType";

  type AccountActivityPropsType = { account: AccountSnapshotType };

  let { account }: AccountActivityPropsType = $props();

  let open = $state<CampaignType | null>(null);

  const campaignOf = (id: string): CampaignType | undefined =>
    account.campaigns.find((c) => c.id === id);

  const capacity = $derived(Math.max(1, Math.min(SLOTS_MAX, account.settings.slots)));

  const free = $derived(
    account.settings.enabled ? Math.max(0, capacity - account.slots.length) : 0,
  );
</script>

{#snippet watched(slot: SlotViewType)}
  <span class="text-xs text-muted-foreground">
    <span class="inline-block"
      ><RichText
        text={t.dashboard_watching_since({ time: strong(formatClock(slot.since)) })}
      /></span
    >{#if slot.last_beacon_at}{" · "}<span class="inline-block"
        ><RichText
          text={t.slot_last_minute({
            time: strong(relativeTime(slot.last_beacon_at, ticker.now)),
          })}
        /></span
      >{/if}{" · "}<span class="inline-block"
      ><RichText text={t.slot_segments({ count: strong(number(slot.segments_requested)) })} /></span
    >
  </span>
{/snippet}

{#snippet points(reason: PointsReason | null)}
  <span class="flex flex-wrap gap-1.5"
    ><MiningBadge kind={SlotPurposeKind.POINTS} />{#if reason}<Badge
        text={POINTS_REASON_LABELS[reason]()}
        tone={BadgeTone.NEUTRAL}
        dot={false}
        icon={POINTS_REASON_ICONS[reason]}
        hint={POINTS_REASON_HINTS[reason]()}
      />{/if}</span
  >
{/snippet}

{#snippet streamer(slot: SlotViewType)}
  <span class="group/twin flex max-w-36 shrink-0 flex-col items-end gap-1.5 text-right">
    <LinkTwin url={channelUrl(slot.channel.login)}>
      <ChannelAvatar
        url={slot.channel.avatar_url}
        name={slot.channel.display_name}
        size={AvatarSize.LG}
        live={slot.channel.live}
      />
    </LinkTwin>
    <span class="flex max-w-full flex-col items-end gap-0.5">
      <ExternalLink
        url={channelUrl(slot.channel.login)}
        text={slot.channel.display_name}
        mark={LinkMark.START}
        look={LinkLook.HOVER}
        size={LinkSize.MD}
        twin={true}
      />
      {#if slot.channel.viewers !== undefined}<ViewerCount count={slot.channel.viewers} />{/if}
    </span>
  </span>
{/snippet}

{#snippet idle(icon: IconName, text: string)}
  <span class="shrink-0"><Icon name={icon} size={16} /></span>
  <span class="flex-1">{text}</span>
{/snippet}

{#each account.slots as slot (slot.channel.id + slot.purpose.kind)}
  {#if slot.drop}
    {@const progress = slot.drop}
    {@const c = campaignOf(progress.campaign_id)}
    {@const d = c?.drops.find((x) => x.id === progress.drop_id)}
    <div
      class="group/game grid grid-cols-[auto_minmax(0,1fr)_auto] items-start gap-x-3.5 gap-y-1.5 border-b px-5 py-4 last:border-b-0"
    >
      <button
        class="shrink-0 rounded-md"
        type="button"
        aria-label={progress.campaign_name}
        data-game-twin
        onclick={() => (open = c ?? null)}
      >
        <Thumb
          src={boxArt(c?.game.box_art_url, BoxArtSize.ACTIVITY)}
          round={false}
          contain={false}
          size={ThumbSize.XL}
          shape={ThumbShape.BOX_ART}
          fallback={IconName.GIFT}
        />
      </button>
      <div class="flex min-w-0 flex-col gap-1.5">
        <span class="flex"><MiningBadge kind={SlotPurposeKind.DROP} /></span>
        <!-- ! Hovering the box art beside lights this up too: both open the campaign. -->
        <button
          class="group flex min-w-0 flex-col text-left transition-colors group-has-[[data-game-twin]:hover]/game:text-link hover:text-link"
          type="button"
          onclick={() => (open = c ?? null)}
        >
          <span class="truncate font-semibold">{progress.game}</span>
          <span
            class="truncate text-hint text-muted-foreground group-hover:text-link group-has-[[data-game-twin]:hover]/game:text-link"
            >{progress.campaign_name}</span
          >
        </button>
      </div>
      {@render streamer(slot)}
      <div class="col-span-3 flex min-w-0 flex-col gap-1.5">
        {@render watched(slot)}
        <button
          class="flex items-center gap-2.5 rounded-lg border bg-muted/50 p-2 text-left transition-colors hover:bg-accent"
          type="button"
          onclick={() => (open = c ?? null)}
        >
          <Thumb
            src={d?.benefits.at(0)?.image_url ?? progress.image_url}
            round={false}
            contain={true}
            size={ThumbSize.MD}
            shape={ThumbShape.SQUARE}
            fallback={IconName.GIFT}
          />
          <div class="flex min-w-0 flex-1 flex-col gap-1">
            <span class="truncate text-xs font-semibold">{progress.drop_name}</span>
            <ProgressLine
              current={progress.current_minutes}
              required={progress.required_minutes}
              stacked={false}
            />
          </div>
        </button>
      </div>
    </div>
  {:else}
    <div
      class="group/twin grid grid-cols-[auto_minmax(0,1fr)_auto] items-start gap-x-3.5 gap-y-1.5 border-b px-5 py-4 last:border-b-0"
    >
      <LinkTwin url={channelUrl(slot.channel.login)}>
        <ChannelAvatar
          url={slot.channel.avatar_url}
          name={slot.channel.display_name}
          size={AvatarSize.LG}
          live={slot.channel.live}
        />
      </LinkTwin>
      <div class="flex min-w-0 flex-col gap-1">
        {@render points(slot.purpose.kind === SlotPurposeKind.POINTS ? slot.purpose.reason : null)}
        <ExternalLink
          url={channelUrl(slot.channel.login)}
          text={slot.channel.display_name}
          mark={LinkMark.END}
          look={LinkLook.HOVER}
          size={LinkSize.MD}
          twin={true}
        />
        <span class="flex min-w-0 items-center gap-2 text-hint text-muted-foreground">
          {#if slot.channel.game}<span class="truncate">{slot.channel.game.name}</span>{/if}
          {#if slot.channel.viewers !== undefined}<ViewerCount count={slot.channel.viewers} />{/if}
        </span>
      </div>
      {#if slot.points}
        <PointsBalance balance={slot.points.balance} earned={slot.points.earned_session} />
      {:else}<span></span>{/if}
      <div class="col-span-3 min-w-0">{@render watched(slot)}</div>
    </div>
  {/if}
{/each}

<div
  class="flex flex-wrap items-center gap-2.5 px-5 py-4 text-hint text-muted-foreground empty:hidden"
>
  {#if !account.settings.enabled}
    {@render idle(IconName.PAUSE, t.activity_paused())}
  {:else if account.status === AccountStatus.LOGIN_REQUIRED}
    {@render idle(IconName.ALERT, t.status_login_required())}
  {:else if account.slots.length === 0}
    {@render idle(IconName.CLOCK, t.activity_nothing())}
    <TextButton
      label={t.nav_account_drops()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => navigate(Page.ACCOUNT_DROPS, account.id)}
    />
    <TextButton
      label={t.nav_account_points()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => navigate(Page.ACCOUNT_POINTS, account.id)}
    />
  {:else if free > 0 && account.settings.mode !== Mode.DROPS}
    {@render idle(
      IconName.PLUS,
      account.settings.points.streamers.length
        ? t.activity_free_slot_offline()
        : t.activity_free_slot(),
    )}
    <TextButton
      label={t.nav_account_points()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => navigate(Page.ACCOUNT_POINTS, account.id)}
    />
  {/if}
</div>

<CampaignDialog campaign={open} contextAccountId={account.id} onclose={() => (open = null)} />
