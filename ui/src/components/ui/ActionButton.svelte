<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import RichText from "./RichText.svelte";
  import Spinner from "./Spinner.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANT_CLASSES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
  } from "./ui.constants";

  type ActionButtonPropsType = {
    icon: IconName;
    label: string;
    busyLabel: string;
    busy: boolean;
    disabled: boolean;
    variant: ButtonVariant;
    size: ButtonSize;
    type: ButtonType;
    class?: string;
    onclick?: () => unknown;
  };

  let {
    icon,
    label,
    busyLabel,
    busy,
    disabled,
    variant,
    size,
    type,
    class: className,
    onclick,
  }: ActionButtonPropsType = $props();

  const iconSize = $derived(size === ButtonSize.SM ? 13 : 14);
</script>

<Button
  variant={BUTTON_VARIANTS[variant]}
  size={BUTTON_SIZES[size]}
  class={cn(BUTTON_VARIANT_CLASSES[variant], className)}
  {type}
  disabled={disabled || busy}
  aria-busy={busy}
  {onclick}
>
  {#if busy}<Spinner size={iconSize} />{:else}<Icon name={icon} size={iconSize} />{/if}
  <span class="truncate"><RichText text={busy ? busyLabel : label} /></span>
</Button>
