// ! The lowering ignores the language, so a Turkish page still finds "INSIDE" from
// "inside"; the accents go after the split (NFD) and the rest comes back together
// (NFC), so Korean and Japanese letters stay whole.
export const normalizeSearch = (text: string): string =>
  text
    .trim()
    .normalize("NFD")
    .replace(/[̀-ͯ]/g, "")
    .normalize("NFC")
    .toLowerCase()
    .replace(/ı/g, "i");

export const matchesSearch = (text: string, needle: string): boolean =>
  normalizeSearch(text).includes(needle);
