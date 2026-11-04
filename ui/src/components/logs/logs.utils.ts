import { LogLevel } from "../../lib/bindings/LogLevel";
import { t } from "../../lib/i18n/i18n.svelte";

export const LOG_LEVEL_LABELS: Partial<Record<LogLevel, () => string>> = {
  [LogLevel.ERROR]: () => t.logs_level_error(),
  [LogLevel.WARN]: () => t.logs_level_warn(),
  [LogLevel.INFO]: () => t.logs_level_info(),
};
