<script lang="ts">
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { Card } from "$lib/components/ui/card";
  import HistoryList from "../HistoryList.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Select from "../ui/Select.svelte";
  import { MenuAlign } from "../ui/ui.constants";

  type HistoryPropsType = { snapshot: SnapshotType };

  let { snapshot }: HistoryPropsType = $props();

  let accountId = $state("");

  const options = $derived([
    { value: "", label: t.history_filter_all() },
    ...snapshot.accounts.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url })),
  ]);
</script>

<PageBody>
  <Card class="px-5 py-4">
    {#key accountId}
      <HistoryList accountId={accountId || null} showAccount={accountId === ""}>
        {#snippet lead()}
          <div class="w-56">
            <Select
              value={accountId}
              {options}
              label={t.common_account()}
              align={MenuAlign.LEFT}
              onchange={(id) => (accountId = id)}
            />
          </div>
        {/snippet}
      </HistoryList>
    {/key}
  </Card>
</PageBody>
