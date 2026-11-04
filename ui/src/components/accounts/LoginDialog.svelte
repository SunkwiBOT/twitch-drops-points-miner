<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { untrack, type Snippet } from "svelte";

  import { startLogin } from "../../lib/api/client";
  import { LoginFlowStatus } from "../../lib/bindings/LoginFlowStatus";
  import { describeCode, describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { account, app, refresh, run, toast } from "../../lib/state/state.svelte";
  import { strong } from "../../lib/text.utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import Icon from "../ui/Icon.svelte";
  import RichText from "../ui/RichText.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";
  import LoginCode from "./LoginCode.svelte";
  import SetupWizard from "./SetupWizard.svelte";

  type KnownAccountsType = { flowId: string; ids: string[] };

  const flow = $derived(app.snapshot?.login ?? null);
  // ! Kept while the dialog fades out: closing cancels the flow, and the snapshot
  // that clears it can land mid-animation.
  let shown = $state<typeof flow>(null);
  let starting = $state(false);

  $effect.pre(() => {
    if (app.loginDialog) {
      shown = flow;
    }
  });

  const start = async (): Promise<void> => {
    starting = true;

    try {
      await startLogin();
      await refresh();
    } catch (e: any) {
      toast(ToastKind.ERROR, describeError(e));
    } finally {
      starting = false;
    }
  };

  let knownAccounts = $state<KnownAccountsType | null>(null);

  $effect(() => {
    if (
      flow?.status === LoginFlowStatus.PENDING &&
      untrack(() => knownAccounts?.flowId) !== flow.flow_id
    ) {
      knownAccounts = {
        flowId: flow.flow_id,
        ids: untrack(() => app.snapshot?.accounts.map((a) => a.id) ?? []),
      };
    }
  });

  const newAccount = $derived(
    shown?.status === LoginFlowStatus.COMPLETED &&
      shown.account_id !== undefined &&
      knownAccounts?.flowId === shown.flow_id &&
      !knownAccounts.ids.includes(shown.account_id)
      ? account(shown.account_id)
      : null,
  );

  let setupButtons = $state<Snippet>();
  let finishing = $state(false);

  const close = (): void => {
    app.loginDialog = false;

    if (flow && flow.status !== LoginFlowStatus.PENDING) {
      void run({ command: CommandName.CANCEL_LOGIN });
    }
  };

  $effect(() => {
    if (app.loginDialog && !flow && !starting && app.snapshot?.vault.unlocked) {
      void start();
    }
  });
</script>

{#snippet actions()}
  {#if newAccount}
    {@render setupButtons?.()}
  {:else if shown?.status === LoginFlowStatus.COMPLETED}
    <TextButton
      label={t.common_close()}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={close}
    />
  {:else if shown && shown.status !== LoginFlowStatus.PENDING}
    <TextButton
      label={t.common_close()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={starting}
      onclick={close}
    />
    <ActionButton
      icon={IconName.REFRESH}
      label={t.login_restart()}
      busyLabel={t.login_starting()}
      busy={starting}
      disabled={false}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={start}
    />
  {/if}
{/snippet}

<Dialog
  open={app.loginDialog}
  title={newAccount ? t.login_setup_title() : t.login_title()}
  wide={newAccount !== null}
  closable={!finishing}
  closeLabel={t.common_close()}
  onclose={close}
  footer={shown && shown.status !== LoginFlowStatus.PENDING ? actions : undefined}
>
  {#if !app.snapshot?.vault.unlocked}
    <p class="text-hint text-muted-foreground">{t.login_vault_locked()}</p>
  {:else if !shown}
    <p class="text-hint text-muted-foreground">{t.common_loading()}</p>
  {:else if shown.status === LoginFlowStatus.PENDING}
    <LoginCode flow={shown} />
  {:else if shown.status === LoginFlowStatus.COMPLETED && newAccount}
    <SetupWizard account={newAccount} onclose={close} bind:footer={setupButtons} bind:finishing />
  {:else if shown.status === LoginFlowStatus.COMPLETED}
    <p class="flex items-center gap-2 text-success-foreground">
      <Icon name={IconName.CHECK} size={18} />
      {t.login_completed()}
    </p>
  {:else}
    <p class="text-destructive">
      <RichText
        text={shown.status === LoginFlowStatus.EXPIRED
          ? t.login_expired()
          : t.login_failed({ error: shown.problem ? strong(describeCode(shown.problem)) : "" })}
      />
    </p>
  {/if}
</Dialog>
