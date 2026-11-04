<script lang="ts">
  // ! Edited after generation: the field is the app's SearchField instead of an
  // input group with the magnifier on the left, so a menu's search looks and behaves
  // like every other search field (magnifier on the right, a clear button, Escape
  // empties it before it closes the menu).
  import { Command as CommandPrimitive } from "bits-ui";
  import SearchField from "../../../../components/ui/SearchField.svelte";

  let {
    ref = $bindable(null),
    class: className,
    value = $bindable(""),
    ...restProps
  }: CommandPrimitive.InputProps = $props();
</script>

<div data-slot="command-input-wrapper" class="p-1 pb-0">
  <CommandPrimitive.Input
    {value}
    bind:ref
    data-slot="command-input"
    class={className}
    {...restProps}
  >
    {#snippet child({ props })}
      <SearchField {...props} bind:value search={null} />
    {/snippet}
  </CommandPrimitive.Input>
</div>
