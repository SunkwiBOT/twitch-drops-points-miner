<script lang="ts">
  import PlusIcon from "@lucide/svelte/icons/plus";

  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { SEARCH_MIN_CHARS, SEARCH_SHOWN_MAX } from "../../lib/search/search.constants";
  import { TwitchSearch } from "../../lib/search/search.svelte";
  import { matchesSearch, normalizeSearch } from "../../lib/search/search.utils";
  import ChipRemoveButton from "./ChipRemoveButton.svelte";
  import ConfirmDialog from "./ConfirmDialog.svelte";
  import SearchField from "./SearchField.svelte";
  import SearchNoResult from "./SearchNoResult.svelte";
  import TextButton from "./TextButton.svelte";
  import {
    BADGE_VARIANTS,
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    BadgeVariant,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    KeyboardKey,
  } from "./ui.constants";

  type TagInputPropsType = {
    id: string;
    items: string[];
    placeholder: string;
    suggestions: string[];
    addLabel: string;
    removeLabel: string;
    confirmTitle: string;
    confirmText: (item: string) => string;
    onchange: (items: string[]) => void;
    remote: boolean;
  };

  type TagSuggestionType = { label: string; image: string | undefined };

  let {
    id,
    items,
    placeholder,
    suggestions,
    addLabel,
    removeLabel,
    confirmTitle,
    confirmText,
    onchange,
    remote,
  }: TagInputPropsType = $props();

  let draft = $state("");
  const twitch = new TwitchSearch();

  const matches = $derived.by((): TagSuggestionType[] => {
    if (draft.trim().length < SEARCH_MIN_CHARS) {
      return [];
    }

    const needle = normalizeSearch(draft);
    const taken = new Set(items.map((item) => item.toLowerCase()));
    const out: TagSuggestionType[] = [];

    const push = (suggestion: TagSuggestionType): void => {
      const key = suggestion.label.toLowerCase();

      if (!taken.has(key)) {
        taken.add(key);
        out.push(suggestion);
      }
    };

    const found = twitch.results.games.map((game) => ({
      label: game.name,
      image: game.box_art_url,
    }));
    const pictures = new Map(found.map((s) => [s.label.toLowerCase(), s.image]));

    suggestions
      .filter((item) => matchesSearch(item, needle))
      .forEach((label) => push({ label, image: pictures.get(label.toLowerCase()) }));
    found.forEach(push);

    return out.slice(0, SEARCH_SHOWN_MAX);
  });

  const add = (value: string): void => {
    const item = value.trim();

    draft = "";
    twitch.clear();

    if (!item || items.some((known) => known.toLowerCase() === item.toLowerCase())) {
      return;
    }

    onchange([...items, item]);
  };

  // ! An IME's Enter only ends the word being composed, and while Twitch answers
  // the first suggestion is not known yet: both add nothing.
  const onkeydown = (event: KeyboardEvent): void => {
    if (event.key !== KeyboardKey.ENTER || event.isComposing) {
      return;
    }

    event.preventDefault();

    if (!twitch.busy) {
      add(matches.at(0)?.label ?? draft);
    }
  };

  let removing = $state<string | null>(null);

  const remove = (): void => {
    const item = removing;
    removing = null;

    if (item !== null) {
      onchange(items.filter((known) => known !== item));
    }
  };
</script>

<div class="flex flex-col gap-2">
  {#if items.length}
    <div class="flex flex-wrap gap-1.5">
      {#each items as item (item)}
        <Badge
          variant={BADGE_VARIANTS[BadgeVariant.CHIP]}
          class="h-6 gap-1 pr-0.5 text-hint font-medium"
        >
          {item}
          <ChipRemoveButton
            label={removeLabel}
            name={item}
            iconSize={12}
            onclick={() => (removing = item)}
          />
        </Badge>
      {/each}
    </div>
  {/if}
  <div class="flex gap-2">
    {#if remote}
      <div class="min-w-0 flex-1">
        <SearchField
          {id}
          {placeholder}
          bind:value={draft}
          search={null}
          oninput={() => twitch.update(draft)}
          {onkeydown}
        />
      </div>
    {:else}
      <Input {id} class="min-w-0 flex-1" {placeholder} bind:value={draft} {onkeydown} />
    {/if}
    <TextButton
      label={addLabel}
      icon={IconName.PLUS}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={!draft.trim()}
      onclick={() => add(draft)}
    />
  </div>
  {#if matches.length}
    <div class="flex flex-wrap gap-1.5">
      {#each matches as match (match.label)}
        <Button
          variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
          size={BUTTON_SIZES[ButtonSize.SM]}
          type={ButtonType.BUTTON}
          onclick={() => add(match.label)}
        >
          {#if match.image}
            <img
              class="h-5 w-3.75 shrink-0 rounded-sm object-cover"
              src={boxArt(match.image, BoxArtSize.ROW)}
              alt=""
            />
          {:else}
            <PlusIcon />
          {/if}
          {match.label}
        </Button>
      {/each}
    </div>
  {/if}
  {#if remote}<SearchNoResult shown={matches.length === 0 && twitch.answered} query={draft} />{/if}
</div>

<ConfirmDialog
  open={removing !== null}
  title={confirmTitle}
  text={confirmText(removing ?? "")}
  confirmLabel={t.confirm_remove()}
  busyLabel={t.common_removing()}
  busy={false}
  onconfirm={remove}
  oncancel={() => (removing = null)}
/>
