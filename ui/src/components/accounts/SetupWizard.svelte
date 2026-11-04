<script lang="ts">
  import type { Snippet } from "svelte";

  import { SetupEnd, SetupSource, SetupStep } from "../../lib/accounts/accounts.constants";
  import { newStreamer, type ImportActionType } from "../../lib/accounts/import.svelte";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import { DropsScope } from "../../lib/bindings/DropsScope";
  import { Mode } from "../../lib/bindings/Mode";
  import type { StreamerSettingsType } from "../../lib/bindings/StreamerSettingsType";
  import {
    ensureChannels,
    knownChannels,
    rememberChannels,
  } from "../../lib/channels/channels.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { account as accountOf, app, saveAccountSettings } from "../../lib/state/state.svelte";
  import { personMarker } from "../../lib/text.utils";
  import ChoiceCards from "../ui/ChoiceCards.svelte";
  import FormField from "../ui/FormField.svelte";
  import RichText from "../ui/RichText.svelte";
  import Select from "../ui/Select.svelte";
  import Setting from "../ui/Setting.svelte";
  import { MenuAlign } from "../ui/ui.constants";
  import SetupActions from "./SetupActions.svelte";
  import SetupStepper from "./SetupStepper.svelte";
  import SetupStreamersStep from "./SetupStreamersStep.svelte";
  import SetupSummary from "./SetupSummary.svelte";
  import StreamerChip from "./StreamerChip.svelte";

  type SetupWizardPropsType = {
    account: AccountSnapshotType;
    onclose: () => void;
    footer: Snippet | undefined;
    finishing: boolean;
  };

  let {
    account,
    onclose,
    footer = $bindable(),
    finishing = $bindable(),
  }: SetupWizardPropsType = $props();

  const others = $derived((app.snapshot?.accounts ?? []).filter((a) => a.id !== account.id));

  let picked = $state<SetupStep | null>(null);
  let source = $state<SetupSource | string>(SetupSource.DEFAULTS);

  const sourceAccount = $derived(
    source === SetupSource.DEFAULTS || source === account.id ? null : accountOf(source),
  );

  const base = $derived(sourceAccount?.settings ?? account.settings);

  let pickedMode = $state<Mode | null>(null);
  let pickedScope = $state<DropsScope | null>(null);
  let pickedAutoClaim = $state<boolean | null>(null);
  let pickedStreamers = $state<StreamerSettingsType[] | null>(null);
  let importAction = $state<ImportActionType | null>(null);

  const mode = $derived(pickedMode ?? base.mode);
  const scope = $derived(pickedScope ?? base.drops.scope);
  const autoClaim = $derived(pickedAutoClaim ?? base.drops.auto_claim);
  const streamers = $derived(pickedStreamers ?? base.points.streamers);

  const sameLogins = (a: StreamerSettingsType[], b: StreamerSettingsType[]): boolean =>
    a.length === b.length && a.every((item, index) => item.login === b[index].login);

  const customized = $derived(
    mode !== base.mode ||
      scope !== base.drops.scope ||
      autoClaim !== base.drops.auto_claim ||
      !sameLogins(streamers, base.points.streamers),
  );

  const sourceOptions = $derived([
    { value: SetupSource.DEFAULTS, label: t.login_setup_defaults() },
    ...others.map((a) => ({
      value: a.id,
      label: t.login_setup_copy_from({ name: a.display_name }),
      image: a.avatar_url,
    })),
    ...(customized ? [{ value: SetupSource.CUSTOM, label: t.login_setup_custom() }] : []),
  ]);

  const pickSource = (value: SetupSource | string): void => {
    if (value === SetupSource.CUSTOM) {
      return;
    }

    source = value;
    pickedMode = null;
    pickedScope = null;
    pickedAutoClaim = null;
    pickedStreamers = null;
  };

  const scopeChoices = $derived([
    { value: DropsScope.ALL, title: t.drops_scope_all(), description: t.drops_scope_all_hint() },
    {
      value: DropsScope.LINKED_ONLY,
      title: t.drops_scope_linked_only(),
      description: t.drops_scope_linked_hint(),
    },
    ...(base.drops.priority_games.length > 0
      ? [
          {
            value: DropsScope.PRIORITY_ONLY,
            title: t.drops_scope_priority_only(),
            description: t.drops_scope_priority_hint(),
          },
        ]
      : []),
  ]);

  const channelFor = (login: string): ChannelType | undefined =>
    sourceAccount?.streamers.find((view) => view.login === login)?.channel ?? knownChannels[login];

  $effect(() => {
    ensureChannels(streamers.map((s) => s.login).filter((login) => !channelFor(login)));
  });

  const addStreamers = (logins: string[], channels: ChannelType[]): void => {
    rememberChannels(channels);
    pickedStreamers = [
      ...$state.snapshot(streamers),
      ...logins
        .filter((login) => !streamers.some((s) => s.login === login))
        .map((login) => newStreamer(base.points.streamer_defaults, login)),
    ];
  };

  const removeStreamer = (login: string): void => {
    pickedStreamers = $state.snapshot(streamers).filter((item) => item.login !== login);
  };

  const steps = $derived([
    ...(others.length > 0 ? [SetupStep.START] : []),
    SetupStep.MODE,
    ...(mode !== Mode.POINTS ? [SetupStep.DROPS] : []),
    ...(mode !== Mode.DROPS ? [SetupStep.STREAMERS] : []),
    SetupStep.SUMMARY,
  ]);

  const step = $derived(picked !== null && steps.includes(picked) ? picked : steps[0]);
  const stepIndex = $derived(steps.indexOf(step));

  const goTo = (index: number): void => {
    picked = steps[Math.max(0, Math.min(steps.length - 1, index))];
  };

  let ending = $state<SetupEnd | null>(null);

  const finish = async (end: SetupEnd): Promise<void> => {
    const start = $state.snapshot(base);
    const settings = {
      ...start,
      proxy: account.settings.proxy,
      enabled: end === SetupEnd.START,
      mode,
      drops: { ...start.drops, scope, auto_claim: autoClaim },
      points: { ...start.points, streamers: $state.snapshot(streamers) },
    };

    ending = end;
    finishing = true;
    const saved = await saveAccountSettings(account.id, settings);
    finishing = false;
    ending = null;

    if (saved) {
      onclose();
    }
  };

  $effect(() => {
    footer = buttons;

    return () => {
      footer = undefined;
    };
  });
</script>

{#snippet stepTitle(title: string, hint: string)}
  <div class="flex flex-col gap-1">
    <h3 class="text-base font-semibold">{title}</h3>
    <p class="text-hint text-muted-foreground"><RichText text={hint} /></p>
  </div>
{/snippet}

{#snippet buttons()}
  <SetupActions
    {step}
    {stepIndex}
    total={steps.length}
    {importAction}
    {ending}
    ongo={goTo}
    onfinish={finish}
  />
{/snippet}

<div class="flex flex-col gap-5">
  <p class="text-hint text-muted-foreground">
    <RichText
      text={t.login_setup_for({ name: personMarker(0) })}
      people={[{ name: account.display_name, avatar: account.avatar_url, channel: null }]}
    />
  </p>
  <SetupStepper {steps} current={stepIndex} disabled={finishing} ongo={goTo} />

  {#if step === SetupStep.START}
    {@render stepTitle(t.login_setup_step_start(), t.login_setup_start_hint())}
    <FormField id="login-setup-from" label={t.login_setup_from()} hint={t.login_setup_copy_hint()}>
      <Select
        id="login-setup-from"
        value={customized ? SetupSource.CUSTOM : sourceAccount ? source : SetupSource.DEFAULTS}
        options={sourceOptions}
        label={t.login_setup_from()}
        align={MenuAlign.LEFT}
        onchange={pickSource}
      />
    </FormField>
  {:else if step === SetupStep.MODE}
    {@render stepTitle(t.account_mode_title(), t.account_mode_desc())}
    <ChoiceCards
      vertical={true}
      value={mode}
      label={t.account_mode_title()}
      onchange={(value) => (pickedMode = value)}
      choices={[
        { value: Mode.AUTO, title: t.mode_auto_title(), description: t.mode_auto_hint() },
        { value: Mode.DROPS, title: t.mode_drops_title(), description: t.mode_drops_hint() },
        { value: Mode.POINTS, title: t.mode_points_title(), description: t.mode_points_hint() },
      ]}
    />
  {:else if step === SetupStep.DROPS}
    {@render stepTitle(t.drops_scope(), t.login_setup_drops_hint())}
    <ChoiceCards
      vertical={true}
      value={scope}
      label={t.drops_scope()}
      onchange={(value) => (pickedScope = value)}
      choices={scopeChoices}
    />
    <Setting
      title={t.account_drops_auto_claim()}
      description={t.drops_auto_claim_hint()}
      checked={autoClaim}
      disabled={false}
      onchange={(value) => (pickedAutoClaim = value)}
    />
  {:else if step === SetupStep.STREAMERS}
    {@render stepTitle(t.login_setup_streamers_title(), t.login_setup_streamers_hint())}
    <SetupStreamersStep
      {account}
      {streamers}
      {channelFor}
      bind:importAction
      onadd={addStreamers}
      onremove={removeStreamer}
    />
  {:else}
    {@render stepTitle(t.login_setup_step_summary(), t.login_setup_summary_hint())}
    <SetupSummary source={sourceAccount} {mode} {scope} {autoClaim} {streamers} />
    {#if mode !== Mode.DROPS && streamers.length > 0}
      <div class="flex flex-wrap gap-1.5">
        {#each streamers as streamer (streamer.login)}<StreamerChip
            login={streamer.login}
            channel={channelFor(streamer.login)}
          />{/each}
      </div>
    {/if}
    <p class="text-hint text-muted-foreground">{t.login_setup_later_hint()}</p>
  {/if}
</div>
