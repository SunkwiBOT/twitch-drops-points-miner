// ! Only the changed fields are replayed: the lists (streamers, games) keep
// changing underneath at once.

import { SvelteMap } from "svelte/reactivity";

import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { t } from "../i18n/i18n.svelte";
import { changed } from "./bulk.svelte";
import { AccountField, ToastKind } from "./state.constants";
import { toast, updateAccountSettings } from "./state.svelte";

export type EditType<T> = (settings: T) => void;

const copyOf = <T>(settings: T): T => $state.snapshot(settings) as T;

export class SettingsEdits<T, F extends string> {
  readonly #changes = new SvelteMap<F, EditType<T>>();

  set(field: F, change: EditType<T>): void {
    this.#changes.set(field, change);
  }

  applied(settings: T): T {
    const copy = copyOf(settings);

    for (const change of this.#changes.values()) {
      change(copy);
    }

    return copy;
  }

  differ(settings: T): boolean {
    if (this.#changes.size === 0) {
      return false;
    }

    return JSON.stringify(this.applied(settings)) !== JSON.stringify($state.snapshot(settings));
  }

  entries(): [F, EditType<T>][] {
    return [...this.#changes.entries()];
  }

  clear(): void {
    this.#changes.clear();
  }
}

type AccountEditsType = SettingsEdits<AccountSettingsType, AccountField>;

const accountEdits = new Map<string, AccountEditsType>();

export const editsOf = (accountId: string): AccountEditsType => {
  let edits = accountEdits.get(accountId);

  if (!edits) {
    edits = new SettingsEdits<AccountSettingsType, AccountField>();
    accountEdits.set(accountId, edits);
  }

  return edits;
};

export const saveAccountEdits = async (account: AccountSnapshotType): Promise<boolean> => {
  const edits = editsOf(account.id);
  const entries = edits.entries();
  const saved = await updateAccountSettings(account.id, (settings) => {
    for (const [, change] of entries) {
      change(settings);
    }
  });

  if (!saved) {
    return false;
  }

  edits.clear();

  const shared = entries.filter(([field]) => field !== AccountField.PROXY);

  if (shared.length === 0) {
    toast(ToastKind.SUCCESS, t.common_saved());

    return true;
  }

  changed(account.id, t.change_settings(), (settings) => {
    for (const [, change] of shared) {
      change(settings);
    }
  });

  return true;
};
