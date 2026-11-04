<script lang="ts">
  import type { ChannelType } from "../../../lib/bindings/ChannelType";
  import type { StreamerSettingsType } from "../../../lib/bindings/StreamerSettingsType";
  import type { StreamerViewType } from "../../../lib/bindings/StreamerViewType";
  import { isLive } from "../../../lib/channels/channels.svelte";
  import { channelUrl } from "../../../lib/events/events.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { ticker } from "../../../lib/state/ticker.svelte";
  import { strong } from "../../../lib/text.utils";
  import { relativeTime } from "../../../lib/time/time.utils";
  import Badge from "../../ui/Badge.svelte";
  import ChannelAvatar from "../../ui/ChannelAvatar.svelte";
  import ExternalLink from "../../ui/ExternalLink.svelte";
  import PointsBalance from "../../ui/PointsBalance.svelte";
  import RichText from "../../ui/RichText.svelte";
  import ViewerCount from "../../ui/ViewerCount.svelte";
  import {
    AvatarSize,
    BadgeTone,
    IconName,
    LinkLook,
    LinkMark,
    LinkSize,
  } from "../../ui/ui.constants";

  type StreamerRowPropsType = {
    streamer: StreamerSettingsType;
    channel: ChannelType | undefined;
    view: StreamerViewType | undefined;
  };

  let { streamer, channel, view }: StreamerRowPropsType = $props();
</script>

<div
  class={["flex min-w-0 flex-1 items-center gap-3 transition", !streamer.enabled && "grayscale"]}
>
  <span class={["inline-flex transition", !streamer.enabled && "opacity-50"]}>
    <ChannelAvatar
      url={channel?.avatar_url}
      name={channel?.display_name ?? streamer.login}
      size={AvatarSize.MD}
      live={isLive(streamer.login)}
    />
  </span>
  <div class="flex min-w-0 flex-1 flex-col gap-0.5">
    <span class="flex min-w-0 flex-wrap items-center gap-1.5">
      <ExternalLink
        url={channelUrl(streamer.login)}
        text={channel?.display_name ?? streamer.login}
        mark={LinkMark.END}
        look={LinkLook.HOVER}
        size={LinkSize.MD}
      />
      {#if view?.watching}<Badge
          text={t.streamer_watching()}
          tone={BadgeTone.SUCCESS}
          dot={true}
          hint={t.badge_hint_watching()}
        />{/if}
      {#if view?.streak_at_risk}<Badge
          text={t.streamer_streak_risk()}
          tone={BadgeTone.WARNING}
          dot={false}
          hint={t.badge_hint_streak_risk()}
          icon={IconName.WATCH_STREAK}
        />{/if}
    </span>
    <span
      class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-0.5 text-xs text-muted-foreground"
    >
      {#if channel?.live}
        {#if channel.game}<span class="min-w-0 [overflow-wrap:anywhere]">{channel.game.name}</span
          >{/if}
        {#if channel.viewers !== undefined}<ViewerCount count={channel.viewers} />{/if}
      {:else if !channel}
        <span class="truncate">{t.points_offline_hint()}</span>
      {/if}
    </span>
  </div>
  {#if view && view.points.balance > 0}
    <PointsBalance
      balance={view.points.balance}
      earned={view.points.earned_session}
      streak={view.points.streak?.count}
    >
      {#if view.points.last_bonus_at}<span
          ><RichText
            text={t.streamer_last_bonus({
              time: strong(relativeTime(view.points.last_bonus_at, ticker.now)),
            })}
          /></span
        >{/if}
    </PointsBalance>
  {/if}
</div>
