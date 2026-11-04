<script lang="ts">
  import type { ImportActionType } from "../../lib/accounts/import.svelte";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import type { StreamerSettingsType } from "../../lib/bindings/StreamerSettingsType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { personMarker } from "../../lib/text.utils";
  import StreamerImport from "../account/StreamerImport.svelte";
  import ConfirmDialog from "../ui/ConfirmDialog.svelte";
  import StreamerChip from "./StreamerChip.svelte";

  type SetupStreamersStepPropsType = {
    account: AccountSnapshotType;
    streamers: StreamerSettingsType[];
    channelFor: (login: string) => ChannelType | undefined;
    importAction: ImportActionType | null;
    onadd: (logins: string[], channels: ChannelType[]) => void;
    onremove: (login: string) => void;
  };

  let {
    account,
    streamers,
    channelFor,
    importAction = $bindable(),
    onadd,
    onremove,
  }: SetupStreamersStepPropsType = $props();

  let removing = $state<string | null>(null);

  const removingChannel = $derived(removing ? channelFor(removing) : undefined);

  const confirmRemove = (): void => {
    const login = removing;

    removing = null;

    if (login !== null) {
      onremove(login);
    }
  };
</script>

{#if streamers.length === 0}
  <p class="text-hint text-muted-foreground">{t.login_setup_streamers_empty()}</p>
{:else}
  <div class="flex flex-wrap gap-1.5">
    {#each streamers as streamer (streamer.login)}<StreamerChip
        login={streamer.login}
        channel={channelFor(streamer.login)}
        onremove={() => (removing = streamer.login)}
      />{/each}
  </div>
{/if}
<StreamerImport
  bind:primary={importAction}
  target={account}
  existing={streamers.map((s) => s.login)}
  {onadd}
  searchable={true}
/>

<ConfirmDialog
  open={removing !== null}
  title={t.confirm_remove_streamer_title()}
  text={t.confirm_remove_setup_streamer_text({ name: personMarker(0) })}
  people={[
    {
      name: removingChannel?.display_name ?? removing ?? "",
      avatar: removingChannel?.avatar_url,
      channel: removing,
    },
  ]}
  confirmLabel={t.confirm_remove()}
  busyLabel={t.common_removing()}
  busy={false}
  onconfirm={confirmRemove}
  oncancel={() => (removing = null)}
/>
