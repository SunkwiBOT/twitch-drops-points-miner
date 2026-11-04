<script lang="ts">
  // ! Edited after generation: the theme comes from the app's settings
  // (Theme.LIGHT becomes sonner's "light"), not from mode-watcher, which the
  // app does not use: it sets data-theme itself. The font is set inline:
  // sonner's own unlayered rule names the system fonts and beats any utility.

  import { Theme } from "../../../bindings/Theme";
  import { app } from "../../../state/state.svelte";
  import { Toaster as Sonner, type ToasterProps as SonnerProps } from "svelte-sonner";
  import Loader2Icon from "@lucide/svelte/icons/loader-2";
  import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
  import OctagonXIcon from "@lucide/svelte/icons/octagon-x";
  import InfoIcon from "@lucide/svelte/icons/info";
  import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";

  let { ...restProps }: SonnerProps = $props();
</script>

<Sonner
  theme={app.snapshot?.settings.theme === Theme.LIGHT ? "light" : "dark"}
  class="toaster group"
  style="--normal-bg: var(--color-popover); --normal-text: var(--color-popover-foreground); --normal-border: var(--color-border); font-family: var(--font-sans);"
  toastOptions={{
    classes: {
      toast: "cn-toast",
    },
  }}
  {...restProps}
>
  {#snippet loadingIcon()}
    <Loader2Icon class="size-4 animate-spin" />
  {/snippet}
  {#snippet successIcon()}
    <CircleCheckIcon class="size-4" />
  {/snippet}
  {#snippet errorIcon()}
    <OctagonXIcon class="size-4" />
  {/snippet}
  {#snippet infoIcon()}
    <InfoIcon class="size-4" />
  {/snippet}
  {#snippet warningIcon()}
    <TriangleAlertIcon class="size-4" />
  {/snippet}
</Sonner>
