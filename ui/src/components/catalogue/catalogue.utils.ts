import { t } from "../../lib/i18n/i18n.svelte";
import { TimelineZoom } from "./timeline.constants";

export const ZOOM_LABELS: Record<TimelineZoom, () => string> = {
  [TimelineZoom.DAY]: () => t.timeline_zoom_day(),
  [TimelineZoom.THREE_DAYS]: () => t.timeline_zoom_three_days(),
  [TimelineZoom.TWO_WEEKS]: () => t.timeline_zoom_two_weeks(),
};
