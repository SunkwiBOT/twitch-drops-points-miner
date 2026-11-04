<script lang="ts">
  import { SetupStep } from "../../lib/accounts/accounts.constants";
  import { cn } from "$lib/utils";
  import Icon from "../ui/Icon.svelte";
  import { IconName } from "../ui/ui.constants";
  import { SETUP_STEP_LABELS } from "./accounts.utils";

  type SetupStepperPropsType = {
    steps: SetupStep[];
    current: number;
    disabled: boolean;
    ongo: (index: number) => void;
  };

  let { steps, current, disabled, ongo }: SetupStepperPropsType = $props();
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
        {SETUP_STEP_LABELS[item]()}
      </button>
    </li>
  {/each}
</ol>
