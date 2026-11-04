import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { CampaignStatus } from "../bindings/CampaignStatus";
import type { CampaignType } from "../bindings/CampaignType";
import type { DropsSettingsType } from "../bindings/DropsSettingsType";
import type { GameType } from "../bindings/GameType";
import type { QueueItemType } from "../bindings/QueueItemType";
import type { SnapshotType } from "../bindings/SnapshotType";
import { percent } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { changed } from "../state/bulk.svelte";
import { updateAccountSettings } from "../state/state.svelte";
import { strong } from "../text.utils";
import type { DropsList } from "./accounts.constants";
import {
  applyDelta,
  deltaOf,
  miningOrder,
  moveLike,
  pinnedPrefix,
  placeOf,
  setCampaignIgnoredIn,
  setDropIgnoredIn,
} from "./changes.utils";

export type IgnoredType = {
  id: string;
  name: string;
  detail: string;
  drop: boolean;
  campaign: CampaignType | null;
};

export type RunningGameType = { game: GameType; drops: number };

const saveDrops = (
  account: AccountSnapshotType,
  mutate: (drops: DropsSettingsType) => void,
): Promise<boolean> => updateAccountSettings(account.id, (settings) => mutate(settings.drops));

export const editDropsList = (
  account: AccountSnapshotType,
  field: DropsList,
  next: string[],
): void => {
  const delta = deltaOf(account.settings.drops[field], next);

  void saveDrops(account, (drops) => (drops[field] = next));
  changed(account.id, t.change_settings(), (settings) => {
    settings.drops[field] = applyDelta(settings.drops[field], delta);
  });
};

export const movePriorityGame = (
  account: AccountSnapshotType,
  list: string[],
  moved: string,
): void => {
  const place = placeOf(list, moved);

  void saveDrops(account, (drops) => (drops.priority_games = list));
  changed(account.id, t.change_settings(), (settings) => {
    const next = moveLike(settings.drops.priority_games, moved, place);

    if (next) {
      settings.drops.priority_games = next;
    }
  });
};

export const campaignOf = (
  account: AccountSnapshotType,
  snapshot: SnapshotType,
  id: string,
): CampaignType | undefined =>
  account.campaigns.find((c) => c.id === id) ?? snapshot.catalog.campaigns.find((c) => c.id === id);

export const catalogueGame = (snapshot: SnapshotType, name: string): GameType | undefined =>
  snapshot.catalog.campaigns.find((c) => c.game.name.toLowerCase() === name.toLowerCase())?.game;

export const queueProgress = (
  account: AccountSnapshotType,
  snapshot: SnapshotType,
  id: string,
): number => {
  const excluded = account.settings.drops.exclude_drop_ids;
  const minable = (campaignOf(account, snapshot, id)?.drops ?? []).filter(
    (d) => d.required_subs === 0 && !excluded.includes(d.id),
  );
  const required = minable.reduce((sum, d) => sum + d.required_minutes, 0);
  const done = minable.reduce((sum, d) => sum + Math.min(d.current_minutes, d.required_minutes), 0);

  return percent(done, required);
};

export const reorderQueue = (
  account: AccountSnapshotType,
  snapshot: SnapshotType,
  items: QueueItemType[],
  moved: QueueItemType,
): void => {
  const id = moved.campaign_id;
  const order = items.map((item) => item.campaign_id);
  const place = placeOf(order, id);

  void saveDrops(account, (drops) => {
    drops.campaign_order = pinnedPrefix(order, [...drops.campaign_order, id]);
  });

  const campaign = campaignOf(account, snapshot, id)?.name ?? id;

  changed(
    account.id,
    t.change_campaign_moved({ campaign: strong(campaign) }),
    (settings, target) => {
      const next = moveLike(miningOrder(settings, target), id, place);

      if (next) {
        settings.drops.campaign_order = pinnedPrefix(next, [...settings.drops.campaign_order, id]);
      }
    },
  );
};

export const resetQueue = (account: AccountSnapshotType): void => {
  void saveDrops(account, (drops) => (drops.campaign_order = []));
  changed(account.id, t.change_order_reset(), (settings) => (settings.drops.campaign_order = []));
};

export const ignoredCampaign = (
  account: AccountSnapshotType,
  snapshot: SnapshotType,
  id: string,
): IgnoredType => {
  const c = campaignOf(account, snapshot, id);

  return { id, name: c?.name ?? id, detail: c?.game.name ?? "", drop: false, campaign: c ?? null };
};

export const ignoredItems = (
  account: AccountSnapshotType,
  snapshot: SnapshotType,
): IgnoredType[] => [
  ...account.settings.drops.exclude_campaign_ids.map((id) =>
    ignoredCampaign(account, snapshot, id),
  ),
  ...account.settings.drops.exclude_drop_ids.map((id) => {
    const c = account.campaigns.find((item) => item.drops.some((d) => d.id === id));

    return {
      id,
      name: c?.drops.find((d) => d.id === id)?.name ?? id,
      detail: c ? `${c.game.name} · ${c.name}` : "",
      drop: true,
      campaign: c ?? null,
    };
  }),
];

export const setIgnored = (
  account: AccountSnapshotType,
  item: IgnoredType,
  ignored: boolean,
): void => {
  const apply = (settings: AccountSettingsType): void =>
    item.drop
      ? setDropIgnoredIn(settings, item.id, ignored)
      : setCampaignIgnoredIn(settings, item.id, ignored);

  void updateAccountSettings(account.id, apply);

  const text = ignored
    ? t.change_ignored({ item: strong(item.name) })
    : t.change_unignored({ item: strong(item.name) });

  changed(account.id, text, apply);
};

export const ongoingCampaigns = (account: AccountSnapshotType, now: number): CampaignType[] =>
  account.campaigns.filter(
    (c) =>
      new Date(c.ends_at).getTime() > now && (c.in_inventory || c.status === CampaignStatus.ACTIVE),
  );

export const runningGames = (snapshot: SnapshotType, now: number): RunningGameType[] => {
  const byName = new Map<string, RunningGameType>();

  for (const c of snapshot.catalog.campaigns) {
    if (new Date(c.ends_at).getTime() <= now) {
      continue;
    }

    const key = c.game.name.toLowerCase();
    const known = byName.get(key) ?? { game: c.game, drops: 0 };

    known.drops += c.drops.length;
    byName.set(key, known);
  }

  return [...byName.values()].sort((a, b) => a.game.name.localeCompare(b.game.name));
};
