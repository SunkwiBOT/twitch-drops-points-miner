<script lang="ts">
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { Page } from "../../lib/state/state.constants";
  import { app, navigate } from "../../lib/state/state.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { Card } from "$lib/components/ui/card";
  import HistoryList from "../HistoryList.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  type AccountHistoryPropsType = { account: AccountSnapshotType };

  let { account }: AccountHistoryPropsType = $props();
</script>

<PageBody>
  <Card class="px-5 py-4">
    {#key account.id}
      <HistoryList accountId={account.id} showAccount={false}>
        {#snippet lead()}
          <TextButton
            label={t.nav_logs()}
            icon={IconName.FILE}
            variant={ButtonVariant.OUTLINE}
            size={ButtonSize.MD}
            type={ButtonType.BUTTON}
            disabled={false}
            onclick={() => {
              app.logsAccount = account.id;
              navigate(Page.LOGS);
            }}
          />
        {/snippet}
      </HistoryList>
    {/key}
  </Card>
</PageBody>
