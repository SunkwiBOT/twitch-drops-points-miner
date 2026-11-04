<script lang="ts">
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { updateAccountSettings } from "../../lib/state/state.svelte";
  import ActionButton from "./ActionButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "./ui.constants";

  type PauseButtonPropsType = { account: AccountSnapshotType; size: ButtonSize };

  let { account, size }: PauseButtonPropsType = $props();

  let pausing = $state<boolean | null>(null);

  const toggle = async (): Promise<void> => {
    const enabled = !account.settings.enabled;
    pausing = !enabled;
    await updateAccountSettings(account.id, (settings) => (settings.enabled = enabled));
    pausing = null;
  };
</script>

<ActionButton
  icon={account.settings.enabled ? IconName.PAUSE : IconName.PLAY}
  label={account.settings.enabled ? t.action_pause() : t.action_resume()}
  busyLabel={pausing === false ? t.action_resuming() : t.action_pausing()}
  busy={pausing !== null}
  disabled={false}
  variant={account.settings.enabled ? ButtonVariant.DEFAULT : ButtonVariant.PRIMARY}
  {size}
  type={ButtonType.BUTTON}
  onclick={toggle}
/>
