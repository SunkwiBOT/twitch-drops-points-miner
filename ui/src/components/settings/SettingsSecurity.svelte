<script lang="ts">
  import { tick } from "svelte";

  import { CommandName } from "../../lib/bindings/CommandName";
  import { isTauri } from "../../lib/api/tauri";
  import { AutoLock } from "../../lib/bindings/AutoLock";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { VaultMode } from "../../lib/bindings/VaultMode";
  import { minutes } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { AUTO_LOCK_MINUTES } from "../../lib/state/state.constants";
  import { app, run } from "../../lib/state/state.svelte";
  import { strong } from "../../lib/text.utils";
  import TextButton from "../ui/TextButton.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import Badge from "../ui/Badge.svelte";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import RichText from "../ui/RichText.svelte";
  import SecretInput from "../ui/SecretInput.svelte";
  import Select from "../ui/Select.svelte";
  import {
    BadgeTone,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    MenuAlign,
  } from "../ui/ui.constants";
  import { PASSWORD_MIN_LENGTH } from "./settings.constants";

  type SettingsSecurityPropsType = { snapshot: SnapshotType; draft: GlobalSettingsType };

  let { snapshot, draft = $bindable() }: SettingsSecurityPropsType = $props();

  const hasPassword = $derived(snapshot.vault.mode === VaultMode.PASSPHRASE);

  const autoLocks = $derived(
    Object.values(AutoLock).map((value) => {
      const count = AUTO_LOCK_MINUTES[value];

      return { value, label: count === null ? t.settings_auto_lock_never() : minutes(count) };
    }),
  );

  let card = $state<HTMLElement>();

  $effect(() => {
    if (!app.revealPassword || !card) {
      return;
    }

    const target = card;

    app.revealPassword = false;
    void tick().then(() => {
      target.scrollIntoView({ block: "center", behavior: "smooth" });
      (target.querySelector<HTMLElement>("input") ?? target).focus({ preventScroll: true });
    });
  });

  let editing = $state(false);
  let current = $state("");
  let next = $state("");
  let confirm = $state("");
  let removal = $state("");
  let savingPassword = $state(false);
  let removingPassword = $state(false);
  let locking = $state(false);

  const mismatch = $derived(confirm.length > 0 && next !== confirm);

  const canSave = $derived(
    next.trim().length >= PASSWORD_MIN_LENGTH &&
      next === confirm &&
      (!hasPassword || current.length > 0),
  );

  const clearForms = (): void => {
    current = "";
    next = "";
    confirm = "";
    removal = "";
  };

  const startEditing = (): void => {
    clearForms();
    editing = true;
  };

  const cancelEditing = (): void => {
    clearForms();
    editing = false;
  };

  const changePassword = async (remove: boolean): Promise<void> => {
    if (!remove && !canSave) {
      return;
    }

    if (remove) {
      removingPassword = true;
    } else {
      savingPassword = true;
    }

    const saved = await run(
      {
        command: CommandName.SET_PASSPHRASE,
        current: hasPassword ? (remove ? removal : current) : null,
        next: remove ? null : next,
      },
      t.common_saved(),
    );

    savingPassword = false;
    removingPassword = false;

    if (saved) {
      cancelEditing();
    }
  };

  const lockNow = async (): Promise<void> => {
    locking = true;
    await run({ command: CommandName.LOCK });
    locking = false;
  };
</script>

<div bind:this={card} tabindex="-1" class="min-w-0 outline-none">
  {#if isTauri()}
    <Panel
      title={t.security_title()}
      description={hasPassword ? t.security_on_text() : t.security_off_text()}
    >
      {#snippet actions()}
        <Badge
          text={hasPassword ? t.security_on() : t.security_off()}
          tone={hasPassword ? BadgeTone.SUCCESS : BadgeTone.MUTED}
          dot={false}
          icon={hasPassword ? IconName.LOCK : IconName.UNLOCK}
        />
      {/snippet}
      {#if !hasPassword || editing}
        <form
          class="flex flex-col gap-3"
          onsubmit={(event) => {
            event.preventDefault();
            void changePassword(false);
          }}
        >
          <div class="grid grid-cols-[repeat(auto-fit,minmax(min(200px,100%),1fr))] gap-3">
            {#if hasPassword}
              <FormField id="pw-current" label={t.settings_vault_current()}>
                <SecretInput id="pw-current" bind:value={current} autocomplete="current-password" />
              </FormField>
            {/if}
            <FormField id="pw-new" label={t.security_new()}>
              <SecretInput id="pw-new" bind:value={next} autocomplete="new-password" />
            </FormField>
            <FormField
              id="pw-confirm"
              label={t.security_confirm()}
              error={mismatch ? t.settings_vault_mismatch() : undefined}
            >
              <SecretInput
                id="pw-confirm"
                aria-invalid={mismatch}
                bind:value={confirm}
                autocomplete="new-password"
              />
            </FormField>
          </div>
          {#if !mismatch}<span class="text-hint text-muted-foreground">{t.security_rules()}</span
            >{/if}
          <div class="flex flex-wrap justify-end gap-2">
            {#if editing}
              <TextButton
                label={t.common_cancel()}
                variant={ButtonVariant.OUTLINE}
                size={ButtonSize.MD}
                type={ButtonType.BUTTON}
                disabled={savingPassword}
                onclick={cancelEditing}
              />
            {/if}
            <ActionButton
              icon={IconName.LOCK}
              label={hasPassword ? t.security_change() : t.security_enable()}
              busyLabel={t.common_saving()}
              busy={savingPassword}
              disabled={!canSave}
              variant={ButtonVariant.PRIMARY}
              size={ButtonSize.MD}
              type={ButtonType.SUBMIT}
            />
          </div>
        </form>
      {:else}
        <div class="flex flex-wrap gap-2">
          <ActionButton
            icon={IconName.LOCK}
            label={t.lock_now()}
            busyLabel={t.lock_locking()}
            busy={locking}
            disabled={false}
            variant={ButtonVariant.PRIMARY}
            size={ButtonSize.MD}
            type={ButtonType.BUTTON}
            onclick={lockNow}
          />
          <TextButton
            label={t.security_change()}
            variant={ButtonVariant.OUTLINE}
            size={ButtonSize.MD}
            type={ButtonType.BUTTON}
            disabled={false}
            onclick={startEditing}
          />
        </div>
        <form
          class="flex flex-col gap-2 border-t pt-4"
          onsubmit={(event) => {
            event.preventDefault();
            void changePassword(true);
          }}
        >
          <FormField id="pw-remove" label={t.settings_vault_current()}>
            <SecretInput id="pw-remove" bind:value={removal} autocomplete="current-password" />
          </FormField>
          <div class="flex justify-end">
            <ActionButton
              icon={IconName.UNLOCK}
              label={t.security_disable()}
              busyLabel={t.common_removing()}
              busy={removingPassword}
              disabled={!removal}
              variant={ButtonVariant.DANGER}
              size={ButtonSize.MD}
              type={ButtonType.SUBMIT}
            />
          </div>
        </form>
      {/if}
    </Panel>
  {:else}
    <Panel title={t.auth_title()} description={t.settings_dashboard_password_hint()}>
      {#snippet actions()}
        <Badge text={t.security_on()} tone={BadgeTone.SUCCESS} dot={false} icon={IconName.LOCK} />
      {/snippet}

      <p class="text-hint text-muted-foreground">
        <RichText
          text={app.auth.public
            ? t.security_access_public({ host: strong(location.host) })
            : t.security_access_local()}
        />
      </p>
      <FormField
        id="set-auto-lock"
        label={t.settings_auto_lock()}
        hint={t.settings_auto_lock_hint()}
      >
        <Select
          id="set-auto-lock"
          value={draft.auto_lock}
          options={autoLocks}
          label={t.settings_auto_lock()}
          align={MenuAlign.LEFT}
          onchange={(value) => (draft.auto_lock = value)}
        />
      </FormField>
    </Panel>
  {/if}
</div>
