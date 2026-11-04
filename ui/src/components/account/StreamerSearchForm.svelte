<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { SEARCH_SHOWN_MAX } from "../../lib/search/search.constants";
  import { TwitchSearch } from "../../lib/search/search.svelte";
  import type { TextPersonType } from "../../lib/text.utils";
  import ChannelAvatar from "../ui/ChannelAvatar.svelte";
  import Icon from "../ui/Icon.svelte";
  import List from "../ui/List.svelte";
  import ListRow from "../ui/ListRow.svelte";
  import RichText from "../ui/RichText.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import SearchNoResult from "../ui/SearchNoResult.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { AvatarSize, ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  type StreamerSearchFormPropsType = {
    query: string;
    problem: string;
    problemPeople: TextPersonType[];
    searching: boolean;
    listed: (login: string) => boolean;
    onsearch: () => void;
    onedit: () => void;
  };

  let {
    query = $bindable(),
    problem,
    problemPeople,
    searching,
    listed,
    onsearch,
    onedit,
  }: StreamerSearchFormPropsType = $props();

  const uid = $props.id();
  const problemId = `${uid}-problem`;

  const twitch = new TwitchSearch();
  const suggested = $derived(twitch.results.channels.slice(0, SEARCH_SHOWN_MAX));

  const submit = (): void => {
    twitch.clear();
    onsearch();
  };

  const pick = (picked: string): void => {
    query = picked;
    submit();
  };
</script>

<form
  class="flex flex-col gap-2"
  onsubmit={(event) => {
    event.preventDefault();
    submit();
  }}
>
  <p class="text-hint text-muted-foreground">{t.streamer_search_hint()}</p>
  <SearchField
    aria-invalid={problem !== ""}
    aria-describedby={problem ? problemId : undefined}
    placeholder={t.account_points_add_placeholder()}
    aria-label={t.account_points_add_placeholder()}
    bind:value={query}
    search={{ label: t.common_search(), busy: twitch.busy || searching, run: submit }}
    oninput={() => {
      onedit();
      twitch.update(query);
    }}
  />
  {#if problem}<p id={problemId} class="text-hint text-destructive">
      <RichText text={problem} people={problemPeople} />
    </p>{/if}
  {#if suggested.length && !searching}
    <List>
      {#each suggested as s (s.login)}
        <ListRow
          title={s.display_name}
          subtitle={listed(s.login) ? t.import_entry_already() : undefined}
          onopen={listed(s.login) ? undefined : () => pick(s.login)}
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
          {#snippet trailing()}
            {#if !listed(s.login)}
              <TextButton
                label={t.search_pick()}
                variant={ButtonVariant.OUTLINE}
                size={ButtonSize.SM}
                type={ButtonType.BUTTON}
                disabled={false}
                onclick={() => pick(s.login)}
              />
            {/if}
          {/snippet}
          {#snippet badges()}
            {#if s.verified}<span
                class="inline-flex text-primary"
                role="img"
                aria-label={t.search_verified()}
                title={t.search_verified()}><Icon name={IconName.VERIFIED} size={16} /></span
              >{/if}
          {/snippet}
        </ListRow>
      {/each}
    </List>
  {/if}
  <SearchNoResult
    shown={!problem && !searching && suggested.length === 0 && twitch.answered}
    {query}
  />
</form>
