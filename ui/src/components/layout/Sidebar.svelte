<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import TextButton from "../ui/TextButton.svelte";
  import { cn } from "$lib/utils";
  import { STATUS_TONES } from "../../lib/accounts/accounts.utils";
  import { runningCampaigns } from "../../lib/catalogue/timeline.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { REPO_URL } from "../../lib/partners.constants";
  import { Page } from "../../lib/state/state.constants";
  import { followLink } from "../../lib/links/links.utils";
  import { app, isAccountPage, navigate, run } from "../../lib/state/state.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { number } from "../../lib/format.utils";
  import Avatar from "../ui/Avatar.svelte";
  import Badge from "../ui/Badge.svelte";
  import Icon from "../ui/Icon.svelte";
  import StatusDot from "../ui/StatusDot.svelte";
  import {
    AvatarSize,
    BadgeTone,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    KeyboardKey,
  } from "../ui/ui.constants";
  import { REORDER_HINT_ID } from "./layout.constants";
  import PoweredBy from "./PoweredBy.svelte";

  type NavItemType = { page: Page; icon: IconName; label: string; count?: number };

  const accounts = $derived(app.snapshot?.accounts ?? []);

  let dragging = $state<string | null>(null);
  let dragOver = $state<string | null>(null);

  const moveAccount = (id: string, target: string): void => {
    const order = accounts.map((a) => a.id).filter((x) => x !== id);
    const at = order.indexOf(target);
    const from = accounts.findIndex((a) => a.id === id);
    const before = accounts.findIndex((a) => a.id === target);

    order.splice(from < before ? at + 1 : at, 0, id);
    void run({ command: CommandName.REORDER_ACCOUNTS, account_ids: order });
  };

  const inAccount = $derived(isAccountPage(app.page));

  const main = $derived<NavItemType[]>([
    { page: Page.HOME, icon: IconName.HOME, label: t.nav_home() },
    {
      page: Page.CATALOGUE,
      icon: IconName.GIFT,
      label: t.nav_catalogue(),
      count: runningCampaigns(app.snapshot?.catalog.campaigns ?? [], ticker.now).length,
    },
    { page: Page.HISTORY, icon: IconName.HISTORY, label: t.nav_history() },
    { page: Page.LOGS, icon: IconName.FILE, label: t.nav_logs() },
    { page: Page.SETTINGS, icon: IconName.SETTINGS, label: t.nav_settings() },
  ]);

  const sub = $derived<NavItemType[]>([
    { page: Page.ACCOUNT, icon: IconName.DASHBOARD, label: t.nav_account_overview() },
    { page: Page.ACCOUNT_DROPS, icon: IconName.GIFT, label: t.nav_account_drops() },
    {
      page: Page.ACCOUNT_INVENTORY,
      icon: IconName.PACKAGE,
      label: t.nav_account_inventory(),
    },
    { page: Page.ACCOUNT_POINTS, icon: IconName.CHANNEL_POINTS, label: t.nav_account_points() },
    { page: Page.ACCOUNT_SPAM, icon: IconName.ZAP, label: t.nav_account_spam() },
    { page: Page.ACCOUNT_HISTORY, icon: IconName.HISTORY, label: t.nav_account_history() },
    { page: Page.ACCOUNT_SETTINGS, icon: IconName.SETTINGS, label: t.nav_account_settings() },
  ]);

  const itemClass = (active: boolean, nested: boolean): string =>
    cn(
      "flex w-full items-center gap-2.5 rounded-md border border-transparent px-2.5 text-left text-sidebar-foreground/85 transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
      nested ? "min-h-8 text-hint" : "min-h-9 text-nav",
      active &&
        "border-primary/50 bg-primary/15 font-semibold text-link hover:bg-primary/15 hover:text-link",
    );
</script>

<nav
  class="flex min-h-0 flex-col border-r bg-sidebar text-sidebar-foreground"
  aria-label={t.nav_main()}
>
  <div class="flex min-h-0 flex-1 flex-col gap-0.5 overflow-y-auto px-2.5 py-3">
    <a
      class="mb-2 flex items-center gap-2.5 rounded-md border border-transparent px-2 py-1.5 text-nav-title leading-tight font-bold text-foreground no-underline hover:border-border hover:bg-sidebar-accent"
      href={REPO_URL}
      rel="noreferrer"
      onclick={(event) => followLink(event, REPO_URL)}
    >
      <img class="size-8 shrink-0 [image-rendering:pixelated]" src="/app-icon.svg" alt="" />
      <span>Twitch Drops<br />& Points Miner</span>
    </a>
    {#each main as item (item.page)}
      <button
        class={itemClass(app.page === item.page, false)}
        type="button"
        aria-current={app.page === item.page ? "page" : undefined}
        onclick={() => navigate(item.page)}
      >
        <Icon name={item.icon} size={18} /><span class="truncate">{item.label}</span>
        {#if item.count}<span class="ml-auto"
            ><Badge text={number(item.count)} tone={BadgeTone.NEUTRAL} dot={false} /></span
          >{/if}
      </button>
    {/each}

    <div
      class="px-2.5 pt-3.5 pb-1.5 text-tiny font-bold tracking-wider text-muted-foreground uppercase"
    >
      {t.nav_accounts()}
    </div>
    {#each accounts as a (a.id)}
      {@const selected = inAccount && app.selectedAccount === a.id}
      <button
        class={cn(
          itemClass(false, false),
          "font-semibold text-sidebar-foreground",
          dragOver === a.id && dragging !== a.id && "shadow-[inset_0_2px_0_var(--primary)]",
        )}
        type="button"
        draggable="true"
        aria-keyshortcuts="Alt+ArrowUp Alt+ArrowDown"
        aria-describedby={REORDER_HINT_ID}
        onclick={() => navigate(Page.ACCOUNT, a.id)}
        onkeydown={(event) => {
          if (
            event.altKey &&
            (event.key === KeyboardKey.ARROW_UP || event.key === KeyboardKey.ARROW_DOWN)
          ) {
            event.preventDefault();

            const index = accounts.findIndex((x) => x.id === a.id);
            const to = event.key === KeyboardKey.ARROW_UP ? index - 1 : index + 1;

            if (to >= 0 && to < accounts.length) {
              moveAccount(a.id, accounts[to].id);
            }
          }
        }}
        ondragstart={(event) => {
          dragging = a.id;
          event.dataTransfer?.setData("text/plain", a.id);
        }}
        ondragover={(event) => {
          if (dragging) {
            event.preventDefault();
            dragOver = a.id;
          }
        }}
        ondrop={(event) => {
          event.preventDefault();

          if (dragging) {
            moveAccount(dragging, a.id);
          }

          dragging = null;
          dragOver = null;
        }}
        ondragend={() => {
          dragging = null;
          dragOver = null;
        }}
      >
        <Avatar url={a.avatar_url} name={a.display_name} size={AvatarSize.SM} />
        <span class="truncate">{a.display_name}</span>
        <span class="ml-auto"><StatusDot tone={STATUS_TONES[a.status]} /></span>
      </button>
      {#if selected}
        <div class="mt-0.5 mb-1.5 ml-5.5 flex flex-col gap-0.5 border-l border-input pl-2.5">
          {#each sub as item (item.page)}
            <button
              class={itemClass(app.page === item.page, true)}
              type="button"
              aria-current={app.page === item.page ? "page" : undefined}
              onclick={() => navigate(item.page, a.id)}
            >
              <Icon name={item.icon} size={16} class="shrink-0" /><span
                class="min-w-0 py-1 leading-tight break-words">{item.label}</span
              >
            </button>
          {/each}
        </div>
      {/if}
    {/each}
    <span id={REORDER_HINT_ID} class="sr-only">{t.nav_account_reorder_hint()}</span>
    <TextButton
      label={t.accounts_add()}
      icon={IconName.PLUS}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={!app.snapshot?.vault.unlocked}
      class="mt-1.5 w-full"
      onclick={() => (app.loginDialog = true)}
    />
  </div>
  <div class="flex items-center gap-2 text-xs text-muted-foreground tabular-nums">
    <span class="h-px flex-1 bg-border"></span>
    <a
      class="no-underline underline-offset-3 transition-colors hover:text-link hover:underline"
      href={REPO_URL}
      rel="noreferrer"
      onclick={(event) => followLink(event, REPO_URL)}
      >v{app.snapshot?.version ?? app.info?.version ?? ""}</a
    >
    {#if !app.connected}<span class="text-destructive">· {t.topbar_disconnected()}</span>{/if}
    <span class="h-px flex-1 bg-border"></span>
  </div>
  <div class="flex flex-col gap-2.5 px-3 pt-2 pb-3">
    <TextButton
      label={t.nav_feedback()}
      icon={IconName.ALERT}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      class="h-auto min-h-8 w-full justify-start py-1.5 text-left leading-snug whitespace-normal"
      onclick={() => (app.feedbackDialog = true)}
    />
    <PoweredBy />
  </div>
</nav>
