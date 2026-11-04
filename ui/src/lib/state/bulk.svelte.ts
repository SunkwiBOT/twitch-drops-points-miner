import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { t } from "../i18n/i18n.svelte";
import { strong } from "../text.utils";
import { BULK_TOAST_KEY, ToastKind } from "./state.constants";
import { accountImage, app, toast, updateAccountSettings } from "./state.svelte";
import type { ToastImageType } from "./state.svelte";

export type AccountChangeType = (
  settings: AccountSettingsType,
  target: AccountSnapshotType,
) => void;

type BulkStateType = { open: boolean; sourceId: string | null; changes: AccountChangeType[] };

export const bulk = $state<BulkStateType>({ open: false, sourceId: null, changes: [] });

export const closeBulk = (): void => {
  bulk.open = false;
  bulk.sourceId = null;
  bulk.changes = [];
};

export const changed = (
  sourceId: string,
  text: string,
  change: AccountChangeType,
  subject?: ToastImageType,
): void => {
  const image = accountImage(sourceId, "");
  const title = subject ? undefined : image.name;

  if ((app.snapshot?.accounts.length ?? 0) < 2) {
    toast(ToastKind.SUCCESS, text, { image: subject ?? image, title });

    return;
  }

  if (bulk.sourceId !== sourceId) {
    bulk.sourceId = sourceId;
    bulk.changes = [];
  }

  bulk.changes.push(change);

  const count = bulk.changes.length;

  toast(
    ToastKind.SUCCESS,
    count === 1 ? text : t.bulk_changes({ count: strong(count), name: strong(image.name) }),
    {
      key: BULK_TOAST_KEY,
      image: count === 1 ? (subject ?? image) : image,
      title: count === 1 ? title : undefined,
      action: { label: t.bulk_apply_others(), run: () => (bulk.open = true) },
      onend: () => {
        if (!bulk.open) {
          closeBulk();
        }
      },
    },
  );
};

export const applyToAccounts = async (accountIds: string[]): Promise<void> => {
  const changes = [...bulk.changes];
  const results = await Promise.all(
    accountIds.map((id) => {
      const target = app.snapshot?.accounts.find((a) => a.id === id);

      if (!target) {
        return Promise.resolve(false);
      }

      return updateAccountSettings(id, (settings) => {
        for (const change of changes) {
          change(settings, target);
        }
      });
    }),
  );

  closeBulk();

  const applied = results.filter(Boolean).length;

  if (applied > 0) {
    toast(ToastKind.SUCCESS, t.bulk_applied({ count: strong(applied) }));
  }
};
