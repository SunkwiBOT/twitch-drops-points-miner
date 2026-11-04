<script lang="ts">
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import RichText from "./RichText.svelte";

  type SearchNoResultPropsType = {
    shown: boolean;
    query: string;
  };

  let { shown, query }: SearchNoResultPropsType = $props();
</script>

<!-- ! Always in the page, empty until needed: a live region added with its text is
     not read out by every screen reader. -->
<p role="status" class={cn("text-hint text-muted-foreground", !shown && "sr-only")}>
  {#if shown}<RichText text={t.search_no_result({ query: strong(query.trim()) })} />{/if}
</p>
