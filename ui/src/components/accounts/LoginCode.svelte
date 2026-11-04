<script lang="ts">
  import { onMount } from "svelte";

  import type { LoginFlowViewType } from "../../lib/bindings/LoginFlowViewType";
  import { mmss } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { TWITCH_CONNECTIONS_URL } from "../../lib/partners.constants";
  import { refresh } from "../../lib/state/state.svelte";
  import { linkMarker } from "../../lib/text.utils";
  import { Progress } from "$lib/components/ui/progress";
  import { cn } from "$lib/utils";
  import CopyButton from "../ui/CopyButton.svelte";
  import ExternalLink from "../ui/ExternalLink.svelte";
  import RichText from "../ui/RichText.svelte";
  import StatusDot from "../ui/StatusDot.svelte";
  import {
    BadgeTone,
    ButtonSize,
    CopyLayout,
    LinkLook,
    LinkMark,
    LinkSize,
  } from "../ui/ui.constants";
  import { COUNTDOWN_TICK_MS, LOW_TIME_MS } from "./accounts.constants";

  type LoginCodePropsType = { flow: LoginFlowViewType };

  let { flow }: LoginCodePropsType = $props();

  let now = $state(Date.now());

  onMount(() => {
    const timer = setInterval(() => (now = Date.now()), COUNTDOWN_TICK_MS);

    return () => clearInterval(timer);
  });

  const total = $derived(new Date(flow.expires_at).getTime() - new Date(flow.issued_at).getTime());
  const left = $derived(Math.max(0, new Date(flow.expires_at).getTime() - now));
  const share = $derived(Math.max(0, Math.min(100, (left / total) * 100)));
  const shortLink = $derived(flow.verification_uri.replace(/^https:\/\/(www\.)?/, ""));

  $effect(() => {
    if (left === 0) {
      void refresh();
    }
  });
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-start gap-3">
    <span
      class="grid size-6 shrink-0 place-items-center rounded-full border border-primary/50 bg-primary/15 text-xs font-bold text-link"
      >1</span
    >
    <div class="flex min-w-0 flex-1 flex-col gap-1">
      <span class="font-semibold">{t.login_step_open()}</span>
      <span class="text-hint text-muted-foreground">{t.login_step_open_hint()}</span>
      <span class="mt-1.5 flex min-w-0 items-center gap-1.5">
        <ExternalLink
          url={flow.verification_uri}
          text={shortLink}
          mark={LinkMark.END}
          look={LinkLook.ALWAYS}
          size={LinkSize.LG}
        />
        <CopyButton
          text={() => flow.verification_uri}
          label={t.common_copy()}
          copiedLabel={t.common_copied()}
          name={t.common_copy_named({ name: t.login_link() })}
          layout={CopyLayout.ICON}
          size={ButtonSize.SM}
        />
      </span>
    </div>
  </div>
  <div class="flex items-start gap-3">
    <span
      class="grid size-6 shrink-0 place-items-center rounded-full border border-primary/50 bg-primary/15 text-xs font-bold text-link"
      >2</span
    >
    <div class="flex min-w-0 flex-1 flex-col gap-1">
      <span class="font-semibold">{t.login_step_check()}</span>
      <div
        class="mt-1.5 flex items-center justify-between gap-3 rounded-lg border border-input bg-background px-4 py-3.5"
      >
        <span class="font-mono text-3xl leading-none font-bold tracking-[0.22em]"
          >{flow.user_code}</span
        >
        <CopyButton
          text={() => flow.user_code}
          label={t.common_copy()}
          copiedLabel={t.common_copied()}
          name={t.common_copy_named({ name: t.login_code() })}
          layout={CopyLayout.ICON}
          size={ButtonSize.MD}
        />
      </div>
    </div>
  </div>
  <div class="flex flex-col gap-1.5">
    <div class="flex items-center justify-between text-hint text-muted-foreground">
      <span class="flex items-center gap-2"
        ><StatusDot tone={BadgeTone.INFO} />{t.login_waiting()}</span
      >
      <span class="font-mono">{mmss(left)}</span>
    </div>
    <Progress
      value={share}
      aria-label={t.login_waiting()}
      class={cn("h-2", left < LOW_TIME_MS && "[&>[data-slot=progress-indicator]]:bg-destructive")}
    />
  </div>
  <p class="text-hint text-muted-foreground">
    <RichText
      text={t.login_note({ settings: linkMarker(0) })}
      links={[{ label: t.login_note_settings(), url: TWITCH_CONNECTIONS_URL }]}
    />
  </p>
</div>
