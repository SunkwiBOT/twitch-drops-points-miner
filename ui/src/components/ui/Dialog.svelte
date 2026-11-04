<script lang="ts">
  import type { Snippet } from "svelte";

  import {
    Dialog as DialogRoot,
    DialogContent,
    DialogHeader,
    DialogTitle,
  } from "$lib/components/ui/dialog";
  import { cn } from "$lib/utils";
  import DialogActions from "./DialogActions.svelte";
  import IconButton from "./IconButton.svelte";
  import { ButtonSize, ButtonVariant, IconName } from "./ui.constants";

  type DialogPropsType = {
    open: boolean;
    title: string;
    wide: boolean;
    closable: boolean;
    closeLabel: string;
    onclose: () => void;
    children: Snippet;
    lead?: Snippet;
    footer?: Snippet;
  };

  let {
    open,
    title,
    wide,
    closable,
    closeLabel,
    onclose,
    children,
    lead,
    footer,
  }: DialogPropsType = $props();

  const behavior = $derived(closable ? "close" : "ignore");

  let content = $state<HTMLElement | null>(null);

  // ! The close cross is never focused first: it would show selected, its tip open.
  const focusFirst = (event: Event): void => {
    event.preventDefault();

    const field = content?.querySelector<HTMLElement>(
      "input:not([type=hidden]):not([disabled]), textarea:not([disabled])",
    );

    (field ?? content)?.focus();
  };
</script>

<DialogRoot
  {open}
  onOpenChange={(next) => {
    if (!next && closable) {
      onclose();
    }
  }}
>
  <DialogContent
    bind:ref={content}
    tabindex={-1}
    onOpenAutoFocus={focusFirst}
    showCloseButton={false}
    interactOutsideBehavior={behavior}
    escapeKeydownBehavior={behavior}
    class={cn(
      // ! One column no wider than the dialog: a long unbroken line (an address)
      // would otherwise widen the grid track and push content past the edge.
      "max-h-[90vh] grid-cols-[minmax(0,1fr)] gap-4 overflow-y-auto p-5 outline-none",
      wide ? "sm:max-w-3xl" : "sm:max-w-lg",
    )}
  >
    <DialogHeader>
      <DialogTitle class="flex items-center gap-2.5 pr-8 text-base font-semibold">
        {#if lead}{@render lead()}{/if}
        <span class="truncate">{title}</span>
      </DialogTitle>
    </DialogHeader>
    <div class="absolute top-[18px] right-4">
      <IconButton
        icon={IconName.X}
        label={closeLabel}
        busy={false}
        disabled={!closable}
        variant={ButtonVariant.GHOST}
        size={ButtonSize.SM}
        onclick={onclose}
      />
    </div>
    {@render children()}
    {#if footer}
      <DialogActions>{@render footer()}</DialogActions>
    {/if}
  </DialogContent>
</DialogRoot>
