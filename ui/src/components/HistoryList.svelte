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
  import { describeError } from "../lib/errors/errors.utils";
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
  import { t } from "../lib/i18n/i18n.svelte";
  import { ToastKind } from "../lib/state/state.constants";
  import { app, openTab, showTab, toast } from "../lib/state/state.svelte";
  import HistoryRow from "./HistoryRow.svelte";
  import { HISTORY_PAGE_SIZE, HISTORY_SKELETON_ROWS } from "./pages/pages.constants";
  import MultiSelect from "./ui/MultiSelect.svelte";
  import Segmented from "./ui/Segmented.svelte";
  import TextButton from "./ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, MenuAlign } from "./ui/ui.constants";

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
  let events = $state<MinerEventType[]>([]);
  const shown = $derived(events.map(shownEvent));
  let cursor = $state<number | null>(null);
  let loading = $state(true);
  let loadingMore = $state(false);
  let sentinel: HTMLDivElement | undefined = $state();

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

  const query = (before?: number): HistoryQueryType => ({
    account_id: accountId ?? undefined,
    category: filter === JOURNAL_ALL ? undefined : filter,
    kinds: ticked.length === offered.length ? undefined : ticked,
    before_id: before,
    limit: HISTORY_PAGE_SIZE,
  });

  const showKinds = (values: EventKind[]): void => {
    hidden = [
      ...hidden.filter((kind) => !offered.includes(kind)),
      ...offered.filter((kind) => !values.includes(kind)),
    ];
  };

  let loadedFor = "";

  $effect(() => {
    const request = query();
    const key = JSON.stringify(request);
    void app.liveEvents.length;
    let current = true;

    if (key !== loadedFor) {
      loadedFor = key;
      events = [];
      cursor = null;
      loading = true;
    }

    fetchHistory(request)
      .then((page) => {
        if (!current) {
          return;
        }

        const oldest = page.events.at(-1)?.id ?? 0;
        const older = cursor === null ? [] : events.filter((event) => event.id < oldest);

        events = [...page.events, ...older];

        if (older.length === 0) {
          cursor = page.next_before_id ?? null;
        }
      })
      .catch((e: unknown) => toast(ToastKind.ERROR, describeError(e)))
      .finally(() => (loading = false));

    return () => {
      current = false;
    };
  });

  const loadMore = async (): Promise<void> => {
    if (cursor === null || loadingMore) {
      return;
    }

    loadingMore = true;

    try {
      const page = await fetchHistory(query(cursor));
      events = [...events, ...page.events];
      cursor = page.next_before_id ?? null;
    } catch (e: any) {
      toast(ToastKind.ERROR, describeError(e));
    } finally {
      loadingMore = false;
    }
  };

  $effect(() => {
    if (!sentinel) {
      return;
    }

    const observer = new IntersectionObserver((entries) => {
      if (entries.some((entry) => entry.isIntersecting)) {
        void loadMore();
      }
    });

    observer.observe(sentinel);

    return () => observer.disconnect();
  });

  $effect(() => {
    ensureChannels(unnamedChannels(events));
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

{#if loading && events.length === 0}
  <Table class="text-hint">
    {@render head()}
    <TableBody>{@render skeleton(HISTORY_SKELETON_ROWS)}</TableBody>
  </Table>
{:else if events.length === 0}
  <EmptyState text={t.history_empty()} />
{:else}
  <Table class="text-hint">
    {@render head()}
    <TableBody>
      {#each shown as event (event.id)}
        <HistoryRow {event} {showAccount} />
      {/each}
      {#if loadingMore}{@render skeleton(2)}{/if}
    </TableBody>
  </Table>
  <div class="flex justify-center pt-3" bind:this={sentinel}>
    {#if !loadingMore && cursor !== null}
      <TextButton
        label={t.history_load_more()}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={loadMore}
      />
    {/if}
  </div>
{/if}
