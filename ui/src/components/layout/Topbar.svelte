<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { signOut } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import { ErrorApiMessage } from "../../lib/bindings/ErrorApiMessage";
  import { Theme } from "../../lib/bindings/Theme";
  import { VaultMode } from "../../lib/bindings/VaultMode";
  import { describeError, toApiError } from "../../lib/errors/errors.utils";
  import { Language, LANGUAGE_NAMES, RESOLVED_LANGUAGES } from "../../lib/i18n/i18n.constants";
  import { locale, t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import { Page, ToastKind } from "../../lib/state/state.constants";
  import {
    account,
    app,
    isAccountPage,
    navigate,
    run,
    sessionEnded,
    setPreference,
    toast,
    toggleTheme,
  } from "../../lib/state/state.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import Avatar from "../ui/Avatar.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import AccountStatusBadges from "../ui/AccountStatusBadges.svelte";
  import PauseButton from "../ui/PauseButton.svelte";
  import Select from "../ui/Select.svelte";
  import {
    AvatarSize,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    MenuAlign,
  } from "../ui/ui.constants";
  import UpdateBadge from "../updates/UpdateBadge.svelte";
  import UpdateConfirmDialog from "../updates/UpdateConfirmDialog.svelte";

  type TopbarPropsType = { title: string; section: string | null };

  let { title, section }: TopbarPropsType = $props();

  const current = $derived(isAccountPage(app.page) ? account(app.selectedAccount) : null);

  const languages = $derived([
    { value: Language.AUTO, label: t.language_auto() },
    ...RESOLVED_LANGUAGES.map((tag) => ({ value: tag, label: LANGUAGE_NAMES[tag] })),
  ]);

  let refreshing = $state(false);

  const refreshAccount = async (id: string): Promise<void> => {
    refreshing = true;
    await run({ command: CommandName.REFRESH_ACCOUNT, account_id: id });
    refreshing = false;
  };

  const passwordSet = $derived(!isTauri() || app.snapshot?.vault.mode === VaultMode.PASSPHRASE);

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

<header class="@container flex min-h-14 items-center gap-2.5 border-b bg-card px-6 py-2.5">
  {#if current}
    <Avatar url={current.avatar_url} name={current.display_name} size={AvatarSize.SM} />
  {/if}
  <h1 class="flex min-w-0 text-xl font-semibold">
    <span class="min-w-0 truncate">{title}</span>
    {#if section}<span class="min-w-0 shrink-[100] truncate ps-1.5 @max-[45rem]:hidden"
        >· {section}</span
      >{/if}
  </h1>
  <span class="flex-1"></span>
  <UpdateBadge />
  {#if current}
    <AccountStatusBadges account={current} />
    {#if current.status === AccountStatus.LOGIN_REQUIRED}
      <TextButton
        label={t.accounts_login_again()}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (app.loginDialog = true)}
      />
    {:else}
      <PauseButton account={current} size={ButtonSize.MD} />
      <IconButton
        icon={IconName.REFRESH}
        label={t.account_refresh()}
        busy={refreshing}
        disabled={!current.settings.enabled}
        variant={ButtonVariant.DEFAULT}
        size={ButtonSize.MD}
        onclick={() => refreshAccount(current.id)}
      />
    {/if}
  {/if}
  {#if app.snapshot}
    <div class="min-w-28 max-w-56 grow-[100] basis-0">
      <Select
        value={locale.preference}
        options={languages}
        label={t.topbar_language()}
        align={MenuAlign.RIGHT}
        onchange={(language) => setPreference({ language })}
      />
    </div>
    <IconButton
      icon={app.snapshot.settings.theme === Theme.LIGHT ? IconName.MOON : IconName.SUN}
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
  {/if}
</header>

<UpdateConfirmDialog />

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
