export const remember = (key: string, value: string): void => {
  try {
    localStorage.setItem(key, value);
  } catch (e: any) {
    // ! Storage can be off (private window); what it keeps is only a convenience.
  }
};

export const recall = (key: string): string | null => {
  try {
    return localStorage.getItem(key);
  } catch (e: any) {
    return null;
  }
};
