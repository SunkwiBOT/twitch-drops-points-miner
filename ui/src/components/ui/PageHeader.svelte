<script lang="ts">
  import type { Snippet } from "svelte";

  import type { TextLinkType } from "../../lib/text.utils";
  import RichText from "./RichText.svelte";

  type PageHeaderPropsType = {
    intro?: string;
    links?: TextLinkType[];
    meta?: Snippet;
    actions?: Snippet;
    children?: Snippet;
  };

  let { intro, links, meta, actions, children }: PageHeaderPropsType = $props();
</script>

<header class="flex flex-col gap-3">
  <div class="flex flex-wrap items-start justify-between gap-x-6 gap-y-3">
    {#if intro}
      <div class="flex max-w-[80ch] min-w-0 flex-[1_1_20rem] flex-col gap-0.5">
        <p class="text-hint text-muted-foreground"><RichText text={intro} {links} /></p>
        {#if meta}<span class="text-xs text-muted-foreground">{@render meta()}</span>{/if}
      </div>
    {/if}
    {#if actions}
      <div class="ml-auto flex shrink-0 flex-wrap items-center justify-end gap-2">
        {@render actions()}
      </div>
    {/if}
  </div>
  {#if children}<div class="flex flex-wrap items-center gap-2.5">{@render children()}</div>{/if}
</header>
