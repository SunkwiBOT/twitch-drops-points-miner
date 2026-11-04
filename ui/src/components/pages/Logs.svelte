<script lang="ts">
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { openTab, showTab } from "../../lib/state/state.svelte";
  import LogLines from "../logs/LogLines.svelte";
  import RequestList from "../logs/RequestList.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { IconName } from "../ui/ui.constants";
  import { segmentId } from "../ui/ui.utils";
  import { LOGS_PANEL_ID, LOGS_TAB_SLUGS, LogsTab } from "./pages.constants";

  type LogsPropsType = { snapshot: SnapshotType };

  let { snapshot }: LogsPropsType = $props();

  const tab = $derived(openTab(Object.values(LogsTab), LOGS_TAB_SLUGS, LogsTab.LINES));
</script>

<PageBody>
  <div>
    <Segmented
      value={tab}
      label={t.logs_title()}
      panel={LOGS_PANEL_ID}
      onchange={(next) => showTab(LOGS_TAB_SLUGS, next)}
      options={[
        { value: LogsTab.LINES, label: t.logs_tab_lines(), icon: IconName.FILE },
        { value: LogsTab.REQUESTS, label: t.logs_tab_requests(), icon: IconName.GLOBE },
      ]}
    />
  </div>

  <div
    id={LOGS_PANEL_ID}
    role="tabpanel"
    aria-labelledby={segmentId(LOGS_PANEL_ID, tab)}
    class="flex flex-col gap-5 *:shrink-0"
  >
    {#if tab === LogsTab.LINES}
      <LogLines {snapshot} />
    {:else}
      <RequestList {snapshot} />
    {/if}
  </div>
</PageBody>
