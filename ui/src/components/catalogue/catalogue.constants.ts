import { DAY_MS, HOUR_MS } from "../../lib/time/time.constants";
import { BadgeTone } from "../ui/ui.constants";

export enum CatalogueView {
  TIMELINE = "TIMELINE",
  CARDS = "CARDS",
}

export const CATALOGUE_VIEW_SLUGS: Record<CatalogueView, string> = {
  [CatalogueView.TIMELINE]: "timeline",
  [CatalogueView.CARDS]: "cards",
};

export enum CatalogueSort {
  ENDING = "ENDING",
  NEW = "NEW",
  GAME = "GAME",
}

export const URGENT_MS = 3 * HOUR_MS;
export const URGENT_TONE = BadgeTone.DANGER;
export const NEW_MS = DAY_MS;
