<script lang="ts">
  import TextButton from "./TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "./ui.constants";

  type LoadMoreSentinelPropsType = {
    label: string;
    busy: boolean;
    more: boolean;
    onmore: () => void;
  };

  let { label, busy, more, onmore }: LoadMoreSentinelPropsType = $props();

  const observe = (node: HTMLElement): (() => void) => {
    const observer = new IntersectionObserver((entries) => {
      if (entries.some((entry) => entry.isIntersecting)) {
        onmore();
      }
    });

    observer.observe(node);

    return () => observer.disconnect();
  };
</script>

<div class="flex justify-center pt-3" {@attach observe}>
  {#if !busy && more}
    <TextButton
      {label}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={onmore}
    />
  {/if}
</div>
