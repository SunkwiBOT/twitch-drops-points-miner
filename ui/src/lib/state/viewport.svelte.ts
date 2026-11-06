import { MediaQuery } from "svelte/reactivity";

import { WIDE_LAYOUT_QUERY } from "./state.constants";

export const wideLayout = new MediaQuery(WIDE_LAYOUT_QUERY);
