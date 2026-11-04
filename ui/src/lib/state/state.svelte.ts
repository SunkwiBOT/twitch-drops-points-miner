import { CommandName } from "../bindings/CommandName";
import {
  fetchAppInfo,
  fetchAuthStatus,
  fetchSnapshot,
  fetchStartupProblem,
  onCloseRequested,
  openUrl,
  requestCatalogRefresh,
  sendCommand,
  subscribe,
} from "../api/client";
import { toast as sonner } from "svelte-sonner";

import ToastCard from "../../components/layout/ToastCard.svelte";
import { isTauri } from "../api/tauri";
import type { UnsubscribeType } from "../api/tauri";
import { setCampaignIgnoredIn, setDropIgnoredIn } from "../accounts/changes.utils";
import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { AppInfoType } from "../bindings/AppInfoType";
import type { AuthStatusType } from "../bindings/AuthStatusType";
import { ClockFormat } from "../bindings/ClockFormat";
import type { CommandType } from "../bindings/CommandType";
import { DateFormat } from "../bindings/DateFormat";
import { ErrorApiMessage } from "../bindings/ErrorApiMessage";
import { EventKind } from "../bindings/EventKind";
import type { GlobalSettingsType } from "../bindings/GlobalSettingsType";
import type { MinerEventType } from "../bindings/MinerEventType";
import type { SnapshotType } from "../bindings/SnapshotType";
import type { StartupProblemType } from "../bindings/StartupProblemType";
import { Theme } from "../bindings/Theme";
import { describeError, toApiError } from "../errors/errors.utils";
import { eventText } from "../events/events.utils";
import { number } from "../format.utils";
import { strong } from "../text.utils";
import { applyLanguage, t } from "../i18n/i18n.svelte";
import { applyTimePreferences } from "../time/time.svelte";
import {
  ACCOUNT_PAGES,
  HistoryMode,
  LINK_TOAST_KEY,
  LIVE_EVENTS_MAX,
  MouseButton,
  Page,
  PAGE_PATHS,
  SETTINGS_SAVE_DELAY_MS,
  SETTINGS_SETTLE_MS,
  TOAST_ACTION_DURATION_MS,
  TOAST_DURATION_MS,
  ToastKind,
} from "./state.constants";
import { parseRoute, routePath } from "./routes.utils";
import {
  KEY_STORAGE_ACCOUNT,
  KEY_STORAGE_PAGE,
  KEY_STORAGE_PREFERENCES,
} from "./storage.constants";

export type ToastImageType = { url: string | undefined; name: string; channel: string | null };

export type ToastActionType = { label: string; run: () => void };

type ToastOptionsType = {
  image?: ToastImageType;
  title?: string;
  action?: ToastActionType;
  key?: string;
  onend?: () => void;
};

type AuthStateType = AuthStatusType & { checked: boolean };

type AppStateType = {
  snapshot: SnapshotType | null;
  connected: boolean;
  page: Page;
  selectedAccount: string | null;
  info: AppInfoType | null;
  startupProblem: StartupProblemType | null;
  auth: AuthStateType;
  loginDialog: boolean;
  closeDialog: boolean;
  feedbackDialog: boolean;
  liveEvents: MinerEventType[];
  logsAccount: string | null;
  tab: string;
  revealPassword: boolean;
};

type PreferencesType = { theme?: Theme; language?: string };

export const app = $state<AppStateType>({
  snapshot: null,
  connected: false,
  page: Page.HOME,
  selectedAccount: null,
  info: null,
  startupProblem: null,
  auth: { authorized: false, public: false, checked: false },
  loginDialog: false,
  closeDialog: false,
  feedbackDialog: false,
  liveEvents: [],
  logsAccount: null,
  tab: "",
  revealPassword: false,
});

export const isAccountPage = (page: Page): boolean => ACCOUNT_PAGES.has(page);

const addressTab = (): string => location.hash.slice(1);

export const openTab = <T extends string>(
  tabs: readonly T[],
  slugs: Record<T, string>,
  fallback: T,
): T => tabs.find((tab) => slugs[tab] === app.tab) ?? fallback;

export const showTab = <T extends string>(slugs: Record<T, string>, tab: T): void => {
  app.tab = slugs[tab];
  history.replaceState(history.state, "", `${location.pathname}${location.search}#${app.tab}`);
};

const PAGES: ReadonlySet<string> = new Set(Object.values(Page));
const isPage = (value: string | null): value is Page => value !== null && PAGES.has(value);

let toastId = 0;

export const dismissToast = (id: string): void => {
  sonner.dismiss(id);
};

export const toast = (kind: ToastKind, text: string, options: ToastOptionsType = {}): void => {
  toastId += 1;

  const id = options.key ?? `toast-${toastId}`;
  const onend = (): void => options.onend?.();

  sonner.custom(ToastCard, {
    id,
    duration: options.action ? TOAST_ACTION_DURATION_MS : TOAST_DURATION_MS[kind],
    componentProps: {
      kind,
      text,
      image: options.image ?? null,
      title: options.title ?? null,
      action: options.action ?? null,
      onclose: () => dismissToast(id),
    },
    onAutoClose: onend,
    onDismiss: onend,
  });
};

export const openLink = async (url: string): Promise<void> => {
  try {
    await openUrl(url);
  } catch (e: any) {
    toast(ToastKind.ERROR, describeError(e));

    return;
  }

  if (isTauri()) {
    const site = new URL(url).host.replace(/^www\./, "");

    toast(ToastKind.INFO, t.link_opened({ site: strong(site) }), { key: LINK_TOAST_KEY });
  }
};

export const accountImage = (accountId: string, fallback: string): ToastImageType => {
  const found = app.snapshot?.accounts.find((a) => a.id === accountId);

  return { url: found?.avatar_url, name: found?.display_name ?? fallback, channel: null };
};

const pending: PreferencesType = {};

type ViewerPreferencesType = Partial<
  Pick<GlobalSettingsType, "language" | "theme" | "time_zone" | "date_format" | "clock_format">
>;

const perViewer = (): boolean => !isTauri();

const isChoice = (choices: Record<string, string>, value: string | undefined): boolean =>
  value === undefined || Object.values(choices).includes(value);

const viewerPreferences = (): ViewerPreferencesType => {
  try {
    const saved = JSON.parse(recall(KEY_STORAGE_PREFERENCES) ?? "{}") as ViewerPreferencesType;

    if (!isChoice(Theme, saved.theme)) {
      delete saved.theme;
    }

    if (!isChoice(DateFormat, saved.date_format)) {
      delete saved.date_format;
    }

    if (!isChoice(ClockFormat, saved.clock_format)) {
      delete saved.clock_format;
    }

    return saved;
  } catch (e: any) {
    return {};
  }
};

const pickViewer = (settings: GlobalSettingsType): ViewerPreferencesType => ({
  language: settings.language,
  theme: settings.theme,
  time_zone: settings.time_zone,
  date_format: settings.date_format,
  clock_format: settings.clock_format,
});

let serverPreferences: ViewerPreferencesType = {};

const applyTheme = (theme: Theme): void => {
  document.documentElement.dataset.theme = theme;
};

const applySnapshot = (snapshot: SnapshotType): void => {
  if (perViewer()) {
    serverPreferences = pickViewer(snapshot.settings);
    Object.assign(snapshot.settings, viewerPreferences());
  }

  if (pending.theme !== undefined) {
    if (snapshot.settings.theme === pending.theme) {
      delete pending.theme;
    } else {
      snapshot.settings.theme = pending.theme;
    }
  }

  if (pending.language !== undefined) {
    if (snapshot.settings.language === pending.language) {
      delete pending.language;
    } else {
      snapshot.settings.language = pending.language;
    }
  }

  keepPendingSettings(snapshot);
  app.snapshot = snapshot;
  applyLanguage(snapshot.settings.language);
  applyTimePreferences(snapshot.settings);
  applyTheme(snapshot.settings.theme);

  if (pendingLogin !== null) {
    const found = snapshot.accounts.find((a) => a.login === pendingLogin);

    pendingLogin = null;

    if (found) {
      app.selectedAccount = found.id;
    } else {
      app.page = Page.HOME;
    }
  }

  if (app.selectedAccount && !snapshot.accounts.some((a) => a.id === app.selectedAccount)) {
    app.selectedAccount = null;

    if (isAccountPage(app.page)) {
      app.page = Page.HOME;
    }
  }

  writeAddress(HistoryMode.REPLACE);
};

export const refresh = async (): Promise<void> => {
  try {
    applySnapshot(await fetchSnapshot());
  } catch (e: any) {
    const error = toApiError(e);

    if (error.code === ErrorApiMessage.UNAUTHORIZED) {
      app.auth = { ...app.auth, authorized: false, checked: true };

      return;
    }

    toast(ToastKind.ERROR, describeError(error));
  }
};

// ! The toasts go too: a toast may name an account or reopen a dialog.
export const sessionEnded = (): void => {
  app.auth = { ...app.auth, authorized: false, checked: true };
  unsubscribe?.();
  unsubscribe = null;
  app.snapshot = null;
  app.liveEvents = [];
  app.logsAccount = null;
  app.loginDialog = false;
  app.feedbackDialog = false;
  sonner.dismiss();
};

// ! A command on an account first waits for that account's pending settings
// save, or the save, sent later with the whole settings, would undo it.
export const run = async (command: CommandType, success?: string): Promise<boolean> => {
  try {
    if ("account_id" in command) {
      await settleSettings(command.account_id);
    }

    await sendCommand(command);

    if (success) {
      toast(ToastKind.SUCCESS, success);
    }

    return true;
  } catch (e: any) {
    const error = toApiError(e);

    if (error.code === ErrorApiMessage.UNAUTHORIZED) {
      sessionEnded();

      return false;
    }

    toast(ToastKind.ERROR, describeError(error));

    return false;
  }
};

export const account = (id: string | null): AccountSnapshotType | null =>
  app.snapshot?.accounts.find((a) => a.id === id) ?? null;

export const saveAccountSettings = (id: string, settings: AccountSettingsType): Promise<boolean> =>
  run({ command: CommandName.SET_ACCOUNT_SETTINGS, account_id: id, settings }, t.common_saved());

export const saveGlobalSettings = (settings: GlobalSettingsType): Promise<boolean> => {
  if (!perViewer()) {
    return run({ command: CommandName.SET_GLOBAL_SETTINGS, settings }, t.common_saved());
  }

  remember(KEY_STORAGE_PREFERENCES, JSON.stringify(pickViewer(settings)));
  applyLanguage(settings.language);
  applyTimePreferences(settings);
  applyTheme(settings.theme);

  if (app.snapshot) {
    app.snapshot.settings = settings;
  }

  return run(
    { command: CommandName.SET_GLOBAL_SETTINGS, settings: { ...settings, ...serverPreferences } },
    t.common_saved(),
  );
};

type PendingSaveType = {
  settings: AccountSettingsType;
  timer: ReturnType<typeof setTimeout> | null;
  saving: boolean;
  version: number;
  saved: number;
  waiters: ((saved: boolean) => void)[];
};

const pendingSettings = new Map<string, PendingSaveType>();

// ! A saved state stays on screen a moment longer: a snapshot sent before the
// core applied it must not flip the control back.
const finishSave = (accountId: string, entry: PendingSaveType, saved: boolean): void => {
  const waiters = entry.waiters;

  entry.waiters = [];

  if (saved) {
    const version = entry.version;

    entry.timer = setTimeout(() => {
      if (pendingSettings.get(accountId) === entry && entry.version === version) {
        pendingSettings.delete(accountId);
      }
    }, SETTINGS_SETTLE_MS);
  } else {
    pendingSettings.delete(accountId);
  }

  for (const resolve of waiters) {
    resolve(saved);
  }
};

const flushSettings = async (accountId: string): Promise<void> => {
  const entry = pendingSettings.get(accountId);

  if (!entry || entry.saving) {
    return;
  }

  entry.saving = true;

  const sent = entry.version;

  try {
    await sendCommand({
      command: CommandName.SET_ACCOUNT_SETTINGS,
      account_id: accountId,
      settings: entry.settings,
    });
    entry.saving = false;
    entry.saved = sent;

    if (entry.version !== sent) {
      await flushSettings(accountId);
    } else {
      finishSave(accountId, entry, true);
    }
  } catch (e: any) {
    finishSave(accountId, entry, false);
    toast(ToastKind.ERROR, describeError(e));
    await refresh();
  }
};

export const updateAccountSettings = (
  accountId: string,
  mutate: (settings: AccountSettingsType) => void,
): Promise<boolean> => {
  const target = app.snapshot?.accounts.find((a) => a.id === accountId);

  if (!target) {
    return Promise.resolve(false);
  }

  const settings = $state.snapshot(target.settings);

  mutate(settings);
  target.settings = settings;

  const entry = pendingSettings.get(accountId) ?? {
    settings,
    timer: null,
    saving: false,
    version: 0,
    saved: 0,
    waiters: [],
  };

  entry.settings = settings;
  entry.version += 1;
  pendingSettings.set(accountId, entry);

  if (entry.timer) {
    clearTimeout(entry.timer);
  }

  entry.timer = setTimeout(() => {
    entry.timer = null;
    void flushSettings(accountId);
  }, SETTINGS_SAVE_DELAY_MS);

  return new Promise((resolve) => entry.waiters.push(resolve));
};

const settleSettings = async (accountId: string): Promise<void> => {
  const entry = pendingSettings.get(accountId);

  if (!entry || (entry.version === entry.saved && !entry.saving)) {
    return;
  }

  if (entry.timer) {
    clearTimeout(entry.timer);
    entry.timer = null;
  }

  const saved = new Promise((resolve) => entry.waiters.push(resolve));

  void flushSettings(accountId);
  await saved;
};

const keepPendingSettings = (snapshot: SnapshotType): void => {
  for (const item of snapshot.accounts) {
    const entry = pendingSettings.get(item.id);

    if (entry) {
      item.settings = entry.settings;
    }
  }
};

export const refreshCatalog = async (): Promise<void> => {
  try {
    const refreshed = await requestCatalogRefresh();

    const count = refreshed.campaign_count;

    toast(ToastKind.SUCCESS, t.catalogue_refreshed({ count, shown: strong(number(count)) }));
  } catch (e: any) {
    toast(ToastKind.ERROR, describeError(e));
  }

  await refresh();
};

export const setPreference = async (change: PreferencesType): Promise<void> => {
  if (!app.snapshot) {
    return;
  }

  const settings = { ...$state.snapshot(app.snapshot.settings), ...change };

  if (perViewer()) {
    remember(KEY_STORAGE_PREFERENCES, JSON.stringify({ ...viewerPreferences(), ...change }));

    if (change.theme !== undefined) {
      applyTheme(change.theme);
    }

    if (change.language !== undefined) {
      applyLanguage(change.language);
    }

    app.snapshot.settings = settings;

    return;
  }

  if (change.theme !== undefined) {
    pending.theme = change.theme;
    applyTheme(change.theme);
  }

  if (change.language !== undefined) {
    pending.language = change.language;
    applyLanguage(change.language);
  }

  app.snapshot.settings = settings;

  try {
    await sendCommand({ command: CommandName.SET_GLOBAL_SETTINGS, settings });
  } catch (e: any) {
    delete pending.theme;
    delete pending.language;
    toast(ToastKind.ERROR, describeError(e));
  }
};

export const toggleTheme = (): Promise<void> =>
  setPreference({ theme: app.snapshot?.settings.theme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT });

const remember = (key: string, value: string): void => {
  try {
    localStorage.setItem(key, value);
  } catch (e: any) {
    // ! Storage can be off (private window); what it keeps is only a convenience.
  }
};

const recall = (key: string): string | null => {
  try {
    return localStorage.getItem(key);
  } catch (e: any) {
    return null;
  }
};

let pendingLogin: string | null = null;

const writeAddress = (mode: HistoryMode): void => {
  const login = isAccountPage(app.page) ? (account(app.selectedAccount)?.login ?? null) : null;

  if ((isAccountPage(app.page) && !login) || pendingLogin !== null) {
    return;
  }

  const path = routePath(app.page, login);

  if (path === location.pathname) {
    return;
  }

  app.tab = "";

  if (mode === HistoryMode.PUSH) {
    history.pushState(null, "", path);
  } else {
    history.replaceState(null, "", path);
  }
};

const followAddress = (): void => {
  const route = parseRoute(location.pathname) ?? { page: Page.HOME, login: null };

  app.page = route.page;
  app.tab = addressTab();

  if (route.login !== null) {
    const found = app.snapshot?.accounts.find((a) => a.login === route.login);

    if (found) {
      app.selectedAccount = found.id;
    } else if (app.snapshot) {
      app.page = Page.HOME;
    } else {
      pendingLogin = route.login;
    }
  }

  remember(KEY_STORAGE_PAGE, app.page);

  if (app.selectedAccount) {
    remember(KEY_STORAGE_ACCOUNT, app.selectedAccount);
  }

  writeAddress(HistoryMode.REPLACE);
};

export const navigate = (page: Page, accountId?: string): void => {
  const next = isAccountPage(page) && !(accountId ?? app.selectedAccount) ? Page.HOME : page;

  app.page = next;
  remember(KEY_STORAGE_PAGE, next);

  if (accountId !== undefined) {
    app.selectedAccount = accountId;
    remember(KEY_STORAGE_ACCOUNT, accountId);
  }

  writeAddress(HistoryMode.PUSH);
};

// ! A browser follows the mouse's back and forward buttons by itself; the desktop
// app's window does not everywhere.
const followMouseButtons = (event: MouseEvent): void => {
  if (event.button === MouseButton.BACK) {
    event.preventDefault();
    history.back();
  } else if (event.button === MouseButton.FORWARD) {
    event.preventDefault();
    history.forward();
  }
};

let unsubscribe: UnsubscribeType | null = null;

export const checkSession = async (): Promise<boolean> => {
  try {
    const status = await fetchAuthStatus();

    if (!status.authorized) {
      sessionEnded();
    }

    return true;
  } catch (e: any) {
    return false;
  }
};

export const connect = async (): Promise<void> => {
  await refresh();
  unsubscribe?.();
  unsubscribe = await subscribe({
    onSnapshot: applySnapshot,
    onEvent: (event) => {
      app.liveEvents = [event, ...app.liveEvents].slice(0, LIVE_EVENTS_MAX);

      const known = accountImage(event.account_id, event.account_login);
      const image = {
        url: event.context.account_avatar ?? known.url,
        name: event.context.account_name ?? known.name,
        channel: null,
      };
      const title = image.name;

      if (event.kind === EventKind.DROP_CLAIMED) {
        toast(ToastKind.SUCCESS, eventText(event, false).text, { image, title });
      }

      if (event.kind === EventKind.LOGIN_COMPLETED) {
        toast(ToastKind.SUCCESS, eventText(event, false).text, { image });
      }

      if (event.kind === EventKind.SESSION_EXPIRED) {
        toast(ToastKind.ERROR, eventText(event, false).text, { image, title });
      }

      if (event.kind === EventKind.CHAT_MENTION) {
        toast(ToastKind.INFO, eventText(event, false).text, { image, title });
      }
    },
    onConnection: (connected) => {
      app.connected = connected;

      if (connected) {
        void refresh();
      } else {
        void checkSession();
      }
    },
  });
};

export const start = async (): Promise<void> => {
  app.startupProblem = await fetchStartupProblem();

  if (app.startupProblem) {
    return;
  }

  const selected = recall(KEY_STORAGE_ACCOUNT);

  if (selected) {
    app.selectedAccount = selected;
  }

  if (location.pathname === PAGE_PATHS[Page.HOME]) {
    const page = recall(KEY_STORAGE_PAGE);

    if (isPage(page) && (!isAccountPage(page) || selected)) {
      app.page = page;
    }
  } else {
    followAddress();
  }

  app.tab = addressTab();
  window.addEventListener("popstate", followAddress);
  window.addEventListener("hashchange", () => (app.tab = addressTab()));

  if (isTauri()) {
    window.addEventListener("mouseup", followMouseButtons);
  }

  try {
    app.info = await fetchAppInfo();
    await onCloseRequested(() => {
      app.closeDialog = true;
    });
  } catch (e: any) {
    console.warn("desktop details unavailable", e);
  }

  try {
    const status = await fetchAuthStatus();

    app.auth = { ...status, checked: true };
  } catch (e: any) {
    app.auth = { ...app.auth, checked: true };
    toast(ToastKind.ERROR, describeError(e));
  }

  if (app.auth.authorized) {
    await connect();
  }
};

export const prioritizeCampaign = async (
  accountIds: string[],
  campaignId: string,
): Promise<void> => {
  let applied = 0;

  for (const accountId of accountIds) {
    const done = await run({
      command: CommandName.ORDER_CAMPAIGN,
      account_id: accountId,
      campaign_id: campaignId,
      position: 0,
    });

    applied += done ? 1 : 0;
  }

  if (applied > 0) {
    toast(ToastKind.SUCCESS, t.campaign_prioritized(), {
      key: `campaign-prioritized-${campaignId}`,
    });
  }
};

export const setCampaignIgnored = async (
  accountIds: string[],
  campaignId: string,
  ignored: boolean,
): Promise<void> => {
  const results = await Promise.all(
    accountIds.map((accountId) =>
      updateAccountSettings(accountId, (settings) =>
        setCampaignIgnoredIn(settings, campaignId, ignored),
      ),
    ),
  );

  if (results.some(Boolean)) {
    toast(ToastKind.SUCCESS, ignored ? t.campaign_ignored() : t.campaign_unignored(), {
      key: `campaign-ignored-${campaignId}`,
    });
  }
};

export const setDropIgnored = (
  accountIds: string[],
  dropId: string,
  ignored: boolean,
): Promise<boolean[]> =>
  Promise.all(
    accountIds.map((accountId) =>
      updateAccountSettings(accountId, (settings) => setDropIgnoredIn(settings, dropId, ignored)),
    ),
  );
