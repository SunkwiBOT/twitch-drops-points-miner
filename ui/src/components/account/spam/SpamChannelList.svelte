<script lang="ts">
  import { cn } from "$lib/utils";
  import type { SearchChannelType } from "../../../lib/bindings/SearchChannelType";
  import { number } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { SEARCH_SHOWN_MAX } from "../../../lib/search/search.constants";
  import { TwitchSearch } from "../../../lib/search/search.svelte";
  import type { SpamHistoryRowType } from "../../../lib/spam/spam.types";
  import ChannelAvatar from "../../ui/ChannelAvatar.svelte";
  import ChipRemoveButton from "../../ui/ChipRemoveButton.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import Icon from "../../ui/Icon.svelte";
  import Panel from "../../ui/Panel.svelte";
  import SearchField from "../../ui/SearchField.svelte";
  import SearchNoResult from "../../ui/SearchNoResult.svelte";
  import TextButton from "../../ui/TextButton.svelte";
  import {
    AvatarSize,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
  } from "../../ui/ui.constants";

  type SpamChannelListPropsType = {
    entries: SpamHistoryRowType[];
    selected: string | null;
    onselect: (login: string) => void;
    onpick: (channel: SearchChannelType) => void;
    onremove: (login: string) => Promise<boolean>;
  };

  let { entries, selected, onselect, onpick, onremove }: SpamChannelListPropsType = $props();

  const uid = $props.id();
  const searchId = `${uid}-search`;

  let query = $state("");
  let list = $state<HTMLElement | null>(null);
  let leaving = $state<{ login: string; index: number } | null>(null);

  const twitch = new TwitchSearch();
  const found = $derived(twitch.results.channels.slice(0, SEARCH_SHOWN_MAX));
  const searching = $derived(query.trim() !== "");

  const focusSearch = (): void => document.getElementById(searchId)?.focus();

  // ! The results leave with the search: the focus goes back to its field.
  const pick = (channel: SearchChannelType): void => {
    query = "";
    twitch.clear();
    onpick(channel);
    focusSearch();
  };

  const remove = async (login: string, index: number): Promise<void> => {
    leaving = { login, index };

    if (!(await onremove(login))) {
      leaving = null;
    }
  };

  // ! The × leaves with its row: the focus goes to the row now in its place,
  // unless the user already moved it elsewhere.
  $effect(() => {
    if (!leaving || entries.some((entry) => entry.login === leaving?.login)) {
      return;
    }

    const focused = document.activeElement;
    const lost = !focused || focused === document.body || list?.contains(focused);

    if (lost) {
      const rows = list?.querySelectorAll<HTMLElement>("[data-channel-row]") ?? [];
      const next = rows[Math.min(leaving.index, rows.length - 1)];

      if (next) {
        next.focus();
      } else {
        focusSearch();
      }
    }

    leaving = null;
  });

  const ROW =
    "flex min-h-13 items-center gap-3 border-b px-3 py-2 transition-colors last:border-b-0";
</script>

<Panel title={t.spam_channels()} count={entries.length}>
  <SearchField
    id={searchId}
    placeholder={t.spam_search_placeholder()}
    aria-label={t.spam_search_placeholder()}
    bind:value={query}
    search={{ label: t.common_search(), busy: twitch.busy, run: () => twitch.now(query) }}
    oninput={() => twitch.update(query)}
    onkeydown={(event) => {
      if (event.key === "Enter" && !event.isComposing) {
        twitch.now(query);
      }
    }}
  />

  {#if searching}
    {#if found.length > 0}
      <div class="flex flex-col overflow-hidden rounded-lg border" role="list">
        {#each found as c (c.login)}
          {@const known = entries.find((entry) => entry.login === c.login)}
          <div class={cn(ROW, "hover:bg-accent")} role="listitem">
            <button
              type="button"
              class="flex min-w-0 flex-1 items-center gap-3 text-left"
              onclick={() => pick(c)}
            >
              <ChannelAvatar
                url={c.avatar_url}
                name={c.display_name}
                size={AvatarSize.MD}
                live={c.live}
              />
              <span class="flex min-w-0 flex-1 flex-col gap-0.5">
                <span class="flex min-w-0 items-center gap-1.5">
                  <span class="truncate font-semibold">{c.display_name}</span>
                  {#if c.verified}<span
                      class="inline-flex text-primary"
                      role="img"
                      aria-label={t.search_verified()}
                      title={t.search_verified()}><Icon name={IconName.VERIFIED} size={16} /></span
                    >{/if}
                </span>
                {#if known}<span class="truncate text-xs text-muted-foreground"
                    >{known.in_points ? t.spam_in_points() : t.spam_in_history()}</span
                  >{/if}
              </span>
            </button>
            <TextButton
              label={t.search_pick()}
              variant={ButtonVariant.OUTLINE}
              size={ButtonSize.SM}
              type={ButtonType.BUTTON}
              disabled={false}
              onclick={() => pick(c)}
            />
          </div>
        {/each}
      </div>
    {/if}
    <SearchNoResult shown={found.length === 0 && twitch.answered} {query} />
  {:else if entries.length === 0}
    <EmptyState text={t.spam_search_hint()} />
  {:else}
    <div
      bind:this={list}
      class="flex max-h-[min(26rem,55dvh)] flex-col overflow-y-auto overscroll-contain rounded-lg border"
      role="list"
    >
      {#each entries as e, index (e.login)}
        {@const current = e.login === selected}
        <div
          class={cn(
            ROW,
            current ? "bg-primary/15 shadow-[inset_3px_0_0_var(--primary)]" : "hover:bg-accent",
          )}
          role="listitem"
        >
          <button
            type="button"
            class="flex min-w-0 flex-1 items-center gap-3 text-left"
            aria-current={current ? "true" : undefined}
            data-channel-row
            onclick={() => onselect(e.login)}
          >
            <ChannelAvatar url={e.avatar} name={e.name} size={AvatarSize.MD} live={e.live} />
            <span class={cn("min-w-0 flex-1 truncate font-semibold", current && "text-link")}
              >{e.name}</span
            >
            {#if e.own}<span
                class="inline-flex shrink-0 items-center gap-1 text-hint font-semibold"
                role="img"
                aria-label={t.spam_balance_unlimited()}
                ><span class="text-link"><Icon name={IconName.CHANNEL_POINTS} size={13} /></span
                ><Icon name={IconName.INFINITY} size={16} /></span
              >{:else if e.balance !== undefined}<span
                class="inline-flex shrink-0 items-center gap-1 text-hint font-semibold tabular-nums"
                ><span class="text-link"><Icon name={IconName.CHANNEL_POINTS} size={13} /></span
                >{number(e.balance)}</span
              >{/if}
          </button>
          {#if !e.in_points}
            <ChipRemoveButton
              label={t.spam_remove_history()}
              name={e.name}
              iconSize={14}
              onclick={() => void remove(e.login, index)}
            />
          {:else}
            <span class="size-5 shrink-0" aria-hidden="true"></span>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</Panel>
