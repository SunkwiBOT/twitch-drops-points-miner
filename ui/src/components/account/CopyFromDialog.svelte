<script lang="ts">
  import { CopySection } from "../../lib/accounts/accounts.constants";
  import type { AccountSettingsType } from "../../lib/bindings/AccountSettingsType";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { personMarker, strong } from "../../lib/text.utils";
  import { changed } from "../../lib/state/bulk.svelte";
  import { app, updateAccountSettings } from "../../lib/state/state.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import FormField from "../ui/FormField.svelte";
  import Select from "../ui/Select.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName, MenuAlign } from "../ui/ui.constants";

  type CopyFromDialogPropsType = {
    open: boolean;
    account: AccountSnapshotType;
    section: CopySection;
    onclose: () => void;
  };

  let { open, account, section, onclose }: CopyFromDialogPropsType = $props();

  const others = $derived((app.snapshot?.accounts ?? []).filter((a) => a.id !== account.id));
  let fromId = $state<string | null>(null);
  const from = $derived(others.find((a) => a.id === fromId) ?? others[0] ?? null);

  const options = $derived(
    others.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url })),
  );

  const drops = $derived(section === CopySection.DROPS);

  const copy = (): void => {
    if (!from) {
      return;
    }

    const source = { name: from.display_name, url: from.avatar_url, channel: null };
    const copied = drops
      ? { drops: $state.snapshot(from.settings.drops) }
      : { points: $state.snapshot(from.settings.points) };

    const take = (settings: AccountSettingsType): void => {
      if (copied.drops) {
        settings.drops = structuredClone(copied.drops);
      }

      if (copied.points) {
        settings.points = structuredClone(copied.points);
      }
    };

    void updateAccountSettings(account.id, take);

    const text = drops
      ? t.change_copied_drops({ source: strong(source.name) })
      : t.change_copied_points({ source: strong(source.name) });

    changed(account.id, text, take, source);
    onclose();
  };
</script>

<Dialog
  {open}
  title={drops ? t.copy_drops_title() : t.copy_points_title()}
  wide={false}
  closable={true}
  closeLabel={t.common_close()}
  {onclose}
>
  {#if others.length === 0}
    <p class="text-hint text-muted-foreground">{t.import_no_account()}</p>
  {:else}
    <FormField
      id="copy-from"
      label={t.import_from()}
      hint={drops
        ? t.copy_drops_hint({ name: personMarker(0) })
        : t.copy_points_hint({ name: personMarker(0) })}
      people={[{ name: account.display_name, avatar: account.avatar_url, channel: null }]}
    >
      <Select
        id="copy-from"
        value={from?.id ?? ""}
        {options}
        label={t.import_from()}
        align={MenuAlign.LEFT}
        onchange={(value) => (fromId = value)}
      />
    </FormField>
  {/if}
  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={onclose}
    />
    <TextButton
      label={t.common_copy()}
      icon={IconName.COPY}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={!from}
      onclick={copy}
    />
  {/snippet}
</Dialog>
