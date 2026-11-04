<script lang="ts">
  import { isTauri } from "../../lib/api/tauri";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { strong } from "../../lib/text.utils";
  import { relativeTime } from "../../lib/time/time.utils";
  import { UpdateCheck, UpdateStage } from "../../lib/updates/updates.constants";
  import {
    askInstall,
    availableVersion,
    checkUpdates,
    installStatus,
    updates,
  } from "../../lib/updates/updates.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import RichText from "../ui/RichText.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";
  import ImageUpdateCommands from "./ImageUpdateCommands.svelte";

  type UpdatesPanelPropsType = { checkedAt: string | null };

  let { checkedAt }: UpdatesPanelPropsType = $props();

  const version = $derived(availableVersion());
  const current = $derived(!version && updates.check === UpdateCheck.CURRENT);

  const status = $derived.by((): string => {
    if (version) {
      return installStatus(version) ?? t.updates_available({ version: strong(version) });
    }

    return current ? t.updates_current() : "";
  });

  const lastChecked = $derived(
    checkedAt
      ? t.updates_last_checked({ time: strong(relativeTime(checkedAt, ticker.now)) })
      : t.updates_never_checked(),
  );
</script>

<div class="flex flex-col gap-1">
  <div aria-live="polite" class="flex flex-col gap-1 empty:hidden">
    {#if status}
      <p class="text-hint" class:text-success-foreground={current}>
        <RichText text={status} />
      </p>
    {/if}
    {#if updates.check === UpdateCheck.FAILED}
      <p class="text-hint text-destructive">{t.updates_check_failed()}</p>
    {/if}
  </div>
  <p class="text-hint text-muted-foreground"><RichText text={lastChecked} /></p>
</div>
{#if !isTauri() && version}
  <ImageUpdateCommands />
{/if}
<div class="flex flex-wrap gap-2">
  <ActionButton
    icon={IconName.REFRESH}
    label={t.updates_check_now()}
    busyLabel={t.updates_checking()}
    busy={updates.check === UpdateCheck.CHECKING}
    disabled={updates.stage !== UpdateStage.IDLE}
    variant={ButtonVariant.DEFAULT}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={checkUpdates}
  />
  {#if isTauri() && version}
    <ActionButton
      icon={IconName.IMPORT}
      label={t.updates_install()}
      busyLabel={t.updates_installing()}
      busy={updates.stage !== UpdateStage.IDLE}
      disabled={false}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={askInstall}
    />
  {/if}
</div>
