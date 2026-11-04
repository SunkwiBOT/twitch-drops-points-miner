<script lang="ts">
  import { untrack } from "svelte";

  import { fetchAutostart, setAutostart } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeError } from "../../lib/errors/errors.utils";
  import { ToastKind } from "../../lib/state/state.constants";
  import { saveGlobalSettings, toast } from "../../lib/state/state.svelte";
  import SettingsAbout from "../settings/SettingsAbout.svelte";
  import SettingsDesktop from "../settings/SettingsDesktop.svelte";
  import SettingsGeneral from "../settings/SettingsGeneral.svelte";
  import SettingsNotifications from "../settings/SettingsNotifications.svelte";
  import SettingsProxy from "../settings/SettingsProxy.svelte";
  import SettingsSecurity from "../settings/SettingsSecurity.svelte";
  import SettingsTime from "../settings/SettingsTime.svelte";
  import UnsavedBar from "../settings/UnsavedBar.svelte";
  import PageBody from "../ui/PageBody.svelte";

  type SettingsPropsType = { snapshot: SnapshotType };

  let { snapshot }: SettingsPropsType = $props();

  const current = (): GlobalSettingsType => $state.snapshot(snapshot.settings);
  const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b);

  let base = $state<GlobalSettingsType>(untrack(current));
  let draft = $state<GlobalSettingsType>(untrack(current));

  const edited = $derived(!same(draft, base));

  $effect(() => {
    const next = current();

    if (!untrack(() => edited)) {
      base = next;
      draft = structuredClone(next);
    }
  });

  let autostart = $state<boolean | null>(null);
  let autostartDraft = $state(false);

  $effect(() => {
    if (!isTauri()) {
      return;
    }

    fetchAutostart()
      .then((enabled) => {
        autostart = enabled;
        autostartDraft = enabled;
      })
      .catch((e: unknown) => console.warn("autostart state unavailable", e));
  });

  const autostartEdited = $derived(autostart !== null && autostartDraft !== autostart);

  const dirty = $derived(edited || autostartEdited);

  let saving = $state(false);

  const save = async (): Promise<void> => {
    saving = true;

    if (edited) {
      const before = new Map(Object.entries(base));
      const changes = Object.fromEntries(
        Object.entries($state.snapshot(draft)).filter(
          ([key, value]) => !same(value, before.get(key)),
        ),
      );
      const proxy = draft.default_proxy.trim();

      if (await saveGlobalSettings({ ...current(), ...changes, default_proxy: proxy })) {
        draft.default_proxy = proxy;
        base = $state.snapshot(draft);
      }
    }

    if (autostartEdited) {
      try {
        await setAutostart(autostartDraft);
        autostart = autostartDraft;
      } catch (e: any) {
        toast(ToastKind.ERROR, describeError(e));
      }
    }

    saving = false;
  };

  const reset = (): void => {
    base = current();
    draft = current();
    autostartDraft = autostart ?? false;
  };
</script>

<PageBody>
  <div class="grid grid-cols-[repeat(auto-fit,minmax(min(420px,100%),1fr))] items-start gap-5">
    <SettingsGeneral bind:draft />
    <SettingsSecurity {snapshot} bind:draft />
  </div>

  <SettingsTime bind:draft />

  <div class="grid grid-cols-[repeat(auto-fit,minmax(min(420px,100%),1fr))] items-start gap-5">
    <SettingsProxy bind:draft />
    {#if isTauri()}
      <SettingsDesktop bind:draft bind:autostart={autostartDraft} />
    {/if}
  </div>

  <SettingsNotifications bind:draft dirty={edited} />

  <SettingsAbout {snapshot} bind:draft />

  {#if dirty}
    <UnsavedBar {saving} oncancel={reset} onsave={save} />
  {/if}
</PageBody>
