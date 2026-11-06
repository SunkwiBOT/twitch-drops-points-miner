<script lang="ts">
  import type { MinerEventType } from "../lib/bindings/MinerEventType";
  import { historyView } from "../lib/events/detail.utils";
  import {
    channelUrl,
    EVENT_ICONS,
    EVENT_KIND_LABELS,
    EVENT_TONES,
    eventIcon,
    MINER_ICONS,
    MINER_LABELS,
    MINER_TONES,
  } from "../lib/events/events.utils";
  import { app } from "../lib/state/state.svelte";
  import { formatShortDateTime } from "../lib/time/time.utils";
  import Avatar from "./ui/Avatar.svelte";
  import Badge from "./ui/Badge.svelte";
  import ChannelAvatar from "./ui/ChannelAvatar.svelte";
  import ExternalLink from "./ui/ExternalLink.svelte";
  import RewardImage from "./ui/RewardImage.svelte";
  import RichText from "./ui/RichText.svelte";
  import Thumb from "./ui/Thumb.svelte";
  import {
    AvatarSize,
    LinkLook,
    LinkMark,
    LinkSize,
    THUMB_PIXELS,
    ThumbShape,
    ThumbSize,
  } from "./ui/ui.constants";

  type HistoryCardPropsType = {
    event: MinerEventType;
    showAccount: boolean;
  };

  let { event, showAccount }: HistoryCardPropsType = $props();

  const { owner, login, accountName, miner, sentence, reward, detail, picture } = $derived(
    historyView(event, app.snapshot?.accounts ?? []),
  );
</script>

<li class="flex gap-2.5 py-3">
  {#if reward}
    <RewardImage url={reward.url} color={reward.color} size={THUMB_PIXELS[ThumbSize.SM]} />
  {:else}<Thumb
      src={picture}
      round={false}
      contain={true}
      size={ThumbSize.SM}
      shape={ThumbShape.SQUARE}
      fallback={eventIcon(event)}
    />{/if}
  <div class="flex min-w-0 flex-1 flex-col gap-1">
    <span class="leading-snug break-words"
      ><RichText text={sentence.text} links={sentence.links} /></span
    >
    {#if detail}<span class="text-xs break-words text-muted-foreground">{detail}</span>{/if}
    <span class="flex flex-wrap items-center gap-x-2.5 gap-y-1 text-xs text-muted-foreground">
      <span class="tabular-nums">{formatShortDateTime(event.at)}</span>
      {#if showAccount && event.account_id}
        <span class="inline-flex min-w-0 items-center gap-1.5">
          <Avatar url={owner?.avatar_url} name={accountName} size={AvatarSize.XS} />
          <span class="font-semibold text-foreground">{accountName}</span>
        </span>
      {/if}
      {#if login}
        {@const channelName = event.context.channel_name ?? login}
        <span class="inline-flex min-w-0">
          <ExternalLink
            url={channelUrl(login)}
            text={channelName}
            mark={LinkMark.END}
            look={LinkLook.HOVER}
            size={LinkSize.MD}
          >
            {#snippet lead()}
              <ChannelAvatar
                url={event.context.avatar}
                name={channelName}
                size={AvatarSize.XS}
                live={false}
              />
            {/snippet}
          </ExternalLink>
        </span>
      {/if}
    </span>
    <span class="flex flex-wrap items-center gap-1.5">
      <Badge
        text={EVENT_KIND_LABELS[event.kind]()}
        tone={EVENT_TONES[event.kind]}
        dot={false}
        icon={EVENT_ICONS[event.kind]}
      />
      {#if miner}<Badge
          text={MINER_LABELS[miner]()}
          tone={MINER_TONES[miner]}
          dot={false}
          icon={MINER_ICONS[miner]}
        />{/if}
    </span>
  </div>
</li>
