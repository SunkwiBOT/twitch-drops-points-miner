<script lang="ts">
  import { MODE_LABELS } from "../../lib/accounts/accounts.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { DropsScope } from "../../lib/bindings/DropsScope";
  import { Mode } from "../../lib/bindings/Mode";
  import type { StreamerSettingsType } from "../../lib/bindings/StreamerSettingsType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Meta from "../ui/Meta.svelte";
  import MetaList from "../ui/MetaList.svelte";
  import { MetaLayout } from "../ui/ui.constants";

  type SetupSummaryPropsType = {
    source: AccountSnapshotType | null;
    mode: Mode;
    scope: DropsScope;
    autoClaim: boolean;
    streamers: StreamerSettingsType[];
  };

  let { source, mode, scope, autoClaim, streamers }: SetupSummaryPropsType = $props();

  const SCOPE_LABELS: Record<DropsScope, () => string> = {
    [DropsScope.ALL]: () => t.drops_scope_all(),
    [DropsScope.LINKED_ONLY]: () => t.drops_scope_linked_only(),
    [DropsScope.PRIORITY_ONLY]: () => t.drops_scope_priority_only(),
  };
</script>

<div class="rounded-lg border px-4 py-2">
  <MetaList>
    {#if source}
      <Meta
        label={t.login_setup_from()}
        value={source.display_name}
        layout={MetaLayout.ROW}
        mono={false}
      />
    {/if}
    <Meta
      label={t.account_mode_title()}
      value={MODE_LABELS[mode]()}
      layout={MetaLayout.ROW}
      mono={false}
    />
    {#if mode !== Mode.POINTS}
      <Meta
        label={t.drops_scope()}
        value={SCOPE_LABELS[scope]()}
        layout={MetaLayout.ROW}
        mono={false}
      />
      <Meta
        label={t.account_drops_auto_claim()}
        value={autoClaim ? t.common_yes() : t.common_no()}
        layout={MetaLayout.ROW}
        mono={false}
      />
    {/if}
    {#if mode !== Mode.DROPS}
      <Meta
        label={t.login_setup_streamers_title()}
        value={String(streamers.length)}
        layout={MetaLayout.ROW}
        mono={false}
      />
    {/if}
  </MetaList>
</div>
