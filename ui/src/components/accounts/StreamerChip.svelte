<script lang="ts">
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import { isLive } from "../../lib/channels/channels.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { Badge as BadgeRoot } from "$lib/components/ui/badge";
  import { cn } from "$lib/utils";
  import ChannelAvatar from "../ui/ChannelAvatar.svelte";
  import ChipRemoveButton from "../ui/ChipRemoveButton.svelte";
  import { AvatarSize, BADGE_VARIANTS, BadgeVariant } from "../ui/ui.constants";

  type StreamerChipPropsType = {
    login: string;
    channel: ChannelType | undefined;
    onremove?: () => void;
  };

  let { login, channel, onremove }: StreamerChipPropsType = $props();

  const name = $derived(channel?.display_name ?? login);
  const live = $derived(isLive(login));
</script>

<BadgeRoot
  variant={BADGE_VARIANTS[BadgeVariant.CHIP]}
  class="h-9 gap-1.5 overflow-visible pr-1.5 pl-1.5 text-hint font-semibold"
>
  <span class={cn("inline-flex", live && "-translate-y-0.5")}
    ><ChannelAvatar url={channel?.avatar_url} {name} size={AvatarSize.SM} {live} /></span
  >
  <span class="max-w-40 truncate">{name}</span>
  {#if onremove}
    <ChipRemoveButton label={t.common_remove()} {name} iconSize={11} onclick={onremove} />
  {/if}
</BadgeRoot>
