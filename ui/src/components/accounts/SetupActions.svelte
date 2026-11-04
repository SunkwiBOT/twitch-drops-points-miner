<script lang="ts">
  import { SetupEnd, SetupStep } from "../../lib/accounts/accounts.constants";
  import type { ImportActionType } from "../../lib/accounts/import.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { Button } from "$lib/components/ui/button";
  import ActionButton from "../ui/ActionButton.svelte";
  import Icon from "../ui/Icon.svelte";
  import RichText from "../ui/RichText.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
  } from "../ui/ui.constants";

  type SetupActionsPropsType = {
    step: SetupStep;
    stepIndex: number;
    total: number;
    importAction: ImportActionType | null;
    ending: SetupEnd | null;
    ongo: (index: number) => void;
    onfinish: (end: SetupEnd) => unknown;
  };

  let { step, stepIndex, total, importAction, ending, ongo, onfinish }: SetupActionsPropsType =
    $props();

  const finishing = $derived(ending !== null);
</script>

<span class="mr-auto self-center text-xs text-muted-foreground"
  ><RichText
    text={t.login_setup_progress({
      current: strong(stepIndex + 1),
      total: strong(total),
    })}
  /></span
>
<ActionButton
  icon={IconName.PAUSE}
  label={t.login_setup_later()}
  busyLabel={t.common_saving()}
  busy={ending === SetupEnd.LATER}
  disabled={finishing}
  variant={ButtonVariant.OUTLINE}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  onclick={() => onfinish(SetupEnd.LATER)}
/>
{#if stepIndex > 0}
  <TextButton
    label={t.common_back()}
    variant={ButtonVariant.OUTLINE}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    disabled={finishing}
    onclick={() => ongo(stepIndex - 1)}
  />
{/if}
{#if step === SetupStep.STREAMERS && importAction}
  <ActionButton
    icon={importAction.icon}
    label={importAction.label}
    busyLabel={importAction.busyLabel}
    busy={importAction.busy}
    disabled={importAction.disabled || finishing}
    variant={ButtonVariant.PRIMARY}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={importAction.run}
  />
{:else if step === SetupStep.SUMMARY}
  <ActionButton
    icon={IconName.PLAY}
    label={t.login_setup_finish()}
    busyLabel={t.common_saving()}
    busy={ending === SetupEnd.START}
    disabled={finishing}
    variant={ButtonVariant.PRIMARY}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={() => onfinish(SetupEnd.START)}
  />
{:else}
  <Button
    variant={BUTTON_VARIANTS[ButtonVariant.PRIMARY]}
    size={BUTTON_SIZES[ButtonSize.MD]}
    type={ButtonType.BUTTON}
    disabled={finishing}
    onclick={() => ongo(stepIndex + 1)}
  >
    {t.common_next()}
    <Icon name={IconName.UP} size={12} class="rotate-90" />
  </Button>
{/if}
