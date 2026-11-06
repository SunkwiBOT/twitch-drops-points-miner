<script lang="ts">
  import { Dialog as DialogPrimitive } from "bits-ui";

  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/state.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import { ButtonSize, ButtonVariant, IconName, MenuAlign } from "../ui/ui.constants";
  import ShellControls from "./ShellControls.svelte";
  import Sidebar from "./Sidebar.svelte";

  let content = $state<HTMLElement | null>(null);

  // ! The drawer opens on the page being shown, not on the logo's link to GitHub.
  const focusCurrent = (event: Event): void => {
    event.preventDefault();
    (content?.querySelector<HTMLElement>("[aria-current=page]") ?? content)?.focus();
  };
</script>

<DialogPrimitive.Root open={app.navDrawer} onOpenChange={(next) => (app.navDrawer = next)}>
  <DialogPrimitive.Portal>
    <DialogPrimitive.Overlay
      class="fixed inset-0 z-50 bg-black/50 duration-200 data-closed:animate-out data-closed:fade-out-0 data-open:animate-in data-open:fade-in-0"
    />
    <DialogPrimitive.Content
      bind:ref={content}
      tabindex={-1}
      onOpenAutoFocus={focusCurrent}
      class="fixed inset-y-0 left-0 z-50 grid w-72 max-w-[85vw] grid-rows-[minmax(0,1fr)] bg-sidebar pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)] pl-[env(safe-area-inset-left)] shadow-2xl outline-none duration-200 data-closed:animate-out data-closed:slide-out-to-left data-open:animate-in data-open:slide-in-from-left"
    >
      <DialogPrimitive.Title class="sr-only">{t.nav_main()}</DialogPrimitive.Title>
      <Sidebar controls={app.snapshot ? controls : undefined} />
      <div class="absolute top-[calc(env(safe-area-inset-top)+0.75rem)] right-2">
        <IconButton
          icon={IconName.X}
          label={t.common_close()}
          busy={false}
          disabled={false}
          variant={ButtonVariant.GHOST}
          size={ButtonSize.MD}
          onclick={() => (app.navDrawer = false)}
        />
      </div>
    </DialogPrimitive.Content>
  </DialogPrimitive.Portal>
</DialogPrimitive.Root>

{#snippet controls()}
  {#if app.snapshot}<ShellControls snapshot={app.snapshot} align={MenuAlign.LEFT} />{/if}
{/snippet}
