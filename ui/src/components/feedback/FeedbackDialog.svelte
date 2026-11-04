<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/state.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import FeedbackChoices from "./FeedbackChoices.svelte";

  const close = (): void => {
    app.feedbackDialog = false;
  };
</script>

<Dialog
  open={app.feedbackDialog}
  title={t.nav_feedback()}
  wide={false}
  closable={true}
  closeLabel={t.common_close()}
  onclose={close}
>
  <FeedbackChoices onchosen={close} />
  <p class="text-hint text-muted-foreground">{t.settings_feedback_text()}</p>
</Dialog>
