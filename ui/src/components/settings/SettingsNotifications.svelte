<script lang="ts">
  import { isTauri } from "../../lib/api/tauri";
  import { testNotifications } from "../../lib/api/client";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import type { NotificationEventsType } from "../../lib/bindings/NotificationEventsType";
  import { NotificationTarget } from "../../lib/bindings/NotificationTarget";
  import { describeCode, describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { toast } from "../../lib/state/state.svelte";
  import { escapeEmphasis, strong } from "../../lib/text.utils";
  import { Input } from "$lib/components/ui/input";
  import ActionButton from "../ui/ActionButton.svelte";
  import FormField from "../ui/FormField.svelte";
  import Icon from "../ui/Icon.svelte";
  import Panel from "../ui/Panel.svelte";
  import SecretInput from "../ui/SecretInput.svelte";
  import Setting from "../ui/Setting.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  type SettingsNotificationsPropsType = {
    draft: GlobalSettingsType;
    dirty: boolean;
  };

  let { draft = $bindable(), dirty }: SettingsNotificationsPropsType = $props();

  const NOTIFY_LABELS: Record<keyof NotificationEventsType, () => string> = {
    drop_claimed: () => t.notify_drop_claimed(),
    drop_progress: () => t.notify_drop_progress(),
    bonus_claimed: () => t.notify_bonus_claimed(),
    prediction_placed: () => t.notify_prediction_placed(),
    prediction_result: () => t.notify_prediction_result(),
    stream_online: () => t.notify_stream_online(),
    stream_offline: () => t.notify_stream_offline(),
    raid: () => t.notify_raid(),
    moment: () => t.notify_moment(),
    chat_mention: () => t.notify_chat_mention(),
    session_expired: () => t.notify_session_expired(),
    errors: () => t.notify_error(),
    update_available: () => t.notify_update_available(),
  };

  const notifyKeys = Object.keys(NOTIFY_LABELS).filter(
    (key): key is keyof NotificationEventsType => key in NOTIFY_LABELS,
  );

  const TARGET_NAMES: Record<NotificationTarget, () => string> = {
    [NotificationTarget.DISCORD]: () => t.settings_discord(),
    [NotificationTarget.TELEGRAM]: () => t.settings_telegram(),
    [NotificationTarget.WEBHOOK]: () => t.settings_webhook(),
  };

  let testingTargets = $state(false);

  const checkTargets = async (): Promise<void> => {
    testingTargets = true;

    try {
      const results = await testNotifications();

      if (results.length === 0) {
        toast(ToastKind.INFO, t.settings_test_none());
      }

      for (const result of results) {
        const target = TARGET_NAMES[result.target]();

        if (result.problem) {
          toast(
            ToastKind.ERROR,
            t.settings_test_failed({
              target: strong(target),
              error: escapeEmphasis(describeCode(result.problem)),
            }),
          );
        } else {
          toast(ToastKind.SUCCESS, t.settings_test_ok({ target: strong(target) }));
        }
      }
    } catch (e: any) {
      toast(ToastKind.ERROR, describeError(e));
    } finally {
      testingTargets = false;
    }
  };
</script>

{#snippet settings_discordMark()}<span class="text-brand-discord"
    ><Icon name={IconName.DISCORD} size={16} /></span
  >{/snippet}

{#snippet settings_telegramMark()}<span class="text-brand-telegram"
    ><Icon name={IconName.TELEGRAM} size={16} /></span
  >{/snippet}

{#snippet settings_webhookMark()}<span class="text-muted-foreground"
    ><Icon name={IconName.WEBHOOK} size={16} /></span
  >{/snippet}

{#snippet settings_system_notificationsMark()}<span class="text-muted-foreground"
    ><Icon name={IconName.BELL} size={16} /></span
  >{/snippet}

<Panel title={t.settings_notifications()}>
  {#snippet actions()}
    <ActionButton
      icon={IconName.BELL}
      label={t.settings_test_notifications()}
      busyLabel={t.common_testing()}
      busy={testingTargets}
      disabled={dirty}
      variant={ButtonVariant.DEFAULT}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={checkTargets}
    />
  {/snippet}
  <div class="flex flex-col divide-y">
    <div class="flex flex-col">
      <Setting
        title={t.settings_discord()}
        lead={settings_discordMark}
        checked={draft.notifications.discord.enabled}
        disabled={false}
        onchange={(enabled) => (draft.notifications.discord.enabled = enabled)}
      />
      {#if draft.notifications.discord.enabled}
        <div class="pb-3">
          <FormField id="n-discord" label={t.settings_discord_url()}>
            <SecretInput
              id="n-discord"
              class="font-mono"
              autocomplete="off"
              bind:value={draft.notifications.discord.webhook_url}
              placeholder="https://discord.com/api/webhooks/…"
            />
          </FormField>
        </div>
      {/if}
    </div>
    <div class="flex flex-col">
      <Setting
        title={t.settings_telegram()}
        lead={settings_telegramMark}
        checked={draft.notifications.telegram.enabled}
        disabled={false}
        onchange={(enabled) => (draft.notifications.telegram.enabled = enabled)}
      />
      {#if draft.notifications.telegram.enabled}
        <div class="grid grid-cols-[repeat(auto-fit,minmax(min(200px,100%),1fr))] gap-3 pb-3">
          <FormField id="n-tg-token" label={t.settings_telegram_token()}>
            <SecretInput
              id="n-tg-token"
              class="font-mono"
              autocomplete="off"
              bind:value={draft.notifications.telegram.bot_token}
            />
          </FormField>
          <FormField id="n-tg-chat" label={t.settings_telegram_chat()}>
            <Input
              id="n-tg-chat"
              class="font-mono"
              bind:value={draft.notifications.telegram.chat_id}
            />
          </FormField>
        </div>
      {/if}
    </div>
    <div class="flex flex-col">
      <Setting
        title={t.settings_webhook()}
        lead={settings_webhookMark}
        checked={draft.notifications.webhook.enabled}
        disabled={false}
        onchange={(enabled) => (draft.notifications.webhook.enabled = enabled)}
      />
      {#if draft.notifications.webhook.enabled}
        <div class="grid grid-cols-[repeat(auto-fit,minmax(min(200px,100%),1fr))] gap-3 pb-3">
          <FormField id="n-wh" label={t.settings_webhook_url()}>
            <SecretInput
              id="n-wh"
              class="font-mono"
              autocomplete="off"
              bind:value={draft.notifications.webhook.url}
            />
          </FormField>
          <FormField id="n-wh-auth" label={t.settings_webhook_auth()}>
            <SecretInput
              id="n-wh-auth"
              class="font-mono"
              autocomplete="off"
              bind:value={draft.notifications.webhook.authorization}
            />
          </FormField>
        </div>
      {/if}
    </div>
    {#if isTauri()}
      <Setting
        title={t.settings_system_notifications()}
        lead={settings_system_notificationsMark}
        checked={draft.notifications.system}
        disabled={false}
        onchange={(enabled) => (draft.notifications.system = enabled)}
      />
    {/if}
  </div>
  <h3 class="pt-1 text-sm font-semibold">{t.settings_notifications_events()}</h3>
  <div class="grid grid-cols-[repeat(auto-fit,minmax(min(280px,100%),1fr))] gap-x-8 *:border-t">
    {#each notifyKeys as key (key)}
      <Setting
        title={NOTIFY_LABELS[key]()}
        checked={draft.notifications.events[key]}
        disabled={false}
        onchange={(enabled) => (draft.notifications.events[key] = enabled)}
      />
    {/each}
  </div>
</Panel>
