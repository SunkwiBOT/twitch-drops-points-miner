<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { cn } from "$lib/utils";
  import type { SpamRewardType } from "../../../lib/bindings/SpamRewardType";
  import { number } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { RewardStatus, SPAM_MAX_COUNT, SPAM_MESSAGE_MAX } from "../../../lib/spam/spam.constants";
  import { burstKey, burstProgress, runningBursts } from "../../../lib/spam/spam.svelte";
  import { countdown, maxCount, UNAVAILABLE_BADGES } from "../../../lib/spam/spam.utils";
  import Badge from "../../ui/Badge.svelte";
  import Icon from "../../ui/Icon.svelte";
  import Spinner from "../../ui/Spinner.svelte";
  import Tip from "../../ui/Tip.svelte";
  import { IconName } from "../../ui/ui.constants";
  import QuantityStepper from "./QuantityStepper.svelte";

  type SpamRewardCardPropsType = {
    accountId: string;
    login: string;
    reward: SpamRewardType;
    status: RewardStatus;
    balance: number;
    free: boolean;
    now: number;
    onredeem: (count: number, text: string) => Promise<number>;
  };

  let { accountId, login, reward, status, balance, free, now, onredeem }: SpamRewardCardPropsType =
    $props();

  const uid = $props.id();

  let count = $state(1);
  let message = $state("");

  const running = $derived(runningBursts[burstKey(accountId, login, reward.id)]);
  const sending = $derived(
    running ? { done: burstProgress[running.id] ?? 0, total: running.total } : null,
  );

  const badge = $derived(UNAVAILABLE_BADGES[status]);
  // ! On the account's own channel Twitch takes no points: only the 25 cap holds.
  const max = $derived(free ? SPAM_MAX_COUNT : maxCount(reward.cost, balance));
  const quantity = $derived(Math.min(count, Math.max(1, max)));
  const cooling = $derived(status === RewardStatus.COOLDOWN);
  const missing = $derived(max === 0 && !sending ? reward.cost - balance : 0);
  const needsText = $derived(reward.is_user_input_required && message.trim() === "");
  const blocked = $derived(sending !== null || cooling || max === 0);
  const cooldownLeft = $derived(
    reward.cooldown_expires_at ? Date.parse(reward.cooldown_expires_at) - now : 0,
  );

  const redeem = async (): Promise<void> => {
    if (blocked || needsText) {
      return;
    }

    const sent = await onredeem(quantity, message.trim());

    if (sent > 0) {
      message = "";
    }
  };
</script>

<article
  class="relative flex flex-col overflow-hidden rounded-md bg-background ring-1 ring-border"
  aria-labelledby="{uid}-title"
>
  <!-- ! The streamer's own color, as Twitch paints it: the default image is
  white and only reads on it. -->
  <div
    class={cn(
      "relative grid h-20 place-items-center",
      !reward.background_color && "bg-muted",
      badge && "opacity-50 grayscale",
    )}
    style:background-color={reward.background_color}
  >
    <img
      class={cn("size-12 object-contain", cooling && "opacity-30")}
      src={reward.image_url}
      alt=""
      loading="lazy"
    />
    {#if cooling}
      <span
        class="absolute top-1/2 left-1/2 inline-flex -translate-1/2 items-center gap-1 rounded-md bg-background/90 px-1.5 py-0.5 text-xs font-semibold whitespace-nowrap tabular-nums"
        ><Icon name={IconName.CLOCK} size={12} />{t.spam_reward_cooldown({
          time: countdown(cooldownLeft),
        })}</span
      >
    {/if}
  </div>
  {#if reward.prompt}
    {@const prompt = reward.prompt}
    <span class="absolute top-1.5 right-1.5">
      <Tip text={prompt}>
        {#snippet children(props)}
          <button
            {...props}
            type="button"
            class="grid size-6 place-items-center rounded-md bg-background/85 text-muted-foreground transition-colors hover:bg-background hover:text-foreground"
            aria-label={t.spam_description({ text: prompt })}
            ><Icon name={IconName.INFO} size={14} /></button
          >
        {/snippet}
      </Tip>
    </span>
  {/if}
  <div class="flex flex-1 flex-col gap-2 p-2.5">
    <h4 id="{uid}-title" class="line-clamp-2 text-hint leading-tight font-semibold">
      {reward.title}
    </h4>
    <div class="flex min-h-8 items-center justify-between gap-2">
      <span class="inline-flex items-center gap-1 text-hint font-semibold tabular-nums"
        ><span class="text-link"><Icon name={IconName.CHANNEL_POINTS} size={13} /></span>{number(
          reward.cost,
        )}</span
      >
      {#if badge}
        <Badge
          text={badge.label()}
          tone={badge.tone}
          dot={false}
          icon={badge.icon}
          hint={badge.hint()}
        />
      {:else}
        <QuantityStepper
          value={quantity}
          {max}
          disabled={blocked}
          onchange={(value) => (count = value)}
        />
      {/if}
    </div>
    {#if !badge}
      {#if reward.is_user_input_required}
        <Input
          class="h-8 text-hint"
          placeholder={t.spam_message_placeholder()}
          aria-label={t.spam_message_label({ reward: reward.title })}
          maxlength={SPAM_MESSAGE_MAX}
          disabled={blocked}
          bind:value={message}
          onkeydown={(event) => {
            if (event.key === "Enter" && !event.isComposing) {
              void redeem();
            }
          }}
        />
      {/if}
      <span class="sr-only" aria-live="polite"
        >{sending ? t.spam_redeeming({ done: sending.done, total: sending.total }) : ""}</span
      >
      <span class="mt-auto">
        {#snippet button(props: Record<string, unknown>)}
          <Button
            {...props}
            class="w-full gap-2 aria-disabled:opacity-50"
            disabled={blocked}
            aria-disabled={needsText || undefined}
            aria-busy={sending !== null}
            onclick={redeem}
          >
            {#if sending}
              <Spinner size={14} />
              {t.spam_redeeming({ done: sending.done, total: sending.total })}
            {:else}
              {t.spam_redeem()}
              {#if !free}<span
                  class="inline-flex items-center gap-1 border-l border-primary-foreground/40 pl-2 font-semibold tabular-nums"
                  ><Icon name={IconName.CHANNEL_POINTS} size={13} />{number(
                    reward.cost * quantity,
                  )}</span
                >{/if}
            {/if}
          </Button>
        {/snippet}
        {#if missing > 0}
          <p
            class="flex h-8 items-center justify-center gap-1 rounded-lg bg-muted text-hint text-muted-foreground tabular-nums"
          >
            {t.spam_missing()}<span class="text-link"
              ><Icon name={IconName.CHANNEL_POINTS} size={13} /></span
            ><span class="font-semibold text-foreground">{number(missing)}</span>
          </p>
        {:else if needsText && !blocked}
          <Tip text={t.spam_message_needed()}>
            {#snippet children(props)}{@render button(props)}{/snippet}
          </Tip>
        {:else}
          {@render button({})}
        {/if}
      </span>
    {/if}
  </div>
</article>
