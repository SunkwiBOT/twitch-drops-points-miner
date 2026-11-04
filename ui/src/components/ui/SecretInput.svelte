<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  import { Input } from "$lib/components/ui/input";
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Icon from "./Icon.svelte";
  import Tip from "./Tip.svelte";
  import { FIELD_BUTTON_CLASSES, IconName } from "./ui.constants";

  type SecretInputPropsType = Omit<HTMLInputAttributes, "type" | "value" | "files"> & {
    id: string;
    value: string;
  };

  let { id, value = $bindable(), class: className, ...rest }: SecretInputPropsType = $props();

  let shown = $state(false);
</script>

<div class="relative w-full min-w-0">
  <Input
    {id}
    type={shown ? "text" : "password"}
    class={cn("pr-9", className)}
    spellcheck={false}
    bind:value
    {...rest}
  />
  <Tip text={shown ? t.secret_hide() : t.secret_show()}>
    {#snippet children(props)}
      <button
        {...props}
        type="button"
        class={FIELD_BUTTON_CLASSES}
        aria-controls={id}
        aria-pressed={shown}
        aria-label={shown ? t.secret_hide() : t.secret_show()}
        onclick={() => (shown = !shown)}
      >
        <Icon name={shown ? IconName.EYE_OFF : IconName.EYE} size={15} />
      </button>
    {/snippet}
  </Tip>
</div>
