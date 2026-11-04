<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IMAGE_UPDATE_COMMANDS } from "../../lib/updates/updates.constants";
  import CopyButton from "../ui/CopyButton.svelte";
  import { ButtonSize, CopyLayout } from "../ui/ui.constants";

  const commands = IMAGE_UPDATE_COMMANDS.join("\n");
</script>

<div class="flex flex-col gap-1.5">
  <p class="text-hint text-muted-foreground">{t.updates_pull_image()}</p>
  <div class="flex items-start gap-1 rounded-lg border bg-muted/50 py-1 ps-3 pe-1">
    <pre class="min-w-0 flex-1 py-1 font-mono text-xs whitespace-pre-wrap">{commands}</pre>
    <CopyButton
      text={() => commands}
      label={t.common_copy()}
      copiedLabel={t.common_copied()}
      layout={CopyLayout.ICON}
      size={ButtonSize.SM}
    />
  </div>
</div>
