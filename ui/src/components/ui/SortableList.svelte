<script lang="ts" generics="T">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import EmptyState from "./EmptyState.svelte";
  import Icon from "./Icon.svelte";
  import IconButton from "./IconButton.svelte";
  import { ButtonSize, ButtonVariant, IconName } from "./ui.constants";

  type SortableListPropsType = {
    items: T[];
    key: (item: T) => string;
    onreorder: (items: T[], moved: T) => void;
    row: Snippet<[T, number]>;
    background?: Snippet<[T]>;
    lead?: Snippet<[T]>;
    header?: Snippet;
    actions?: Snippet<[T, number]>;
    empty: string;
    upLabel: string;
    downLabel: string;
  };

  let {
    items,
    key,
    onreorder,
    row,
    background,
    lead,
    header,
    actions,
    empty,
    upLabel,
    downLabel,
  }: SortableListPropsType = $props();

  let dragging = $state<number | null>(null);
  let over = $state<{ index: number; after: boolean } | null>(null);

  const move = (from: number, to: number): void => {
    if (to < 0 || to >= items.length || from === to) {
      return;
    }

    const next = [...items];

    next.splice(to, 0, ...next.splice(from, 1));
    onreorder(next, items[from]);
  };

  const drop = (): void => {
    if (dragging === null || !over) {
      return;
    }

    const target = over.index + (over.after ? 1 : 0);

    move(dragging, dragging < target ? target - 1 : target);
    dragging = null;
    over = null;
  };
</script>

{#if items.length === 0}
  <EmptyState text={empty} />
{:else}
  <div class="flex flex-col overflow-hidden rounded-lg border">
    {#if header}
      <div
        class="flex min-h-11 flex-wrap items-center gap-x-3 gap-y-2 border-b bg-muted/50 px-3 py-1.5"
      >
        {@render header()}
      </div>
    {/if}
    <div class="flex flex-col" role="list">
      {#each items as item, index (key(item))}
        <div
          class={cn(
            "relative isolate overflow-hidden border-b bg-card last:border-b-0",
            dragging === index && "opacity-45",
            over?.index === index &&
              dragging !== index &&
              (over.after
                ? "shadow-[inset_0_-2px_0_var(--primary)]"
                : "shadow-[inset_0_2px_0_var(--primary)]"),
          )}
          role="listitem"
          ondragover={(event) => {
            if (dragging === null) {
              return;
            }

            event.preventDefault();
            const box = event.currentTarget.getBoundingClientRect();

            over = { index, after: event.clientY > box.top + box.height / 2 };
          }}
          ondrop={(event) => {
            event.preventDefault();
            drop();
          }}
        >
          {#if background}{@render background(item)}{/if}
          <!-- ! On a phone the row's buttons take a line of their own, under the content. -->
          <div class="flex min-h-13 items-center gap-3 px-3 py-2.5 max-sm:flex-wrap max-sm:gap-2">
            {#if lead}{@render lead(item)}{/if}
            <span
              class="inline-flex cursor-grab rounded-sm px-0.5 py-1 text-link transition-colors hover:bg-accent hover:text-link-hover active:cursor-grabbing max-sm:hidden"
              role="button"
              tabindex="-1"
              aria-hidden="true"
              draggable="true"
              ondragstart={(event) => {
                dragging = index;

                if (event.dataTransfer) {
                  event.dataTransfer.setData("text/plain", key(item));
                  event.dataTransfer.effectAllowed = "move";
                }
              }}
              ondragend={() => {
                dragging = null;
                over = null;
              }}
            >
              <Icon name={IconName.GRIP} size={18} class="[stroke-width:3.25]" />
            </span>
            <span
              class="-ml-2 w-4 shrink-0 text-right text-xs text-muted-foreground tabular-nums max-sm:ml-0"
              >{index + 1}</span
            >
            <div class="flex min-w-0 flex-1 items-center gap-3">{@render row(item, index)}</div>
            <div class="flex shrink-0 items-center gap-1.5 max-sm:w-full max-sm:justify-end">
              {#if actions}{@render actions(item, index)}{/if}
              <IconButton
                icon={IconName.ARROW_UP}
                label={upLabel}
                busy={false}
                disabled={index === 0}
                variant={ButtonVariant.GHOST}
                size={ButtonSize.SM}
                onclick={() => move(index, index - 1)}
              />
              <IconButton
                icon={IconName.ARROW_DOWN}
                label={downLabel}
                busy={false}
                disabled={index === items.length - 1}
                variant={ButtonVariant.GHOST}
                size={ButtonSize.SM}
                onclick={() => move(index, index + 1)}
              />
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}
