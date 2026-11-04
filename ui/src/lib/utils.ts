import { type ClassValue, clsx } from "clsx";
import { extendTailwindMerge } from "tailwind-merge";

// ! The theme's text roles are font sizes: merging them keeps the colour. Its
// named widths join the container scale, so `w-toast` replaces another width.
const twMerge = extendTailwindMerge({
  extend: {
    theme: {
      container: ["toast"],
    },
    classGroups: {
      "font-size": [
        {
          text: [
            "hint",
            "tiny",
            "nav",
            "nav-title",
            "initial-xs",
            "initial-sm",
            "live-tag-sm",
            "live-tag",
          ],
        },
      ],
    },
  },
});

export const cn = (...inputs: ClassValue[]): string => twMerge(clsx(inputs));

// oxlint-disable-next-line no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// oxlint-disable-next-line no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };
