import axios from "axios";

import { toApiError } from "../errors/errors.utils";
import { REQUEST_TIMEOUT_MS } from "./api.constants";

type QueryValueType = string | number | boolean | readonly (string | number)[] | null | undefined;

const queryString = (params: Record<string, QueryValueType>): string => {
  const query = new URLSearchParams();

  for (const [key, value] of Object.entries(params)) {
    if (value === undefined || value === null) {
      continue;
    }

    query.append(key, Array.isArray(value) ? value.join(",") : String(value));
  }

  return query.toString();
};

export const http = axios.create({
  timeout: REQUEST_TIMEOUT_MS,
  headers: { Accept: "application/json" },
  paramsSerializer: { serialize: queryString },
});

http.interceptors.response.use(undefined, (e: unknown) => Promise.reject(toApiError(e)));
