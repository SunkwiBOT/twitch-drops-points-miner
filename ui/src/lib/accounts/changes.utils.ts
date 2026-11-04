// ! Lists differ between accounts, so a place is kept as the item's neighbours
// rather than as a bare rank.

import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";

export type PlaceType = { previous: string | null; next: string | null; index: number };

export type DeltaType = { added: string[]; removed: string[] };

export const placeOf = (list: string[], key: string): PlaceType => {
  const index = list.indexOf(key);

  return {
    previous: index > 0 ? list[index - 1] : null,
    next: index >= 0 && index < list.length - 1 ? list[index + 1] : null,
    index: Math.max(index, 0),
  };
};

const placeLike = (list: string[], key: string, place: PlaceType): string[] => {
  const rest = list.filter((item) => item !== key);
  let at = Math.min(place.index, rest.length);

  if (place.previous !== null && rest.includes(place.previous)) {
    at = rest.indexOf(place.previous) + 1;
  } else if (place.next !== null && rest.includes(place.next)) {
    at = rest.indexOf(place.next);
  }

  rest.splice(at, 0, key);

  return rest;
};

export const moveLike = (list: string[], key: string, place: PlaceType): string[] | null =>
  list.includes(key) ? placeLike(list, key, place) : null;

export const insertLike = (list: string[], key: string, place: PlaceType): string[] | null =>
  list.includes(key) ? null : placeLike(list, key, place);

export const deltaOf = (before: string[], after: string[]): DeltaType => ({
  added: after.filter((item) => !before.includes(item)),
  removed: before.filter((item) => !after.includes(item)),
});

export const applyDelta = (list: string[], delta: DeltaType): string[] => [
  ...list.filter((item) => !delta.removed.includes(item)),
  ...delta.added.filter((item) => !list.includes(item)),
];

export const reorderBy = <T>(items: T[], keyOf: (item: T) => string, order: string[]): T[] => {
  const byKey = new Map(items.map((item) => [keyOf(item), item]));

  return order.flatMap((key) => byKey.get(key) ?? []);
};

export const setCampaignIgnoredIn = (
  settings: AccountSettingsType,
  campaignId: string,
  ignored: boolean,
): void => {
  const others = settings.drops.exclude_campaign_ids.filter((id) => id !== campaignId);

  settings.drops.exclude_campaign_ids = ignored ? [...others, campaignId] : others;

  if (ignored) {
    settings.drops.campaign_order = settings.drops.campaign_order.filter((id) => id !== campaignId);
  }
};

export const setDropIgnoredIn = (
  settings: AccountSettingsType,
  dropId: string,
  ignored: boolean,
): void => {
  const others = settings.drops.exclude_drop_ids.filter((id) => id !== dropId);

  settings.drops.exclude_drop_ids = ignored ? [...others, dropId] : others;
};

export const miningOrder = (
  settings: AccountSettingsType,
  target: AccountSnapshotType,
): string[] => {
  const placed = settings.drops.campaign_order;

  return [
    ...placed,
    ...target.queue.map((item) => item.campaign_id).filter((id) => !placed.includes(id)),
  ];
};

// ! Only the start of the mining order down to the last campaign placed by hand
// stays pinned: the ones after keep the automatic order, so new campaigns and
// priority games still find their place.
export const pinnedPrefix = (order: string[], placed: Iterable<string>): string[] => {
  const keep = new Set(placed);
  const last = order.reduce((at, id, index) => (keep.has(id) ? index : at), -1);

  return order.slice(0, last + 1);
};
