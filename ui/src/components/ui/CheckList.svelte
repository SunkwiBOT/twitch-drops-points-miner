<script lang="ts" module>
  export type CheckItemType = {
    value: string;
    label: string;
    image: string | undefined;
    hint: string;
    channel: { live: boolean } | null;
  };
</script>

<script lang="ts">
  import TextButton from "./TextButton.svelte";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import { cn } from "$lib/utils";
  import Avatar from "./Avatar.svelte";
  import ChannelAvatar from "./ChannelAvatar.svelte";
  import EmptyState from "./EmptyState.svelte";
  import { AvatarSize, ButtonSize, ButtonType, ButtonVariant } from "./ui.constants";

  type CheckListPropsType = {
    items: CheckItemType[];
    selected: string[];
    label: string;
    allLabel: string;
    noneLabel: string;
    countLabel: string;
    empty: string;
    onchange: (selected: string[]) => void;
  };

  let {
    items,
    selected,
    label,
    allLabel,
    noneLabel,
    countLabel,
    empty,
    onchange,
  }: CheckListPropsType = $props();

  const chosen = $derived(new Set(selected));

  const toggle = (value: string, checked: boolean): void =>
    onchange(checked ? [...selected, value] : selected.filter((item) => item !== value));
</script>

{#if items.length === 0}
  <EmptyState text={empty} />
{:else}
  <div class="flex flex-col overflow-hidden rounded-lg border">
    <div class="flex flex-wrap items-center justify-between gap-2 border-b bg-muted/50 px-3 py-2">
      <span class="text-hint text-muted-foreground">{countLabel}</span>
      <div class="flex gap-1.5">
        <TextButton
          label={allLabel}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={items.length === 0 || items.every((item) => chosen.has(item.value))}
          onclick={() => onchange(items.map((item) => item.value))}
        />
        <TextButton
          label={noneLabel}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={selected.length === 0}
          onclick={() => onchange([])}
        />
      </div>
    </div>
    <ul class="m-0 flex max-h-80 list-none flex-col overflow-y-auto p-0" aria-label={label}>
      {#each items as item (item.value)}
        <li class="border-b last:border-b-0">
          <label
            class={cn(
              "flex cursor-pointer items-center gap-3 px-3 py-2.5 hover:bg-accent",
              chosen.has(item.value) && "bg-primary/5",
            )}
          >
            <Checkbox
              checked={chosen.has(item.value)}
              onCheckedChange={(checked) => toggle(item.value, checked === true)}
            />
            {#if item.channel}
              <ChannelAvatar
                url={item.image}
                name={item.label}
                size={AvatarSize.SM}
                live={item.channel.live}
              />
            {:else}
              <Avatar url={item.image} name={item.label} size={AvatarSize.SM} />
            {/if}
            <span class="flex-1 truncate font-semibold">{item.label}</span>
            {#if item.hint}<span class="truncate text-xs text-muted-foreground">{item.hint}</span
              >{/if}
          </label>
        </li>
      {/each}
    </ul>
  </div>
{/if}
