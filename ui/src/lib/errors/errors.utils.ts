import axios from "axios";

import type { ApiErrorType } from "../bindings/ApiErrorType";
import { ErrorApiMessage } from "../bindings/ErrorApiMessage";
import { t } from "../i18n/i18n.svelte";

class ApiError extends Error {
  readonly code: ErrorApiMessage;

  constructor(code: ErrorApiMessage) {
    super(code);
    this.code = code;
  }
}

const CODES: ReadonlySet<string> = new Set(Object.values(ErrorApiMessage));

const isErrorCode = (value: unknown): value is ErrorApiMessage =>
  typeof value === "string" && CODES.has(value);

const codeOf = (payload: unknown): ErrorApiMessage | null =>
  typeof payload === "object" && payload !== null && "code" in payload && isErrorCode(payload.code)
    ? payload.code
    : null;

export const toApiError = (error: unknown): ApiError => {
  if (error instanceof ApiError) {
    return error;
  }

  if (axios.isAxiosError<ApiErrorType>(error)) {
    if (!error.response) {
      return new ApiError(ErrorApiMessage.SERVER_UNREACHABLE);
    }

    return new ApiError(codeOf(error.response.data) ?? ErrorApiMessage.UNKNOWN_ERROR);
  }

  return new ApiError(codeOf(error) ?? ErrorApiMessage.UNKNOWN_ERROR);
};

const MESSAGES: Record<ErrorApiMessage, (code: ErrorApiMessage) => string> = {
  [ErrorApiMessage.NETWORK_ERROR]: () => t.error_network(),
  [ErrorApiMessage.TWITCH_UNAVAILABLE]: () => t.error_twitch_unavailable(),
  [ErrorApiMessage.SESSION_EXPIRED]: () => t.error_session_expired(),
  [ErrorApiMessage.INVALID_RESPONSE]: () => t.error_invalid_response(),
  [ErrorApiMessage.TWITCH_REJECTED]: () => t.error_twitch_rejected(),
  [ErrorApiMessage.DEVICE_CODE_EXPIRED]: () => t.error_device_code_expired(),
  [ErrorApiMessage.DEVICE_CODE_PENDING]: () => t.error_device_code_pending(),
  [ErrorApiMessage.STORAGE_ERROR]: () => t.error_storage(),
  [ErrorApiMessage.VAULT_LOCKED]: () => t.error_vault_locked(),
  [ErrorApiMessage.WRONG_PASSPHRASE]: () => t.error_wrong_passphrase(),
  [ErrorApiMessage.INVALID_VAULT]: () => t.error_invalid_vault(),
  [ErrorApiMessage.UNKNOWN_ACCOUNT]: () => t.error_unknown_account(),
  [ErrorApiMessage.UNKNOWN_CHANNEL]: () => t.error_unknown_channel(),
  [ErrorApiMessage.INVALID_PROXY]: () => t.error_invalid_proxy(),
  [ErrorApiMessage.INVALID_STREAMER_LOGIN]: () => t.error_invalid_streamer_login(),
  [ErrorApiMessage.PASSPHRASE_REQUIRED]: () => t.error_passphrase_required(),
  [ErrorApiMessage.NOTIFICATION_FAILED]: () => t.error_notification_failed(),
  [ErrorApiMessage.BEACON_BLOCKED]: () => t.beacon_blocked(),
  [ErrorApiMessage.CATALOG_RATE_LIMITED]: () => t.error_catalog_rate_limited(),
  [ErrorApiMessage.UNAUTHORIZED]: () => t.error_dashboard_unauthorized(),
  [ErrorApiMessage.WRONG_PASSWORD]: () => t.auth_wrong(),
  [ErrorApiMessage.TOO_MANY_ATTEMPTS]: () => t.error_too_many_attempts(),
  [ErrorApiMessage.HTTPS_REQUIRED]: () => t.error_https_required(),
  [ErrorApiMessage.FORWARDED_FOR_REQUIRED]: () => t.error_forwarded_for_required(),
  [ErrorApiMessage.CROSS_SITE_REQUEST]: () => t.error_cross_site_request(),
  [ErrorApiMessage.UPDATE_SIGNATURE_REFUSED]: () => t.error_update_signature_refused(),
  [ErrorApiMessage.UPDATE_INSTALL_FAILED]: () => t.error_update_install_failed(),
  [ErrorApiMessage.INVALID_REQUEST]: () => t.error_invalid_request(),
  [ErrorApiMessage.NOT_FOUND]: () => t.error_not_found(),
  [ErrorApiMessage.NOT_SUPPORTED]: () => t.error_not_supported(),
  [ErrorApiMessage.SERVER_UNREACHABLE]: () => t.error_server_unreachable(),
  [ErrorApiMessage.UNKNOWN_ERROR]: (code) => t.error_unknown({ code }),
};

export const describeCode = (code: ErrorApiMessage): string => MESSAGES[code](code);

export const describeError = (error: unknown): string => describeCode(toApiError(error).code);
