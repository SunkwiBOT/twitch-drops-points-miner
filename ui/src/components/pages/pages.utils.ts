import { LogLevel } from "../../lib/bindings/LogLevel";
import { t } from "../../lib/i18n/i18n.svelte";
import { TimelineZoom } from "../catalogue/timeline.constants";

export const LOG_LEVEL_LABELS: Partial<Record<LogLevel, () => string>> = {
  [LogLevel.ERROR]: () => t.logs_level_error(),
  [LogLevel.WARN]: () => t.logs_level_warn(),
  [LogLevel.INFO]: () => t.logs_level_info(),
};

export const ZOOM_LABELS: Record<TimelineZoom, () => string> = {
  [TimelineZoom.DAY]: () => t.timeline_zoom_day(),
  [TimelineZoom.THREE_DAYS]: () => t.timeline_zoom_three_days(),
  [TimelineZoom.TWO_WEEKS]: () => t.timeline_zoom_two_weeks(),
};
