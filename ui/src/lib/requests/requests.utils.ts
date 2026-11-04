import { RequestKind } from "../bindings/RequestKind";
import { RequestOutcome } from "../bindings/RequestOutcome";
import { RequestSource } from "../bindings/RequestSource";
import type { RequestEntryType } from "../bindings/RequestEntryType";
import type { RequestsQueryType } from "../bindings/RequestsQueryType";
import type { RequestsResponseType } from "../bindings/RequestsResponseType";
import { SocketEvent } from "../bindings/SocketEvent";
import { t } from "../i18n/i18n.svelte";
import { RequestKindGroup, TARGET_BREAK_AFTER } from "./requests.constants";

export type RequestCursorType = Pick<RequestsQueryType, "before_live_id" | "before_stored_id">;

export const REQUEST_KIND_LABELS: Record<RequestKind, () => string> = {
  [RequestKind.GQL]: () => t.request_kind_gql(),
  [RequestKind.AUTH]: () => t.request_kind_auth(),
  [RequestKind.PLAYLIST]: () => t.request_kind_playlist(),
  [RequestKind.SEGMENT]: () => t.request_kind_segment(),
  [RequestKind.CHANNEL_PAGE]: () => t.request_kind_channel_page(),
  [RequestKind.BEACON]: () => t.request_kind_beacon(),
  [RequestKind.PUBSUB]: () => t.request_kind_pubsub(),
  [RequestKind.CHAT]: () => t.request_kind_chat(),
  [RequestKind.CATALOG]: () => t.request_kind_catalog(),
  [RequestKind.DISCORD]: () => t.request_kind_discord(),
  [RequestKind.TELEGRAM]: () => t.request_kind_telegram(),
  [RequestKind.WEBHOOK]: () => t.request_kind_webhook(),
  [RequestKind.PICTURE]: () => t.request_kind_picture(),
  [RequestKind.UPDATE_CHECK]: () => t.request_kind_update_check(),
  [RequestKind.UPDATE_DOWNLOAD]: () => t.request_kind_update_download(),
  [RequestKind.PROXY_TEST]: () => t.request_kind_proxy_test(),
};

export const REQUEST_KIND_GROUP_LABELS: Record<RequestKindGroup, () => string> = {
  [RequestKindGroup.TWITCH]: () => t.requests_types_twitch(),
  [RequestKindGroup.OTHER]: () => t.requests_types_other(),
};

export const SOCKET_EVENT_LABELS: Record<SocketEvent, () => string> = {
  [SocketEvent.CONNECT]: () => t.request_socket_connect(),
  [SocketEvent.RECONNECT]: () => t.request_socket_reconnect(),
  [SocketEvent.CLOSE]: () => t.request_socket_close(),
};

export const OUTCOME_LABELS: Partial<Record<RequestOutcome, () => string>> = {
  [RequestOutcome.OK]: () => t.request_outcome_ok(),
  [RequestOutcome.TIMEOUT]: () => t.request_outcome_timeout(),
  [RequestOutcome.CONNECT]: () => t.request_outcome_connect(),
  [RequestOutcome.NETWORK]: () => t.request_outcome_network(),
  [RequestOutcome.GRAPHQL]: () => t.request_outcome_graphql(),
  [RequestOutcome.CLOSED]: () => t.request_outcome_closed(),
};

export const requestKey = (entry: RequestEntryType): string => `${entry.source}-${entry.id}`;

// ! A host and path wraps after a slash, not at the first hyphen of the host.
export const targetParts = (target: string): string[] =>
  target
    .split(TARGET_BREAK_AFTER)
    .map((part, index, parts) =>
      index < parts.length - 1 ? `${part}${TARGET_BREAK_AFTER}` : part,
    );

export const isOlder = (entry: RequestEntryType, than: RequestEntryType): boolean =>
  entry.source === than.source
    ? entry.id < than.id
    : entry.source === RequestSource.STORED && than.source === RequestSource.LIVE;

export const nextCursor = (page: RequestsResponseType): RequestCursorType | null => {
  if (page.next_live_id !== undefined) {
    return { before_live_id: page.next_live_id };
  }

  if (page.next_stored_id !== undefined) {
    return { before_stored_id: page.next_stored_id };
  }

  return null;
};

export const resultText = (entry: RequestEntryType): string => {
  const shownStatus =
    entry.outcome === RequestOutcome.OK || entry.outcome === RequestOutcome.STATUS;

  if (shownStatus && entry.status !== undefined) {
    return String(entry.status);
  }

  return OUTCOME_LABELS[entry.outcome]?.() ?? entry.outcome;
};
