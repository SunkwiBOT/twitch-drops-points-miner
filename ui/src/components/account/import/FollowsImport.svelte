<script lang="ts">
  import { fromChannels, type ImportCandidateType } from "../../../lib/accounts/import.svelte";
  import { fetchFollows } from "../../../lib/api/client";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../../lib/bindings/ChannelType";
  import { rememberChannels } from "../../../lib/channels/channels.svelte";
  import { describeError } from "../../../lib/errors/errors.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { personMarker } from "../../../lib/text.utils";
  import ActionButton from "../../ui/ActionButton.svelte";
  import RichText from "../../ui/RichText.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../../ui/ui.constants";

  type FollowsImportPropsType = {
    open: boolean;
    target: AccountSnapshotType;
  };

  let { open, target }: FollowsImportPropsType = $props();

  let follows = $state<ChannelType[] | null>(null);
  let loading = $state(false);
  let problem = $state("");

  const load = async (): Promise<void> => {
    loading = true;
    problem = "";

    try {
      follows = await fetchFollows(target.id);
      rememberChannels(follows);
    } catch (e: any) {
      problem = describeError(e);
    } finally {
      loading = false;
    }
  };

  $effect(() => {
    if (open && follows === null && !loading && !problem) {
      void load();
    }
  });

  const candidates = $derived(follows ? fromChannels(follows) : null);

  export const offered = (): ImportCandidateType[] | null => candidates;
</script>

{#if open}
  <div class="flex flex-wrap items-center justify-between gap-2">
    <span class="text-hint text-muted-foreground"
      ><RichText
        text={t.import_follows_hint({ name: personMarker(0) })}
        people={[{ name: target.display_name, avatar: target.avatar_url, channel: null }]}
      /></span
    >
    <ActionButton
      icon={IconName.REFRESH}
      label={t.common_refresh()}
      busyLabel={t.common_loading()}
      busy={loading}
      disabled={false}
      variant={ButtonVariant.DEFAULT}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      onclick={load}
    />
  </div>
  {#if problem}<p class="text-hint text-destructive">{problem}</p>{/if}
{/if}
