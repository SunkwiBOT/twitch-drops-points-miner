<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { untrack } from "svelte";

  import { maskProxyCredentials } from "../../lib/accounts/accounts.utils";
  import { SLOTS_MAX } from "../../lib/accounts/slots.constants";
  import {
    copyOptions,
    differingOptions,
    sameOptions,
    STREAMER_OPTIONS,
  } from "../../lib/accounts/options.utils";
  import { testProxy } from "../../lib/api/client";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { Mode } from "../../lib/bindings/Mode";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { maskId } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { personMarker, strong } from "../../lib/text.utils";
  import { AccountField, Page, ToastKind } from "../../lib/state/state.constants";
  import { editsOf } from "../../lib/state/edits.svelte";
  import { app, navigate, reportFailure, run, toast } from "../../lib/state/state.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import CopyButton from "../ui/CopyButton.svelte";
  import ChoiceCards from "../ui/ChoiceCards.svelte";
  import ConfirmDialog from "../ui/ConfirmDialog.svelte";
  import FormField from "../ui/FormField.svelte";
  import Meta from "../ui/Meta.svelte";
  import AccountHeading from "../ui/AccountHeading.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PauseButton from "../ui/PauseButton.svelte";
  import Panel from "../ui/Panel.svelte";
  import SecretInput from "../ui/SecretInput.svelte";
  import Setting from "../ui/Setting.svelte";
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import StreamerOptions from "./StreamerOptions.svelte";
  import {
    ButtonSize,
    ButtonType,
    ButtonVariant,
    CopyLayout,
    IconName,
    MetaLayout,
  } from "../ui/ui.constants";

  type AccountSettingsPropsType = { account: AccountSnapshotType; snapshot: SnapshotType };

  const PROXY_EXAMPLE = "socks5://user:pass@host:1080";

  let { account, snapshot }: AccountSettingsPropsType = $props();

  const edits = editsOf(untrack(() => account.id));
  const shown = $derived(edits.applied(account.settings));
  let testing = $state(false);

  let defaults = $state(structuredClone(untrack(() => shown.points.streamer_defaults)));

  $effect(() => {
    const next = shown.points.streamer_defaults;

    untrack(() => {
      if (!sameOptions(next, defaults)) {
        defaults = structuredClone(next);
      }
    });
  });

  const setDefaults = (): void => {
    const next = $state.snapshot(defaults);
    const changes = differingOptions(
      next,
      account.settings.points.streamer_defaults,
      STREAMER_OPTIONS,
    );

    edits.set(AccountField.STREAMER_DEFAULTS, (settings) =>
      copyOptions(next, settings.points.streamer_defaults, changes),
    );
  };

  const checkProxy = async (): Promise<void> => {
    testing = true;

    try {
      toast(ToastKind.SUCCESS, t.settings_proxy_ok({ ms: strong(await testProxy(shown.proxy)) }));
    } catch (e: any) {
      reportFailure(e);
    } finally {
      testing = false;
    }
  };

  let confirming = $state(false);
  let removing = $state(false);

  const remove = async (): Promise<void> => {
    removing = true;
    const removed = await run({ command: CommandName.REMOVE_ACCOUNT, account_id: account.id });
    removing = false;

    if (removed) {
      confirming = false;
      navigate(Page.HOME);
    }
  };
</script>

{#snippet copy(field: string, value: string)}
  <CopyButton
    text={() => value}
    label={t.common_copy()}
    copiedLabel={t.common_copied()}
    name={t.common_copy_named({ name: field })}
    layout={CopyLayout.ICON}
    size={ButtonSize.SM}
  />
{/snippet}

<PageBody>
  <Panel title={t.account_twitch_title()}>
    <AccountHeading
      {account}
      subtitle={account.settings.enabled
        ? t.account_state_running_hint()
        : t.account_state_paused_hint()}
    >
      {#snippet actions()}
        <PauseButton {account} size={ButtonSize.MD} />
        <TextButton
          label={t.accounts_login_again()}
          icon={IconName.REFRESH}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => (app.loginDialog = true)}
        />
        <TextButton
          label={t.account_remove()}
          icon={IconName.TRASH}
          variant={ButtonVariant.DANGER}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => (confirming = true)}
        />
      {/snippet}
    </AccountHeading>
    <dl
      class="m-0 grid grid-cols-[repeat(auto-fit,minmax(min(200px,100%),1fr))] gap-x-6 gap-y-3 border-t pt-4"
    >
      <Meta
        label={t.account_twitch_name()}
        value={account.display_name}
        layout={MetaLayout.STACKED}
        mono={false}
      >
        {#snippet after()}{@render copy(t.account_twitch_name(), account.display_name)}{/snippet}
      </Meta>
      <Meta
        label={t.account_twitch_login()}
        value={account.login}
        layout={MetaLayout.STACKED}
        mono={true}
      >
        {#snippet after()}{@render copy(t.account_twitch_login(), account.login)}{/snippet}
      </Meta>
      <Meta
        label={t.account_twitch_id()}
        value={maskId(account.id)}
        layout={MetaLayout.STACKED}
        mono={true}
      >
        {#snippet after()}{@render copy(t.account_twitch_id(), account.id)}{/snippet}
      </Meta>
    </dl>
    <p class="text-hint text-muted-foreground">{t.account_twitch_hint()}</p>
  </Panel>

  <div class="grid grid-cols-[repeat(auto-fit,minmax(min(420px,100%),1fr))] items-start gap-5">
    <Panel title={t.account_mode_title()} description={t.account_mode_desc()}>
      <ChoiceCards
        vertical={true}
        value={shown.mode}
        label={t.account_mode_title()}
        onchange={(mode) => edits.set(AccountField.MODE, (settings) => (settings.mode = mode))}
        choices={[
          { value: Mode.AUTO, title: t.mode_auto_title(), description: t.mode_auto_hint() },
          { value: Mode.DROPS, title: t.mode_drops_title(), description: t.mode_drops_hint() },
          { value: Mode.POINTS, title: t.mode_points_title(), description: t.mode_points_hint() },
        ]}
      />
    </Panel>
    <Panel title={t.account_slots_title()} description={t.account_slots_hint()}>
      <ChoiceCards
        vertical={false}
        value={shown.slots}
        label={t.account_slots_title()}
        onchange={(slots) => edits.set(AccountField.SLOTS, (settings) => (settings.slots = slots))}
        choices={[
          { value: SLOTS_MAX, title: t.slots_two_title(), description: t.slots_two_hint() },
          { value: 1, title: t.slots_one_title(), description: t.slots_one_hint() },
        ]}
      />
    </Panel>

    <Panel title={t.settings_proxy()} description={t.account_proxy_hint()}>
      <FormField id="account-proxy" label={t.account_proxy()} hint={t.settings_proxy_hint()}>
        <SecretInput
          id="account-proxy"
          class="font-mono"
          autocomplete="off"
          value={shown.proxy}
          oninput={(event) => {
            const value = event.currentTarget.value.trim();
            edits.set(AccountField.PROXY, (settings) => (settings.proxy = value));
          }}
          placeholder={maskProxyCredentials(snapshot.settings.default_proxy) || PROXY_EXAMPLE}
        />
      </FormField>
      <div class="flex flex-wrap justify-end gap-2">
        <ActionButton
          icon={IconName.GLOBE}
          label={t.settings_proxy_test()}
          busyLabel={t.common_testing()}
          busy={testing}
          disabled={!shown.proxy}
          variant={ButtonVariant.DEFAULT}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          onclick={checkProxy}
        />
      </div>
    </Panel>
    <Panel title={t.account_claims_priorities_title()}>
      <div>
        <Setting
          title={t.account_drops_auto_claim()}
          description={t.drops_auto_claim_hint()}
          checked={shown.drops.auto_claim}
          disabled={false}
          onchange={(value) =>
            edits.set(AccountField.AUTO_CLAIM, (settings) => (settings.drops.auto_claim = value))}
        />
        <Setting
          title={t.account_points_streak_priority()}
          description={t.points_streak_priority_hint()}
          checked={shown.points.streak_priority}
          disabled={false}
          onchange={(value) =>
            edits.set(
              AccountField.STREAK_PRIORITY,
              (settings) => (settings.points.streak_priority = value),
            )}
        />
        <Setting
          title={t.points_subscriber_priority()}
          description={t.points_subscriber_priority_hint()}
          checked={shown.points.subscriber_priority}
          disabled={false}
          onchange={(value) =>
            edits.set(
              AccountField.SUBSCRIBER_PRIORITY,
              (settings) => (settings.points.subscriber_priority = value),
            )}
        />
      </div>
    </Panel>
  </div>
  <Panel title={t.points_streamer_defaults()} description={t.points_streamer_defaults_hint()}>
    <StreamerOptions bind:options={defaults} mixed={[]} onchange={setDefaults} />
  </Panel>
  <AccountSaveBar {account} />
</PageBody>

<ConfirmDialog
  open={confirming}
  title={t.accounts_remove_title()}
  text={t.accounts_remove_text({ name: personMarker(0) })}
  people={[{ name: account.display_name, avatar: account.avatar_url, channel: null }]}
  confirmLabel={t.common_remove()}
  busyLabel={t.common_removing()}
  busy={removing}
  onconfirm={remove}
  oncancel={() => (confirming = false)}
/>
