<script lang="ts">
  import { untrack } from "svelte";

  import { t } from "../../lib/i18n/i18n.svelte";
  import { applyToAccounts, bulk, closeBulk } from "../../lib/state/bulk.svelte";
  import { account, app } from "../../lib/state/state.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { personMarker, strong } from "../../lib/text.utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import CheckList from "../ui/CheckList.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import RichText from "../ui/RichText.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  const source = $derived(account(bulk.sourceId));
  const others = $derived((app.snapshot?.accounts ?? []).filter((a) => a.id !== bulk.sourceId));

  const items = $derived(
    others.map((a) => ({
      value: a.id,
      label: a.display_name,
      image: a.avatar_url,
      hint: "",
      channel: null,
    })),
  );

  let selected = $state<string[]>([]);
  let applying = $state(false);

  $effect(() => {
    if (bulk.open) {
      selected = untrack(() => others.map((a) => a.id));
    }
  });

  const apply = async (): Promise<void> => {
    applying = true;
    await applyToAccounts(selected);
    applying = false;
  };
</script>

<Dialog
  open={bulk.open}
  title={t.bulk_title()}
  wide={false}
  closable={!applying}
  closeLabel={t.common_close()}
  onclose={closeBulk}
>
  <div class="flex flex-col gap-3">
    <p class="text-hint text-muted-foreground">
      <RichText
        text={t.bulk_hint({
          count: strong(bulk.changes.length),
          name: personMarker(0),
        })}
        people={[{ name: source?.display_name ?? "", avatar: source?.avatar_url, channel: null }]}
      />
    </p>
    <CheckList
      {items}
      {selected}
      label={t.bulk_accounts()}
      allLabel={t.common_select_all()}
      noneLabel={t.common_select_none()}
      countLabel={t.common_selected_count({ count: selected.length, total: items.length })}
      empty={t.bulk_no_accounts()}
      onchange={(value) => (selected = value)}
    />
  </div>
  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={applying}
      onclick={closeBulk}
    />
    <ActionButton
      icon={IconName.CHECK}
      label={t.bulk_apply({ count: selected.length })}
      busyLabel={t.common_saving()}
      busy={applying}
      disabled={selected.length === 0}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={apply}
    />
  {/snippet}
</Dialog>
