<script lang="ts">
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DISCORD_NAME, DISCORD_URL, GITHUB_NAME, ISSUES_URL } from "../../lib/partners.constants";
  import { openLink } from "../../lib/state/state.svelte";
  import Icon from "../ui/Icon.svelte";
  import { IconName } from "../ui/ui.constants";

  type FeedbackChoicesPropsType = { onchosen?: () => void };

  type ChoiceType = { url: string; icon: IconName; name: string; text: string; tile: string };

  let { onchosen }: FeedbackChoicesPropsType = $props();

  const choices = $derived<ChoiceType[]>([
    {
      url: DISCORD_URL,
      icon: IconName.DISCORD,
      name: DISCORD_NAME,
      text: t.feedback_discord_text(),
      tile: "bg-brand-discord text-brand-discord-foreground",
    },
    {
      url: ISSUES_URL,
      icon: IconName.GITHUB,
      name: GITHUB_NAME,
      text: t.feedback_github_text(),
      tile: "bg-foreground text-background",
    },
  ]);

  const choose = (choice: ChoiceType): void => {
    onchosen?.();
    void openLink(choice.url);
  };
</script>

<div class="grid gap-2">
  {#each choices as choice (choice.url)}
    <a
      class="group grid grid-cols-[auto_minmax(0,1fr)] items-center gap-3 rounded-lg border bg-card p-3 no-underline transition-colors hover:border-primary/50 hover:bg-muted/40"
      href={choice.url}
      rel="noreferrer"
      onclick={(event) => {
        event.preventDefault();
        choose(choice);
      }}
    >
      <span class={cn("flex size-10 shrink-0 items-center justify-center rounded-lg", choice.tile)}
        ><Icon name={choice.icon} size={22} /></span
      >
      <span class="flex min-w-0 flex-col gap-0.5">
        <span
          class="flex items-center gap-1.5 text-sm font-semibold text-foreground group-hover:text-link"
          >{choice.name}<span class="opacity-70"><Icon name={IconName.EXTERNAL} size={13} /></span
          ></span
        >
        <span class="text-hint leading-snug text-muted-foreground">{choice.text}</span>
      </span>
    </a>
  {/each}
</div>
