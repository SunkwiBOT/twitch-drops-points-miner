// ! Only the answer to the text now in the field shows: an answer kept from an
// earlier text would let Enter add what the user no longer asks for.

import { onDestroy } from "svelte";

import { searchTwitch } from "../api/client";
import type { SearchResponseType } from "../bindings/SearchResponseType";
import { SEARCH_DEBOUNCE_MS, SEARCH_MIN_CHARS } from "./search.constants";

const NOTHING: SearchResponseType = { games: [], channels: [] };

type AnswerType = { text: string; found: SearchResponseType };

export class TwitchSearch {
  busy = $state(false);

  #text = $state("");
  #answer = $state<AnswerType>({ text: "", found: NOTHING });
  #timer: ReturnType<typeof setTimeout> | undefined;
  #latest = 0;
  #asked: string | null = null;

  constructor() {
    onDestroy(() => this.clear());
  }

  get results(): SearchResponseType {
    return this.#answer.text === this.#text ? this.#answer.found : NOTHING;
  }

  get answered(): boolean {
    return !this.busy && this.#text.length >= SEARCH_MIN_CHARS && this.#answer.text === this.#text;
  }

  update(query: string): void {
    this.#run(query, SEARCH_DEBOUNCE_MS);
  }

  now(query: string): void {
    this.#run(query, 0);
  }

  clear(): void {
    clearTimeout(this.#timer);
    this.#latest += 1;
    this.#asked = null;
    this.#text = "";
    this.#answer = { text: "", found: NOTHING };
    this.busy = false;
  }

  #run(query: string, delay: number): void {
    const text = query.trim();

    if (this.#asked === text && this.#text === text) {
      return;
    }

    clearTimeout(this.#timer);

    const id = ++this.#latest;

    this.#text = text;
    this.#asked = null;

    if (text.length < SEARCH_MIN_CHARS || this.#answer.text === text) {
      this.busy = false;

      return;
    }

    this.busy = true;
    this.#timer = setTimeout(async () => {
      this.#asked = text;

      try {
        const found = await searchTwitch(text);

        if (id === this.#latest) {
          this.#answer = { text, found };
        }
      } catch (e: any) {
        if (id === this.#latest) {
          this.#answer = { text: "", found: NOTHING };
        }
      } finally {
        if (id === this.#latest) {
          this.#asked = null;
          this.busy = false;
        }
      }
    }, delay);
  }
}
