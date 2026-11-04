<script lang="ts">
  import { Switch } from "$lib/components/ui/switch";

  type TogglePropsType = {
    checked: boolean;
    disabled: boolean;
    label: string;
    onchange: (value: boolean) => void;
  };

  let { checked, disabled, label, onchange }: TogglePropsType = $props();

  let root = $state<HTMLButtonElement | null>(null);

  export const focus = (): void => {
    root?.focus();
  };
</script>

<Switch
  bind:ref={root}
  {checked}
  {disabled}
  aria-label={label}
  class="data-unchecked:bg-switch-unchecked dark:data-unchecked:bg-switch-unchecked"
  onCheckedChange={(value) => onchange(value)}
/>
