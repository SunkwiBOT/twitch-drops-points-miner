import type { NotificationEventsType } from "../../lib/bindings/NotificationEventsType";
import { NotificationTarget } from "../../lib/bindings/NotificationTarget";
import { t } from "../../lib/i18n/i18n.svelte";

export const NOTIFY_LABELS: Record<keyof NotificationEventsType, () => string> = {
  drop_claimed: () => t.notify_drop_claimed(),
  drop_progress: () => t.notify_drop_progress(),
  bonus_claimed: () => t.notify_bonus_claimed(),
  prediction_placed: () => t.notify_prediction_placed(),
  prediction_result: () => t.notify_prediction_result(),
  stream_online: () => t.notify_stream_online(),
  stream_offline: () => t.notify_stream_offline(),
  raid: () => t.notify_raid(),
  moment: () => t.notify_moment(),
  chat_mention: () => t.notify_chat_mention(),
  session_expired: () => t.notify_session_expired(),
  errors: () => t.notify_error(),
  update_available: () => t.notify_update_available(),
};

export const NOTIFY_KEYS = Object.keys(NOTIFY_LABELS).filter(
  (key): key is keyof NotificationEventsType => key in NOTIFY_LABELS,
);

export const TARGET_NAMES: Record<NotificationTarget, () => string> = {
  [NotificationTarget.DISCORD]: () => t.settings_discord(),
  [NotificationTarget.TELEGRAM]: () => t.settings_telegram(),
  [NotificationTarget.WEBHOOK]: () => t.settings_webhook(),
};
