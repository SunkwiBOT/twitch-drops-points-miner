<script lang="ts">
  import { untrack } from "svelte";

  import { ImportSource } from "../../lib/accounts/accounts.constants";
  import type { ImportActionType, ImportCandidateType } from "../../lib/accounts/import.svelte";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import { ensureChannels, isLive } from "../../lib/channels/channels.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import CheckList from "../ui/CheckList.svelte";
  import RichText from "../ui/RichText.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { IconName } from "../ui/ui.constants";
  import AccountImport from "./import/AccountImport.svelte";
  import FollowsImport from "./import/FollowsImport.svelte";
  import PasteImport from "./import/PasteImport.svelte";
  import SearchImport from "./import/SearchImport.svelte";

  type StreamerImportPropsType = {
    target: AccountSnapshotType;
    existing: string[];
    onadd: (logins: string[], channels: ChannelType[]) => void;
    searchable: boolean;
    primary: ImportActionType | null;
  };

  let {
    target,
    existing,
    onadd,
    searchable,
    primary = $bindable(),
  }: StreamerImportPropsType = $props();

  let source = $state<ImportSource>(
    untrack(() => searchable) ? ImportSource.SEARCH : ImportSource.PASTE,
  );

  const sources = $derived([
    ...(searchable ? [{ value: ImportSource.SEARCH, label: t.common_search() }] : []),
    { value: ImportSource.PASTE, label: t.import_tab_paste() },
    { value: ImportSource.ACCOUNT, label: t.import_tab_account() },
    { value: ImportSource.FOLLOWS, label: t.import_tab_follows() },
  ]);

  // ! Raw: the streamers offered are handed on as they are, and a proxy cannot be
  // cloned.
  let pasteOffer = $state.raw<ImportCandidateType[] | null>(null);
  let accountOffer = $state.raw<ImportCandidateType[] | null>(null);
  let followsOffer = $state.raw<ImportCandidateType[] | null>(null);
  let check = $state.raw<ImportActionType | null>(null);
  let paste = $state<ReturnType<typeof PasteImport>>();

  const offered = $derived(
    {
      [ImportSource.SEARCH]: null,
      [ImportSource.PASTE]: pasteOffer,
      [ImportSource.ACCOUNT]: accountOffer,
      [ImportSource.FOLLOWS]: followsOffer,
    }[source],
  );
  const candidates = $derived((offered ?? []).filter((c) => !existing.includes(c.login)));

  $effect(() => {
    ensureChannels(candidates.filter((c) => !c.channel).map((c) => c.login));
  });

  const already = $derived((offered?.length ?? 0) - candidates.length);
  let selected = $state<string[]>([]);

  // ! Keyed on the logins: a snapshot that only refreshes the same channels keeps
  // what the user unticked.
  const offeredLogins = $derived(candidates.map((c) => c.login).join("\n"));

  $effect(() => {
    selected = offeredLogins ? offeredLogins.split("\n") : [];
  });

  const items = $derived(
    candidates.map((c) => ({
      value: c.login,
      label: c.channel?.display_name ?? c.login,
      image: c.channel?.avatar_url,
      hint: "",
      channel: { live: isLive(c.login) },
    })),
  );

  const add = (): void => {
    const chosen = candidates.filter((c) => selected.includes(c.login));

    onadd(
      chosen.map((c) => c.login),
      chosen.flatMap((c) => c.channel ?? []),
    );

    if (source === ImportSource.PASTE) {
      paste?.clear();
    }
  };

  $effect(() => {
    if (source === ImportSource.PASTE && check) {
      primary = check;
    } else if (offered && items.length > 0) {
      primary = {
        label: t.import_add({ count: selected.length }),
        icon: IconName.PLUS,
        busy: false,
        busyLabel: "",
        disabled: selected.length === 0,
        run: add,
      };
    } else {
      primary = null;
    }
  });
</script>

<div class="flex flex-col gap-3">
  <Segmented
    value={source}
    options={sources}
    label={t.import_source()}
    onchange={(next) => (source = next)}
  />

  {#if searchable}
    <SearchImport open={source === ImportSource.SEARCH} {existing} {onadd} />
  {/if}
  <PasteImport
    bind:this={paste}
    bind:offered={pasteOffer}
    bind:check
    open={source === ImportSource.PASTE}
    {existing}
  />
  <AccountImport bind:offered={accountOffer} open={source === ImportSource.ACCOUNT} {target} />
  <FollowsImport bind:offered={followsOffer} open={source === ImportSource.FOLLOWS} {target} />

  {#if offered}
    <CheckList
      {items}
      {selected}
      label={t.import_title()}
      allLabel={t.common_select_all()}
      noneLabel={t.common_select_none()}
      countLabel={t.common_selected_count({ count: selected.length, total: items.length })}
      empty={already > 0 ? t.import_none_new() : t.import_nothing()}
      onchange={(value) => (selected = value)}
    />
    {#if already > 0 && candidates.length > 0}
      <p class="text-hint text-muted-foreground">
        <RichText text={t.import_already({ count: strong(already) })} />
      </p>
    {/if}
  {/if}
</div>
