<script lang="ts">
  import { untrack } from "svelte";

  import { cleanLogin } from "../../lib/accounts/accounts.utils";
  import { insertLike, placeOf, reorderBy } from "../../lib/accounts/changes.utils";
  import { newStreamer } from "../../lib/accounts/import.svelte";
  import { lookupChannel } from "../../lib/api/client";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import type { StreamerSettingsType } from "../../lib/bindings/StreamerSettingsType";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { escapeEmphasis, personMarker, strong, type TextPersonType } from "../../lib/text.utils";
  import { changed } from "../../lib/state/bulk.svelte";
  import { updateAccountSettings } from "../../lib/state/state.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import StreamerChannelCard from "./StreamerChannelCard.svelte";
  import StreamerOptions from "./StreamerOptions.svelte";
  import StreamerSearchForm from "./StreamerSearchForm.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/ui.constants";

  type StreamerDialogPropsType = {
    open: boolean;
    account: AccountSnapshotType;
    login: string | null;
    known: ChannelType | undefined;
    onfound: (channel: ChannelType) => void;
    onclose: () => void;
  };

  let { open, account, login, known, onfound, onclose }: StreamerDialogPropsType = $props();

  let query = $state("");
  let searching = $state(false);
  let problem = $state("");
  let problemPeople = $state<TextPersonType[]>([]);
  let channel = $state<ChannelType | null>(null);
  let draft = $state<StreamerSettingsType | null>(null);

  const listed = (name: string): boolean =>
    account.settings.points.streamers.some((s) => s.login === name);

  const lookUp = async (wanted: string): Promise<void> => {
    try {
      const found = await lookupChannel(wanted);

      if (open && login === wanted) {
        channel = found;
      }
    } catch (e: any) {
      // ! Without an answer, the card names the streamer by login.
    }
  };

  $effect(() => {
    if (!open) {
      return;
    }

    untrack(() => {
      problem = "";
      query = "";

      const current = login
        ? account.settings.points.streamers.find((s) => s.login === login)
        : null;

      draft = current ? $state.snapshot(current) : null;
      channel = login ? (known ?? null) : null;

      if (login && !known) {
        void lookUp(login);
      }
    });
  });

  const search = async (): Promise<void> => {
    if (searching) {
      return;
    }

    const wanted = cleanLogin(query);

    problem = "";

    if (!wanted) {
      return;
    }

    if (listed(wanted)) {
      problem = t.points_already_added({ login: strong(wanted) });

      return;
    }

    searching = true;

    try {
      const found = await lookupChannel(wanted);

      if (listed(found.login)) {
        problem = t.points_already_added({ login: personMarker(0) });
        problemPeople = [
          { name: found.display_name, avatar: found.avatar_url, channel: found.login },
        ];

        return;
      }

      onfound(found);
      channel = found;
      draft = newStreamer(account.settings.points.streamer_defaults, found.login);
    } catch (e: any) {
      problem = escapeEmphasis(describeError(e));
    } finally {
      searching = false;
    }
  };

  const save = (): void => {
    if (!draft) {
      return;
    }

    const next = $state.snapshot(draft);
    const subject = {
      url: channel?.avatar_url,
      name: channel?.display_name ?? next.login,
      channel: next.login,
    };

    if (login) {
      void updateAccountSettings(account.id, (settings) => {
        const index = settings.points.streamers.findIndex((s) => s.login === next.login);

        if (index >= 0) {
          settings.points.streamers[index] = next;
        }
      });

      changed(
        account.id,
        t.change_streamer_edited({ streamer: strong(subject.name) }),
        (settings) => {
          const index = settings.points.streamers.findIndex((s) => s.login === next.login);

          if (index >= 0) {
            const enabled = settings.points.streamers[index].enabled;
            settings.points.streamers[index] = { ...structuredClone(next), enabled };
          }
        },
        subject,
      );
    } else {
      const order = [...account.settings.points.streamers.map((s) => s.login), next.login];
      const place = placeOf(order, next.login);

      void updateAccountSettings(account.id, (settings) => {
        if (!settings.points.streamers.some((s) => s.login === next.login)) {
          settings.points.streamers.push(next);
        }
      });

      changed(
        account.id,
        t.change_streamer_added({ streamer: strong(subject.name) }),
        (settings) => {
          const logins = settings.points.streamers.map((s) => s.login);
          const placed = insertLike(logins, next.login, place);

          if (placed) {
            settings.points.streamers = reorderBy(
              [...settings.points.streamers, structuredClone(next)],
              (s) => s.login,
              placed,
            );
          }
        },
        subject,
      );
    }

    onclose();
  };
</script>

<Dialog
  {open}
  title={login ? t.streamer_edit_title() : t.points_add_title()}
  wide={draft !== null}
  closable={!searching}
  closeLabel={t.common_close()}
  {onclose}
>
  <div class="flex flex-col gap-4">
    {#if !login && !channel}
      <StreamerSearchForm
        bind:query
        {problem}
        {problemPeople}
        {searching}
        {listed}
        onsearch={() => void search()}
        onedit={() => (problem = "")}
      />
    {/if}

    {#if channel || login}
      <StreamerChannelCard
        {channel}
        name={channel?.display_name ?? login ?? ""}
        onchange={login
          ? null
          : () => {
              channel = null;
              draft = null;
            }}
      />
    {/if}

    {#if draft}
      <StreamerOptions bind:options={draft} mixed={[]} />
    {/if}
  </div>
  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={searching}
      onclick={onclose}
    />
    {#if draft}
      <TextButton
        label={login ? t.common_save() : t.common_add()}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={save}
      />
    {/if}
  {/snippet}
</Dialog>
