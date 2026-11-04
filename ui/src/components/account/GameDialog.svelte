<script lang="ts">
  import TextButton from "../ui/TextButton.svelte";
  import type { GameType } from "../../lib/bindings/GameType";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt, number } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { SEARCH_MIN_CHARS, SEARCH_SHOWN_MAX } from "../../lib/search/search.constants";
  import { TwitchSearch } from "../../lib/search/search.svelte";
  import { matchesSearch, normalizeSearch } from "../../lib/search/search.utils";
  import Dialog from "../ui/Dialog.svelte";
  import List from "../ui/List.svelte";
  import ListRow from "../ui/ListRow.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import SearchNoResult from "../ui/SearchNoResult.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import {
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    ThumbShape,
    ThumbSize,
  } from "../ui/ui.constants";

  type GameDialogPropsType = {
    open: boolean;
    catalogue: { game: GameType; drops: number }[];
    taken: string[];
    onadd: (name: string) => void;
    onclose: () => void;
  };

  type OfferType = { name: string; art: string | undefined; drops: number };

  let { open, catalogue, taken, onadd, onclose }: GameDialogPropsType = $props();

  let query = $state("");
  const twitch = new TwitchSearch();

  $effect(() => {
    if (!open) {
      query = "";
      twitch.clear();
    }
  });

  const isTaken = (name: string): boolean =>
    taken.some((known) => known.toLowerCase() === name.toLowerCase());

  const offers = $derived.by((): OfferType[] => {
    if (query.trim().length < SEARCH_MIN_CHARS) {
      return [];
    }

    const needle = normalizeSearch(query);

    const out: OfferType[] = [];
    const seen = new Set<string>();

    const push = (offer: OfferType): void => {
      const key = offer.name.toLowerCase();

      if (!seen.has(key)) {
        seen.add(key);
        out.push(offer);
      }
    };

    catalogue
      .filter(({ game }) => matchesSearch(game.name, needle))
      .forEach(({ game, drops }) => push({ name: game.name, art: game.box_art_url, drops }));

    const dropsOf = (name: string): number =>
      catalogue.find(({ game }) => game.name.toLowerCase() === name.toLowerCase())?.drops ?? 0;

    twitch.results.games.forEach((game) =>
      push({ name: game.name, art: game.box_art_url, drops: dropsOf(game.name) }),
    );

    return out.slice(0, SEARCH_SHOWN_MAX);
  });

  const add = (name: string): void => {
    const game = name.trim();

    if (game && !isTaken(game)) {
      onadd(game);
    }
  };
</script>

<Dialog
  {open}
  title={t.drops_add_game()}
  wide={false}
  closable={true}
  closeLabel={t.common_close()}
  {onclose}
>
  <form
    class="flex flex-col gap-2"
    onsubmit={(event) => {
      event.preventDefault();
      // ! Only real games go in: Enter picks the first suggestion, never the typed text.
      const first = offers.find((offer) => !isTaken(offer.name));

      if (first) {
        add(first.name);
      }
    }}
  >
    <p class="text-hint text-muted-foreground">{t.drops_add_game_hint()}</p>
    <SearchField
      placeholder={t.drops_add_game_placeholder()}
      aria-label={t.drops_add_game_placeholder()}
      bind:value={query}
      search={{ label: t.common_search(), busy: twitch.busy, run: () => twitch.now(query) }}
      oninput={() => twitch.update(query)}
    />
    {#if offers.length}
      <List>
        {#each offers as offer (offer.name)}
          {@const already = isTaken(offer.name)}
          <ListRow
            title={offer.name}
            details={[
              offer.drops
                ? t.game_drops_available({ count: offer.drops, shown: strong(number(offer.drops)) })
                : "",
              already ? t.import_entry_already() : "",
            ].filter(Boolean)}
            onopen={already ? undefined : () => add(offer.name)}
            muted={already}
          >
            {#snippet lead()}
              <Thumb
                src={boxArt(offer.art, BoxArtSize.ROW)}
                round={false}
                contain={false}
                size={ThumbSize.MD}
                shape={ThumbShape.BOX_ART}
                fallback={IconName.GIFT}
              />
            {/snippet}
            {#snippet trailing()}
              {#if !already}
                <TextButton
                  label={t.search_pick()}
                  variant={ButtonVariant.OUTLINE}
                  size={ButtonSize.SM}
                  type={ButtonType.BUTTON}
                  disabled={false}
                  onclick={() => add(offer.name)}
                />
              {/if}
            {/snippet}
          </ListRow>
        {/each}
      </List>
    {/if}
    <SearchNoResult shown={offers.length === 0 && twitch.answered} {query} />
  </form>
  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={onclose}
    />
  {/snippet}
</Dialog>
