<script lang="ts">
  import {
    IMPORT_BATCH,
    IMPORT_STAGGER_MS,
    ImportEntryStatus,
  } from "../../../lib/accounts/accounts.constants";
  import { normalizeLogin, splitEntries } from "../../../lib/accounts/accounts.utils";
  import {
    fromChannels,
    type ImportActionType,
    type ImportCandidateType,
  } from "../../../lib/accounts/import.svelte";
  import { lookupChannels } from "../../../lib/api/client";
  import type { ChannelType } from "../../../lib/bindings/ChannelType";
  import { CHANNELS_LOOKUP_MAX } from "../../../lib/channels/channels.constants";
  import { isLive, rememberChannels } from "../../../lib/channels/channels.svelte";
  import { describeError } from "../../../lib/errors/errors.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { strong } from "../../../lib/text.utils";
  import { Textarea } from "$lib/components/ui/textarea";
  import { cn } from "$lib/utils";
  import ChannelAvatar from "../../ui/ChannelAvatar.svelte";
  import Icon from "../../ui/Icon.svelte";
  import RichText from "../../ui/RichText.svelte";
  import Spinner from "../../ui/Spinner.svelte";
  import TextButton from "../../ui/TextButton.svelte";
  import {
    AvatarSize,
    BADGE_TONE_CLASSES,
    BadgeTone,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
  } from "../../ui/ui.constants";

  type PasteImportPropsType = {
    open: boolean;
    existing: string[];
    offered: ImportCandidateType[] | null;
    check: ImportActionType | null;
  };

  type EntryType = {
    entry: string;
    login: string | null;
    status: ImportEntryStatus;
    channel: ChannelType | null;
  };

  let {
    open,
    existing,
    offered = $bindable(),
    check = $bindable(),
  }: PasteImportPropsType = $props();

  const ENTRY_CLASSES: Record<ImportEntryStatus, string> = {
    [ImportEntryStatus.PENDING]: "border-dashed border-input text-muted-foreground",
    [ImportEntryStatus.FOUND]: BADGE_TONE_CLASSES[BadgeTone.SUCCESS],
    [ImportEntryStatus.ALREADY]: BADGE_TONE_CLASSES[BadgeTone.MUTED],
    [ImportEntryStatus.MISSING]: BADGE_TONE_CLASSES[BadgeTone.DANGER],
    [ImportEntryStatus.UNCHECKED]: BADGE_TONE_CLASSES[BadgeTone.WARNING],
    [ImportEntryStatus.INVALID]: BADGE_TONE_CLASSES[BadgeTone.DANGER],
  };

  const ENTRY_TITLES: Record<ImportEntryStatus, (() => string) | null> = {
    [ImportEntryStatus.PENDING]: null,
    [ImportEntryStatus.FOUND]: null,
    [ImportEntryStatus.ALREADY]: () => t.import_entry_already(),
    [ImportEntryStatus.MISSING]: () => t.import_entry_missing(),
    [ImportEntryStatus.UNCHECKED]: () => t.import_entry_unchecked(),
    [ImportEntryStatus.INVALID]: () => t.import_entry_invalid(),
  };

  let pasted = $state("");
  let checking = $state(false);
  let problem = $state("");
  let entries = $state<EntryType[]>([]);
  const found = $derived(entries.flatMap((e) => (e.channel ? [e.channel] : [])));
  const done = $derived(entries.filter((e) => e.status !== ImportEntryStatus.PENDING).length);

  const failed = $derived(
    entries.filter(
      (e) => e.status === ImportEntryStatus.MISSING || e.status === ImportEntryStatus.INVALID,
    ).length,
  );

  const unchecked = $derived(
    entries.filter((e) => e.status === ImportEntryStatus.UNCHECKED).length,
  );

  const wait = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

  const settle = async (index: number, status: ImportEntryStatus, channel: ChannelType | null) => {
    entries[index] = { ...entries[index], status, channel };
    await wait(IMPORT_STAGGER_MS);
  };

  const run = async (): Promise<void> => {
    const names = splitEntries(pasted);
    problem = "";

    if (names.length === 0) {
      return;
    }

    if (names.length > CHANNELS_LOOKUP_MAX) {
      problem = t.import_too_many({ max: strong(CHANNELS_LOOKUP_MAX) });

      return;
    }

    const seen = new Set<string>();

    entries = names.flatMap((entry) => {
      const login = normalizeLogin(entry);
      const key = login ?? entry.toLowerCase();

      if (seen.has(key)) {
        return [];
      }

      seen.add(key);

      return [
        {
          entry,
          login,
          status: login ? ImportEntryStatus.PENDING : ImportEntryStatus.INVALID,
          channel: null,
        },
      ];
    });

    await lookUp(entries.flatMap((e, index) => (e.login ? [{ login: e.login, index }] : [])));
  };

  const lookUp = async (pending: { login: string; index: number }[]): Promise<void> => {
    checking = true;

    try {
      for (let from = 0; from < pending.length; from += IMPORT_BATCH) {
        const batch = pending.slice(from, from + IMPORT_BATCH);
        const result = await lookupChannels(batch.map((item) => item.login));

        rememberChannels(result.channels);

        for (const item of batch) {
          const channel = result.channels.find((c) => c.login === item.login) ?? null;
          const status = result.unchecked.includes(item.login)
            ? ImportEntryStatus.UNCHECKED
            : !channel
              ? ImportEntryStatus.MISSING
              : existing.includes(channel.login)
                ? ImportEntryStatus.ALREADY
                : ImportEntryStatus.FOUND;

          await settle(item.index, status, channel);
        }
      }
    } catch (e: any) {
      problem = describeError(e);
      entries = [];
    } finally {
      checking = false;
    }
  };

  const checkAgain = async (): Promise<void> => {
    const pending = entries.flatMap((e, index) =>
      e.login && e.status === ImportEntryStatus.UNCHECKED ? [{ login: e.login, index }] : [],
    );

    for (const item of pending) {
      entries[item.index] = { ...entries[item.index], status: ImportEntryStatus.PENDING };
    }

    await lookUp(pending);
  };

  export const clear = (): void => {
    entries = [];
    pasted = "";
  };

  $effect(() => {
    offered = entries.length > 0 && !checking ? fromChannels(found) : null;
  });

  $effect(() => {
    check =
      checking || (entries.length === 0 && pasted.trim())
        ? {
            label: t.import_check(),
            icon: IconName.SEARCH,
            busy: checking,
            busyLabel: t.points_checking(),
            disabled: !pasted.trim(),
            run: () => void run(),
          }
        : null;
  });
</script>

{#if open}
  <form
    class="flex flex-col gap-2"
    onsubmit={(event) => {
      event.preventDefault();
      void run();
    }}
  >
    {#if entries.length === 0}
      <Textarea
        rows={5}
        placeholder={t.import_paste_placeholder()}
        aria-label={t.import_tab_paste()}
        aria-invalid={problem !== ""}
        bind:value={pasted}
        oninput={() => (problem = "")}
      />
    {:else}
      <ul
        class="m-0 flex max-h-56 min-h-[7.5rem] list-none flex-wrap content-start gap-1.5 overflow-y-auto rounded-lg border border-input bg-input/30 p-2"
        aria-label={t.import_tab_paste()}
        aria-busy={checking}
      >
        {#each entries as e (e.entry)}
          <li
            class={cn(
              "inline-flex h-9 items-center gap-1.5 rounded-md border px-1.5 text-hint transition-colors duration-300",
              ENTRY_CLASSES[e.status],
            )}
            title={ENTRY_TITLES[e.status]?.()}
          >
            {#if e.status === ImportEntryStatus.PENDING}
              <Spinner size={12} />
            {:else if e.channel}
              {@const live = isLive(e.channel.login)}
              <span class={cn("inline-flex", live && "-translate-y-0.5")}
                ><ChannelAvatar
                  url={e.channel.avatar_url}
                  name={e.channel.display_name}
                  size={AvatarSize.SM}
                  {live}
                /></span
              >
            {:else if e.status === ImportEntryStatus.UNCHECKED}
              <Icon name={IconName.ALERT} size={12} />
            {:else}
              <Icon name={IconName.X} size={12} />
            {/if}
            <span class="font-medium">{e.channel?.display_name ?? e.entry}</span>
          </li>
        {/each}
      </ul>
    {/if}
    <div class="flex flex-wrap items-center justify-between gap-2">
      <span
        class="text-hint text-muted-foreground tabular-nums"
        aria-live="polite"
        aria-busy={checking}
      >
        {#if entries.length === 0}
          <RichText text={t.import_paste_hint({ max: strong(CHANNELS_LOOKUP_MAX) })} />
        {:else if checking}
          <RichText
            text={t.import_progress({ done: strong(done), total: strong(entries.length) })}
          />
        {:else}
          <RichText
            text={t.import_summary({ found: strong(found.length), missing: strong(failed) })}
          />
        {/if}
      </span>
      {#if entries.length > 0}
        <div class="flex flex-wrap gap-2">
          {#if unchecked > 0 && !checking}
            <TextButton
              label={t.import_check_again({ count: unchecked })}
              icon={IconName.REFRESH}
              variant={ButtonVariant.OUTLINE}
              size={ButtonSize.MD}
              type={ButtonType.BUTTON}
              disabled={false}
              onclick={() => void checkAgain()}
            />
          {/if}
          <TextButton
            label={t.import_edit()}
            icon={IconName.EDIT}
            variant={ButtonVariant.OUTLINE}
            size={ButtonSize.MD}
            type={ButtonType.BUTTON}
            disabled={checking}
            onclick={() => (entries = [])}
          />
        </div>
      {/if}
    </div>
    {#if problem}<p class="text-hint text-destructive">
        <RichText text={problem} />
      </p>{/if}
  </form>
{/if}
