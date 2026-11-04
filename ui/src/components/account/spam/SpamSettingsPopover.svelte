<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import type { SpamPointsSettingsType } from "../../../lib/bindings/SpamPointsSettingsType";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { boundNumberInput } from "../../../lib/input.utils";
  import { SPAM_DELAY_STEP_MS, SPAM_MAX_DELAY_MS } from "../../../lib/spam/spam.constants";
  import { overlay } from "../../../lib/state/overlay.svelte";
  import FormField from "../../ui/FormField.svelte";
  import Icon from "../../ui/Icon.svelte";
  import Setting from "../../ui/Setting.svelte";
  import Tip from "../../ui/Tip.svelte";
  import {
    BUTTON_VARIANTS,
    ButtonVariant,
    ICON_BUTTON_SIZES,
    ButtonSize,
    IconName,
    MENU_EDGE_GAP,
  } from "../../ui/ui.constants";

  type SpamSettingsPopoverPropsType = {
    settings: SpamPointsSettingsType;
    onchange: (next: SpamPointsSettingsType) => void;
  };

  let { settings, onchange }: SpamSettingsPopoverPropsType = $props();

  let open = $state(false);

  const uid = $props.id();
  const delayId = `${uid}-delay`;
</script>

<Popover bind:open>
  <Tip text={t.spam_settings()}>
    {#snippet children(tip)}
      <PopoverTrigger {...tip}>
        {#snippet child({ props })}
          <Button
            {...props}
            variant={BUTTON_VARIANTS[ButtonVariant.DEFAULT]}
            size={ICON_BUTTON_SIZES[ButtonSize.MD]}
            aria-label={t.spam_settings()}
            aria-expanded={open}
          >
            <Icon name={IconName.SETTINGS} size={16} />
          </Button>
        {/snippet}
      </PopoverTrigger>
    {/snippet}
  </Tip>
  <PopoverContent
    class="w-[26rem] gap-0 p-4"
    align="end"
    collisionPadding={{
      top: MENU_EDGE_GAP,
      right: MENU_EDGE_GAP,
      left: MENU_EDGE_GAP,
      bottom: overlay.bottom + MENU_EDGE_GAP,
    }}
  >
    <h3 class="text-base font-semibold">{t.spam_settings()}</h3>
    <Setting
      title={t.spam_show_disabled()}
      checked={settings.show_disabled}
      disabled={false}
      onchange={(value) => onchange({ ...settings, show_disabled: value })}
    />
    <Setting
      title={t.spam_show_out_of_stock()}
      checked={settings.show_out_of_stock}
      disabled={false}
      onchange={(value) => onchange({ ...settings, show_out_of_stock: value })}
    />
    <div class="border-t pt-3">
      <FormField id={delayId} label={t.spam_delay()} hint={t.spam_delay_hint()}>
        <div class="relative w-32">
          <Input
            id={delayId}
            class="pr-10 tabular-nums"
            type="number"
            min="0"
            max={SPAM_MAX_DELAY_MS}
            step={SPAM_DELAY_STEP_MS}
            value={settings.delay_ms}
            onchange={(event) =>
              onchange({
                ...settings,
                delay_ms: boundNumberInput(event.currentTarget, 0, SPAM_MAX_DELAY_MS),
              })}
          />
          <span
            class="pointer-events-none absolute inset-y-0 right-3 grid place-items-center text-muted-foreground"
            aria-hidden="true">{t.spam_delay_unit()}</span
          >
        </div>
      </FormField>
    </div>
  </PopoverContent>
</Popover>
