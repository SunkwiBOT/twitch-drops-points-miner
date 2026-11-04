<script lang="ts">
  import { fetchLogs, fetchLogsReport, openLogsDirectory } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import { LogLevel } from "../../lib/bindings/LogLevel";
  import type { LogLineType } from "../../lib/bindings/LogLineType";
  import type { LogsQueryType } from "../../lib/bindings/LogsQueryType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeError } from "../../lib/errors/errors.utils";
  import { formatClockWithSeconds } from "../../lib/time/time.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { app, toast } from "../../lib/state/state.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { Card } from "$lib/components/ui/card";
  import { cn } from "$lib/utils";
  import Avatar from "../ui/Avatar.svelte";
  import CopyButton from "../ui/CopyButton.svelte";
  import LogSkeleton from "../ui/LogSkeleton.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import MultiSelect from "../ui/MultiSelect.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Select from "../ui/Select.svelte";
  import {
    AvatarSize,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    CopyLayout,
    IconName,
    MenuAlign,
  } from "../ui/ui.constants";
  import {
    LOG_LEVEL_CLASSES,
    LOG_LEVELS_KEPT,
    LOG_LINE_CLASSES,
    LOG_LIST_CLASSES,
    LOGS_PAGE_SIZE,
    LOGS_REFRESH_MS,
    LOGS_SKELETON_ROWS,
  } from "../pages/pages.constants";
  import { LOG_LEVEL_LABELS } from "../pages/pages.utils";

  type LogLinesPropsType = { snapshot: SnapshotType };

  let { snapshot }: LogLinesPropsType = $props();

  const accountOfLogin = (login: string) =>
    app.snapshot?.accounts.find((a) => a.login === login.toLowerCase());

  let filter = $state(app.logsAccount ?? "");
  app.logsAccount = null;

  let levels = $state<LogLevel[]>([...LOG_LEVELS_KEPT]);

  let lines = $state<LogLineType[]>([]);
  let cursor = $state<number | null>(null);
  let loading = $state(true);
  let loadingMore = $state(false);
  let sentinel: HTMLDivElement | undefined = $state();

  const options = $derived([
    { value: "", label: t.logs_filter_all() },
    ...snapshot.accounts.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url })),
  ]);

  const filters = (): LogsQueryType => ({
    account_id: filter || undefined,
    levels: levels.length === LOG_LEVELS_KEPT.length ? undefined : levels,
  });

  const query = (before?: number): LogsQueryType => ({
    ...filters(),
    before_id: before,
    limit: LOGS_PAGE_SIZE,
  });

  $effect(() => {
    const request = query();
    let current = true;

    const load = async (): Promise<void> => {
      try {
        const page = await fetchLogs(request);

        if (!current) {
          return;
        }

        const newest = [...page.lines].reverse();
        const oldest = newest.at(-1)?.id ?? 0;
        const older = cursor === null ? [] : lines.filter((line) => line.id < oldest);

        lines = [...newest, ...older];

        if (older.length === 0) {
          cursor = page.next_before_id ?? null;
        }
      } catch (e: any) {
        console.warn("log refresh failed", e);
      } finally {
        loading = false;
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

    loadingMore = true;

    try {
      const page = await fetchLogs(query(cursor));
      lines = [...lines, ...[...page.lines].reverse()];
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

  const restart = (): void => {
    lines = [];
    cursor = null;
    loading = true;
  };

  const filterBy = (next: string): void => {
    if (next === filter) {
      return;
    }

    filter = next;
    restart();
  };

  const filterLevels = (next: LogLevel[]): void => {
    levels = LOG_LEVELS_KEPT.filter((level) => next.includes(level));
    restart();
  };

  const report = (): Promise<string> => fetchLogsReport(filters());

  const openFolder = async (): Promise<void> => {
    try {
      await openLogsDirectory();
    } catch (e: any) {
      toast(ToastKind.ERROR, describeError(e));
    }
  };
</script>

<PageHeader intro={t.logs_subtitle()}>
  {#snippet actions()}
    <CopyButton
      text={report}
      label={t.logs_copy()}
      copiedLabel={t.logs_copied()}
      layout={CopyLayout.LABELLED}
      size={ButtonSize.MD}
    />
    {#if isTauri()}
      <TextButton
        label={t.logs_open_folder()}
        icon={IconName.FILE}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={openFolder}
      />
    {/if}
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
      value={filter}
      {options}
      label={t.campaign_account()}
      align={MenuAlign.LEFT}
      onchange={filterBy}
    />
  </div>
  <div class="w-56">
    <MultiSelect
      values={levels}
      options={LOG_LEVELS_KEPT.map((level) => ({
        value: level,
        label: LOG_LEVEL_LABELS[level]?.() ?? level,
      }))}
      label={t.logs_levels()}
      allLabel={t.logs_levels_all()}
      noneLabel={t.logs_levels_none()}
      countLabel={(count) => t.logs_levels_count({ count })}
      align={MenuAlign.LEFT}
      onchange={filterLevels}
    />
  </div>
</PageHeader>

<Card class="gap-0 px-5 py-3">
  {#if loading && lines.length === 0}
    <LogSkeleton rows={LOGS_SKELETON_ROWS} />
  {:else if lines.length === 0}
    <EmptyState text={t.logs_empty()} />
  {:else}
    <div class={LOG_LIST_CLASSES}>
      {#each lines as line (line.id)}
        <div class={LOG_LINE_CLASSES}>
          <span class="whitespace-nowrap text-muted-foreground"
            >{formatClockWithSeconds(line.at)}</span
          >
          <span class={cn("hidden md:block", LOG_LEVEL_CLASSES[line.level])}>{line.level}</span>
          <span class="hidden min-w-0 items-center gap-1.5 md:flex">
            {#if line.account}
              {@const owner = accountOfLogin(line.account)}
              <Avatar
                url={owner?.avatar_url}
                name={owner?.display_name ?? line.account}
                size={AvatarSize.XS}
              />
              <span class="truncate text-link">{owner?.display_name ?? line.account}</span>
            {/if}
          </span>
          <span>{line.text}</span>
        </div>
      {/each}
    </div>
    {#if loadingMore}<LogSkeleton rows={2} />{/if}
    <div class="flex justify-center pt-3" bind:this={sentinel}>
      {#if !loadingMore && cursor !== null}
        <TextButton
          label={t.logs_load_more()}
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
