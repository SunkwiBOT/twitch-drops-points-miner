<script lang="ts">
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt, shortDuration } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { NEW_MS, URGENT_MS } from "../pages/pages.constants";
  import Badge from "../ui/Badge.svelte";
  import { BadgeTone, IconName } from "../ui/ui.constants";
  import LinkBadge from "./LinkBadge.svelte";
  import TimeLeftTag from "./TimeLeftTag.svelte";

  type CampaignCardPropsType = {
    campaign: CampaignType;
    ordered: boolean;
    onopen: (campaign: CampaignType) => void;
  };

  let { campaign, ordered, onopen }: CampaignCardPropsType = $props();

  const left = $derived(new Date(campaign.ends_at).getTime() - ticker.now);
  const isNew = $derived(ticker.now - new Date(campaign.starts_at).getTime() < NEW_MS);
  const art = $derived(boxArt(campaign.game.box_art_url, BoxArtSize.CARD));
</script>

<button
  class="group flex min-w-0 flex-col gap-1.5 rounded-lg text-left outline-none"
  type="button"
  onclick={() => onopen(campaign)}
>
  <span
    class="relative block aspect-[3/4] overflow-hidden rounded-lg bg-muted transition-[translate,box-shadow] duration-150 group-hover:translate-x-0.75 group-hover:-translate-y-0.75 group-hover:shadow-[-4px_4px_0_var(--primary)] group-focus-visible:translate-x-0.75 group-focus-visible:-translate-y-0.75 group-focus-visible:shadow-[-4px_4px_0_var(--primary)]"
  >
    {#if art}<img class="block size-full object-cover" src={art} alt="" loading="lazy" />{/if}
    <span class="absolute bottom-1.5 left-1.5"
      ><TimeLeftTag
        text={left > 0 ? shortDuration(left) : t.campaign_ended()}
        urgent={left < URGENT_MS}
      /></span
    >
    {#if ordered || isNew}
      <span class="absolute top-1.5 right-1.5">
        {#if ordered}
          <Badge
            text={t.queue_reason_pinned()}
            tone={BadgeTone.PRIMARY}
            dot={false}
            icon={IconName.PIN}
          />
        {:else}
          <Badge text={t.catalogue_new()} tone={BadgeTone.PRIMARY} dot={false} />
        {/if}
      </span>
    {/if}
  </span>
  <span class="line-clamp-2 min-h-[2lh] text-hint leading-tight font-semibold" title={campaign.name}
    >{campaign.name}</span
  >
  <span class="-mt-1 truncate text-xs text-muted-foreground">{campaign.game.name}</span>
  <span class="flex flex-wrap gap-1">
    <Badge
      text={t.catalogue_rewards({ count: campaign.drops.length })}
      tone={BadgeTone.NEUTRAL}
      dot={false}
    />
    <LinkBadge linked={campaign.linked} />
  </span>
</button>
