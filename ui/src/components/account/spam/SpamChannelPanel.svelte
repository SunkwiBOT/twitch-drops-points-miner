<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  import { Card } from "$lib/components/ui/card";
  import { fetchSpamBalance, fetchSpamChannel } from "../../../lib/api/client";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { SpamPointsSettingsType } from "../../../lib/bindings/SpamPointsSettingsType";
  import type { SpamRewardType } from "../../../lib/bindings/SpamRewardType";
  import { ensureBanners, knownBanners } from "../../../lib/channels/channels.svelte";
  import { describeError } from "../../../lib/errors/errors.utils";
  import { channelUrl } from "../../../lib/events/events.utils";
  import { number } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { BalanceTracker } from "../../../lib/spam/balance.svelte";
  import { SPAM_CLOCK_MS, UNAVAILABLE_STATUSES } from "../../../lib/spam/spam.constants";
  import { redeemBurst } from "../../../lib/spam/spam.svelte";
  import type { SpamChannelType, SpamRewardItemType } from "../../../lib/spam/spam.types";
  import { burstOutcome, rewardStatus, shownUnavailable } from "../../../lib/spam/spam.utils";
  import { ToastKind } from "../../../lib/state/state.constants";
  import {
    accountImage,
    isLocked,
    reportFailure,
    saveSpamSettings,
    sessionEndedBy,
    toast,
  } from "../../../lib/state/state.svelte";
  import { strong } from "../../../lib/text.utils";
  import { SECOND_MS } from "../../../lib/time/time.constants";
  import Badge from "../../ui/Badge.svelte";
  import ChannelAvatar from "../../ui/ChannelAvatar.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import ExternalLink from "../../ui/ExternalLink.svelte";
  import LinkTwin from "../../ui/LinkTwin.svelte";
  import IconButton from "../../ui/IconButton.svelte";
  import ViewerCount from "../../ui/ViewerCount.svelte";
  import {
    AvatarSize,
    BadgeTone,
    ButtonSize,
    ButtonVariant,
    IconName,
    LinkLook,
    LinkMark,
    LinkSize,
  } from "../../ui/ui.constants";
  import SpamBalance from "./SpamBalance.svelte";
  import SpamRewardCard from "./SpamRewardCard.svelte";
  import SpamRewardSkeleton from "./SpamRewardSkeleton.svelte";
  import SpamSettingsPopover from "./SpamSettingsPopover.svelte";

  type SpamChannelPanelPropsType = {
    account: AccountSnapshotType;
    channel: SpamChannelType;
    settings: SpamPointsSettingsType;
  };

  let { account, channel, settings }: SpamChannelPanelPropsType = $props();

  let rewards = $state<SpamRewardType[] | null>(null);
  let problem = $state<string | null>(null);
  let loading = $state(false);
  let gone = false;
  let now = $state(Date.now());

  const balance = new BalanceTracker(
    () => fetchSpamBalance(account.id, channel.login),
    () => rewards !== null,
  );

  const load = async (): Promise<void> => {
    loading = true;

    try {
      const read = await fetchSpamChannel(account.id, channel.login);

      if (!gone) {
        rewards = read.rewards;
        balance.loaded(read.balance);
        problem = null;
      }
    } catch (e: any) {
      if (gone) {
        sessionEndedBy(e);
      } else if (rewards) {
        reportFailure(e);
      } else if (!sessionEndedBy(e)) {
        problem = describeError(e);
      }
    } finally {
      loading = false;
    }
  };

  onMount(() => {
    void load();

    const clock = setInterval(() => (now = Date.now()), SPAM_CLOCK_MS);

    return () => clearInterval(clock);
  });

  onDestroy(() => (gone = true));

  $effect(() => {
    ensureBanners([channel.login]);
  });

  const cards = $derived(
    (rewards ?? []).map((reward): SpamRewardItemType => ({
      reward,
      status: rewardStatus(reward, now),
    })),
  );
  const ready = $derived(cards.filter((item) => !UNAVAILABLE_STATUSES.has(item.status)));
  const unavailable = $derived(
    cards.filter(
      (item) => UNAVAILABLE_STATUSES.has(item.status) && shownUnavailable(item.reward, settings),
    ),
  );

  // ! The toast comes wherever the user went, but never over a lock screen: it
  // names the account.
  const report = (kind: ToastKind, text: string): void => {
    if (!isLocked()) {
      toast(kind, text, {
        image: accountImage(account.id, account.login),
        title: account.display_name,
      });
    }
  };

  const redeem = async (reward: SpamRewardType, count: number, text: string): Promise<number> => {
    let sent = 0;

    try {
      const burst = await redeemBurst(
        {
          account_id: account.id,
          login: channel.login,
          reward_id: reward.id,
          count,
          text: text || undefined,
        },
        settings.delay_ms,
      );

      if (burst) {
        sent = burst.sent;
        report(
          burst.sent === count ? ToastKind.SUCCESS : ToastKind.ERROR,
          t.spam_toast({
            count: number(count),
            reward: strong(reward.title),
            channel: strong(channel.name),
            result: burstOutcome(count, burst),
          }),
        );
      } else {
        report(ToastKind.INFO, t.spam_result_lost({ reward: strong(reward.title) }));
      }
    } catch (e: any) {
      if (!sessionEndedBy(e)) {
        report(ToastKind.ERROR, describeError(e));
      }
    }

    if (gone) {
      return sent;
    }

    if (sent > 0 && reward.cooldown_seconds && rewards) {
      const until = new Date(Date.now() + reward.cooldown_seconds * SECOND_MS).toISOString();

      rewards = rewards.map((item) =>
        item.id === reward.id ? { ...item, cooldown_expires_at: until } : item,
      );
    }

    await balance.read();

    return sent;
  };
</script>

{#snippet rewardSection(id: string, title: string, items: SpamRewardItemType[])}
  <section class="flex flex-col gap-3" aria-labelledby={id}>
    <h3 {id} class="flex items-center gap-2 text-sm font-semibold">
      {title}<Badge text={number(items.length)} tone={BadgeTone.NEUTRAL} dot={false} />
    </h3>
    {#if items.length > 0}
      <div class="grid grid-cols-[repeat(auto-fill,minmax(11rem,1fr))] gap-2.5">
        {#each items as item (item.reward.id)}
          <SpamRewardCard
            accountId={account.id}
            login={channel.login}
            reward={item.reward}
            status={item.status}
            balance={balance.current?.balance ?? 0}
            free={channel.login === account.login}
            {now}
            onredeem={(count, text) => redeem(item.reward, count, text)}
          />
        {/each}
      </div>
    {:else}
      <EmptyState text={t.spam_no_rewards()} />
    {/if}
  </section>
{/snippet}

<Card class="gap-0 overflow-visible py-0">
  <!-- ! -top-6 undoes PageBody's p-6: stuck, the header sits on the page's very top edge,
       with no reward scrolling above it. -->
  <header
    class="group/twin sticky -top-6 z-10 isolate flex flex-wrap items-center gap-x-4 gap-y-3 overflow-hidden rounded-t-xl border-b bg-card px-5 py-4"
  >
    {#if knownBanners[channel.login]}
      <span
        class="absolute inset-0 -z-10 bg-cover bg-center opacity-20 [mask-image:linear-gradient(to_right,black,transparent_75%)] dark:opacity-30"
        style:background-image="url({knownBanners[channel.login]})"
        aria-hidden="true"
      ></span>
    {/if}
    <LinkTwin url={channelUrl(channel.login)} live={channel.live}>
      <ChannelAvatar
        url={channel.avatar}
        name={channel.name}
        size={AvatarSize.LG}
        live={channel.live}
      />
    </LinkTwin>
    <div class="flex min-w-0 flex-[1_1_12rem] flex-col gap-1">
      <h2 class="flex min-w-0 text-lg font-semibold">
        <ExternalLink
          url={channelUrl(channel.login)}
          text={channel.name}
          mark={LinkMark.END}
          look={LinkLook.HOVER}
          size={LinkSize.LG}
          twin={true}
        />
      </h2>
      <span class="flex min-w-0 flex-wrap items-center gap-x-2 text-xs text-muted-foreground">
        {#if channel.live}
          {#if channel.game}<span class="truncate">{channel.game}</span>{/if}
          {#if channel.viewers !== undefined}<ViewerCount count={channel.viewers} />{/if}
        {:else}
          <span>{t.points_offline_hint()}</span>
        {/if}
      </span>
    </div>
    <div class="flex shrink-0 items-center gap-4">
      {#if balance.current}
        <SpamBalance
          balance={balance.current.balance}
          changedAt={balance.current.changed_at}
          tracking={balance.tracking}
          free={channel.login === account.login}
          {now}
        />
      {/if}
      <div class="flex items-center gap-2">
        <IconButton
          icon={IconName.REFRESH}
          label={t.spam_refresh()}
          busy={loading}
          disabled={false}
          variant={ButtonVariant.DEFAULT}
          size={ButtonSize.MD}
          onclick={load}
        />
        <SpamSettingsPopover {settings} onchange={(next) => void saveSpamSettings(next)} />
      </div>
    </div>
  </header>

  <div class="flex flex-col gap-5 p-5">
    {#if problem && !rewards}
      <EmptyState title={t.spam_load_failed()} text={problem} icon={IconName.ALERT} />
    {:else if !rewards}
      <SpamRewardSkeleton />
    {:else}
      {@render rewardSection("spam-ready", t.spam_rewards(), ready)}
      {#if unavailable.length > 0}
        {@render rewardSection("spam-unavailable", t.spam_unavailable(), unavailable)}
      {/if}
    {/if}
  </div>
</Card>
