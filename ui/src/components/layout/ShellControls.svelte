<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { signOut } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import { ErrorApiMessage } from "../../lib/bindings/ErrorApiMessage";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { Theme } from "../../lib/bindings/Theme";
  import { VaultMode } from "../../lib/bindings/VaultMode";
  import { describeError, toApiError } from "../../lib/errors/errors.utils";
  import { Language, LANGUAGE_NAMES, RESOLVED_LANGUAGES } from "../../lib/i18n/i18n.constants";
  import { locale, t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import { Page, ToastKind } from "../../lib/state/state.constants";
  import {
    app,
    navigate,
    run,
    sessionEnded,
    setPreference,
    toast,
    toggleTheme,
  } from "../../lib/state/state.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import Select from "../ui/Select.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName, MenuAlign } from "../ui/ui.constants";

  type ShellControlsPropsType = { snapshot: SnapshotType; align: MenuAlign };

  let { snapshot, align }: ShellControlsPropsType = $props();

  const languages = $derived([
    { value: Language.AUTO, label: t.language_auto() },
    ...RESOLVED_LANGUAGES.map((tag) => ({ value: tag, label: LANGUAGE_NAMES[tag] })),
  ]);

  const passwordSet = $derived(!isTauri() || snapshot.vault.mode === VaultMode.PASSPHRASE);

  const padlockLabel = $derived.by((): string => {
    if (!isTauri()) {
      return t.lock_dashboard();
    }

    return passwordSet ? t.lock_now() : t.lock_set_password();
  });

  let locking = $state(false);
  let confirmingLock = $state(false);

  const lock = async (): Promise<void> => {
    confirmingLock = false;
    locking = true;

    if (isTauri()) {
      await run({ command: CommandName.LOCK });
      locking = false;

      return;
    }

    try {
      await signOut();
    } catch (e: any) {
      if (toApiError(e).code !== ErrorApiMessage.UNAUTHORIZED) {
        toast(ToastKind.ERROR, describeError(e));
        locking = false;

        return;
      }
    }

    sessionEnded();
  };

  const askLock = (): void => {
    if (overlay.unsaved) {
      confirmingLock = true;

      return;
    }

    void lock();
  };

  const showPasswordCard = (): void => {
    app.revealPassword = true;
    navigate(Page.SETTINGS);
  };
</script>

<div class="min-w-28 max-w-56 grow-[100] basis-0">
  <Select
    value={locale.preference}
    options={languages}
    label={t.topbar_language()}
    {align}
    onchange={(language) => setPreference({ language })}
  />
</div>
<IconButton
  icon={snapshot.settings.theme === Theme.LIGHT ? IconName.MOON : IconName.SUN}
  label={t.topbar_theme()}
  busy={false}
  disabled={false}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  onclick={toggleTheme}
/>
<IconButton
  icon={IconName.LOCK}
  label={padlockLabel}
  busy={locking}
  disabled={false}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  onclick={passwordSet ? askLock : showPasswordCard}
/>

<Dialog
  open={confirmingLock}
  title={t.lock_unsaved_title()}
  wide={false}
  closable={true}
  closeLabel={t.common_close()}
  onclose={() => (confirmingLock = false)}
>
  <p class="text-hint text-muted-foreground">{t.lock_unsaved_text()}</p>

  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => (confirmingLock = false)}
    />
    <TextButton
      label={t.lock_anyway()}
      icon={IconName.LOCK}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={lock}
    />
  {/snippet}
</Dialog>
