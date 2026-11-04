<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import { openLink } from "../../lib/state/state.svelte";
  import Icon from "./Icon.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANT_CLASSES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonVariant,
    IconName,
  } from "./ui.constants";

  type ExternalButtonPropsType = {
    url: string;
    label: string;
    variant: ButtonVariant;
    size: ButtonSize;
    icon?: IconName;
    class?: string;
  };

  let { url, label, variant, size, icon, class: className }: ExternalButtonPropsType = $props();

  const iconSize = $derived(size === ButtonSize.SM ? 13 : 14);
</script>

<Button
  href={url}
  rel="noreferrer"
  variant={BUTTON_VARIANTS[variant]}
  size={BUTTON_SIZES[size]}
  class={cn("no-underline hover:no-underline", BUTTON_VARIANT_CLASSES[variant], className)}
  onclick={(event: MouseEvent) => {
    event.preventDefault();
    void openLink(url);
  }}
>
  {#if icon}<Icon name={icon} size={iconSize} />{/if}
  <span class="truncate">{label}</span>
  <span class="shrink-0 opacity-70"><Icon name={IconName.EXTERNAL} size={iconSize - 1} /></span>
</Button>
