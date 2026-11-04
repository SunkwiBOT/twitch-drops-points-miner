import { writeText } from "@tauri-apps/plugin-clipboard-manager";

import { isTauri } from "./api/tauri";

export const copyText = async (text: string): Promise<boolean> => {
  if (isTauri()) {
    try {
      await writeText(text);

      return true;
    } catch (e: any) {
      console.warn("Tauri clipboard refused the text, trying the browser's", e);
    }
  }

  try {
    await navigator.clipboard.writeText(text);

    return true;
  } catch (e: any) {
    // ! Over plain HTTP (the dashboard on a home network) the browser has no
    // clipboard API: the old way still works. The field goes inside the open
    // dialog, whose focus trap would otherwise take the selection away.
    const host = document.activeElement?.closest('[role="dialog"]') ?? document.body;
    const area = document.createElement("textarea");

    area.value = text;
    area.readOnly = true;
    area.style.position = "fixed";
    area.style.opacity = "0";
    host.appendChild(area);
    area.focus();
    area.select();

    let copied = false;

    try {
      copied = document.execCommand("copy");
    } catch (e: any) {
      console.warn("the browser refused to copy", e);
    }

    area.remove();

    return copied;
  }
};
