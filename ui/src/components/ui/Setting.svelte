<script lang="ts">
  import { tick, type Snippet } from "svelte";

  import { t } from "../../lib/i18n/i18n.svelte";
  import Badge from "./Badge.svelte";
  import Toggle from "./Toggle.svelte";
  import { BadgeTone } from "./ui.constants";
  import { YES_NO_CHOICES } from "./ui.utils";

  type SettingPropsType = {
    title: string;
    description?: string;
    checked: boolean;
    disabled: boolean;
    onchange: (value: boolean) => void;
    lead?: Snippet;
    mark?: string;
  };

  let { title, description, checked, disabled, onchange, lead, mark }: SettingPropsType = $props();

  let toggle = $state<ReturnType<typeof Toggle> | null>(null);

  // ! The pressed button leaves with its group: the focus moves to the switch that
  // takes its place, not back to the top of the page or the dialog.
  const choose = async (value: boolean): Promise<void> => {
    onchange(value);
    await tick();
    toggle?.focus();
  };
</script>

<div
  class="flex items-center justify-between gap-6 py-3 [&+&]:border-t"
  data-setting
  class:opacity-60={disabled}
>
  <div class="flex min-w-0 flex-col gap-0.5">
    <span class="flex items-center gap-2 text-sm font-medium text-foreground"
      >{#if lead}{@render lead()}{/if}{title}{#if mark}<Badge
          text={mark}
          tone={BadgeTone.NEUTRAL}
          dot={false}
        />{/if}</span
    >
    {#if description}<span class="text-hint text-muted-foreground">{description}</span>{/if}
  </div>
  {#if mark}
    <div
      role="group"
      aria-label={t.setting_with_mark({ option: title, mark })}
      class="flex shrink-0 overflow-hidden rounded-md border border-input"
    >
      {#each YES_NO_CHOICES as choice (choice.value)}
        <button
          type="button"
          class="h-8 px-3 text-sm font-medium text-muted-foreground transition-colors outline-none hover:bg-muted hover:text-foreground focus-visible:z-10 focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none [&+&]:border-l [&+&]:border-input"
          {disabled}
          onclick={() => void choose(choice.value)}>{choice.label()}</button
        >
      {/each}
    </div>
  {:else}
    <Toggle bind:this={toggle} {checked} {onchange} {disabled} label={title} />
  {/if}
</div>
