// ! Tags mirror the Paraglide locales in project.inlang/settings.json.
export enum Language {
  AUTO = "AUTO",
  EN = "en",
  FR = "fr",
  DE = "de",
  ESES = "es-ES",
  ES419 = "es-419",
  PTPT = "pt-PT",
  PTBR = "pt-BR",
  IT = "it",
  NL = "nl",
  PL = "pl",
  RU = "ru",
  UK = "uk",
  CS = "cs",
  HU = "hu",
  RO = "ro",
  BG = "bg",
  EL = "el",
  TR = "tr",
  SV = "sv",
  DA = "da",
  NB = "nb",
  FI = "fi",
  ZHCN = "zh-CN",
  ZHTW = "zh-TW",
  JA = "ja",
  KO = "ko",
  TH = "th",
  VI = "vi",
  ID = "id",
}

export type ResolvedLanguageType = Exclude<Language, Language.AUTO>;

export const RESOLVED_LANGUAGES: readonly ResolvedLanguageType[] = Object.values(Language).filter(
  (value): value is ResolvedLanguageType => value !== Language.AUTO,
);

const RESOLVED_TAGS: ReadonlySet<string> = new Set(RESOLVED_LANGUAGES);

export const isResolvedLanguage = (value: string): value is ResolvedLanguageType =>
  RESOLVED_TAGS.has(value);

export const LANGUAGE_NAMES: Record<ResolvedLanguageType, string> = {
  [Language.EN]: "English",
  [Language.FR]: "Français",
  [Language.DE]: "Deutsch",
  [Language.ESES]: "Español (España)",
  [Language.ES419]: "Español (Latinoamérica)",
  [Language.PTPT]: "Português (Portugal)",
  [Language.PTBR]: "Português (Brasil)",
  [Language.IT]: "Italiano",
  [Language.NL]: "Nederlands",
  [Language.PL]: "Polski",
  [Language.RU]: "Русский",
  [Language.UK]: "Українська",
  [Language.CS]: "Čeština",
  [Language.HU]: "Magyar",
  [Language.RO]: "Română",
  [Language.BG]: "Български",
  [Language.EL]: "Ελληνικά",
  [Language.TR]: "Türkçe",
  [Language.SV]: "Svenska",
  [Language.DA]: "Dansk",
  [Language.NB]: "Norsk bokmål",
  [Language.FI]: "Suomi",
  [Language.ZHCN]: "简体中文",
  [Language.ZHTW]: "繁體中文",
  [Language.JA]: "日本語",
  [Language.KO]: "한국어",
  [Language.TH]: "ไทย",
  [Language.VI]: "Tiếng Việt",
  [Language.ID]: "Bahasa Indonesia",
};

export const resolveSystemLocale = (candidates: readonly string[]): ResolvedLanguageType => {
  for (const candidate of candidates) {
    const lower = candidate.toLowerCase();
    const exact = RESOLVED_LANGUAGES.find((tag) => tag.toLowerCase() === lower);

    if (exact) return exact;
    if (lower.startsWith("es")) return lower.includes("es-es") ? Language.ESES : Language.ES419;
    if (lower.startsWith("pt")) return lower.includes("br") ? Language.PTBR : Language.PTPT;

    if (lower.startsWith("zh"))
      return lower.includes("tw") || lower.includes("hant") || lower.includes("hk")
        ? Language.ZHTW
        : Language.ZHCN;

    if (lower.startsWith("no") || lower.startsWith("nn")) return Language.NB;

    const base = lower.split("-")[0];
    const match = RESOLVED_LANGUAGES.find((tag) => tag.toLowerCase() === base);

    if (match) return match;
  }

  return Language.EN;
};
