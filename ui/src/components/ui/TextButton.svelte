<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANT_CLASSES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
  } from "./ui.constants";

  type TextButtonPropsType = {
    label: string;
    icon?: IconName;
    variant: ButtonVariant;
    size: ButtonSize;
    type: ButtonType;
    disabled: boolean;
    class?: string;
    onclick?: () => unknown;
  };

  let {
    label,
    icon,
    variant,
    size,
    type,
    disabled,
    class: className,
    onclick,
  }: TextButtonPropsType = $props();

  const iconSize = $derived(size === ButtonSize.SM ? 13 : 14);
</script>

<Button
  variant={BUTTON_VARIANTS[variant]}
  size={BUTTON_SIZES[size]}
  class={cn(BUTTON_VARIANT_CLASSES[variant], className)}
  {type}
  {disabled}
  {onclick}
>
  {#if icon}<Icon name={icon} size={iconSize} />{/if}
  {label}
</Button>
