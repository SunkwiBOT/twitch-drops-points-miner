<script lang="ts">
  import type { ImportCandidateType } from "../../../lib/accounts/import.svelte";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import { accountChannel } from "../../../lib/channels/channels.svelte";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { app } from "../../../lib/state/state.svelte";
  import FormField from "../../ui/FormField.svelte";
  import Select from "../../ui/Select.svelte";
  import { MenuAlign } from "../../ui/ui.constants";

  type AccountImportPropsType = {
    open: boolean;
    target: AccountSnapshotType;
  };

  let { open, target }: AccountImportPropsType = $props();

  const others = $derived((app.snapshot?.accounts ?? []).filter((a) => a.id !== target.id));
  let fromId = $state<string | null>(null);
  const from = $derived(others.find((a) => a.id === fromId) ?? others[0] ?? null);

  const accountOptions = $derived(
    others.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url })),
  );

  const candidates = $derived(
    from
      ? from.settings.points.streamers.map((streamer) => ({
          login: streamer.login,
          channel: accountChannel(from, streamer.login),
        }))
      : null,
  );

  export const offered = (): ImportCandidateType[] | null => candidates;
</script>

{#if open}
  {#if others.length === 0}
    <p class="text-hint text-muted-foreground">{t.import_no_account()}</p>
  {:else}
    <FormField id="import-from" label={t.import_from()}>
      <Select
        id="import-from"
        value={from?.id ?? ""}
        options={accountOptions}
        label={t.import_from()}
        align={MenuAlign.LEFT}
        onchange={(value) => (fromId = value)}
      />
    </FormField>
  {/if}
{/if}
