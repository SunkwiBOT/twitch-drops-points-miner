<script lang="ts">
  import { Card } from "$lib/components/ui/card";
  import { fetchRequests, fetchRequestsReport } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import type { RequestEntryType } from "../../lib/bindings/RequestEntryType";
  import type { RequestKind } from "../../lib/bindings/RequestKind";
  import type { RequestsQueryType } from "../../lib/bindings/RequestsQueryType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeError } from "../../lib/errors/errors.utils";
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
  import { ToastKind } from "../../lib/state/state.constants";
  import { app, toast } from "../../lib/state/state.svelte";
  import CopyButton from "../ui/CopyButton.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import LogSkeleton from "../ui/LogSkeleton.svelte";
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
  import { LOGS_REFRESH_MS, LOGS_SKELETON_ROWS } from "../pages/pages.constants";
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

  let entries = $state<RequestEntryType[]>([]);
  let cursor = $state<RequestCursorType | null>(null);
  let loading = $state(true);
  let loadingMore = $state(false);
  let problem = $state<string | null>(null);
  let sentinel: HTMLDivElement | undefined = $state();
  let generation = 0;

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

  const query = (from: RequestCursorType | null): RequestsQueryType => ({
    ...filters(),
    ...from,
    limit: REQUESTS_PAGE_SIZE,
  });

  $effect(() => {
    const request = query(null);
    let current = true;
    let refreshing = false;

    const load = async (): Promise<void> => {
      if (refreshing) {
        return;
      }

      refreshing = true;

      try {
        const page = await fetchRequests(request);

        if (!current) {
          return;
        }

        const oldest = page.requests.at(-1);
        const older = oldest ? entries.filter((entry) => isOlder(entry, oldest)) : [];

        entries = [...page.requests, ...older];
        problem = null;

        if (older.length === 0) {
          cursor = nextCursor(page);
        }
      } catch (e: any) {
        if (current) {
          problem = describeError(e);
        }
      } finally {
        refreshing = false;

        if (current) {
          loading = false;
        }
      }
    };

    void load();
    const timer = setInterval(() => void load(), LOGS_REFRESH_MS);

    return () => {
      current = false;
      clearInterval(timer);
    };
  });

  const loadMore = async (): Promise<void> => {
    if (cursor === null || loadingMore) {
      return;
    }

    const asked = generation;

    loadingMore = true;

    try {
      const page = await fetchRequests(query(cursor));

      if (asked !== generation) {
        return;
      }

      entries = [...entries, ...page.requests];
      cursor = nextCursor(page);
    } catch (e: any) {
      if (asked === generation) {
        toast(ToastKind.ERROR, describeError(e));
      }
    } finally {
      if (asked === generation) {
        loadingMore = false;
      }
    }
  };

  $effect(() => {
    if (!sentinel) {
      return;
    }

    const observer = new IntersectionObserver((seen) => {
      if (seen.some((entry) => entry.isIntersecting)) {
        void loadMore();
      }
    });

    observer.observe(sentinel);

    return () => observer.disconnect();
  });

  const restart = (): void => {
    generation += 1;
    entries = [];
    cursor = null;
    problem = null;
    loading = true;
    loadingMore = false;
  };

  const filterAccount = (next: string): void => {
    if (next === accountId) {
      return;
    }

    accountId = next;
    restart();
  };

  const filterKinds = (next: RequestKind[]): void => {
    hidden = offered.filter((kind) => !next.includes(kind));
    restart();
  };

  const filterErrors = (next: boolean): void => {
    errorsOnly = next;
    restart();
  };

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
      label={t.campaign_account()}
      align={MenuAlign.LEFT}
      onchange={filterAccount}
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
      onchange={filterKinds}
    />
  </div>
  <label class="flex cursor-pointer items-center gap-2 text-hint">
    <Toggle
      checked={errorsOnly}
      disabled={false}
      label={t.requests_errors_only()}
      onchange={filterErrors}
    />
    <span>{t.requests_errors_only()}</span>
  </label>
</PageHeader>

<Card class="@container gap-0 px-5 py-3">
  {#if loading && entries.length === 0}
    <LogSkeleton rows={LOGS_SKELETON_ROWS} />
  {:else if entries.length === 0 && problem}
    <EmptyState title={t.requests_load_failed()} text={problem} icon={IconName.ALERT} />
  {:else if entries.length === 0}
    <EmptyState text={errorsOnly ? t.requests_empty_errors() : t.requests_empty()} />
  {:else}
    <div class={REQUEST_LIST_CLASSES} role="list" aria-label={t.logs_tab_requests()}>
      {#each entries as entry (requestKey(entry))}
        <RequestRow {entry} account={accountOf(entry.account_id)} />
      {/each}
    </div>
    {#if loadingMore}<LogSkeleton rows={2} />{/if}
    <div class="flex justify-center pt-3" bind:this={sentinel}>
      {#if !loadingMore && cursor !== null}
        <TextButton
          label={t.requests_load_more()}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={loadMore}
        />
      {/if}
    </div>
  {/if}
</Card>
