<script lang="ts">
  import type { ChannelType } from "../../../lib/bindings/ChannelType";
  import type { SearchChannelType } from "../../../lib/bindings/SearchChannelType";
  import { searchedChannel } from "../../../lib/channels/channels.svelte";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { SEARCH_SHOWN_MAX } from "../../../lib/search/search.constants";
  import { TwitchSearch } from "../../../lib/search/search.svelte";
  import ChannelAvatar from "../../ui/ChannelAvatar.svelte";
  import Icon from "../../ui/Icon.svelte";
  import List from "../../ui/List.svelte";
  import ListRow from "../../ui/ListRow.svelte";
  import SearchField from "../../ui/SearchField.svelte";
  import SearchNoResult from "../../ui/SearchNoResult.svelte";
  import TextButton from "../../ui/TextButton.svelte";
  import {
    AvatarSize,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
  } from "../../ui/ui.constants";

  type SearchImportPropsType = {
    open: boolean;
    existing: string[];
    onadd: (logins: string[], channels: ChannelType[]) => void;
  };

  let { open, existing, onadd }: SearchImportPropsType = $props();

  let query = $state("");
  const twitch = new TwitchSearch();
  const suggested = $derived(twitch.results.channels.slice(0, SEARCH_SHOWN_MAX));
  const listed = (login: string): boolean => existing.includes(login);

  const addFound = (found: SearchChannelType): void => {
    if (!listed(found.login)) {
      onadd([found.login], [searchedChannel(found)]);
    }
  };

  const addFirst = (event: SubmitEvent): void => {
    event.preventDefault();
    const first = suggested.find((s) => !listed(s.login));

    if (first) {
      addFound(first);
    }
  };
</script>

{#if open}
  <form class="flex flex-col gap-2" onsubmit={addFirst}>
    <SearchField
      placeholder={t.account_points_add_placeholder()}
      aria-label={t.account_points_add_placeholder()}
      bind:value={query}
      search={{ label: t.common_search(), busy: twitch.busy, run: () => twitch.now(query) }}
      oninput={() => twitch.update(query)}
    />
    {#if suggested.length}
      <List>
        {#each suggested as s (s.login)}
          <ListRow
            title={s.display_name}
            subtitle={listed(s.login) ? t.import_entry_already() : undefined}
            onopen={listed(s.login) ? undefined : () => addFound(s)}
            muted={listed(s.login)}
          >
            {#snippet lead()}
              <ChannelAvatar
                url={s.avatar_url}
                name={s.display_name}
                size={AvatarSize.MD}
                live={s.live}
              />
            {/snippet}
            {#snippet badges()}
              {#if s.verified}<span
                  class="inline-flex text-primary"
                  role="img"
                  aria-label={t.search_verified()}
                  title={t.search_verified()}><Icon name={IconName.VERIFIED} size={16} /></span
                >{/if}
            {/snippet}
            {#snippet trailing()}
              {#if !listed(s.login)}
                <TextButton
                  label={t.common_add()}
                  icon={IconName.PLUS}
                  variant={ButtonVariant.OUTLINE}
                  size={ButtonSize.SM}
                  type={ButtonType.BUTTON}
                  disabled={false}
                  onclick={() => addFound(s)}
                />
              {/if}
            {/snippet}
          </ListRow>
        {/each}
      </List>
    {/if}
    <SearchNoResult shown={suggested.length === 0 && twitch.answered} {query} />
  </form>
{/if}
