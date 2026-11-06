<script lang="ts">
  import { TableCell, TableRow } from "$lib/components/ui/table";
  import type { MinerEventType } from "../lib/bindings/MinerEventType";
  import { historyView } from "../lib/events/detail.utils";
  import {
    eventIcon,
    channelUrl,
    EVENT_ICONS,
    EVENT_KIND_LABELS,
    EVENT_TONES,
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

  type HistoryRowPropsType = {
    event: MinerEventType;
    showAccount: boolean;
  };

  let { event, showAccount }: HistoryRowPropsType = $props();

  const { owner, login, accountName, miner, sentence, reward, detail, picture } = $derived(
    historyView(event, app.snapshot?.accounts ?? []),
  );
</script>

<TableRow>
  <TableCell class="pl-0 text-xs text-muted-foreground tabular-nums"
    >{formatShortDateTime(event.at)}</TableCell
  >
  {#if showAccount}
    <TableCell>
      {#if event.account_id}
        <span
          class="inline-flex max-w-40 min-w-32 items-center gap-1.5 whitespace-nowrap"
          title={accountName}
        >
          <Avatar url={owner?.avatar_url} name={accountName} size={AvatarSize.XS} />
          <span class="truncate font-semibold">{accountName}</span>
        </span>
      {:else}
        <span class="text-muted-foreground">—</span>
      {/if}
    </TableCell>
  {/if}
  <TableCell class="max-w-0 min-w-72 whitespace-normal">
    <span class="flex items-center gap-2.5">
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
      <span class="flex min-w-0 flex-col">
        <span class="leading-snug break-words"
          ><RichText text={sentence.text} links={sentence.links} /></span
        >
        {#if detail}<span class="truncate text-xs text-muted-foreground">{detail}</span>{/if}
      </span>
    </span>
  </TableCell>
  <TableCell>
    {#if login}
      {@const channelName = event.context.channel_name ?? login}
      <span class="inline-flex max-w-48 min-w-40 whitespace-nowrap" title={channelName}>
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
              size={AvatarSize.MD}
              live={false}
            />
          {/snippet}
        </ExternalLink>
      </span>
    {:else}
      <span class="text-muted-foreground">—</span>
    {/if}
  </TableCell>
  <TableCell>
    <Badge
      text={EVENT_KIND_LABELS[event.kind]()}
      tone={EVENT_TONES[event.kind]}
      dot={false}
      icon={EVENT_ICONS[event.kind]}
    />
  </TableCell>
  <TableCell>
    {#if miner}
      <Badge
        text={MINER_LABELS[miner]()}
        tone={MINER_TONES[miner]}
        dot={false}
        icon={MINER_ICONS[miner]}
      />
    {:else}
      <span class="text-muted-foreground">—</span>
    {/if}
  </TableCell>
</TableRow>
