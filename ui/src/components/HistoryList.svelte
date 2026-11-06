<script lang="ts">
  import type { Snippet } from "svelte";

  import EmptyState from "./ui/EmptyState.svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
  } from "$lib/components/ui/table";
  import { fetchHistory } from "../lib/api/client";
  import type { EventKind } from "../lib/bindings/EventKind";
  import type { HistoryQueryType } from "../lib/bindings/HistoryQueryType";
  import type { MinerEventType } from "../lib/bindings/MinerEventType";
  import { ensureChannels } from "../lib/channels/channels.svelte";
  import {
    EVENT_ICONS,
    EVENT_KIND_LABELS,
    JOURNAL_ALL,
    JOURNAL_FILTER_SLUGS,
    JOURNAL_FILTERS,
    KIND_GROUP_LABELS,
    kindGroupsOf,
  } from "../lib/events/events.utils";
  import { shownEvent, unnamedChannels } from "../lib/events/shown.svelte";
  import { PagedFeed } from "../lib/feed/feed.svelte";
  import { t } from "../lib/i18n/i18n.svelte";
  import { wideLayout } from "../lib/state/viewport.svelte";
  import { app, openTab, showTab } from "../lib/state/state.svelte";
  import {
    HISTORY_MORE_SKELETON_ROWS,
    HISTORY_PAGE_SIZE,
    HISTORY_SKELETON_ROWS,
  } from "./history.constants";
  import HistoryCard from "./HistoryCard.svelte";
  import HistoryRow from "./HistoryRow.svelte";
  import LoadMoreSentinel from "./ui/LoadMoreSentinel.svelte";
  import MultiSelect from "./ui/MultiSelect.svelte";
  import Segmented from "./ui/Segmented.svelte";
  import { IconName, MenuAlign } from "./ui/ui.constants";

  type HistoryListPropsType = {
    accountId: string | null;
    showAccount: boolean;
    lead?: Snippet;
  };

  let { accountId, showAccount, lead }: HistoryListPropsType = $props();

  const filter = $derived(
    openTab(
      JOURNAL_FILTERS.map((item) => item.value),
      JOURNAL_FILTER_SLUGS,
      JOURNAL_ALL,
    ),
  );

  let hidden = $state<EventKind[]>([]);

  const groups = $derived(kindGroupsOf(filter));
  const offered = $derived([...new Set(groups.flatMap((entry) => entry.kinds))]);
  const ticked = $derived(offered.filter((kind) => !hidden.includes(kind)));

  const kindOptions = $derived(
    groups.flatMap((entry) =>
      entry.kinds.map((kind) => ({
        value: kind,
        label: EVENT_KIND_LABELS[kind](),
        icon: EVENT_ICONS[kind],
        group: KIND_GROUP_LABELS[entry.group](),
      })),
    ),
  );

  const query = (): HistoryQueryType => ({
    account_id: accountId ?? undefined,
    category: filter === JOURNAL_ALL ? undefined : filter,
    kinds: ticked.length === offered.length ? undefined : ticked,
  });

  const feed = new PagedFeed<HistoryQueryType, MinerEventType, number>({
    fetch: async (request, before) => {
      const page = await fetchHistory({
        ...request,
        before_id: before ?? undefined,
        limit: HISTORY_PAGE_SIZE,
      });

      return { items: page.events, next: page.next_before_id ?? null };
    },
    isOlder: (event, oldest) => event.id < oldest.id,
  });

  const shown = $derived(feed.items.map(shownEvent));

  const showKinds = (values: EventKind[]): void => {
    hidden = [
      ...hidden.filter((kind) => !offered.includes(kind)),
      ...offered.filter((kind) => !values.includes(kind)),
    ];
  };

  $effect(() => {
    void app.liveEvents.length;
    void feed.show(query());
  });

  $effect(() => {
    ensureChannels(unnamedChannels(feed.items));
  });
</script>

<div class="mb-3 flex flex-wrap items-center gap-2.5">
  {#if lead}{@render lead()}{/if}
  <span class="flex-1"></span>
  <div class="w-56">
    <MultiSelect
      values={ticked}
      options={kindOptions}
      label={t.journal_types()}
      allLabel={t.journal_types_all()}
      noneLabel={t.journal_types_none()}
      countLabel={(count) => t.journal_types_count({ count })}
      align={MenuAlign.RIGHT}
      onchange={showKinds}
    />
  </div>
  <Segmented
    value={filter}
    options={JOURNAL_FILTERS.map((item) => ({
      value: item.value,
      label: item.label(),
      icon: item.icon,
    }))}
    label={t.journal_category()}
    onchange={(next) => showTab(JOURNAL_FILTER_SLUGS, next)}
  />
</div>

{#snippet head()}
  <TableHeader>
    <TableRow class="hover:bg-transparent">
      <TableHead class="pl-0">{t.journal_col_time()}</TableHead>
      {#if showAccount}<TableHead>{t.journal_col_account()}</TableHead>{/if}
      <TableHead class="w-full">{t.journal_col_event()}</TableHead>
      <TableHead>{t.journal_col_channel()}</TableHead>
      <TableHead>{t.journal_col_type()}</TableHead>
      <TableHead>{t.journal_col_miner()}</TableHead>
    </TableRow>
  </TableHeader>
{/snippet}

{#snippet skeleton(rows: number)}
  {#each { length: rows }, index (index)}
    <TableRow class="hover:bg-transparent" aria-hidden="true">
      <TableCell class="pl-0"><Skeleton class="h-3 w-24" /></TableCell>
      {#if showAccount}<TableCell><Skeleton class="h-5 w-20 rounded-full" /></TableCell>{/if}
      <TableCell>
        <span class="flex items-center gap-2.5">
          <Skeleton class="size-7 shrink-0 rounded-md" />
          <Skeleton class="h-3.5 w-2/3" />
        </span>
      </TableCell>
      <TableCell><Skeleton class="h-5 w-24 rounded-full" /></TableCell>
      <TableCell><Skeleton class="h-5 w-20 rounded-full" /></TableCell>
      <TableCell><Skeleton class="h-5 w-24 rounded-full" /></TableCell>
    </TableRow>
  {/each}
{/snippet}

{#snippet cardSkeleton(rows: number)}
  {#each { length: rows }, index (index)}
    <li class="flex gap-2.5 py-3" aria-hidden="true">
      <Skeleton class="size-7 shrink-0 rounded-md" />
      <span class="flex flex-1 flex-col gap-2">
        <Skeleton class="h-3.5 w-4/5" />
        <Skeleton class="h-3 w-1/2" />
      </span>
    </li>
  {/each}
{/snippet}

{#if feed.loading && feed.items.length === 0}
  {#if wideLayout.current}
    <Table class="text-hint">
      {@render head()}
      <TableBody>{@render skeleton(HISTORY_SKELETON_ROWS)}</TableBody>
    </Table>
  {:else}
    <ul class="flex flex-col divide-y text-hint">{@render cardSkeleton(HISTORY_SKELETON_ROWS)}</ul>
  {/if}
{:else if feed.items.length === 0 && feed.problem}
  <EmptyState title={t.history_load_failed()} text={feed.problem} icon={IconName.ALERT} />
{:else if feed.items.length === 0}
  <EmptyState text={t.history_empty()} />
{:else}
  {#if wideLayout.current}
    <Table class="text-hint">
      {@render head()}
      <TableBody>
        {#each shown as event (event.id)}
          <HistoryRow {event} {showAccount} />
        {/each}
        {#if feed.loadingMore}{@render skeleton(HISTORY_MORE_SKELETON_ROWS)}{/if}
      </TableBody>
    </Table>
  {:else}
    <!-- ! On a phone the table's six columns leave the event a few letters: one card per event. -->
    <ul class="flex flex-col divide-y text-hint">
      {#each shown as event (event.id)}
        <HistoryCard {event} {showAccount} />
      {/each}
      {#if feed.loadingMore}{@render cardSkeleton(HISTORY_MORE_SKELETON_ROWS)}{/if}
    </ul>
  {/if}
  <LoadMoreSentinel
    label={t.history_load_more()}
    busy={feed.loadingMore}
    more={feed.hasMore}
    onmore={feed.loadMore}
  />
{/if}
