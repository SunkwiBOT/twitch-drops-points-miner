import { m } from "../paraglide/messages.js";
import { setLocale } from "../paraglide/runtime.js";
import { isResolvedLanguage, Language, resolveSystemLocale } from "./i18n.constants";
import type { ResolvedLanguageType } from "./i18n.constants";

const systemLocale = (): ResolvedLanguageType =>
  resolveSystemLocale(navigator.languages.length ? navigator.languages : [navigator.language]);

export const locale = $state<{ preference: Language; current: ResolvedLanguageType }>({
  preference: Language.AUTO,
  current: systemLocale(),
});

export const applyLanguage = (preference: string): void => {
  const chosen = isResolvedLanguage(preference) ? preference : null;

  locale.preference = chosen ?? Language.AUTO;
  locale.current = chosen ?? systemLocale();
  void setLocale(locale.current, { reload: false });
  document.documentElement.lang = locale.current;
};

export const t: typeof m = new Proxy(m, {
  get(target, key: keyof typeof m) {
    return (
      inputs: Record<string, unknown> = {},
      options: { locale?: ResolvedLanguageType } = {},
    ): string =>
      Reflect.apply(target[key], undefined, [inputs, { locale: locale.current, ...options }]);
  },
});
