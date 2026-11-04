<script lang="ts" generics="T extends string | number">
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";

  import { Button } from "$lib/components/ui/button";
  import {
    Command,
    CommandEmpty,
    CommandInput,
    CommandItem,
    CommandList,
  } from "$lib/components/ui/command";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import Avatar from "./Avatar.svelte";
  import {
    AvatarSize,
    BUTTON_VARIANTS,
    ButtonVariant,
    MENU_EDGE_GAP,
    MENU_ITEM_CLASSES,
    MenuAlign,
    SELECT_SEARCH_FROM,
  } from "./ui.constants";
  import { matchOption } from "./ui.utils";

  type OptionType = {
    value: T;
    label: string;
    description?: string;
    meta?: string;
    image?: string;
  };

  type SelectPropsType = {
    value: T | null;
    options: OptionType[];
    label: string;
    id?: string;
    placeholder?: string;
    align: MenuAlign;
    onchange: (value: T) => void;
  };

  let { value, options, label, id, placeholder, align, onchange }: SelectPropsType = $props();

  let open = $state(false);

  const current = $derived(options.find((option) => option.value === value));
</script>

<Popover bind:open>
  <PopoverTrigger>
    {#snippet child({ props })}
      <Button
        {...props}
        {id}
        variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
        role="combobox"
        aria-expanded={open}
        aria-label={label}
        class="w-full min-w-0 justify-between font-normal"
      >
        <span class="flex min-w-0 items-center gap-2">
          {#if current?.image}
            <Avatar url={current.image} name={current.label} size={AvatarSize.XS} />
          {/if}
          {#if current}
            <span class="truncate">{current.label}</span>
          {:else}
            <span class="truncate text-muted-foreground">{placeholder ?? ""}</span>
          {/if}
        </span>
        <ChevronsUpDownIcon class="opacity-50" />
      </Button>
    {/snippet}
  </PopoverTrigger>
  <PopoverContent
    class={cn(
      "max-h-(--bits-popover-content-available-height) w-(--bits-popover-anchor-width) min-w-56 p-0",
      options.some((option) => option.description || option.meta) && "min-w-80",
    )}
    align={align === MenuAlign.RIGHT ? "end" : "start"}
    collisionPadding={{
      top: MENU_EDGE_GAP,
      right: MENU_EDGE_GAP,
      left: MENU_EDGE_GAP,
      bottom: overlay.bottom + MENU_EDGE_GAP,
    }}
  >
    <Command
      value={value === null ? "" : String(value)}
      loop={true}
      filter={matchOption}
      label={t.select_search()}
    >
      {#if options.length >= SELECT_SEARCH_FROM}
        <CommandInput placeholder={t.select_search()} />
      {/if}
      <CommandList class="max-h-80 p-1" aria-label={label}>
        <CommandEmpty>{t.select_empty()}</CommandEmpty>
        {#each options as option (option.value)}
          <CommandItem
            value={String(option.value)}
            keywords={[option.label]}
            data-checked={option.value === value ? "true" : undefined}
            class={MENU_ITEM_CLASSES}
            onSelect={() => {
              onchange(option.value);
              open = false;
            }}
          >
            {#if option.image}
              <Avatar url={option.image} name={option.label} size={AvatarSize.XS} />
            {/if}
            <span class="flex min-w-0 flex-1 flex-col gap-0.5">
              <span class="truncate">{option.label}</span>
              {#if option.description}
                <span class="text-xs leading-snug font-normal text-muted-foreground"
                  >{option.description}</span
                >
              {/if}
            </span>
            {#if option.meta}
              <span class="shrink-0 text-xs font-normal text-muted-foreground tabular-nums"
                >{option.meta}</span
              >
            {/if}
          </CommandItem>
        {/each}
      </CommandList>
    </Command>
  </PopoverContent>
</Popover>
