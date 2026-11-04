<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { cn } from "$lib/utils";
  import { isTauri } from "../../lib/api/tauri";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import { strong } from "../../lib/text.utils";
  import { UPDATE_BADGE_CLASSES } from "../../lib/updates/updates.constants";
  import { availableVersion, showUpdate } from "../../lib/updates/updates.svelte";
  import Icon from "../ui/Icon.svelte";
  import RichText from "../ui/RichText.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    MENU_EDGE_GAP,
  } from "../ui/ui.constants";
  import ImageUpdateCommands from "./ImageUpdateCommands.svelte";

  const version = $derived(availableVersion());

  let open = $state(false);
  let content = $state<HTMLElement | null>(null);

  // ! The popover itself takes the focus, not its copy button: that button's
  // tip would open at once and take the first Escape.
  const focusContent = (event: Event): void => {
    event.preventDefault();
    content?.focus();
  };
</script>

{#snippet label(text: string)}
  <Icon name={IconName.IMPORT} size={14} />
  {text}
{/snippet}

{#if version}
  {#if isTauri()}
    <Button
      variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
      size={BUTTON_SIZES[ButtonSize.MD]}
      class={cn("shrink-0", UPDATE_BADGE_CLASSES)}
      type={ButtonType.BUTTON}
      onclick={showUpdate}
    >
      {@render label(t.updates_badge({ version }))}
    </Button>
  {:else}
    <Popover bind:open>
      <PopoverTrigger>
        {#snippet child({ props })}
          <Button
            {...props}
            variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
            size={BUTTON_SIZES[ButtonSize.MD]}
            class={cn("shrink-0", UPDATE_BADGE_CLASSES)}
            type={ButtonType.BUTTON}
            aria-expanded={open}
          >
            {@render label(t.updates_badge({ version }))}
          </Button>
        {/snippet}
      </PopoverTrigger>
      <PopoverContent
        bind:ref={content}
        onOpenAutoFocus={focusContent}
        class="w-88 gap-3 p-4"
        align="end"
        collisionPadding={{
          top: MENU_EDGE_GAP,
          right: MENU_EDGE_GAP,
          left: MENU_EDGE_GAP,
          bottom: overlay.bottom + MENU_EDGE_GAP,
        }}
      >
        <p class="text-hint font-medium">
          <RichText text={t.updates_available({ version: strong(version) })} />
        </p>
        <ImageUpdateCommands />
      </PopoverContent>
    </Popover>
  {/if}
{/if}
