<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { Card } from "$lib/components/ui/card";
  import ActionButton from "../ui/ActionButton.svelte";
  import Icon from "../ui/Icon.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  type UnsavedBarPropsType = {
    saving: boolean;
    oncancel: () => void;
    onsave: () => void;
  };

  let { saving, oncancel, onsave }: UnsavedBarPropsType = $props();

  let bar = $state<HTMLElement | null>(null);

  $effect(() => {
    overlay.unsaved = true;

    return () => {
      overlay.unsaved = false;
    };
  });

  $effect(() => {
    if (!bar) {
      return;
    }

    const element = bar;

    const measure = (): void => {
      overlay.bottom = Math.max(0, window.innerHeight - element.getBoundingClientRect().top);
    };

    const observer = new ResizeObserver(measure);

    observer.observe(element);
    window.addEventListener("resize", measure);
    measure();

    return () => {
      observer.disconnect();
      window.removeEventListener("resize", measure);
      overlay.bottom = 0;
    };
  });
</script>

<Card
  bind:ref={bar}
  class="sticky bottom-0 z-10 mt-auto flex-row flex-wrap items-center justify-between gap-3 bg-unsaved px-4 py-3 shadow-lg ring-2 ring-primary"
>
  <span class="flex items-center gap-2 text-hint font-medium text-foreground">
    <span class="text-link"><Icon name={IconName.ALERT} size={16} /></span>
    {t.settings_unsaved()}
  </span>
  <div class="flex gap-2">
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={saving}
      onclick={oncancel}
    />
    <ActionButton
      icon={IconName.CHECK}
      label={t.common_save()}
      busyLabel={t.common_saving()}
      busy={saving}
      disabled={false}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={onsave}
    />
  </div>
</Card>
