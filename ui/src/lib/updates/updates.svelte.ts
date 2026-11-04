import {
  applyUpdate,
  checkForUpdate,
  downloadUpdate,
  fetchFoundUpdate,
  onUpdateAvailable,
  onUpdateProgress,
  takeFailedUpdate,
} from "../api/client";
import { isTauri } from "../api/tauri";
import type { UnsubscribeType, UpdateInfoType, UpdateProgressType } from "../api/tauri";
import { ErrorApiMessage } from "../bindings/ErrorApiMessage";
import { describeCode, toApiError } from "../errors/errors.utils";
import { percent } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { Page, ToastKind } from "../state/state.constants";
import { app, dismissToast, navigate, toast } from "../state/state.svelte";
import { strong } from "../text.utils";
import { UPDATE_TOAST_KEY, UpdateCheck, UpdateStage } from "./updates.constants";

type UpdatesStateType = {
  check: UpdateCheck;
  available: string | null;
  stage: UpdateStage;
  progress: UpdateProgressType | null;
  confirming: boolean;
  reveal: boolean;
};

export const updates = $state<UpdatesStateType>({
  check: UpdateCheck.IDLE,
  available: null,
  stage: UpdateStage.IDLE,
  progress: null,
  confirming: false,
  reveal: false,
});

let offered: string | null = null;

const locked = (): boolean => app.snapshot?.vault.unlocked === false;

export const availableVersion = (): string | null =>
  updates.available ?? app.snapshot?.update_available ?? null;

export const installStatus = (version: string): string | null => {
  const bold = strong(version);

  if (updates.stage === UpdateStage.INSTALLING) {
    return t.updates_restarting({ version: bold });
  }

  if (updates.stage === UpdateStage.DOWNLOADING) {
    const progress = updates.progress;

    return progress?.total
      ? t.updates_downloading_percent({
          version: bold,
          percent: strong(percent(progress.downloaded, progress.total)),
        })
      : t.updates_downloading({ version: bold });
  }

  return null;
};

export const checkUpdates = async (): Promise<void> => {
  if (updates.check === UpdateCheck.CHECKING || updates.stage !== UpdateStage.IDLE) {
    return;
  }

  updates.check = UpdateCheck.CHECKING;

  try {
    updates.available = await checkForUpdate();
    updates.check = updates.available ? UpdateCheck.AVAILABLE : UpdateCheck.CURRENT;
    offered = updates.available ?? offered;
  } catch (e: any) {
    console.warn("the update check failed", e);
    updates.check = UpdateCheck.FAILED;
  }
};

export const askInstall = (): void => {
  if (!isTauri() || locked() || !availableVersion() || updates.stage !== UpdateStage.IDLE) {
    return;
  }

  dismissToast(UPDATE_TOAST_KEY);
  updates.confirming = true;
};

const INSTALL_FAILURES_SAID: ReadonlySet<ErrorApiMessage> = new Set([
  ErrorApiMessage.UPDATE_SIGNATURE_REFUSED,
  ErrorApiMessage.UPDATE_INSTALL_FAILED,
  ErrorApiMessage.VAULT_LOCKED,
]);

const installFailure = (code: ErrorApiMessage): string => {
  if (code === ErrorApiMessage.NETWORK_ERROR) {
    return t.updates_download_failed();
  }

  return INSTALL_FAILURES_SAID.has(code) ? describeCode(code) : t.updates_install_failed();
};

export const installUpdate = async (): Promise<void> => {
  const confirmed = availableVersion();

  if (!confirmed || locked() || updates.stage !== UpdateStage.IDLE) {
    return;
  }

  updates.stage = UpdateStage.DOWNLOADING;
  updates.progress = null;

  let stopProgress: UnsubscribeType | null = null;

  try {
    stopProgress = await onUpdateProgress((progress) => {
      updates.progress = progress;
    });

    const version = await downloadUpdate();

    // ! A check found a newer one since the confirmation: it is confirmed
    // first, never installed unseen.
    if (version !== confirmed) {
      updates.available = version;
      updates.stage = UpdateStage.IDLE;

      return;
    }

    updates.stage = UpdateStage.INSTALLING;
    await applyUpdate(version);
  } catch (e: any) {
    console.warn("the update could not be installed", e);
    updates.stage = UpdateStage.IDLE;
    updates.confirming = false;
    toast(ToastKind.ERROR, installFailure(toApiError(e).code), { key: UPDATE_TOAST_KEY });
  } finally {
    stopProgress?.();
  }
};

export const showUpdate = (): void => {
  updates.reveal = true;
  navigate(Page.SETTINGS);
};

const offer = (found: UpdateInfoType): void => {
  if (updates.stage !== UpdateStage.IDLE) {
    return;
  }

  updates.available = found.version;
  updates.check = UpdateCheck.AVAILABLE;

  if (offered === found.version) {
    return;
  }

  offered = found.version;

  if (locked()) {
    toast(ToastKind.INFO, t.updates_available_locked({ version: found.version }), {
      key: UPDATE_TOAST_KEY,
    });

    return;
  }

  toast(ToastKind.INFO, t.updates_available({ version: found.version }), {
    key: UPDATE_TOAST_KEY,
    action: { label: t.updates_install(), run: askInstall },
  });
};

const sayFailedUpdate = async (): Promise<void> => {
  try {
    const failed = await takeFailedUpdate();

    if (failed) {
      toast(ToastKind.ERROR, t.updates_failed_restarted({ version: strong(failed.version) }));
    }
  } catch (e: any) {
    console.warn("the failed update was not read", e);
  }
};

export const followUpdates = async (): Promise<void> => {
  if (!isTauri() || app.startupProblem) {
    return;
  }

  await sayFailedUpdate();

  try {
    await onUpdateAvailable(offer);

    const found = await fetchFoundUpdate();

    if (found) {
      offer(found);
    }
  } catch (e: any) {
    console.warn("the updates are not followed", e);
  }
};
