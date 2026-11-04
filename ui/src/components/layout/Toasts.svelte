<script lang="ts">
  import { Toaster } from "$lib/components/ui/sonner";
  import { t } from "../../lib/i18n/i18n.svelte";
</script>

<Toaster position="bottom-right" visibleToasts={4} gap={8} containerAriaLabel={t.toasts_region()} />
