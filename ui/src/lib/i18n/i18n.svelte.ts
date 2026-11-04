import type { m } from "../paraglide/messages.js";
import { KEY_STORAGE_LANGUAGE } from "../state/storage.constants";
import { recall, remember } from "../state/storage.utils";
import { isResolvedLanguage, Language, resolveSystemLocale } from "./i18n.constants";
import type { ResolvedLanguageType } from "./i18n.constants";

type LanguageMessagesType = Record<keyof typeof m, (inputs: never) => string>;

// ! Not Paraglide's `m`: it imports every language into the main chunk.
const LOADERS: Record<ResolvedLanguageType, () => Promise<LanguageMessagesType>> = {
  [Language.EN]: () => import("../paraglide/messages/en.js"),
  [Language.FR]: () => import("../paraglide/messages/fr.js"),
  [Language.DE]: () => import("../paraglide/messages/de.js"),
  [Language.ESES]: () => import("../paraglide/messages/es-ES.js"),
  [Language.ES419]: () => import("../paraglide/messages/es-419.js"),
  [Language.PTPT]: () => import("../paraglide/messages/pt-PT.js"),
  [Language.PTBR]: () => import("../paraglide/messages/pt-BR.js"),
  [Language.IT]: () => import("../paraglide/messages/it.js"),
  [Language.NL]: () => import("../paraglide/messages/nl.js"),
  [Language.PL]: () => import("../paraglide/messages/pl.js"),
  [Language.RU]: () => import("../paraglide/messages/ru.js"),
  [Language.UK]: () => import("../paraglide/messages/uk.js"),
  [Language.CS]: () => import("../paraglide/messages/cs.js"),
  [Language.HU]: () => import("../paraglide/messages/hu.js"),
  [Language.RO]: () => import("../paraglide/messages/ro.js"),
  [Language.BG]: () => import("../paraglide/messages/bg.js"),
  [Language.EL]: () => import("../paraglide/messages/el.js"),
  [Language.TR]: () => import("../paraglide/messages/tr.js"),
  [Language.SV]: () => import("../paraglide/messages/sv.js"),
  [Language.DA]: () => import("../paraglide/messages/da.js"),
  [Language.NB]: () => import("../paraglide/messages/nb.js"),
  [Language.FI]: () => import("../paraglide/messages/fi.js"),
  [Language.ZHCN]: () => import("../paraglide/messages/zh-CN.js"),
  [Language.ZHTW]: () => import("../paraglide/messages/zh-TW.js"),
  [Language.JA]: () => import("../paraglide/messages/ja.js"),
  [Language.KO]: () => import("../paraglide/messages/ko.js"),
  [Language.TH]: () => import("../paraglide/messages/th.js"),
  [Language.VI]: () => import("../paraglide/messages/vi.js"),
  [Language.ID]: () => import("../paraglide/messages/id.js"),
};

const loaded = new Map<ResolvedLanguageType, LanguageMessagesType>();

const systemLocale = (): ResolvedLanguageType =>
  resolveSystemLocale(navigator.languages.length ? navigator.languages : [navigator.language]);

const lastShown = (): ResolvedLanguageType | null => {
  const kept = recall(KEY_STORAGE_LANGUAGE);

  return kept !== null && isResolvedLanguage(kept) ? kept : null;
};

export const locale = $state<{ preference: Language; current: ResolvedLanguageType }>({
  preference: Language.AUTO,
  current: lastShown() ?? systemLocale(),
});

let wanted: ResolvedLanguageType | null = null;

const loadLanguage = async (language: ResolvedLanguageType): Promise<void> => {
  if (!loaded.has(language)) {
    loaded.set(language, await LOADERS[language]());
  }
};

const show = (language: ResolvedLanguageType): void => {
  document.documentElement.lang = language;

  if (locale.current !== language) {
    locale.current = language;
    remember(KEY_STORAGE_LANGUAGE, language);
  }
};

// ! English stands in for a language whose chunk did not load: the page mounts
// either way.
export const prepareLanguage = async (): Promise<void> => {
  for (const language of new Set<ResolvedLanguageType>([locale.current, Language.EN])) {
    try {
      await loadLanguage(language);
      wanted = language;
      show(language);

      return;
    } catch (e: any) {
      console.error("language not loaded", language, e);
    }
  }
};

// ! The language shown changes once its chunk is in: never a message missing.
// A language that did not load is tried again only once another one is chosen.
export const applyLanguage = async (preference: string): Promise<boolean> => {
  const chosen = isResolvedLanguage(preference) ? preference : null;
  const language = chosen ?? systemLocale();

  locale.preference = chosen ?? Language.AUTO;

  if (language === wanted) {
    return true;
  }

  wanted = language;

  if (loaded.has(language)) {
    show(language);

    return true;
  }

  try {
    await loadLanguage(language);
  } catch (e: any) {
    console.error("language not loaded", language, e);

    return wanted !== language;
  }

  if (wanted === language) {
    show(language);
  }

  return true;
};

const message =
  (key: keyof typeof m) =>
  (inputs: Record<string, unknown> = {}): string => {
    const messages = loaded.get(locale.current);

    return messages ? Reflect.apply(messages[key], undefined, [inputs]) : "";
  };

export const t = new Proxy({} as typeof m, {
  get(_target, key: keyof typeof m) {
    return message(key);
  },
});
