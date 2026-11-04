import { normalizeSearch } from "../../lib/search/search.utils";

// ! The command list's default fuzzy match keeps rows that share no word with the query.
export const matchOption = (_value: string, search: string, keywords?: string[]): number => {
  const query = normalizeSearch(search);

  if (!query) {
    return 1;
  }

  const label = normalizeSearch((keywords ?? []).join(" "));

  if (label.startsWith(query)) {
    return 1;
  }

  return label.includes(query) ? 0.5 : 0;
};

export const segmentId = (panel: string, value: string): string => `${panel}-${value}`;
