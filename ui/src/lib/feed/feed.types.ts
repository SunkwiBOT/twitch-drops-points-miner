export type FeedPageType<ItemType, CursorType> = {
  items: ItemType[];
  next: CursorType | null;
};

export type FeedSourceType<QueryType, ItemType, CursorType> = {
  fetch: (
    query: QueryType,
    cursor: CursorType | null,
  ) => Promise<FeedPageType<ItemType, CursorType>>;
  isOlder: (item: ItemType, oldest: ItemType) => boolean;
};
