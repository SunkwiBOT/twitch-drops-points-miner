<script lang="ts">
  import { Card } from "$lib/components/ui/card";
  import { fetchRequests, fetchRequestsReport } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import type { RequestEntryType } from "../../lib/bindings/RequestEntryType";
  import type { RequestKind } from "../../lib/bindings/RequestKind";
  import type { RequestsQueryType } from "../../lib/bindings/RequestsQueryType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { PagedFeed } from "../../lib/feed/feed.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import {
    DESKTOP_ONLY_KINDS,
    REQUEST_KIND_GROUPS,
    REQUEST_LIST_CLASSES,
    REQUESTS_PAGE_SIZE,
  } from "../../lib/requests/requests.constants";
  import {
    isOlder,
    nextCursor,
    REQUEST_KIND_GROUP_LABELS,
    REQUEST_KIND_LABELS,
    requestKey,
    type RequestCursorType,
  } from "../../lib/requests/requests.utils";
  import { app } from "../../lib/state/state.svelte";
  import CopyButton from "../ui/CopyButton.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import LoadMoreSentinel from "../ui/LoadMoreSentinel.svelte";
  import MultiSelect from "../ui/MultiSelect.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Select from "../ui/Select.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import Toggle from "../ui/Toggle.svelte";
  import {
    ButtonSize,
    ButtonType,
    ButtonVariant,
    CopyLayout,
    IconName,
    MenuAlign,
  } from "../ui/ui.constants";
  import LogSkeleton from "./LogSkeleton.svelte";
  import { LOGS_MORE_SKELETON_ROWS, LOGS_REFRESH_MS, LOGS_SKELETON_ROWS } from "./logs.constants";
  import RequestRow from "./RequestRow.svelte";

  type RequestListPropsType = { snapshot: SnapshotType };

  let { snapshot }: RequestListPropsType = $props();

  const groups = REQUEST_KIND_GROUPS.map((entry) => ({
    group: entry.group,
    kinds: entry.kinds.filter((kind) => isTauri() || !DESKTOP_ONLY_KINDS.has(kind)),
  }));
  const offered = groups.flatMap((entry) => entry.kinds);

  let accountId = $state("");
  let hidden = $state<RequestKind[]>([]);
  let errorsOnly = $state(false);

  const ticked = $derived(offered.filter((kind) => !hidden.includes(kind)));

  const accountOptions = $derived([
    { value: "", label: t.logs_filter_all() },
    ...snapshot.accounts.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url })),
  ]);

  const kindOptions = $derived(
    groups.flatMap((entry) =>
      entry.kinds.map((kind) => ({
        value: kind,
        label: REQUEST_KIND_LABELS[kind](),
        group: REQUEST_KIND_GROUP_LABELS[entry.group](),
      })),
    ),
  );

  const accountOf = (id: string | undefined) =>
    id === undefined ? undefined : snapshot.accounts.find((a) => a.id === id);

  const filters = (): RequestsQueryType => ({
    account_id: accountId || undefined,
    kinds: ticked.length === offered.length ? undefined : ticked,
    errors_only: errorsOnly || undefined,
  });

  const feed = new PagedFeed<RequestsQueryType, RequestEntryType, RequestCursorType>({
    fetch: async (query, from) => {
      const page = await fetchRequests({ ...query, ...from, limit: REQUESTS_PAGE_SIZE });

      return { items: page.requests, next: nextCursor(page) };
    },
    isOlder,
  });

  $effect(() => {
    void feed.show(filters());
    const timer = setInterval(() => void feed.refresh(), LOGS_REFRESH_MS);

    return () => clearInterval(timer);
  });

  const report = (): Promise<string> => fetchRequestsReport(filters());
</script>

<PageHeader intro={t.requests_subtitle()}>
  {#snippet actions()}
    <CopyButton
      text={report}
      label={t.logs_copy()}
      copiedLabel={t.requests_copied()}
      layout={CopyLayout.LABELLED}
      size={ButtonSize.MD}
    />
    <TextButton
      label={t.logs_report()}
      icon={IconName.ALERT}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => (app.feedbackDialog = true)}
    />
  {/snippet}
  <div class="w-56">
    <Select
      value={accountId}
      options={accountOptions}
      label={t.common_account()}
      align={MenuAlign.LEFT}
      onchange={(next) => (accountId = next)}
    />
  </div>
  <div class="w-56">
    <MultiSelect
      values={ticked}
      options={kindOptions}
      label={t.requests_types()}
      allLabel={t.requests_types_all()}
      noneLabel={t.requests_types_none()}
      countLabel={(count) => t.requests_types_count({ count })}
      align={MenuAlign.LEFT}
      onchange={(next) => (hidden = offered.filter((kind) => !next.includes(kind)))}
    />
  </div>
  <label class="flex cursor-pointer items-center gap-2 text-hint">
    <Toggle
      checked={errorsOnly}
      disabled={false}
      label={t.requests_errors_only()}
      onchange={(next) => (errorsOnly = next)}
    />
    <span>{t.requests_errors_only()}</span>
  </label>
</PageHeader>

<Card class="@container gap-0 px-5 py-3">
  {#if feed.loading && feed.items.length === 0}
    <LogSkeleton rows={LOGS_SKELETON_ROWS} />
  {:else if feed.items.length === 0 && feed.problem}
    <EmptyState title={t.requests_load_failed()} text={feed.problem} icon={IconName.ALERT} />
  {:else if feed.items.length === 0}
    <EmptyState text={errorsOnly ? t.requests_empty_errors() : t.requests_empty()} />
  {:else}
    <div class={REQUEST_LIST_CLASSES} role="list" aria-label={t.logs_tab_requests()}>
      {#each feed.items as entry (requestKey(entry))}
        <RequestRow {entry} account={accountOf(entry.account_id)} />
      {/each}
    </div>
    {#if feed.loadingMore}<LogSkeleton rows={LOGS_MORE_SKELETON_ROWS} />{/if}
    <LoadMoreSentinel
      label={t.requests_load_more()}
      busy={feed.loadingMore}
      more={feed.hasMore}
      onmore={feed.loadMore}
    />
  {/if}
</Card>
