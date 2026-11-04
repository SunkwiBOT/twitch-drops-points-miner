<script lang="ts">
  import RichText from "./RichText.svelte";

  type DetailLinePropsType = {
    parts: string[];
  };

  let { parts }: DetailLinePropsType = $props();

  const shown = $derived(parts.filter(Boolean));
</script>

<span class="text-xs text-muted-foreground"
  >{#each shown as part, index (index)}{#if index > 0}{" · "}{/if}<span class="inline-block"
      ><RichText text={part} /></span
    >{/each}</span
>
