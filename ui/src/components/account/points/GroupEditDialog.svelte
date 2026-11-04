<script lang="ts">
  import { untrack } from "svelte";

  import { GROUP_CHIPS_MAX, type StreamerOption } from "../../../lib/accounts/accounts.constants";
  import { differingOptions, mixedOptions, withParts } from "../../../lib/accounts/options.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../../lib/bindings/ChannelType";
  import type { StreamerSettingsType } from "../../../lib/bindings/StreamerSettingsType";
  import { number } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import StreamerChip from "../../accounts/StreamerChip.svelte";
  import { Badge as BadgeRoot } from "$lib/components/ui/badge";
  import Dialog from "../../ui/Dialog.svelte";
  import TextButton from "../../ui/TextButton.svelte";
  import {
    BADGE_VARIANTS,
    BadgeVariant,
    ButtonSize,
    ButtonType,
    ButtonVariant,
  } from "../../ui/ui.constants";
  import StreamerOptions from "../StreamerOptions.svelte";

  type GroupEditDialogPropsType = {
    open: boolean;
    account: AccountSnapshotType;
    logins: string[];
    channelOf: (login: string) => ChannelType | undefined;
    onsave: (values: StreamerSettingsType, changes: StreamerOption[]) => void;
    onclose: () => void;
  };

  let { open, account, logins, channelOf, onsave, onclose }: GroupEditDialogPropsType = $props();

  let members = $state<string[]>([]);
  let draft = $state<StreamerSettingsType | null>(null);
  let mixed = $state<StreamerOption[]>([]);
  let touched = $state<StreamerOption[]>([]);

  const group = $derived(
    account.settings.points.streamers.filter((s) => members.includes(s.login)),
  );

  $effect(() => {
    if (!open) {
      return;
    }

    untrack(() => {
      members = [...logins];

      const streamers = $state.snapshot(group);

      draft = streamers[0] ?? null;
      mixed = mixedOptions(streamers);
      touched = [];
    });
  });

  const shownMixed = $derived(mixed.filter((option) => !touched.includes(option)));

  const chips = $derived(
    group.length > GROUP_CHIPS_MAX ? group.slice(0, GROUP_CHIPS_MAX - 1) : group,
  );

  const touch = (option: StreamerOption): void => {
    touched = [...new Set([...touched, ...withParts(option)])];
  };

  const changes = $derived.by((): StreamerOption[] => {
    const values = draft;

    if (!values) {
      return [];
    }

    return touched.filter((option) =>
      group.some((streamer) => differingOptions(values, streamer, [option]).length > 0),
    );
  });

  const save = (): void => {
    if (draft) {
      onsave($state.snapshot(draft), changes);
    }
  };
</script>

<Dialog
  {open}
  title={t.points_group_title({ count: group.length })}
  wide={true}
  closable={true}
  closeLabel={t.common_close()}
  {onclose}
>
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-2">
      <div class="flex flex-wrap gap-1.5">
        {#each chips as streamer (streamer.login)}<StreamerChip
            login={streamer.login}
            channel={channelOf(streamer.login)}
          />{/each}
        {#if chips.length < group.length}<BadgeRoot
            variant={BADGE_VARIANTS[BadgeVariant.CHIP]}
            class="h-7 px-2.5 text-hint font-semibold"
            >+{number(group.length - chips.length)}</BadgeRoot
          >{/if}
      </div>
      <p class="text-hint text-muted-foreground">{t.points_group_hint()}</p>
    </div>

    {#if draft}
      <StreamerOptions bind:options={draft} mixed={shownMixed} onchange={touch} />
    {/if}
  </div>
  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={onclose}
    />
    <TextButton
      label={t.common_save()}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={changes.length === 0}
      onclick={save}
    />
  {/snippet}
</Dialog>
