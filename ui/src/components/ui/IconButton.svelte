<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import Tip from "./Tip.svelte";
  import {
    BUTTON_VARIANT_CLASSES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonVariant,
    ICON_BUTTON_SIZES,
    IconName,
  } from "./ui.constants";

  type IconButtonPropsType = {
    icon: IconName;
    label: string;
    busy: boolean;
    disabled: boolean;
    variant: ButtonVariant;
    size: ButtonSize;
    onclick: () => unknown;
  };

  let { icon, label, busy, disabled, variant, size, onclick }: IconButtonPropsType = $props();

  const iconSize = $derived(size === ButtonSize.SM ? 14 : 16);
</script>

<Tip text={label}>
  {#snippet children(props)}
    <Button
      {...props}
      variant={BUTTON_VARIANTS[variant]}
      size={ICON_BUTTON_SIZES[size]}
      class={cn(BUTTON_VARIANT_CLASSES[variant])}
      type="button"
      aria-label={label}
      disabled={disabled || busy}
      aria-busy={busy}
      {onclick}
    >
      {#if busy}<Spinner size={iconSize} />{:else}<Icon name={icon} size={iconSize} />{/if}
    </Button>
  {/snippet}
</Tip>
