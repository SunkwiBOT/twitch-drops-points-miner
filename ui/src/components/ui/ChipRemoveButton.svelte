<script lang="ts">
  import Icon from "./Icon.svelte";
  import Tip from "./Tip.svelte";
  import { IconName } from "./ui.constants";

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
      class="flex size-5 items-center justify-center rounded-full text-destructive hover:bg-destructive/10 dark:hover:bg-destructive/20"
      type="button"
      aria-label={`${label} ${name}`}
      {onclick}
    >
      <Icon name={IconName.X} size={iconSize} />
    </button>
  {/snippet}
</Tip>
