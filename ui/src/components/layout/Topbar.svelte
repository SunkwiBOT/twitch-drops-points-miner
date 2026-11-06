<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { account, app, isAccountPage, run } from "../../lib/state/state.svelte";
  import { wideLayout } from "../../lib/state/viewport.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import Avatar from "../ui/Avatar.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import AccountStatusBadges from "../ui/AccountStatusBadges.svelte";
  import PauseButton from "../ui/PauseButton.svelte";
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
  import ShellControls from "./ShellControls.svelte";

  type TopbarPropsType = { title: string; section: string | null };

  let { title, section }: TopbarPropsType = $props();

  const current = $derived(isAccountPage(app.page) ? account(app.selectedAccount) : null);

  let refreshing = $state(false);

  const refreshAccount = async (id: string): Promise<void> => {
    refreshing = true;
    await run({ command: CommandName.REFRESH_ACCOUNT, account_id: id });
    refreshing = false;
  };
</script>

<header
  class="@container flex min-h-14 items-center gap-2.5 border-b bg-card px-6 py-2.5 max-md:gap-2 max-md:px-3"
>
  {#if !wideLayout.current}
    <IconButton
      icon={IconName.MENU}
      label={t.nav_open_menu()}
      busy={false}
      disabled={false}
      variant={ButtonVariant.GHOST}
      size={ButtonSize.MD}
      onclick={() => (app.navDrawer = true)}
    />
  {/if}
  {#if current}
    <Avatar url={current.avatar_url} name={current.display_name} size={AvatarSize.SM} />
  {/if}
  <h1 class="flex min-w-0 text-xl font-semibold max-md:text-lg">
    <span class="min-w-0 truncate">{title}</span>
    {#if section}<span class="min-w-0 shrink-[100] truncate ps-1.5 @max-[45rem]:hidden"
        >· {section}</span
      >{/if}
  </h1>
  <span class="flex-1"></span>
  <UpdateBadge />
  {#if current}
    {#if wideLayout.current}<AccountStatusBadges account={current} />{/if}
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
  {#if app.snapshot && wideLayout.current}
    <ShellControls snapshot={app.snapshot} align={MenuAlign.RIGHT} />
  {/if}
</header>

<UpdateConfirmDialog />
