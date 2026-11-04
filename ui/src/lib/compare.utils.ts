// ! Compared as JSON: both values must list their keys in the same order, as
// copies of one shape do.
export const sameValue = (a: unknown, b: unknown): boolean =>
  JSON.stringify(a) === JSON.stringify(b);
