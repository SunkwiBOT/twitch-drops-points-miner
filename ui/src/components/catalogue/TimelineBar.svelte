<script lang="ts">
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import {
    estimateLabelWidth,
    getBarGeometry,
    getLabelPlacement,
    getNextBarIndex,
  } from "../../lib/catalogue/timeline.utils";
  import type { TimelineItemType, TimelineRangeType } from "../../lib/catalogue/timeline.utils";
  import { shortDuration } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { formatClock, formatDayMonth, formatWeekdayDayMonth } from "../../lib/time/time.utils";
  import { BarLabelPlacement, TIMELINE_PILL_MS } from "./timeline.constants";
  import RichText from "../ui/RichText.svelte";
  import TimeLeftTag from "./TimeLeftTag.svelte";

  type TimelineBarPropsType = {
    item: TimelineItemType;
    gameName: string;
    range: TimelineRangeType;
    top: number;
    height: number;
    laneWidth: number;
    now: number;
    todayEnd: number;
    onopen: (campaign: CampaignType) => void;
  };

  let {
    item,
    gameName,
    range,
    top,
    height,
    laneWidth,
    now,
    todayEnd,
    onopen,
  }: TimelineBarPropsType = $props();

  const geometry = $derived(getBarGeometry(range, item.start, item.end, now));

  const endLabel = $derived.by(() => {
    if (geometry?.isCutRight) {
      return t.timeline_until({ date: strong(formatDayMonth(item.end)) });
    }

    return item.end < todayEnd
      ? t.timeline_ends_today({ time: strong(formatClock(item.end)) })
      : t.timeline_ends_on({
          date: strong(formatWeekdayDayMonth(item.end)),
          time: strong(formatClock(item.end)),
        });
  });

  const left = $derived(item.end - now);

  const pill = $derived(
    left > 0 && left < TIMELINE_PILL_MS ? t.timeline_left({ time: shortDuration(left) }) : "",
  );

  const placement = $derived(
    geometry
      ? getLabelPlacement(
          geometry,
          laneWidth,
          estimateLabelWidth(item.campaign.name, endLabel.replaceAll("**", ""), pill),
        )
      : BarLabelPlacement.INSIDE,
  );

  const onkeydown = (event: KeyboardEvent): void => {
    const track =
      event.currentTarget instanceof HTMLElement ? event.currentTarget.parentElement : null;

    if (!track) {
      return;
    }

    const bars = Array.from(track.querySelectorAll<HTMLButtonElement>(":scope > [data-bar]"));
    const next = getNextBarIndex(
      bars.findIndex((bar) => bar === event.currentTarget),
      event.key,
      bars.length,
    );
    const target = next === null ? undefined : bars.at(next);

    if (target) {
      event.preventDefault();
      target.focus();
    }
  };
</script>

{#snippet label()}
  <span class="sr-only">{`${gameName}: `}</span>
  <span class="min-w-0 truncate">{item.campaign.name}</span>
  <span class="shrink-0 font-medium text-muted-foreground"><RichText text={endLabel} /></span>
  {#if pill}<TimeLeftTag text={pill} urgent={true} />{/if}
{/snippet}

{#if geometry}
  <button
    class="bar text-hint font-semibold text-foreground"
    class:cut-left={geometry.isCutLeft}
    class:cut-right={geometry.isCutRight}
    class:outside={placement !== BarLabelPlacement.INSIDE}
    data-bar
    type="button"
    title={`${gameName} · ${item.campaign.name}`}
    style:left="{geometry.left}%"
    style:width="{geometry.width}%"
    style:top="{top}px"
    style:height="{height}px"
    style:--p="{geometry.progress}%"
    onclick={() => onopen(item.campaign)}
    {onkeydown}
  >
    {#if placement === BarLabelPlacement.INSIDE}
      {@render label()}
    {:else}
      <span class="beside" class:left={placement === BarLabelPlacement.LEFT}>
        {@render label()}
      </span>
    {/if}
  </button>
{/if}

<style>
  .bar {
    position: absolute;
    z-index: 3;
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 6px;
    padding: 0 8px;
    border: 0;
    border-radius: var(--radius);
    background: linear-gradient(to right, var(--muted) 0 var(--p), var(--input) var(--p));
    line-height: 1.2;
    text-align: left;
    white-space: nowrap;
    cursor: pointer;
  }
  .bar::after {
    content: "";
    position: absolute;
    inset: 0;
    z-index: -1;
    border-radius: inherit;
    background: var(--foreground);
    opacity: 0;
    transition: opacity 0.12s ease;
    pointer-events: none;
  }
  .bar:hover::after {
    opacity: 0.1;
  }
  .bar:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: -2px;
  }
  .bar.cut-left {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }
  .bar.cut-right {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }
  .bar:not(.outside) {
    overflow: hidden;
  }
  .bar.outside {
    padding: 0;
  }
  .beside {
    position: absolute;
    top: 50%;
    left: calc(100% + 8px);
    display: flex;
    align-items: center;
    gap: 6px;
    transform: translateY(-50%);
  }
  .beside.left {
    left: auto;
    right: calc(100% + 8px);
  }
</style>
