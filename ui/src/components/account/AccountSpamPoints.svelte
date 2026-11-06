<script lang="ts">
  import { onDestroy, tick } from "svelte";

  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { CommandName } from "../../lib/bindings/CommandName";
  import type { SearchChannelType } from "../../lib/bindings/SearchChannelType";
  import {
    accountChannel,
    ensureChannels,
    rememberChannels,
    searchedChannel,
  } from "../../lib/channels/channels.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { cancelBursts } from "../../lib/spam/spam.svelte";
  import type { SpamChannelType, SpamHistoryRowType } from "../../lib/spam/spam.types";
  import { ToastKind } from "../../lib/state/state.constants";
  import { run, toast } from "../../lib/state/state.svelte";
  import { strong } from "../../lib/text.utils";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Panel from "../ui/Panel.svelte";
  import { IconName } from "../ui/ui.constants";
  import SpamChannelList from "./spam/SpamChannelList.svelte";
  import SpamChannelPanel from "./spam/SpamChannelPanel.svelte";

  type AccountSpamPointsPropsType = { account: AccountSnapshotType; snapshot: SnapshotType };

  let { account, snapshot }: AccountSpamPointsPropsType = $props();

  let selected = $state<string | null>(null);
  let list = $state<HTMLElement | null>(null);
  let panel = $state<HTMLElement | null>(null);

  // ! In one column (a phone, a narrow window) the rewards open under the list, out of sight.
  const show = (login: string): void => {
    selected = login;
    void tick().then(() => {
      if (
        list &&
        panel &&
        panel.getBoundingClientRect().top >= list.getBoundingClientRect().bottom
      ) {
        panel.scrollIntoView({ block: "start", behavior: "smooth" });
      }
    });
  };

  const channelOf = (login: string): ChannelType | undefined => accountChannel(account, login);

  $effect(() => {
    ensureChannels(
      account.spam_channels.map((entry) => entry.login).filter((login) => !channelOf(login)?.id),
    );
  });

  const entries = $derived(
    account.spam_channels.map((entry): SpamHistoryRowType => {
      const channel = channelOf(entry.login);

      return {
        login: entry.login,
        name: channel?.display_name ?? entry.login,
        avatar: channel?.avatar_url,
        live: channel?.live === true,
        balance: entry.balance,
        in_points: entry.in_points,
        own: entry.login === account.login,
      };
    }),
  );

  const current = $derived.by((): SpamChannelType | null => {
    if (!selected) {
      return null;
    }

    const channel = channelOf(selected);

    return {
      login: selected,
      name: channel?.display_name ?? selected,
      avatar: channel?.avatar_url,
      live: channel?.live === true,
      game: channel?.game?.name,
      viewers: channel?.viewers,
    };
  });

  const pick = async (found: SearchChannelType): Promise<void> => {
    if (!channelOf(found.login)) {
      rememberChannels([searchedChannel(found)]);
    }

    show(found.login);

    if (!entries.some((entry) => entry.login === found.login && entry.in_points)) {
      await run({
        command: CommandName.ADD_SPAM_CHANNEL,
        account_id: account.id,
        login: found.login,
      });
    }
  };

  const remove = async (login: string): Promise<boolean> => {
    const name = channelOf(login)?.display_name ?? login;
    const removed = await run({
      command: CommandName.FORGET_SPAM_CHANNEL,
      account_id: account.id,
      login,
    });

    if (!removed) {
      return false;
    }

    if (selected === login) {
      selected = null;
    }

    toast(ToastKind.SUCCESS, t.spam_removed({ channel: strong(name) }));

    return true;
  };

  onDestroy(() => cancelBursts(account.id));
</script>

<PageBody>
  <div class="@container">
    <div class="grid items-start gap-5 @4xl:grid-cols-[minmax(0,21rem)_minmax(0,1fr)]">
      <div bind:this={list} class="@4xl:sticky @4xl:top-0">
        <SpamChannelList {entries} {selected} onselect={show} onpick={pick} onremove={remove} />
      </div>
      <div bind:this={panel} class="min-w-0 scroll-mt-3">
        {#if current}
          {#key current.login}
            <SpamChannelPanel
              {account}
              channel={current}
              settings={snapshot.settings.spam_points}
            />
          {/key}
        {:else}
          <Panel title={t.spam_rewards()}>
            <EmptyState title={t.spam_pick_title()} text={t.spam_pick_text()} icon={IconName.ZAP} />
          </Panel>
        {/if}
      </div>
    </div>
  </div>
</PageBody>
