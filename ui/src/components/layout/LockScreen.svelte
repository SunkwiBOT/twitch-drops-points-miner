<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { sendCommand, signIn } from "../../lib/api/client";
  import { ErrorApiMessage } from "../../lib/bindings/ErrorApiMessage";
  import { describeError, toApiError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { app, connect, refresh, sessionEnded, toast } from "../../lib/state/state.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import FormField from "../ui/FormField.svelte";
  import SecretInput from "../ui/SecretInput.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";
  import { LockKind } from "./layout.constants";

  type LockScreenPropsType = { kind: LockKind };

  let { kind }: LockScreenPropsType = $props();

  let password = $state("");
  let wrong = $state(false);
  let busy = $state(false);

  const dashboard = $derived(kind === LockKind.DASHBOARD);

  const openDashboard = async (): Promise<void> => {
    await signIn(password);
    app.auth = { ...app.auth, authorized: true, checked: true };
    await connect();
  };

  const unlockVault = async (): Promise<void> => {
    await sendCommand({ command: CommandName.UNLOCK, passphrase: password });
    await refresh();
  };

  const submit = async (event: SubmitEvent): Promise<void> => {
    event.preventDefault();
    busy = true;
    wrong = false;

    try {
      await (dashboard ? openDashboard() : unlockVault());
      password = "";
    } catch (e: any) {
      const error = toApiError(e);
      const wrongCode = dashboard
        ? ErrorApiMessage.WRONG_PASSWORD
        : ErrorApiMessage.WRONG_PASSPHRASE;

      if (error.code === wrongCode) {
        wrong = true;
      } else if (error.code === ErrorApiMessage.UNAUTHORIZED) {
        sessionEnded();
      } else {
        toast(ToastKind.ERROR, describeError(error));
      }
    } finally {
      busy = false;
    }
  };
</script>

<div class="grid h-full w-full place-items-center bg-background p-6">
  <form
    class="flex w-full max-w-100 flex-col gap-4 rounded-xl border bg-card p-7 shadow-lg"
    onsubmit={submit}
  >
    <img class="size-16 self-center [image-rendering:pixelated]" src="/app-icon.svg" alt="" />
    <h1 class="text-center text-xl font-semibold">
      {dashboard ? t.auth_title() : t.unlock_title()}
    </h1>
    <p class="text-center text-hint text-muted-foreground">
      {dashboard ? t.auth_text() : t.unlock_text()}
    </p>
    <FormField
      id="lock-password"
      label={dashboard ? t.auth_password() : t.unlock_passphrase()}
      error={wrong ? (dashboard ? t.auth_wrong() : t.unlock_wrong()) : undefined}
    >
      <SecretInput
        id="lock-password"
        bind:value={password}
        autocomplete="current-password"
        aria-invalid={wrong}
      />
    </FormField>
    <ActionButton
      icon={IconName.UNLOCK}
      label={dashboard ? t.auth_button() : t.unlock_button()}
      busyLabel={dashboard ? t.auth_signing_in() : t.unlock_unlocking()}
      {busy}
      disabled={!password}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.SUBMIT}
      class="w-full"
    />
  </form>
</div>
