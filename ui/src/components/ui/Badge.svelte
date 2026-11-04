<script lang="ts">
  import { Badge as BadgeRoot } from "$lib/components/ui/badge";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import StatusDot from "./StatusDot.svelte";
  import Tip from "./Tip.svelte";
  import {
    BADGE_TONE_CLASSES,
    BADGE_VARIANTS,
    BadgeTone,
    BadgeVariant,
    IconName,
  } from "./ui.constants";

  type BadgePropsType = {
    text: string;
    tone: BadgeTone;
    dot: boolean;
    icon?: IconName;
    hint?: string;
  };

  let { text, tone, dot, icon, hint }: BadgePropsType = $props();
</script>

{#snippet content()}
  {#if dot}<StatusDot {tone} />{/if}
  {#if icon}<Icon name={icon} size={12} />{/if}
  {text}
{/snippet}

{#if hint}
  <Tip text={hint}>
    {#snippet children(props)}
      <BadgeRoot
        {...props}
        variant={BADGE_VARIANTS[BadgeVariant.TONE]}
        class={cn(BADGE_TONE_CLASSES[tone], "cursor-help")}
      >
        {@render content()}
      </BadgeRoot>
    {/snippet}
  </Tip>
{:else}
  <BadgeRoot variant={BADGE_VARIANTS[BadgeVariant.TONE]} class={cn(BADGE_TONE_CLASSES[tone])}>
    {@render content()}
  </BadgeRoot>
{/if}
