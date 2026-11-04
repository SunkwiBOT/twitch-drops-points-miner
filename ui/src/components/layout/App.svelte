<script lang="ts">
  import { TooltipProvider } from "$lib/components/ui/tooltip";
  import { isTauri } from "../../lib/api/tauri";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { noteActivity, noteReturn, watchSession } from "../../lib/state/autolock";
  import { closeBulk } from "../../lib/state/bulk.svelte";
  import { Page } from "../../lib/state/state.constants";
  import {
    account,
    app,
    isAccountPage,
    isDashboardLocked,
    isVaultLocked,
    navigate,
  } from "../../lib/state/state.svelte";
  import AccountDrops from "../account/AccountDrops.svelte";
  import AccountHistory from "../account/AccountHistory.svelte";
  import AccountInventory from "../account/AccountInventory.svelte";
  import AccountOverview from "../account/AccountOverview.svelte";
  import AccountPoints from "../account/AccountPoints.svelte";
  import AccountSettings from "../account/AccountSettings.svelte";
  import AccountSpamPoints from "../account/AccountSpamPoints.svelte";
  import ApplyToAccountsDialog from "../accounts/ApplyToAccountsDialog.svelte";
  import LoginDialog from "../accounts/LoginDialog.svelte";
  import Catalogue from "../pages/Catalogue.svelte";
  import History from "../pages/History.svelte";
  import Home from "../pages/Home.svelte";
  import Logs from "../pages/Logs.svelte";
  import Settings from "../pages/Settings.svelte";
  import CloseDialog from "./CloseDialog.svelte";
  import FeedbackDialog from "../feedback/FeedbackDialog.svelte";
  import { APP_TITLE, LockKind } from "./layout.constants";
  import LockScreen from "./LockScreen.svelte";
  import Sidebar from "./Sidebar.svelte";
  import StartupProblem from "./StartupProblem.svelte";
  import Toasts from "./Toasts.svelte";
  import Topbar from "./Topbar.svelte";
  import { TOOLTIP_DELAY_MS } from "../ui/ui.constants";

  const current = $derived(account(app.selectedAccount));

  const dashboardLocked = $derived(isDashboardLocked());
  const vaultLocked = $derived(isVaultLocked());

  const title = $derived.by(() => {
    switch (app.page) {
      case Page.HOME:
        return t.home_title();
      case Page.CATALOGUE:
        return t.catalogue_title();
      case Page.HISTORY:
        return t.history_title();
      case Page.LOGS:
        return t.logs_title();
      case Page.SETTINGS:
        return t.settings_title();
      default:
        return current?.display_name ?? "";
    }
  });

  const section = $derived.by(() => {
    switch (app.page) {
      case Page.ACCOUNT_DROPS:
        return t.nav_account_drops();
      case Page.ACCOUNT_POINTS:
        return t.nav_account_points();
      case Page.ACCOUNT_SPAM:
        return t.nav_account_spam();
      case Page.ACCOUNT_INVENTORY:
        return t.nav_account_inventory();
      case Page.ACCOUNT_HISTORY:
        return t.nav_account_history();
      case Page.ACCOUNT_SETTINGS:
        return t.nav_account_settings();
      default:
        return null;
    }
  });

  $effect(() => {
    if (!isTauri()) {
      const full = section ? `${title} · ${section}` : title;

      document.title = full && !dashboardLocked ? `${full} · ${APP_TITLE}` : APP_TITLE;
    }
  });

  $effect(() => {
    if (dashboardLocked || vaultLocked) {
      closeBulk();
      app.loginDialog = false;
      app.feedbackDialog = false;
    }
  });

  $effect(() => {
    if (app.snapshot && !current && isAccountPage(app.page)) {
      navigate(Page.HOME);
    }
  });

  $effect(() => {
    watchSession(!isTauri() && app.auth.authorized, app.snapshot?.settings.auto_lock ?? null);
  });
</script>

<!-- ! The user's own activity keeps a dashboard session open; the page's
timers and its push channel never do. -->
<svelte:window
  onpointerdowncapture={noteActivity}
  onpointermovecapture={noteActivity}
  onkeydowncapture={noteActivity}
  onwheelcapture={noteActivity}
  ontouchstartcapture={noteActivity}
/>
<svelte:document onvisibilitychange={noteReturn} onscrollcapture={noteActivity} />

<TooltipProvider delayDuration={TOOLTIP_DELAY_MS}>
  {#if app.startupProblem}
    <StartupProblem problem={app.startupProblem} />
  {:else if !app.auth.checked}
    <div class="grid h-full w-full place-items-center bg-background">
      <p class="text-muted-foreground">{t.common_loading()}</p>
    </div>
  {:else if dashboardLocked}
    <LockScreen kind={LockKind.DASHBOARD} />
  {:else if vaultLocked}
    <LockScreen kind={LockKind.VAULT} />
  {:else}
    <div class="grid h-full w-full grid-cols-[248px_minmax(0,1fr)]">
      <Sidebar />
      <div class="flex h-full min-h-0 min-w-0 flex-col">
        <Topbar {title} {section} />
        {#if !app.snapshot}
          <div class="p-6"><p class="text-muted-foreground">{t.common_loading()}</p></div>
        {:else if app.page === Page.HOME}
          <Home snapshot={app.snapshot} />
        {:else if app.page === Page.CATALOGUE}
          <Catalogue snapshot={app.snapshot} />
        {:else if app.page === Page.HISTORY}
          <History snapshot={app.snapshot} />
        {:else if app.page === Page.LOGS}
          <Logs snapshot={app.snapshot} />
        {:else if app.page === Page.SETTINGS}
          <Settings snapshot={app.snapshot} />
        {:else if current}
          {#key current.id}
            {#if app.page === Page.ACCOUNT}
              <AccountOverview account={current} snapshot={app.snapshot} />
            {:else if app.page === Page.ACCOUNT_DROPS}
              <AccountDrops account={current} snapshot={app.snapshot} />
            {:else if app.page === Page.ACCOUNT_POINTS}
              <AccountPoints account={current} />
            {:else if app.page === Page.ACCOUNT_SPAM}
              <AccountSpamPoints account={current} snapshot={app.snapshot} />
            {:else if app.page === Page.ACCOUNT_INVENTORY}
              <AccountInventory account={current} />
            {:else if app.page === Page.ACCOUNT_HISTORY}
              <AccountHistory account={current} />
            {:else if app.page === Page.ACCOUNT_SETTINGS}
              <AccountSettings account={current} snapshot={app.snapshot} />
            {/if}
          {/key}
        {/if}
      </div>
    </div>
    <LoginDialog />
    <ApplyToAccountsDialog />
  {/if}

  <CloseDialog />
  <FeedbackDialog />
  <Toasts />
</TooltipProvider>
