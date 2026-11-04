import { RequestKind } from "../bindings/RequestKind";

export enum RequestKindGroup {
  TWITCH = "TWITCH",
  OTHER = "OTHER",
}

export const REQUEST_KIND_GROUPS: readonly { group: RequestKindGroup; kinds: RequestKind[] }[] = [
  {
    group: RequestKindGroup.TWITCH,
    kinds: [
      RequestKind.GQL,
      RequestKind.AUTH,
      RequestKind.PLAYLIST,
      RequestKind.SEGMENT,
      RequestKind.CHANNEL_PAGE,
      RequestKind.BEACON,
      RequestKind.PUBSUB,
      RequestKind.CHAT,
    ],
  },
  {
    group: RequestKindGroup.OTHER,
    kinds: [
      RequestKind.CATALOG,
      RequestKind.DISCORD,
      RequestKind.TELEGRAM,
      RequestKind.WEBHOOK,
      RequestKind.PICTURE,
      RequestKind.UPDATE_CHECK,
      RequestKind.UPDATE_DOWNLOAD,
      RequestKind.PROXY_TEST,
    ],
  },
];

export const DESKTOP_ONLY_KINDS: ReadonlySet<RequestKind> = new Set([
  RequestKind.PICTURE,
  RequestKind.UPDATE_DOWNLOAD,
]);

export const REQUESTS_PAGE_SIZE = 100;

export const GROUP_COUNT_PREFIX = "×";
export const TIME_RANGE_SEPARATOR = "–";
export const PART_SEPARATOR = " · ";
export const NO_TARGET = "—";

export const TARGET_BREAK_AFTER = "/";

// ! Sized on the list, not the window: the sidebar takes its share of a narrow window.
export const REQUEST_LIST_CLASSES =
  "grid grid-cols-[max-content_minmax(0,1fr)_max-content] gap-x-3 font-mono text-xs leading-normal @2xl:grid-cols-[max-content_7rem_minmax(0,1fr)_max-content_max-content] @4xl:grid-cols-[max-content_7rem_max-content_minmax(0,1fr)_max-content_max-content]";
export const REQUEST_LINE_CLASSES =
  "col-span-full grid grid-cols-subgrid border-b py-1 break-words last:border-b-0";
