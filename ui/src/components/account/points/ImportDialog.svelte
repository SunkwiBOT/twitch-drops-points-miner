<script lang="ts">
  import { newStreamer } from "../../../lib/accounts/import.svelte";
  import type { AccountSettingsType } from "../../../lib/bindings/AccountSettingsType";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../../lib/bindings/ChannelType";
  import { rememberChannels } from "../../../lib/channels/channels.svelte";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { changed } from "../../../lib/state/bulk.svelte";
  import { app, updateAccountSettings } from "../../../lib/state/state.svelte";
  import { strong } from "../../../lib/text.utils";
  import ActionButton from "../../ui/ActionButton.svelte";
  import Dialog from "../../ui/Dialog.svelte";
  import Select from "../../ui/Select.svelte";
  import TextButton from "../../ui/TextButton.svelte";
  import {
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    MenuAlign,
  } from "../../ui/ui.constants";
  import StreamerImport from "../StreamerImport.svelte";

  type ImportDialogPropsType = {
    open: boolean;
    account: AccountSnapshotType;
    onclose: () => void;
  };

  let { open, account, onclose }: ImportDialogPropsType = $props();

  let importer = $state<ReturnType<typeof StreamerImport>>();
  let targetId = $state<string | null>(null);

  const importAction = $derived(importer?.primary() ?? null);

  const accounts = $derived(app.snapshot?.accounts ?? []);
  const target = $derived(accounts.find((a) => a.id === targetId) ?? account);

  const close = (): void => {
    onclose();
    targetId = null;
  };

  const imported = (logins: string[], channels: ChannelType[]): void => {
    const into = target;
    close();

    if (logins.length === 0) {
      return;
    }

    rememberChannels(channels);

    const append = (settings: AccountSettingsType): void => {
      const points = settings.points;

      for (const login of logins) {
        if (!points.streamers.some((item) => item.login === login)) {
          points.streamers.push(newStreamer(points.streamer_defaults, login));
        }
      }
    };

    void updateAccountSettings(into.id, append);
    changed(into.id, t.change_streamers_imported({ count: strong(logins.length) }), append);
  };
</script>

<Dialog
  {open}
  title={t.import_title()}
  wide={true}
  closable={true}
  closeLabel={t.common_close()}
  {onclose}
>
  <StreamerImport
    bind:this={importer}
    {target}
    existing={target.settings.points.streamers.map((s) => s.login)}
    onadd={imported}
    searchable={false}
  />
  {#snippet footer()}
    <div class="w-56 sm:mr-auto">
      <Select
        value={target.id}
        options={accounts.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url }))}
        label={t.import_into()}
        align={MenuAlign.LEFT}
        onchange={(id) => (targetId = id)}
      />
    </div>
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={close}
    />
    {#if importAction}
      <ActionButton
        icon={importAction.icon}
        label={importAction.label}
        busyLabel={importAction.busyLabel}
        busy={importAction.busy}
        disabled={importAction.disabled}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        onclick={importAction.run}
      />
    {:else}
      <TextButton
        label={t.import_add({ count: 0 })}
        icon={IconName.PLUS}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={true}
      />
    {/if}
  {/snippet}
</Dialog>
