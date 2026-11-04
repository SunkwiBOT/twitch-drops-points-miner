<script lang="ts">
  import type { StreamerOption } from "../../../lib/accounts/accounts.constants";
  import { optionLabels } from "../../../lib/accounts/options.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import Dialog from "../../ui/Dialog.svelte";
  import TextButton from "../../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../../ui/ui.constants";

  type DefaultsOfferDialogPropsType = {
    open: boolean;
    options: StreamerOption[];
    onyes: () => void;
    onno: () => void;
  };

  let { open, options, onyes, onno }: DefaultsOfferDialogPropsType = $props();

  const names = $derived(optionLabels(options));
</script>

<Dialog
  {open}
  title={t.points_defaults_offer_title({ count: names.length })}
  wide={true}
  closable={true}
  closeLabel={t.common_close()}
  onclose={onno}
>
  <div class="flex flex-col gap-3">
    <p class="text-hint text-muted-foreground">
      {t.points_defaults_offer_text({ count: names.length })}
    </p>
    <ul
      class="m-0 list-disc pl-5 text-sm font-medium text-foreground marker:text-muted-foreground sm:columns-2 sm:gap-x-10"
    >
      {#each names as name (name)}<li class="mb-1 break-inside-avoid">{name}</li>{/each}
    </ul>
  </div>
  {#snippet footer()}
    <TextButton
      label={t.common_no()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={onno}
    />
    <TextButton
      label={t.common_yes()}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={onyes}
    />
  {/snippet}
</Dialog>
