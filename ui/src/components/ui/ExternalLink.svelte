<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import { openLink } from "../../lib/state/state.svelte";
  import Icon from "./Icon.svelte";
  import { IconName, LINK_SIZES, LinkLook, LinkMark, LinkSize } from "./ui.constants";

  type ExternalLinkPropsType = {
    url: string;
    text: string;
    mark: LinkMark;
    look: LinkLook;
    size: LinkSize;
    lead?: Snippet;
  };

  let { url, text, mark, look, size, lead }: ExternalLinkPropsType = $props();

  const marked = $derived(mark !== LinkMark.NONE);
  const always = $derived(look === LinkLook.ALWAYS);
</script>

{#snippet arrow()}
  <span
    class={cn(
      "shrink-0 transition-opacity",
      always
        ? "opacity-100"
        : "opacity-0 group-hover/link:opacity-100 group-focus-visible/link:opacity-100",
    )}><Icon name={IconName.EXTERNAL} size={LINK_SIZES[size].icon} /></span
  >
{/snippet}

<a
  class={cn(
    "group/link min-w-0 font-semibold no-underline transition-colors hover:text-link",
    always ? "text-link" : "text-foreground",
    LINK_SIZES[size].text,
    lead || marked ? "inline-flex max-w-full items-center gap-1.5 align-bottom" : "inline",
  )}
  href={url}
  rel="noreferrer"
  onclick={(event) => {
    event.preventDefault();
    void openLink(url);
  }}
  >{#if lead}{@render lead()}{/if}{#if mark === LinkMark.START}{@render arrow()}{/if}<span
    class={cn(
      "underline-offset-3 group-hover/link:underline",
      always && "underline",
      (lead || marked) && LINK_SIZES[size].fit,
    )}>{text}</span
  >{#if mark === LinkMark.END}{@render arrow()}{/if}</a
>
