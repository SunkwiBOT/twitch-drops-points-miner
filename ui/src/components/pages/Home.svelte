<script lang="ts">
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";

  import { MODE_LABELS } from "../../lib/accounts/accounts.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import { EventKind } from "../../lib/bindings/EventKind";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { POINTS_ROUTINE } from "../../lib/events/events.utils";
  import { number } from "../../lib/format.utils";
  import { strong } from "../../lib/text.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { Page } from "../../lib/state/state.constants";
  import { app, navigate } from "../../lib/state/state.svelte";
  import AccountActivity from "../AccountActivity.svelte";
  import ActivityFeed from "../ActivityFeed.svelte";
  import { Button } from "$lib/components/ui/button";
  import TextButton from "../ui/TextButton.svelte";
  import { Card } from "$lib/components/ui/card";
  import AccountHeading from "../ui/AccountHeading.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Panel from "../ui/Panel.svelte";
  import PauseButton from "../ui/PauseButton.svelte";
  import RichText from "../ui/RichText.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
  } from "../ui/ui.constants";

  type HomePropsType = { snapshot: SnapshotType };

  const RECENT_MAX = 14;

  let { snapshot }: HomePropsType = $props();

  const events = $derived(
    [...app.liveEvents, ...snapshot.recent_events]
      .filter((e) => !POINTS_ROUTINE.has(e.kind) && e.kind !== EventKind.DROP_PROGRESS)
      .filter((e, i, all) => all.findIndex((o) => o.at === e.at && o.title === e.title) === i)
      .slice(0, RECENT_MAX),
  );

  const stats = (a: (typeof snapshot.accounts)[number]): string[] =>
    [
      t.accounts_stats_drops({ count: strong(number(a.stats.drops_claimed)) }),
      t.accounts_stats_points({ count: strong(number(a.stats.points_earned)) }),
      t.accounts_stats_minutes({ count: strong(number(a.stats.minutes_watched)) }),
      a.stats.bonuses_claimed > 0
        ? t.accounts_stats_bonuses({ count: strong(number(a.stats.bonuses_claimed)) })
        : "",
      a.stats.predictions_won + a.stats.predictions_lost > 0
        ? t.accounts_stats_bets({
            won: strong(number(a.stats.predictions_won)),
            lost: strong(number(a.stats.predictions_lost)),
          })
        : "",
    ].filter(Boolean);
</script>

<PageBody>
  {#if snapshot.accounts.length === 0}
    <EmptyState
      title={t.dashboard_no_accounts_title()}
      text={t.dashboard_no_accounts_text()}
      icon={IconName.USERS}
    >
      {#snippet action()}
        <TextButton
          label={t.accounts_add()}
          icon={IconName.PLUS}
          variant={ButtonVariant.PRIMARY}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => (app.loginDialog = true)}
        />
      {/snippet}
    </EmptyState>
  {:else}
    <PageHeader intro={t.home_rules()} />
    <div class="grid grid-cols-[repeat(auto-fit,minmax(min(460px,100%),1fr))] items-start gap-5">
      {#each snapshot.accounts as a (a.id)}
        <Card class="min-w-0 gap-0 py-0">
          <header class="border-b px-5 py-4">
            <AccountHeading account={a} subtitle={MODE_LABELS[a.settings.mode]()}>
              {#snippet actions()}
                {#if a.status === AccountStatus.LOGIN_REQUIRED}
                  <TextButton
                    label={t.accounts_login_again()}
                    variant={ButtonVariant.PRIMARY}
                    size={ButtonSize.SM}
                    type={ButtonType.BUTTON}
                    disabled={false}
                    onclick={() => (app.loginDialog = true)}
                  />
                {:else}
                  <PauseButton account={a} size={ButtonSize.SM} />
                {/if}
                <Button
                  variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
                  size={BUTTON_SIZES[ButtonSize.SM]}
                  type={ButtonType.BUTTON}
                  onclick={() => navigate(Page.ACCOUNT, a.id)}
                >
                  {t.home_open()}
                  <ChevronRightIcon />
                </Button>
              {/snippet}
            </AccountHeading>
          </header>
          <div class="flex flex-col"><AccountActivity account={a} /></div>
          <footer
            class="flex flex-wrap gap-x-4 gap-y-1 border-t bg-muted/60 px-5 py-3 text-hint text-muted-foreground tabular-nums"
          >
            {#each stats(a) as line (line)}<span><RichText text={line} /></span>{/each}
          </footer>
        </Card>
      {/each}
    </div>

    <Panel title={t.dashboard_recent()}>
      {#snippet actions()}
        <TextButton
          label={t.nav_history()}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => navigate(Page.HISTORY)}
        />
      {/snippet}
      <ActivityFeed {events} showAccount={true} wide={false} empty={t.history_empty()} />
    </Panel>
  {/if}
</PageBody>
