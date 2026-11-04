<script lang="ts">
  import { Input } from "$lib/components/ui/input";
  import { cn } from "$lib/utils";
  import { openDataDirectory, quitApp } from "../../lib/api/client";
  import type { StartupProblemType } from "../../lib/bindings/StartupProblemType";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { toast } from "../../lib/state/state.svelte";
  import CopyButton from "../ui/CopyButton.svelte";
  import FormField from "../ui/FormField.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import {
    ButtonSize,
    ButtonType,
    ButtonVariant,
    CopyLayout,
    IconName,
    NOTICE_TONE_CLASSES,
    NoticeTone,
  } from "../ui/ui.constants";

  type StartupProblemPropsType = { problem: StartupProblemType };

  let { problem }: StartupProblemPropsType = $props();

  const act = async (action: () => Promise<void>): Promise<void> => {
    try {
      await action();
    } catch (e: any) {
      toast(ToastKind.ERROR, describeError(e));
    }
  };
</script>

<div class="grid h-full w-full place-items-center bg-background p-6">
  <div class="flex w-full max-w-150 flex-col gap-4 rounded-xl border bg-card p-7 shadow-lg">
    <img class="size-16 self-center [image-rendering:pixelated]" src="/app-icon.svg" alt="" />
    <h1 class="text-center text-xl font-semibold">{t.startup_title()}</h1>
    <p class="text-center text-hint text-muted-foreground">{t.startup_text()}</p>
    <p
      class={cn(
        "rounded-lg border px-3 py-2 font-mono text-xs break-words",
        NOTICE_TONE_CLASSES[NoticeTone.DANGER],
      )}
    >
      {problem.detail}
    </p>
    <FormField id="startup-data-dir" label={t.settings_data_folder()}>
      <div class="flex gap-1.5">
        <Input id="startup-data-dir" class="font-mono" value={problem.data_dir} readonly={true} />
        <CopyButton
          text={() => problem.data_dir}
          label={t.common_copy()}
          copiedLabel={t.common_copied()}
          name={t.common_copy_named({ name: t.settings_data_folder() })}
          layout={CopyLayout.ICON}
          size={ButtonSize.MD}
        />
      </div>
    </FormField>
    <div class="flex flex-wrap justify-end gap-2">
      <TextButton
        label={t.settings_open_data()}
        icon={IconName.FILE}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => act(openDataDirectory)}
      />
      <TextButton
        label={t.startup_quit()}
        icon={IconName.X}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => act(quitApp)}
      />
    </div>
  </div>
</div>
