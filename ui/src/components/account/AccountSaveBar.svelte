<script lang="ts">
  import { untrack } from "svelte";

  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { editsOf, saveAccountEdits } from "../../lib/state/edits.svelte";
  import UnsavedBar from "../settings/UnsavedBar.svelte";

  type AccountSaveBarPropsType = { account: AccountSnapshotType };

  let { account }: AccountSaveBarPropsType = $props();

  const edits = editsOf(untrack(() => account.id));
  const dirty = $derived(edits.differ(account.settings));
  let saving = $state(false);

  const save = async (): Promise<void> => {
    saving = true;
    await saveAccountEdits(account);
    saving = false;
  };
</script>

{#if dirty}
  <UnsavedBar {saving} oncancel={() => edits.clear()} onsave={save} />
{/if}
