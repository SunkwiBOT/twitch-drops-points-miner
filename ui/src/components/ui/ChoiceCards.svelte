<script lang="ts" generics="T extends string | number">
  import { RadioGroup, RadioGroupItem } from "$lib/components/ui/radio-group";
  import { cn } from "$lib/utils";

  type ChoiceType = { value: T; title: string; description: string };

  type ChoiceCardsPropsType = {
    value: T;
    choices: ChoiceType[];
    label: string;
    vertical: boolean;
    onchange: (value: T) => void;
  };

  let { value, choices, label, vertical, onchange }: ChoiceCardsPropsType = $props();
</script>

<RadioGroup
  value={String(value)}
  aria-label={label}
  class={cn("grid gap-2", vertical ? "grid-cols-1" : "grid-cols-1 sm:grid-cols-2")}
  onValueChange={(next) => {
    const choice = choices.find((item) => String(item.value) === next);

    if (choice) {
      onchange(choice.value);
    }
  }}
>
  {#each choices as choice (choice.value)}
    <label
      class="grid cursor-pointer grid-cols-[auto_minmax(0,1fr)] content-start items-center gap-x-2.5 gap-y-1.5 rounded-lg border bg-card p-3 transition-colors hover:bg-muted/40 has-[[data-state=checked]]:border-primary has-[[data-state=checked]]:bg-primary/10"
    >
      <RadioGroupItem value={String(choice.value)} />
      <span class="text-sm font-medium text-foreground">{choice.title}</span>
      <span class="col-span-2 text-hint leading-snug text-muted-foreground"
        >{choice.description}</span
      >
    </label>
  {/each}
</RadioGroup>
