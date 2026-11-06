<script lang="ts" generics="T extends string">
  import { Tabs, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
  import { number } from "../../lib/format.utils";
  import Badge from "./Badge.svelte";
  import Icon from "./Icon.svelte";
  import { BadgeTone, type IconName } from "./ui.constants";
  import { segmentId } from "./ui.utils";

  type OptionType = { value: T; label: string; icon?: IconName; count?: number };

  type SegmentedPropsType = {
    value: T;
    options: OptionType[];
    label: string;
    panel?: string;
    onchange: (value: T) => void;
  };

  let { value, options, label, panel, onchange }: SegmentedPropsType = $props();
</script>

<Tabs
  class="max-w-full min-w-0"
  {value}
  onValueChange={(next) => {
    const option = options.find((item) => item.value === next);

    if (option) {
      onchange(option.value);
    }
  }}
>
  <!-- ! On a phone the tabs scroll sideways inside their bar: wider, they would make the
       whole page scroll sideways. -->
  <TabsList
    aria-label={label}
    class="max-w-full justify-start overflow-x-auto [scrollbar-width:none]"
  >
    {#each options as option (option.value)}
      <TabsTrigger
        value={option.value}
        id={panel ? segmentId(panel, option.value) : undefined}
        aria-controls={panel}
        class="px-2.5"
      >
        {#if option.icon}<Icon name={option.icon} size={14} />{/if}
        {option.label}
        {#if option.count}<Badge
            text={number(option.count)}
            tone={BadgeTone.NEUTRAL}
            dot={false}
          />{/if}
      </TabsTrigger>
    {/each}
  </TabsList>
</Tabs>
