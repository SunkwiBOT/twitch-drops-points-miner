<script lang="ts">
  import { closeWindow } from "../../lib/api/client";
  import { CloseAction } from "../../lib/bindings/CloseAction";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { app, toast } from "../../lib/state/state.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import ChoiceCards from "../ui/ChoiceCards.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  let choice = $state(CloseAction.QUIT);
  let closing = $state(false);

  const confirm = async (): Promise<void> => {
    closing = true;

    try {
      await closeWindow(choice);
      app.closeDialog = false;
    } catch (e: any) {
      toast(ToastKind.ERROR, describeError(e));
    } finally {
      closing = false;
    }
  };
</script>

<Dialog
  open={app.closeDialog}
  title={t.close_title()}
  wide={false}
  closable={!closing}
  closeLabel={t.common_cancel()}
  onclose={() => (app.closeDialog = false)}
>
  <p class="text-hint text-muted-foreground">{t.close_text()}</p>
  <ChoiceCards
    value={choice}
    label={t.close_title()}
    vertical={false}
    onchange={(value) => (choice = value)}
    choices={[
      { value: CloseAction.QUIT, title: t.close_quit_title(), description: t.close_quit_text() },
      { value: CloseAction.TRAY, title: t.close_tray_title(), description: t.close_tray_text() },
    ]}
  />
  <p class="text-hint text-muted-foreground">{t.close_remember()}</p>

  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={closing}
      onclick={() => (app.closeDialog = false)}
    />
    <ActionButton
      icon={choice === CloseAction.QUIT ? IconName.X : IconName.DOWN}
      label={choice === CloseAction.QUIT ? t.close_quit_title() : t.close_tray_title()}
      busyLabel={t.close_closing()}
      busy={closing}
      disabled={false}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={confirm}
    />
  {/snippet}
</Dialog>
