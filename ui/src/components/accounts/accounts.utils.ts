import { SetupStep } from "../../lib/accounts/accounts.constants";
import { t } from "../../lib/i18n/i18n.svelte";

export const SETUP_STEP_LABELS: Record<SetupStep, () => string> = {
  [SetupStep.START]: () => t.login_setup_step_start(),
  [SetupStep.MODE]: () => t.login_setup_step_mode(),
  [SetupStep.DROPS]: () => t.login_setup_step_drops(),
  [SetupStep.STREAMERS]: () => t.login_setup_step_streamers(),
  [SetupStep.SUMMARY]: () => t.login_setup_step_summary(),
};
