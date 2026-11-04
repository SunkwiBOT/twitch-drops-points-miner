<script lang="ts">
  import { untrack } from "svelte";

  import { dropsScopeChoices } from "../../../lib/accounts/scope.utils";
  import { editsOf } from "../../../lib/state/edits.svelte";
  import { AccountField } from "../../../lib/state/state.constants";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import { DropsOrder } from "../../../lib/bindings/DropsOrder";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import ChoiceCards from "../../ui/ChoiceCards.svelte";
  import FormField from "../../ui/FormField.svelte";
  import Panel from "../../ui/Panel.svelte";
  import Select from "../../ui/Select.svelte";
  import Setting from "../../ui/Setting.svelte";
  import { MenuAlign } from "../../ui/ui.constants";

  type DropsSettingsTabPropsType = { account: AccountSnapshotType };

  let { account }: DropsSettingsTabPropsType = $props();

  const edits = editsOf(untrack(() => account.id));
  const drops = $derived(edits.applied(account.settings).drops);
</script>

<div class="grid grid-cols-[repeat(auto-fit,minmax(min(460px,100%),1fr))] items-start gap-5">
  <Panel title={t.drops_claim_title()}>
    <div>
      <Setting
        title={t.account_drops_auto_claim()}
        description={t.drops_auto_claim_hint()}
        checked={drops.auto_claim}
        disabled={false}
        onchange={(value) =>
          edits.set(AccountField.AUTO_CLAIM, (s) => (s.drops.auto_claim = value))}
      />
      <Setting
        title={t.account_drops_skip_unfinishable()}
        description={t.drops_skip_hint()}
        checked={drops.skip_unfinishable}
        disabled={false}
        onchange={(value) =>
          edits.set(AccountField.SKIP_UNFINISHABLE, (s) => (s.drops.skip_unfinishable = value))}
      />
      <Setting
        title={t.drops_mine_points()}
        description={t.drops_mine_points_hint()}
        checked={drops.mine_points}
        disabled={false}
        onchange={(value) =>
          edits.set(AccountField.MINE_POINTS, (s) => (s.drops.mine_points = value))}
      />
      <Setting
        title={t.drops_points_followed_only()}
        description={t.drops_points_followed_only_hint()}
        checked={drops.points_followed_only}
        disabled={!drops.mine_points}
        onchange={(value) =>
          edits.set(
            AccountField.POINTS_FOLLOWED_ONLY,
            (s) => (s.drops.points_followed_only = value),
          )}
      />
      <Setting
        title={t.drops_streamers_first()}
        description={t.drops_streamers_first_hint()}
        checked={drops.streamers_first}
        disabled={false}
        onchange={(value) =>
          edits.set(AccountField.STREAMERS_FIRST, (s) => (s.drops.streamers_first = value))}
      />
    </div>
  </Panel>
  <Panel title={t.drops_scope()}>
    <ChoiceCards
      vertical={true}
      value={drops.scope}
      label={t.drops_scope()}
      onchange={(scope) => edits.set(AccountField.SCOPE, (s) => (s.drops.scope = scope))}
      choices={dropsScopeChoices(true)}
    />
    <Setting
      title={t.drops_linked_first()}
      description={t.drops_linked_first_hint()}
      checked={drops.linked_first}
      disabled={false}
      onchange={(value) =>
        edits.set(AccountField.LINKED_FIRST, (s) => (s.drops.linked_first = value))}
    />
    <FormField id="drops-order" label={t.drops_order()}>
      <Select
        id="drops-order"
        value={drops.order}
        label={t.drops_order()}
        align={MenuAlign.LEFT}
        onchange={(order) => edits.set(AccountField.ORDER, (s) => (s.drops.order = order))}
        options={[
          {
            value: DropsOrder.ENDING_SOONEST,
            label: t.drops_order_ending_soonest(),
            description: t.drops_order_ending_hint(),
          },
          {
            value: DropsOrder.LOWEST_REMAINING,
            label: t.drops_order_lowest_remaining(),
            description: t.drops_order_lowest_hint(),
          },
        ]}
      />
    </FormField>
  </Panel>
</div>
