<script lang="ts">
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import Tip from "./Tip.svelte";
  import { BUTTON_VARIANT_CLASSES, ButtonVariant, IconName } from "./ui.constants";

  type ChipRemoveButtonPropsType = {
    label: string;
    name: string;
    iconSize: number;
    onclick: () => void;
  };

  let { label, name, iconSize, onclick }: ChipRemoveButtonPropsType = $props();
</script>

<Tip text={label}>
  {#snippet children(props)}
    <button
      {...props}
      class={cn(
        "flex size-5 items-center justify-center rounded-full",
        BUTTON_VARIANT_CLASSES[ButtonVariant.DANGER_GHOST],
      )}
      type="button"
      aria-label={`${label} ${name}`}
      {onclick}
    >
      <Icon name={IconName.X} size={iconSize} />
    </button>
  {/snippet}
</Tip>
