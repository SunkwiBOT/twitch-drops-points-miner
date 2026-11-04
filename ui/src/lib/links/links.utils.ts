import { openLink } from "../state/state.svelte";

// ! The page never follows an external link itself: the desktop app opens it in the
// system browser instead of navigating its own window away.
export const followLink = (event: MouseEvent, url: string): void => {
  event.preventDefault();
  void openLink(url);
};
