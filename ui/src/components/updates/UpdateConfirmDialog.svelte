<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/state.svelte";
  import { strong } from "../../lib/text.utils";
  import { UpdateStage } from "../../lib/updates/updates.constants";
  import {
    availableVersion,
    installStatus,
    installUpdate,
    updates,
  } from "../../lib/updates/updates.svelte";
  import { megabytes } from "../../lib/updates/updates.utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import ProgressLine from "../ui/ProgressLine.svelte";
  import RichText from "../ui/RichText.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  const version = $derived(availableVersion() ?? "");
  const busy = $derived(updates.stage !== UpdateStage.IDLE);
  const status = $derived(installStatus(version));

  const download = $derived.by(() => {
    const progress = updates.progress;

    return updates.stage === UpdateStage.DOWNLOADING && progress?.total
      ? {
          done: progress.downloaded,
          total: progress.total,
          label: `${megabytes(progress.downloaded)} / ${megabytes(progress.total)}`,
        }
      : null;
  });

  const spoken = $derived.by((): string => {
    if (updates.stage === UpdateStage.INSTALLING) {
      return t.updates_restarting({ version });
    }

    return updates.stage === UpdateStage.DOWNLOADING ? t.updates_downloading({ version }) : "";
  });

  const close = (): void => {
    updates.confirming = false;
  };
</script>

<Dialog
  open={updates.confirming && version !== ""}
  title={t.updates_confirm_title({ version })}
  wide={false}
  closable={!busy}
  closeLabel={t.common_close()}
  onclose={close}
>
  <div class="flex flex-col">
    <p class="text-hint text-muted-foreground">
      <RichText
        text={t.updates_confirm_text({
          version: strong(version),
          current: strong(app.snapshot?.version ?? ""),
        })}
      />
    </p>
    <p class="sr-only" aria-live="polite">{spoken}</p>
    <div>
      {#if status}
        <div class="mt-3 flex flex-col gap-2">
          <p class="text-hint"><RichText text={status} /></p>
          {#if download}
            <ProgressLine
              current={download.done}
              required={download.total}
              label={download.label}
              stacked={false}
            />
          {/if}
        </div>
      {/if}
    </div>
  </div>

  {#snippet footer()}
    <TextButton
      label={t.updates_later()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={busy}
      onclick={close}
    />
    <ActionButton
      icon={IconName.IMPORT}
      label={t.updates_install()}
      busyLabel={t.updates_installing()}
      {busy}
      disabled={false}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={installUpdate}
    />
  {/snippet}
</Dialog>
