<script lang="ts">
  import { DropsList } from "../../../lib/accounts/accounts.constants";
  import { editDropsList, setIgnored, type IgnoredType } from "../../../lib/accounts/drops.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import type { SnapshotType } from "../../../lib/bindings/SnapshotType";
  import { BoxArtSize } from "../../../lib/format.constants";
  import { boxArt } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { escapeEmphasis, strong } from "../../../lib/text.utils";
  import TextButton from "../../ui/TextButton.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import FormField from "../../ui/FormField.svelte";
  import List from "../../ui/List.svelte";
  import ListRow from "../../ui/ListRow.svelte";
  import Panel from "../../ui/Panel.svelte";
  import TagInput from "../../ui/TagInput.svelte";
  import Thumb from "../../ui/Thumb.svelte";
  import {
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    ThumbShape,
    ThumbSize,
  } from "../../ui/ui.constants";

  type DropsExclusionsTabPropsType = {
    account: AccountSnapshotType;
    snapshot: SnapshotType;
    ignored: IgnoredType[];
    onopen: (campaign: CampaignType) => void;
  };

  let { account, snapshot, ignored, onopen }: DropsExclusionsTabPropsType = $props();

  const drops = $derived(account.settings.drops);
  const games = $derived([...new Set(snapshot.catalog.campaigns.map((c) => c.game.name))].sort());
</script>

<div class="grid grid-cols-[repeat(auto-fit,minmax(min(460px,100%),1fr))] items-start gap-5">
  <Panel title={t.ignored_title()} description={t.ignored_hint()}>
    {#if ignored.length === 0}
      <EmptyState text={t.ignored_empty()} />
    {:else}
      <List>
        {#each ignored as item (item.id)}
          {@const campaign = item.campaign}
          <ListRow
            title={item.name}
            subtitle={item.detail ? escapeEmphasis(item.detail) : undefined}
            onopen={campaign ? () => onopen(campaign) : undefined}
            muted={true}
          >
            {#snippet lead()}
              <Thumb
                src={item.drop
                  ? campaign?.drops.find((d) => d.id === item.id)?.benefits.at(0)?.image_url
                  : boxArt(campaign?.game.box_art_url, BoxArtSize.ROW)}
                round={false}
                contain={item.drop}
                size={ThumbSize.MD}
                shape={item.drop ? ThumbShape.SQUARE : ThumbShape.BOX_ART}
                fallback={item.drop ? IconName.GIFT : IconName.CALENDAR}
              />
            {/snippet}
            {#snippet trailing()}
              <TextButton
                label={t.ignored_restore()}
                icon={IconName.EYE}
                variant={ButtonVariant.OUTLINE}
                size={ButtonSize.SM}
                type={ButtonType.BUTTON}
                disabled={false}
                onclick={() => setIgnored(account, item, false)}
              />
            {/snippet}
          </ListRow>
        {/each}
      </List>
    {/if}
  </Panel>
  <Panel title={t.drops_exclusions_title()}>
    <FormField id="drops-exclude-games" label={t.account_drops_exclude_games()}>
      <TagInput
        id="drops-exclude-games"
        items={drops.exclude_games}
        suggestions={games}
        placeholder={t.account_drops_priority_placeholder()}
        addLabel={t.common_add()}
        removeLabel={t.common_remove()}
        confirmTitle={t.confirm_remove_excluded_game_title()}
        confirmText={(game) => t.confirm_remove_excluded_game_text({ name: strong(game) })}
        onchange={(list) => editDropsList(account, DropsList.EXCLUDE_GAMES, list)}
        remote={true}
      />
    </FormField>
    <FormField
      id="drops-exclude-names"
      label={t.account_drops_exclude_names()}
      hint={t.account_drops_exclude_names_hint()}
    >
      <TagInput
        id="drops-exclude-names"
        items={drops.exclude_names}
        suggestions={[]}
        placeholder={t.drops_exclude_names_placeholder()}
        addLabel={t.common_add()}
        removeLabel={t.common_remove()}
        confirmTitle={t.confirm_remove_excluded_word_title()}
        confirmText={(word) => t.confirm_remove_excluded_word_text({ name: strong(word) })}
        onchange={(list) => editDropsList(account, DropsList.EXCLUDE_NAMES, list)}
        remote={false}
      />
    </FormField>
  </Panel>
</div>
