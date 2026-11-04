<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  import { Input } from "$lib/components/ui/input";
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import Tip from "./Tip.svelte";
  import {
    CLEAR_SEARCH_KEY,
    FIELD_BUTTON_CLASSES,
    FIELD_CLEAR_BUTTON_CLASSES,
    IconName,
    SEARCH_FIELD_PADDING,
  } from "./ui.constants";

  type SearchActionType = { label: string; busy: boolean; run: () => void };

  type SearchFieldPropsType = Omit<HTMLInputAttributes, "type" | "value" | "files"> & {
    value: string;
    search: SearchActionType | null;
  };

  let {
    value = $bindable(),
    search,
    class: className,
    onkeydown,
    ...rest
  }: SearchFieldPropsType = $props();

  let input = $state<HTMLInputElement | null>(null);

  const clear = (): void => {
    if (!input) {
      return;
    }

    input.value = "";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    input.focus();
  };
</script>

<div class="relative w-full min-w-0">
  <Input
    bind:ref={input}
    type="search"
    class={cn(SEARCH_FIELD_PADDING, "[&::-webkit-search-cancel-button]:hidden", className)}
    spellcheck={false}
    bind:value
    {...rest}
    onkeydown={(event) => {
      // ! A filled field empties first; in an empty one Escape keeps its meaning
      // (closing the menu or the dialog).
      if (event.key === CLEAR_SEARCH_KEY && value !== "" && !event.isComposing) {
        event.preventDefault();
        event.stopPropagation();
        clear();
      }

      onkeydown?.(event);
    }}
  />

  {#if value !== ""}
    <Tip text={t.common_clear_search()}>
      {#snippet children(props)}
        <button
          {...props}
          type="button"
          class={FIELD_CLEAR_BUTTON_CLASSES}
          aria-label={t.common_clear_search()}
          onclick={clear}
        >
          <Icon name={IconName.X} size={14} />
        </button>
      {/snippet}
    </Tip>
  {/if}

  {#if search}
    <Tip text={search.label}>
      {#snippet children(props)}
        <button
          {...props}
          type="button"
          class={FIELD_BUTTON_CLASSES}
          aria-label={search.label}
          aria-busy={search.busy}
          onclick={search.run}
        >
          {#if search.busy}
            <Spinner size={15} />
          {:else}
            <Icon name={IconName.SEARCH} size={15} />
          {/if}
        </button>
      {/snippet}
    </Tip>
  {:else}
    <span
      class="pointer-events-none absolute inset-y-0 right-0 grid w-8 place-items-center text-muted-foreground"
      aria-hidden="true"
    >
      <Icon name={IconName.SEARCH} size={15} />
    </span>
  {/if}
</div>
