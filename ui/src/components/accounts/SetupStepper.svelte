<script lang="ts">
  import { SetupStep } from "../../lib/accounts/accounts.constants";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { cn } from "$lib/utils";
  import Icon from "../ui/Icon.svelte";
  import { IconName } from "../ui/ui.constants";

  type SetupStepperPropsType = {
    steps: SetupStep[];
    current: number;
    disabled: boolean;
    ongo: (index: number) => void;
  };

  let { steps, current, disabled, ongo }: SetupStepperPropsType = $props();

  const STEP_LABELS: Record<SetupStep, () => string> = {
    [SetupStep.START]: () => t.login_setup_step_start(),
    [SetupStep.MODE]: () => t.login_setup_step_mode(),
    [SetupStep.DROPS]: () => t.login_setup_step_drops(),
    [SetupStep.STREAMERS]: () => t.login_setup_step_streamers(),
    [SetupStep.SUMMARY]: () => t.login_setup_step_summary(),
  };
</script>

<ol class="m-0 flex list-none flex-wrap items-center gap-x-2 gap-y-1.5 p-0">
  {#each steps as item, index (item)}
    <li class="flex items-center gap-2">
      {#if index > 0}<span class="h-px w-5 bg-border" aria-hidden="true"></span>{/if}
      <button
        class={cn(
          "flex items-center gap-1.5 rounded-full px-1 py-0.5 text-hint text-muted-foreground disabled:cursor-default",
          index === current && "font-semibold text-foreground",
        )}
        type="button"
        disabled={index > current || disabled}
        aria-current={index === current ? "step" : undefined}
        onclick={() => ongo(index)}
      >
        <span
          class={cn(
            "grid size-5.5 place-items-center rounded-full border text-tiny font-bold",
            index === current && "border-primary bg-primary text-primary-foreground",
            index < current && "border-success-emphasis/40 bg-success text-success-foreground",
          )}
        >
          {#if index < current}<Icon name={IconName.CHECK} size={11} />{:else}{index + 1}{/if}
        </span>
        {STEP_LABELS[item]()}
      </button>
    </li>
  {/each}
</ol>
