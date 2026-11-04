<script lang="ts">
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeCode } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_URL } from "../../lib/partners.constants";
  import { openTab, showTab } from "../../lib/state/state.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { runningCampaigns } from "../../lib/catalogue/timeline.utils";
  import { matchesSearch, normalizeSearch } from "../../lib/search/search.utils";
  import { linkMarker, strong } from "../../lib/text.utils";
  import { relativeTime } from "../../lib/time/time.utils";
  import CampaignCard from "../catalogue/CampaignCard.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import CatalogRefreshButton from "../catalogue/CatalogRefreshButton.svelte";
  import CatalogueTimeline from "../catalogue/CatalogueTimeline.svelte";
  import { TIMELINE_ZOOMS, TimelineZoom } from "../catalogue/timeline.constants";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import RichText from "../ui/RichText.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { IconName } from "../ui/ui.constants";
  import {
    CATALOGUE_VIEW_SLUGS,
    CatalogueSort,
    CatalogueView,
  } from "../catalogue/catalogue.constants";
  import { ZOOM_LABELS } from "../catalogue/catalogue.utils";

  type CataloguePropsType = { snapshot: SnapshotType };

  let { snapshot }: CataloguePropsType = $props();

  let query = $state("");
  const view = $derived(
    openTab(Object.values(CatalogueView), CATALOGUE_VIEW_SLUGS, CatalogueView.TIMELINE),
  );
  let sort = $state(CatalogueSort.ENDING);
  let open = $state<CampaignType | null>(null);

  const time = (iso: string): number => new Date(iso).getTime();

  const running = $derived(runningCampaigns(snapshot.catalog.campaigns, ticker.now));

  const active = $derived.by(() => {
    const needle = normalizeSearch(query);

    return running.filter(
      (c) => !needle || matchesSearch(c.game.name, needle) || matchesSearch(c.name, needle),
    );
  });

  const sorted = $derived.by(() => {
    const list = [...active];

    switch (sort) {
      case CatalogueSort.ENDING:
        return list.sort((a, b) => time(a.ends_at) - time(b.ends_at));
      case CatalogueSort.NEW:
        return list.sort((a, b) => time(b.starts_at) - time(a.starts_at));
      case CatalogueSort.GAME:
        return list.sort(
          (a, b) => a.game.name.localeCompare(b.game.name) || a.name.localeCompare(b.name),
        );
    }
  });

  const views = $derived([
    { value: CatalogueView.TIMELINE, label: t.catalogue_view_timeline(), icon: IconName.CALENDAR },
    { value: CatalogueView.CARDS, label: t.catalogue_view_cards(), icon: IconName.GRID },
  ]);

  let zoom = $state(TimelineZoom.TWO_WEEKS);

  const sorts = $derived([
    { value: CatalogueSort.ENDING, label: t.catalogue_sort_ending() },
    { value: CatalogueSort.NEW, label: t.catalogue_sort_new() },
    { value: CatalogueSort.GAME, label: t.catalogue_sort_game() },
  ]);
</script>

<PageBody>
  <PageHeader
    intro={t.catalogue_intro({ site: linkMarker(0) })}
    links={[{ label: DROPS_API_NAME, url: DROPS_WEB_URL }]}
  >
    {#snippet meta()}
      {#if snapshot.catalog.updated_at}
        <RichText
          text={t.catalogue_last_update({
            time: strong(relativeTime(snapshot.catalog.updated_at, ticker.now)),
          })}
        />{#if snapshot.catalog.fetched_at}{" · "}<RichText
            text={t.catalogue_fetched({
              time: strong(relativeTime(snapshot.catalog.fetched_at, ticker.now)),
              minutes: strong(Math.round(snapshot.catalog.interval_seconds / 60)),
            })}
          />{/if}
      {/if}
    {/snippet}
    {#snippet actions()}<CatalogRefreshButton label={t.common_refresh()} />{/snippet}
    <div class="w-full max-w-96">
      <SearchField
        placeholder={t.campaigns_search_placeholder()}
        aria-label={t.campaigns_search_placeholder()}
        bind:value={query}
        search={null}
      />
    </div>
    <span class="flex-1"></span>
    <Segmented
      value={view}
      options={views}
      label={t.catalogue_view()}
      onchange={(next) => showTab(CATALOGUE_VIEW_SLUGS, next)}
    />
  </PageHeader>

  {#if view === CatalogueView.CARDS}
    <Segmented
      value={sort}
      options={sorts}
      label={t.catalogue_sort()}
      onchange={(next) => (sort = next)}
    />
  {:else}
    <Segmented
      value={zoom}
      options={TIMELINE_ZOOMS.map((option) => ({ value: option, label: ZOOM_LABELS[option]() }))}
      label={t.timeline_zoom()}
      onchange={(next) => (zoom = next)}
    />
  {/if}

  {#if active.length === 0}
    <EmptyState
      text={snapshot.catalog.problem
        ? t.dashboard_catalog_error({ error: describeCode(snapshot.catalog.problem) })
        : running.length
          ? t.catalogue_empty()
          : t.catalogue_none()}
      icon={IconName.GIFT}
    />
  {:else if view === CatalogueView.TIMELINE}
    <CatalogueTimeline
      campaigns={active}
      now={ticker.now}
      {zoom}
      onopen={(campaign) => (open = campaign)}
    />
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-4">
      {#each sorted as c (c.id)}
        <CampaignCard campaign={c} ordered={false} onopen={(campaign) => (open = campaign)} />
      {/each}
    </div>
  {/if}
</PageBody>

<CampaignDialog campaign={open} contextAccountId={null} onclose={() => (open = null)} />
