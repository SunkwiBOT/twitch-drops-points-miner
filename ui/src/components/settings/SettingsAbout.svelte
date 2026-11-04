<script lang="ts">
  import { tick } from "svelte";

  import { isTauri } from "../../lib/api/tauri";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { UpdateInterval } from "../../lib/bindings/UpdateInterval";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_URL, REPO_URL } from "../../lib/partners.constants";
  import { Page } from "../../lib/state/state.constants";
  import { navigate } from "../../lib/state/state.svelte";
  import { updates } from "../../lib/updates/updates.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import CatalogRefreshButton from "../catalogue/CatalogRefreshButton.svelte";
  import FeedbackChoices from "../feedback/FeedbackChoices.svelte";
  import UpdatesPanel from "../updates/UpdatesPanel.svelte";
  import Badge from "../ui/Badge.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import Select from "../ui/Select.svelte";
  import {
    BadgeTone,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    MenuAlign,
  } from "../ui/ui.constants";

  type SettingsAboutPropsType = { snapshot: SnapshotType; draft: GlobalSettingsType };

  let { snapshot, draft = $bindable() }: SettingsAboutPropsType = $props();

  const intervals = $derived([
    { value: UpdateInterval.AT_START, label: t.updates_interval_start() },
    { value: UpdateInterval.MINUTES30, label: t.updates_interval_minutes({ count: 30 }) },
    { value: UpdateInterval.HOURS1, label: t.updates_interval_hours({ count: 1 }) },
    { value: UpdateInterval.HOURS3, label: t.updates_interval_hours({ count: 3 }) },
    { value: UpdateInterval.HOURS6, label: t.updates_interval_hours({ count: 6 }) },
    { value: UpdateInterval.HOURS12, label: t.updates_interval_hours({ count: 12 }) },
    { value: UpdateInterval.HOURS24, label: t.updates_interval_hours({ count: 24 }) },
  ]);

  let about = $state<HTMLElement>();

  $effect(() => {
    if (!updates.reveal || !about) {
      return;
    }

    const target = about;

    updates.reveal = false;
    void tick().then(() => {
      target.scrollIntoView({ block: "center", behavior: "smooth" });
      target.focus({ preventScroll: true });
    });
  });
</script>

<div class="grid grid-cols-[repeat(auto-fit,minmax(min(300px,100%),1fr))] items-start gap-5">
  <div class="grid min-w-0 gap-5">
    <Panel title={t.settings_catalog()} description={t.settings_catalog_text()}>
      <div class="flex flex-wrap gap-2">
        <CatalogRefreshButton label={t.settings_catalog_refresh()} />
        <ExternalButton
          url={DROPS_WEB_URL}
          label={DROPS_API_NAME}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.MD}
        />
      </div>
    </Panel>
    <Panel title={t.settings_feedback()} description={t.settings_feedback_text()}>
      <FeedbackChoices />
      <div class="flex">
        <TextButton
          label={t.nav_logs()}
          icon={IconName.FILE}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => navigate(Page.LOGS)}
        />
      </div>
    </Panel>
  </div>
  <div bind:this={about} tabindex="-1" class="min-w-0 outline-none">
    <Panel title={t.settings_about()} description={t.settings_about_text()}>
      {#snippet actions()}
        <Badge text={`v${snapshot.version}`} tone={BadgeTone.NEUTRAL} dot={false} />
      {/snippet}
      <FormField
        id="set-update-interval"
        label={t.updates_interval()}
        hint={isTauri() ? t.updates_interval_hint() : t.updates_interval_hint_server()}
      >
        <Select
          id="set-update-interval"
          value={draft.update_interval}
          options={intervals}
          label={t.updates_interval()}
          align={MenuAlign.LEFT}
          onchange={(value) => (draft.update_interval = value)}
        />
      </FormField>
      <UpdatesPanel checkedAt={snapshot.update_checked_at ?? null} />
      <div class="flex">
        <ExternalButton
          url={REPO_URL}
          label={t.settings_about_repo()}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.MD}
          icon={IconName.FILE}
        />
      </div>
    </Panel>
  </div>
</div>
