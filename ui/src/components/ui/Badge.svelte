<script lang="ts">
  import { on } from "svelte/events";

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
    class?: string;
  };

  let { text, tone, dot, icon, hint, class: className }: BadgePropsType = $props();

  let cut = $state(false);

  // ! scrollWidth is rounded: a label cut by less than a pixel would read as whole.
  const measure = (label: HTMLElement): void => {
    const range = document.createRange();
    const style = getComputedStyle(label);

    range.selectNodeContents(label);

    const room =
      label.getBoundingClientRect().width -
      parseFloat(style.paddingLeft) -
      parseFloat(style.paddingRight);

    cut = range.getBoundingClientRect().width > room + 0.01;
  };
</script>

{#snippet content()}
  {#if dot}<StatusDot {tone} />{/if}
  {#if icon}<Icon name={icon} size={12} />{/if}
  <!-- ! The padding leaves room for ink past the line box (Thai marks): only the badge clips it. -->
  <span
    class="-m-0.5 min-w-0 overflow-hidden p-0.5 leading-3.5 text-ellipsis"
    title={cut && !hint ? text : undefined}
    {@attach (label) => on(label, "pointerenter", () => measure(label))}>{text}</span
  >
{/snippet}

{#if hint}
  <Tip text={cut ? `${text} · ${hint}` : hint}>
    {#snippet children(props)}
      <BadgeRoot
        {...props}
        variant={BADGE_VARIANTS[BadgeVariant.TONE]}
        class={cn(BADGE_TONE_CLASSES[tone], "max-w-full cursor-help", className)}
      >
        {@render content()}
      </BadgeRoot>
    {/snippet}
  </Tip>
{:else}
  <BadgeRoot
    variant={BADGE_VARIANTS[BadgeVariant.TONE]}
    class={cn(BADGE_TONE_CLASSES[tone], "max-w-full", className)}
  >
    {@render content()}
  </BadgeRoot>
{/if}
