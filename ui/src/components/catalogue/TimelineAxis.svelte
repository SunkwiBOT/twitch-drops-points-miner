<script lang="ts">
  import { getTickLabelStep } from "../../lib/catalogue/timeline.utils";
  import type { TimelineRangeType, TimelineTickType } from "../../lib/catalogue/timeline.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { formatClock, formatDayMonth, formatWeekdayDay } from "../../lib/time/time.utils";
  import { LABEL_ROOM_PX, TimelineZoom } from "./timeline.constants";

  type TimelineAxisPropsType = {
    ticks: TimelineTickType[];
    range: TimelineRangeType;
    zoom: TimelineZoom;
    now: number;
    nowPercent: number;
    laneWidth: number;
  };

  let {
    ticks,
    range,
    zoom,
    now,
    nowPercent,
    laneWidth = $bindable(),
  }: TimelineAxisPropsType = $props();

  const tickLabel = (tick: TimelineTickType): string => {
    if (zoom === TimelineZoom.TWO_WEEKS) {
      return tick.isFirstOfMonth ? formatDayMonth(tick.time) : formatWeekdayDay(tick.time);
    }

    return tick.isMidnight ? formatWeekdayDay(tick.time) : formatClock(tick.time);
  };

  const labels = $derived(ticks.map(tickLabel));
  const labelStep = $derived(getTickLabelStep(labels, range, laneWidth));
</script>

<div class="axis rounded-t-xl border-b bg-card" aria-hidden="true">
  <div class="corner"></div>
  <div class="scale" bind:clientWidth={laneWidth}>
    {#each ticks as tick, index (tick.time)}
      <div
        class="tick text-xs text-muted-foreground"
        class:midnight={tick.isMidnight}
        style:left="{tick.percent}%"
      >
        {#if index % labelStep === 0 && ((100 - tick.percent) / 100) * laneWidth >= LABEL_ROOM_PX}
          <span class="label">
            {#if tick.isMidnight}<b class="text-hint font-bold text-foreground">{labels[index]}</b
              >{:else}{labels[index]}{/if}
          </span>
        {/if}
      </div>
    {/each}
    <div
      class="now-tab rounded-r-md bg-primary text-xs font-bold text-primary-foreground"
      style:left="{nowPercent}%"
    >
      {t.timeline_now({ time: formatClock(now) })}
    </div>
  </div>
</div>

<style>
  .axis {
    position: sticky;
    top: calc(var(--timeline-sticky-top, 0px) - var(--scroller-padding));
    z-index: 8;
    display: grid;
    grid-template-columns: var(--head-width) minmax(0, 1fr);
    height: 44px;
  }
  .corner {
    border-right: 1px solid var(--border);
  }
  .scale {
    position: relative;
    overflow: hidden;
  }
  .tick {
    position: absolute;
    top: 0;
    bottom: 0;
    padding: 5px 0 0 6px;
    border-left: 1px solid var(--muted);
    line-height: 1.3;
    white-space: nowrap;
  }
  .tick.midnight {
    border-left-color: var(--border);
  }
  .label {
    position: relative;
    z-index: 1;
    padding-right: 4px;
    background: var(--card);
  }
  .now-tab {
    position: absolute;
    bottom: 4px;
    z-index: 2;
    margin-left: -1px;
    height: 18px;
    padding: 0 6px;
    line-height: 18px;
    white-space: nowrap;
  }
</style>
