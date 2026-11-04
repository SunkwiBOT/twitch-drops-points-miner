import { ACCOUNT_PAGES, ACCOUNT_PATH, Page, PAGE_PATHS } from "./state.constants";

export type RouteType = { page: Page; login: string | null };

export const routePath = (page: Page, login: string | null): string => {
  if (!ACCOUNT_PAGES.has(page)) {
    return PAGE_PATHS[page];
  }

  return `${ACCOUNT_PATH}/${encodeURIComponent(login ?? "")}${PAGE_PATHS[page]}`;
};

export const parseRoute = (path: string): RouteType | null => {
  const clean = path.length > 1 ? path.replace(/\/+$/, "") : path;
  const prefix = `${ACCOUNT_PATH}/`;

  if (!clean.startsWith(prefix)) {
    const page = Object.values(Page).find(
      (candidate) => !ACCOUNT_PAGES.has(candidate) && PAGE_PATHS[candidate] === clean,
    );

    return page ? { page, login: null } : null;
  }

  const [encoded = "", ...rest] = clean.slice(prefix.length).split("/");
  let login: string;

  try {
    login = decodeURIComponent(encoded).toLowerCase();
  } catch (e: any) {
    return null;
  }

  const tail = rest.length > 0 ? `/${rest.join("/")}` : "";
  const page = [...ACCOUNT_PAGES].find((candidate) => PAGE_PATHS[candidate] === tail);

  return login && page ? { page, login } : null;
};
