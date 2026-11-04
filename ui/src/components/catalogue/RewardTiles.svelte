<script lang="ts">
  import { Progress } from "$lib/components/ui/progress";
  import { BenefitKind } from "../../lib/bindings/BenefitKind";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { TimedDropType } from "../../lib/bindings/TimedDropType";
  import { percent, shortDuration } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { MINUTE_MS } from "../../lib/time/time.constants";
  import { cn } from "$lib/utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import Badge from "../ui/Badge.svelte";
  import Icon from "../ui/Icon.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import RichText from "../ui/RichText.svelte";
  import { BadgeTone, ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";
  import { RewardState } from "./rewards.constants";

  type RewardTilesPropsType = {
    campaign: CampaignType;
    ignored: string[];
    onclaim?: (instanceId: string) => Promise<boolean>;
    onignore?: (dropId: string, ignored: boolean) => Promise<unknown>;
  };

  let { campaign, ignored, onclaim, onignore }: RewardTilesPropsType = $props();

  let claiming = $state<string | null>(null);
  let claimedHere = $state<string[]>([]);
  let toggling = $state<string | null>(null);

  const KIND_LABELS: Record<BenefitKind, (() => string) | null> = {
    [BenefitKind.BADGE]: () => t.reward_kind_badge(),
    [BenefitKind.IN_GAME_ITEM]: () => t.reward_kind_item(),
    [BenefitKind.OTHER]: null,
  };

  const stateOf = (drop: TimedDropType): RewardState => {
    if (drop.claimed || (drop.instance_id && claimedHere.includes(drop.instance_id))) {
      return RewardState.CLAIMED;
    }

    if (drop.instance_id) {
      return RewardState.READY;
    }

    if (drop.required_subs > 0 || ignored.includes(drop.id)) {
      return RewardState.SKIPPED;
    }

    return campaign.in_inventory ? RewardState.IN_PROGRESS : RewardState.TO_EARN;
  };

  const requirement = (drop: TimedDropType): string => {
    if (drop.required_subs > 0) {
      return t.reward_subscribe();
    }

    if (ignored.includes(drop.id)) {
      return t.reward_ignored();
    }

    return drop.required_minutes > 0
      ? t.reward_watch({ time: strong(shortDuration(drop.required_minutes * MINUTE_MS)) })
      : t.reward_watch_stream();
  };

  const claim = async (instanceId: string): Promise<void> => {
    if (!onclaim) {
      return;
    }

    claiming = instanceId;

    if (await onclaim(instanceId)) {
      claimedHere = [...claimedHere, instanceId];
    }

    claiming = null;
  };

  const toggleIgnored = async (dropId: string): Promise<void> => {
    if (!onignore) {
      return;
    }

    toggling = dropId;
    await onignore(dropId, !ignored.includes(dropId));
    toggling = null;
  };
</script>

<div class="grid grid-cols-[repeat(auto-fill,minmax(116px,1fr))] gap-2.5">
  {#each campaign.drops as d (d.id)}
    {@const state = stateOf(d)}
    {@const benefit = d.benefits.at(0)}
    {@const kind = benefit ? KIND_LABELS[benefit.kind] : null}
    <div
      class={cn(
        "group flex flex-col overflow-hidden rounded-md bg-background ring-1 ring-border",
        state === RewardState.CLAIMED && "ring-success-emphasis/60",
        state === RewardState.READY && "ring-2 ring-primary",
      )}
    >
      <span class="relative grid aspect-square place-items-center bg-muted text-muted-foreground">
        {#if benefit?.image_url}<img
            class={cn(
              "size-full object-contain p-2",
              state === RewardState.SKIPPED && "opacity-40 grayscale",
            )}
            src={benefit.image_url}
            alt=""
            loading="lazy"
          />{:else}<Icon name={IconName.GIFT} size={28} />{/if}
        {#if d.required_subs > 0}<span class="absolute top-1.5 left-1.5"
            ><Badge
              text={t.reward_sub_only()}
              tone={BadgeTone.WARNING}
              dot={false}
              hint={t.badge_hint_sub_only()}
            /></span
          >{/if}
        {#if state === RewardState.CLAIMED}<span class="absolute top-1.5 left-1.5"
            ><Badge
              text={t.reward_state_claimed()}
              tone={BadgeTone.SUCCESS}
              dot={false}
              icon={IconName.CHECK}
              hint={t.badge_hint_claimed()}
            /></span
          >{:else if state === RewardState.READY}<span class="absolute top-1.5 left-1.5"
            ><Badge
              text={t.reward_state_ready()}
              tone={BadgeTone.PRIMARY}
              dot={false}
              hint={t.badge_hint_ready()}
            /></span
          >{/if}
        {#if onignore && d.required_subs === 0 && state !== RewardState.CLAIMED && state !== RewardState.READY}
          <span
            class={cn(
              "absolute top-1 right-1 rounded-md bg-background/85 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100",
              ignored.includes(d.id) && "opacity-100",
            )}
          >
            <IconButton
              icon={ignored.includes(d.id) ? IconName.EYE : IconName.EYE_OFF}
              label={ignored.includes(d.id) ? t.reward_unignore() : t.reward_ignore()}
              busy={toggling === d.id}
              disabled={false}
              variant={ButtonVariant.GHOST}
              size={ButtonSize.SM}
              onclick={() => toggleIgnored(d.id)}
            />
          </span>
        {/if}
      </span>
      <span class="flex flex-1 flex-col items-center gap-1.5 p-2 text-center">
        <span class="line-clamp-2 text-xs leading-tight font-semibold">{d.name}</span>
        {#if kind}<Badge
            text={kind()}
            tone={BadgeTone.ACCENT}
            dot={false}
            hint={benefit?.kind === BenefitKind.IN_GAME_ITEM ? t.badge_hint_item() : undefined}
          />{/if}
        <span class="mt-auto flex w-full flex-col items-center gap-1">
          {#if state === RewardState.IN_PROGRESS}
            <Progress
              value={percent(d.current_minutes, d.required_minutes)}
              class="h-1.5 w-full"
              aria-label={t.campaign_progress({
                current: String(d.current_minutes),
                required: String(d.required_minutes),
              })}
            />
            <small class="text-tiny text-muted-foreground tabular-nums"
              ><RichText
                text={t.campaign_progress({
                  current: strong(d.current_minutes),
                  required: strong(d.required_minutes),
                })}
              /></small
            >
          {:else if state === RewardState.READY && d.instance_id && onclaim}
            {@const instanceId = d.instance_id}
            <ActionButton
              icon={IconName.GIFT}
              label={t.reward_claim()}
              busyLabel={t.campaign_claiming()}
              busy={claiming === instanceId}
              disabled={claiming !== null}
              variant={ButtonVariant.PRIMARY}
              size={ButtonSize.SM}
              type={ButtonType.BUTTON}
              class="w-full"
              onclick={() => claim(instanceId)}
            />
          {:else if state === RewardState.TO_EARN || state === RewardState.SKIPPED}
            <small class="text-tiny text-muted-foreground"><RichText text={requirement(d)} /></small
            >
          {/if}
        </span>
      </span>
    </div>
  {/each}
</div>
