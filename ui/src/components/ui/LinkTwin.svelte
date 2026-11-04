<script lang="ts">
  import type { Snippet } from "svelte";

  import { followLink } from "../../lib/links/links.utils";

  type LinkTwinPropsType = { url: string; children: Snippet };

  let { url, children }: LinkTwinPropsType = $props();
</script>

<!-- ! The name's link beside it is the one people tab to and screen readers read:
     this one only widens the target, and lights that name up on hover. -->
<a
  class="inline-flex shrink-0"
  href={url}
  rel="noreferrer"
  tabindex="-1"
  aria-hidden="true"
  data-link-twin
  onclick={(event) => followLink(event, url)}>{@render children()}</a
>
