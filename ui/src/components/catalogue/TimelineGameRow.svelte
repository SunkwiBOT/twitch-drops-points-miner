<script lang="ts">
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { getLaneLayout, getLaneTop } from "../../lib/catalogue/timeline.utils";
  import type { TimelineRangeType, TimelineRowType } from "../../lib/catalogue/timeline.utils";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import TimelineBar from "./TimelineBar.svelte";

  type TimelineGameRowPropsType = {
    row: TimelineRowType;
    range: TimelineRangeType;
    laneWidth: number;
    now: number;
    todayEnd: number;
    onopen: (campaign: CampaignType) => void;
  };

  let { row, range, laneWidth, now, todayEnd, onopen }: TimelineGameRowPropsType = $props();

  const layout = $derived(getLaneLayout(row.items.length));
  const art = $derived(boxArt(row.game.box_art_url, BoxArtSize.ROW));
  const first = $derived(row.items[0].campaign);
</script>

<li class="game-row group border-b last:border-b-0">
  <!-- ! Mouse targets only: the bars stay the keyboard controls, so these skip
       the tab order. -->
  <button
    class="game flex min-w-0 items-center gap-2.5 border-r bg-card px-3 py-1.5 text-left transition-colors group-hover:bg-accent"
    type="button"
    tabindex="-1"
    onclick={() => onopen(first)}
  >
    {#if art}
      <img
        class="block h-10 w-7.5 shrink-0 rounded-md bg-muted object-cover"
        src={art}
        alt=""
        loading="lazy"
        decoding="async"
      />
    {:else}
      <span class="block h-10 w-7.5 shrink-0 rounded-md bg-muted"></span>
    {/if}
    <span class="flex min-w-0 flex-col">
      <span class="truncate font-semibold">{row.game.name}</span>
      <span class="truncate text-xs text-muted-foreground max-[860px]:hidden">
        {t.catalogue_campaigns({ count: row.items.length })} · {t.catalogue_rewards({
          count: row.rewards,
        })}
      </span>
    </span>
  </button>
  <div class="lane transition-colors group-hover:bg-accent" style:min-height="{layout.height}px">
    <div class="track" style:height="{layout.height}px">
      <button
        class="lane-hit"
        type="button"
        tabindex="-1"
        aria-label={first.name}
        style:top="0"
        style:height="{layout.height}px"
        onclick={() => onopen(first)}
      ></button>
      {#each row.items as item, index (item.campaign.id)}
        <button
          class="lane-hit"
          type="button"
          tabindex="-1"
          aria-label={item.campaign.name}
          style:top="{getLaneTop(layout, index)}px"
          style:height="{layout.laneHeight}px"
          onclick={() => onopen(item.campaign)}
        ></button>
      {/each}
      {#each row.items as item, index (item.campaign.id)}
        <TimelineBar
          {item}
          gameName={row.game.name}
          {range}
          top={getLaneTop(layout, index)}
          height={layout.laneHeight}
          {laneWidth}
          {now}
          {todayEnd}
          {onopen}
        />
      {/each}
    </div>
  </div>
</li>

<style>
  .game-row {
    display: grid;
    grid-template-columns: var(--head-width) minmax(0, 1fr);
  }
  .game {
    position: relative;
    z-index: 2;
    width: 100%;
    font: inherit;
    cursor: pointer;
  }
  .lane {
    position: relative;
    display: flex;
    align-items: center;
  }
  .track {
    position: relative;
    width: 100%;
  }
  .lane-hit {
    position: absolute;
    left: 0;
    right: 0;
    padding: 0;
    border: 0;
    background: transparent;
    cursor: pointer;
  }
</style>
