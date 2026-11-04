import { untrack } from "svelte";

import { describeError } from "../errors/errors.utils";
import { ToastKind } from "../state/state.constants";
import { toast } from "../state/state.svelte";
import type { FeedSourceType } from "./feed.types";

export class PagedFeed<QueryType, ItemType, CursorType> {
  items = $state<ItemType[]>([]);
  loading = $state(true);
  loadingMore = $state(false);
  problem = $state<string | null>(null);

  #cursor = $state<CursorType | null>(null);
  readonly hasMore = $derived(this.#cursor !== null);

  #source: FeedSourceType<QueryType, ItemType, CursorType>;
  #query: QueryType | null = null;
  #key: string | null = null;
  // ! Bumped by every new query: an answer to an older one is dropped, so a slow
  // page of the previous filters never lands in the new list.
  #generation = 0;
  #refreshing: number | null = null;
  #again = false;

  constructor(source: FeedSourceType<QueryType, ItemType, CursorType>) {
    this.#source = source;
  }

  show(query: QueryType): Promise<void> {
    return untrack(() => {
      const key = JSON.stringify(query);

      if (key !== this.#key) {
        this.#key = key;
        this.#query = query;
        this.#generation += 1;
        this.#refreshing = null;
        this.#again = false;
        this.items = [];
        this.#cursor = null;
        this.problem = null;
        this.loading = true;
        this.loadingMore = false;
      }

      return this.refresh();
    });
  }

  async refresh(): Promise<void> {
    const asked = this.#generation;
    const query = this.#query;

    if (query === null) {
      return;
    }

    // ! One request at a time: a refresh asked meanwhile runs once that one is back,
    // so a slow answer never overwrites a newer one and no new event is missed.
    if (this.#refreshing === asked) {
      this.#again = true;

      return;
    }

    this.#refreshing = asked;

    try {
      const page = await this.#source.fetch(query, null);

      if (asked !== this.#generation) {
        return;
      }

      const oldest = page.items.at(-1);
      const older = oldest ? this.items.filter((item) => this.#source.isOlder(item, oldest)) : [];

      this.items = [...page.items, ...older];
      this.problem = null;

      if (older.length === 0) {
        this.#cursor = page.next;
      }
    } catch (e: any) {
      if (asked === this.#generation) {
        this.problem = describeError(e);
      }
    } finally {
      if (asked === this.#generation) {
        this.#refreshing = null;
        this.loading = false;

        if (this.#takeAgain()) {
          void this.refresh();
        }
      }
    }
  }

  #takeAgain(): boolean {
    const again = this.#again;

    this.#again = false;

    return again;
  }

  loadMore = async (): Promise<void> => {
    const asked = this.#generation;
    const query = this.#query;
    const cursor = this.#cursor;

    if (query === null || cursor === null || this.loadingMore) {
      return;
    }

    this.loadingMore = true;

    try {
      const page = await this.#source.fetch(query, cursor);

      if (asked !== this.#generation) {
        return;
      }

      this.items = [...this.items, ...page.items];
      this.#cursor = page.next;
    } catch (e: any) {
      if (asked === this.#generation) {
        toast(ToastKind.ERROR, describeError(e));
      }
    } finally {
      if (asked === this.#generation) {
        this.loadingMore = false;
      }
    }
  };
}
