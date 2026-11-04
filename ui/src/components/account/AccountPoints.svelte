<script lang="ts">
  import { untrack } from "svelte";

  import { editsOf } from "../../lib/state/edits.svelte";
  import { AccountField } from "../../lib/state/state.constants";
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import ConfirmDialog from "../ui/ConfirmDialog.svelte";
  import { CopySection, type StreamerOption } from "../../lib/accounts/accounts.constants";
  import { moveLike, placeOf, reorderBy } from "../../lib/accounts/changes.utils";
  import { copyOptions, differingOptions } from "../../lib/accounts/options.utils";
  import type { AccountSettingsType } from "../../lib/bindings/AccountSettingsType";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import type { StreamerSettingsType } from "../../lib/bindings/StreamerSettingsType";
  import {
    ensureChannels,
    knownChannels,
    rememberChannels,
  } from "../../lib/channels/channels.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { personMarker, strong } from "../../lib/text.utils";
  import { changed } from "../../lib/state/bulk.svelte";
  import type { ToastImageType } from "../../lib/state/state.svelte";
  import { app, updateAccountSettings } from "../../lib/state/state.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Panel from "../ui/Panel.svelte";
  import Setting from "../ui/Setting.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";
  import CopyFromDialog from "./CopyFromDialog.svelte";
  import DefaultsOfferDialog from "./points/DefaultsOfferDialog.svelte";
  import GroupEditDialog from "./points/GroupEditDialog.svelte";
  import ImportDialog from "./points/ImportDialog.svelte";
  import StreamerList from "./points/StreamerList.svelte";
  import StreamerDialog from "./StreamerDialog.svelte";

  type AccountPointsPropsType = { account: AccountSnapshotType };

  let { account }: AccountPointsPropsType = $props();

  const streamers = $derived(account.settings.points.streamers);
  const hasOthers = $derived((app.snapshot?.accounts.length ?? 0) > 1);

  const update = (mutate: (list: StreamerSettingsType[]) => void): Promise<boolean> =>
    updateAccountSettings(account.id, (settings) => mutate(settings.points.streamers));

  const channelOf = (login: string): ChannelType | undefined =>
    account.streamers.find((s) => s.login === login)?.channel ?? knownChannels[login];

  $effect(() => {
    ensureChannels(streamers.map((s) => s.login).filter((login) => !channelOf(login)));
  });

  const subjectOf = (login: string): ToastImageType => {
    const channel = channelOf(login);

    return { url: channel?.avatar_url, name: channel?.display_name ?? login, channel: login };
  };

  let adding = $state(false);
  let editing = $state<string | null>(null);
  let importing = $state(false);
  let copying = $state(false);

  const members = $derived(
    streamers
      .map((s) => s.login)
      .sort()
      .join("\n"),
  );

  let picked = $state<{ members: string; logins: string[] }>({ members: "", logins: [] });
  const selected = $derived(picked.members === members ? picked.logins : []);

  const select = (logins: string[]): void => {
    picked = { members, logins };
  };

  let grouping = $state(false);

  let offer = $state<{ options: StreamerOption[]; values: StreamerSettingsType } | null>(null);
  let offering = $state(false);

  const saveGroup = (values: StreamerSettingsType, changes: StreamerOption[]): void => {
    const logins = new Set(selected);

    grouping = false;
    select([]);

    if (changes.length === 0) {
      return;
    }

    const apply = (settings: AccountSettingsType): void => {
      for (const streamer of settings.points.streamers) {
        if (logins.has(streamer.login)) {
          copyOptions(values, streamer, changes);
        }
      }
    };

    void updateAccountSettings(account.id, apply);
    changed(account.id, t.change_streamers_edited({ count: strong(logins.size) }), apply);

    const offered =
      logins.size > 1
        ? differingOptions(values, account.settings.points.streamer_defaults, changes)
        : [];

    if (offered.length > 0) {
      offer = { options: offered, values };
      offering = true;
    }
  };

  const takeDefaults = (): void => {
    offering = false;

    if (!offer) {
      return;
    }

    const { options, values } = offer;

    const apply = (settings: AccountSettingsType): void => {
      copyOptions(values, settings.points.streamer_defaults, options);
    };

    void updateAccountSettings(account.id, apply);
    changed(account.id, t.change_streamer_defaults(), apply);
  };

  const setEnabled = (login: string, enabled: boolean): void => {
    const subject = subjectOf(login);

    const apply = (list: StreamerSettingsType[]): void => {
      const streamer = list.find((item) => item.login === login);

      if (streamer) {
        streamer.enabled = enabled;
      }
    };

    void update(apply);

    changed(
      account.id,
      enabled
        ? t.change_streamer_enabled({ streamer: strong(subject.name) })
        : t.change_streamer_disabled({ streamer: strong(subject.name) }),
      (settings) => apply(settings.points.streamers),
      subject,
    );
  };

  let removing = $state<string | null>(null);
  const removingChannel = $derived(removing ? channelOf(removing) : undefined);

  const remove = (login: string): void => {
    const subject = subjectOf(login);

    void update((list) => {
      const index = list.findIndex((item) => item.login === login);

      if (index >= 0) {
        list.splice(index, 1);
      }
    });

    changed(
      account.id,
      t.change_streamer_removed({ streamer: strong(subject.name) }),
      (settings) => {
        settings.points.streamers = settings.points.streamers.filter((s) => s.login !== login);
      },
      subject,
    );
  };

  const reorder = (list: StreamerSettingsType[], moved: StreamerSettingsType): void => {
    const order = list.map((item) => item.login);
    const place = placeOf(order, moved.login);
    const subject = subjectOf(moved.login);

    void update((current) => {
      current.splice(0, current.length, ...reorderBy(current, (item) => item.login, order));
    });

    changed(
      account.id,
      t.change_streamer_moved({ streamer: strong(subject.name) }),
      (settings) => {
        const logins = settings.points.streamers.map((s) => s.login);
        const next = moveLike(logins, moved.login, place);

        if (next) {
          settings.points.streamers = reorderBy(settings.points.streamers, (s) => s.login, next);
        }
      },
      subject,
    );
  };

  const edits = editsOf(untrack(() => account.id));

  const shown = $derived(edits.applied(account.settings).points);

  const setStreakPriority = (value: boolean): void => {
    edits.set(
      AccountField.STREAK_PRIORITY,
      (settings) => (settings.points.streak_priority = value),
    );
  };

  const setSubscriberPriority = (value: boolean): void => {
    edits.set(
      AccountField.SUBSCRIBER_PRIORITY,
      (settings) => (settings.points.subscriber_priority = value),
    );
  };
</script>

<PageBody>
  <Panel
    title={t.account_points_streamers()}
    count={streamers.length}
    description={t.account_points_streamers_hint()}
  >
    {#snippet actions()}
      {#if hasOthers}
        <TextButton
          label={t.copy_points_from_account()}
          icon={IconName.COPY}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => (copying = true)}
        />
      {/if}
      <TextButton
        label={t.points_import()}
        icon={IconName.IMPORT}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (importing = true)}
      />
      <TextButton
        label={t.points_add_title()}
        icon={IconName.PLUS}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (adding = true)}
      />
    {/snippet}
    <div>
      <Setting
        title={t.account_points_streak_priority()}
        description={t.points_streak_priority_hint()}
        checked={shown.streak_priority}
        disabled={false}
        onchange={setStreakPriority}
      />
      <Setting
        title={t.points_subscriber_priority()}
        description={t.points_subscriber_priority_hint()}
        checked={shown.subscriber_priority}
        disabled={false}
        onchange={setSubscriberPriority}
      />
    </div>
    <StreamerList
      {account}
      {channelOf}
      {selected}
      onselect={select}
      oneditselected={() => (grouping = true)}
      onreorder={reorder}
      ontoggle={setEnabled}
      onedit={(login) => (editing = login)}
      onremove={(login) => (removing = login)}
    />
  </Panel>
  <AccountSaveBar {account} />
</PageBody>

<ConfirmDialog
  open={removing !== null}
  title={t.confirm_remove_streamer_title()}
  text={t.confirm_remove_streamer_text({ name: personMarker(0) })}
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
  onconfirm={() => {
    if (removing) {
      remove(removing);
    }

    removing = null;
  }}
  oncancel={() => (removing = null)}
/>

<StreamerDialog
  open={adding || editing !== null}
  {account}
  login={editing}
  known={editing ? channelOf(editing) : undefined}
  onfound={(channel) => rememberChannels([channel])}
  onclose={() => {
    adding = false;
    editing = null;
  }}
/>

<ImportDialog open={importing} {account} onclose={() => (importing = false)} />

<GroupEditDialog
  open={grouping}
  {account}
  logins={selected}
  {channelOf}
  onsave={saveGroup}
  onclose={() => (grouping = false)}
/>

<DefaultsOfferDialog
  open={offering}
  options={offer?.options ?? []}
  onyes={takeDefaults}
  onno={() => (offering = false)}
/>

<CopyFromDialog
  open={copying}
  {account}
  section={CopySection.POINTS}
  onclose={() => (copying = false)}
/>
