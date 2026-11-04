<script lang="ts">
  import EmptyState from "./ui/EmptyState.svelte";
  import { EventKind } from "../lib/bindings/EventKind";
  import type { MinerEventType } from "../lib/bindings/MinerEventType";
  import {
    EVENT_ICONS,
    EVENT_KIND_LABELS,
    EVENT_TONES,
    eventText,
  } from "../lib/events/events.utils";
  import { ensureChannels } from "../lib/channels/channels.svelte";
  import { shownEvent, unnamedChannels } from "../lib/events/shown.svelte";
  import { BoxArtSize } from "../lib/format.constants";
  import { boxArt } from "../lib/format.utils";
  import { app } from "../lib/state/state.svelte";
  import { formatClock, formatShortDateTime } from "../lib/time/time.utils";
  import { cn } from "$lib/utils";
  import Avatar from "./ui/Avatar.svelte";
  import Badge from "./ui/Badge.svelte";
  import ChannelAvatar from "./ui/ChannelAvatar.svelte";
  import RichText from "./ui/RichText.svelte";
  import Thumb from "./ui/Thumb.svelte";
  import { AvatarSize, IconName, ThumbShape, ThumbSize } from "./ui/ui.constants";

  type ActivityFeedPropsType = {
    events: MinerEventType[];
    showAccount: boolean;
    wide: boolean;
    empty: string;
  };

  type PictureType = {
    src: string | null | undefined;
    round: boolean;
    contain: boolean;
    channel?: string;
  };

  let { events, showAccount, wide, empty }: ActivityFeedPropsType = $props();

  const shown = $derived(events.map(shownEvent));

  $effect(() => {
    ensureChannels(unnamedChannels(events));
  });

  const accountOf = (id: string) => app.snapshot?.accounts.find((a) => a.id === id);

  const picture = (event: MinerEventType): PictureType => {
    const { reward, art, avatar } = event.context;

    if (reward) {
      return { src: reward, round: false, contain: true };
    }

    const about =
      event.kind === EventKind.DROP_PROGRESS || event.kind === EventKind.CAMPAIGN_FINISHED;

    if (art && about) {
      return { src: boxArt(art, BoxArtSize.ACTIVITY), round: false, contain: false };
    }

    if (avatar) {
      return { src: avatar, round: false, contain: false, channel: event.context.channel };
    }

    if (art) {
      return { src: boxArt(art, BoxArtSize.ACTIVITY), round: false, contain: false };
    }

    if (!event.account_id) {
      return { src: null, round: false, contain: false };
    }

    return { src: accountOf(event.account_id)?.avatar_url, round: true, contain: false };
  };
</script>

{#if events.length === 0}
  <EmptyState text={empty} />
{:else}
  <ul class="m-0 flex list-none flex-col p-0">
    {#each shown as e (e.id + e.at + e.title)}
      {@const p = picture(e)}
      {@const acc = accountOf(e.account_id)}
      {@const sentence = eventText(e, true)}
      <li
        class={cn(
          "grid items-center gap-3 border-b py-2.5 last:border-b-0",
          wide
            ? "grid-cols-[7.5rem_2.25rem_minmax(0,1fr)_auto]"
            : "grid-cols-[3rem_2.25rem_minmax(0,1fr)_auto]",
        )}
      >
        <span class="text-xs text-muted-foreground tabular-nums">
          {wide ? formatShortDateTime(e.at) : formatClock(e.at)}
        </span>
        {#if p.channel}
          <span class="flex justify-center">
            <ChannelAvatar
              url={p.src}
              name={e.context.channel_name ?? p.channel}
              size={AvatarSize.MD}
              live={false}
            />
          </span>
        {:else}
          <Thumb
            src={p.src}
            round={p.round}
            contain={p.contain}
            size={ThumbSize.MD}
            shape={ThumbShape.SQUARE}
            fallback={e.account_id ? IconName.CLOCK : EVENT_ICONS[e.kind]}
          />
        {/if}
        <span class="flex min-w-0 flex-col gap-0.5">
          <span class="text-sm leading-snug break-words"
            ><RichText text={sentence.text} links={sentence.links} /></span
          >
          {#if (showAccount && e.account_id) || e.detail}
            <span class="flex min-w-0 items-center gap-1.5 text-xs text-muted-foreground">
              {#if showAccount && e.account_id}
                <Avatar
                  url={acc?.avatar_url}
                  name={acc?.display_name ?? e.account_login}
                  size={AvatarSize.XS}
                />
                <span class="font-semibold text-foreground/80"
                  >{acc?.display_name ?? e.account_login}</span
                >
              {/if}
              {#if e.detail}<span class="truncate"
                  >{showAccount && e.account_id ? "· " : ""}{e.detail}</span
                >{/if}
            </span>
          {/if}
        </span>
        <Badge
          text={EVENT_KIND_LABELS[e.kind]()}
          tone={EVENT_TONES[e.kind]}
          dot={false}
          icon={EVENT_ICONS[e.kind]}
        />
      </li>
    {/each}
  </ul>
{/if}
