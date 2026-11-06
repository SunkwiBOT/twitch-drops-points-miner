<script lang="ts">
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import {
    getTimelineRange,
    getTimelineTicks,
    getTodayBand,
    nextZoneMidnight,
    toRangePercent,
    toTimelineRows,
  } from "../../lib/catalogue/timeline.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { timePreferences } from "../../lib/time/time.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/ui.constants";
  import TimelineAxis from "./TimelineAxis.svelte";
  import TimelineGameRow from "./TimelineGameRow.svelte";
  import { DEFAULT_LANE_WIDTH, TIMELINE_PAGE_SIZE, TimelineZoom } from "./timeline.constants";

  type CatalogueTimelinePropsType = {
    campaigns: CampaignType[];
    now: number;
    zoom: TimelineZoom;
    onopen: (campaign: CampaignType) => void;
  };

  let { campaigns, now, zoom, onopen }: CatalogueTimelinePropsType = $props();

  let showAll = $state(false);
  let laneWidth = $state(DEFAULT_LANE_WIDTH);

  const zone = $derived(timePreferences.zone);
  const range = $derived(getTimelineRange(zoom, now, zone));
  const ticks = $derived(getTimelineTicks(range, zone));
  const today = $derived(getTodayBand(range, now, zone));
  const todayEnd = $derived(nextZoneMidnight(now, zone));
  const nowPercent = $derived(toRangePercent(range, now));
  const rows = $derived(toTimelineRows(campaigns));
  const shown = $derived(showAll ? rows : rows.slice(0, TIMELINE_PAGE_SIZE));
</script>

<div class="flex min-w-0 flex-col gap-3">
  <div class="timeline rounded-xl bg-card ring-1 ring-foreground/10">
    <TimelineAxis {ticks} {range} {zoom} {now} {nowPercent} bind:laneWidth />

    <div class="body rounded-b-xl">
      <div class="guides" aria-hidden="true">
        {#each ticks as tick (tick.time)}
          <div class="line" class:midnight={tick.isMidnight} style:left="{tick.percent}%"></div>
        {/each}
        <div class="today" style:left="{today.left}%" style:width="{today.width}%"></div>
        <div class="now-line" style:left="{nowPercent}%"></div>
      </div>
      <ul class="rows">
        {#each shown as row (row.game.id)}
          <TimelineGameRow {row} {range} {laneWidth} {now} {todayEnd} {onopen} />
        {/each}
      </ul>
    </div>
  </div>

  {#if shown.length < rows.length}
    <div class="flex">
      <TextButton
        label={t.timeline_show_all({ count: rows.length })}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (showAll = true)}
      />
    </div>
  {/if}
</div>

<style>
  .timeline {
    --head-width: 248px;
    /* ! The padding of the page's scroller (PageBody, p-6, p-3 on a phone): a sticky box
       stops that far inside it, so the axis is pulled up by as much to stick to the very top. */
    --scroller-padding: 24px;
    position: relative;
    isolation: isolate;
  }

  .body {
    position: relative;
    overflow: hidden;
  }
  .guides {
    position: absolute;
    top: 0;
    bottom: 0;
    left: var(--head-width);
    right: 0;
    z-index: 1;
    overflow: hidden;
    pointer-events: none;
  }
  .line {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 1px;
    background: var(--muted);
  }
  .line.midnight {
    background: var(--border);
  }
  .today {
    position: absolute;
    top: 0;
    bottom: 0;
    background: var(--foreground);
    opacity: 0.03;
  }
  .now-line {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    margin-left: -1px;
    background: var(--primary);
  }
  .rows {
    margin: 0;
    padding: 0;
    list-style: none;
  }

  @media (max-width: 860px) {
    .timeline {
      --head-width: 150px;
    }
  }

  /* PageBody's max-md:p-3. */
  @media (width < 48rem) {
    .timeline {
      --head-width: 116px;
      --scroller-padding: 12px;
    }
  }
</style>
