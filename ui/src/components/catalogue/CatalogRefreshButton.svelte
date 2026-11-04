<script lang="ts">
  import { mmss } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app, refreshCatalog } from "../../lib/state/state.svelte";
  import { SECOND_MS } from "../../lib/time/time.constants";
  import ActionButton from "../ui/ActionButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  type CatalogRefreshButtonPropsType = { label: string };

  let { label }: CatalogRefreshButtonPropsType = $props();

  let busy = $state(false);
  let now = $state(Date.now());

  const next = $derived(new Date(app.snapshot?.catalog.next_refresh_at ?? 0).getTime());
  const wait = $derived(Math.max(0, next - now));

  $effect(() => {
    const until = next;

    now = Date.now();

    if (until <= now) {
      return;
    }

    const timer = setInterval(() => {
      now = Date.now();

      if (now >= until) {
        clearInterval(timer);
      }
    }, SECOND_MS);

    return () => clearInterval(timer);
  });

  const refresh = async (): Promise<void> => {
    busy = true;
    await refreshCatalog();
    now = Date.now();
    busy = false;
  };
</script>

<ActionButton
  icon={IconName.REFRESH}
  label={wait > 0 ? t.catalogue_refresh_wait({ time: mmss(wait) }) : label}
  busyLabel={t.common_refreshing()}
  {busy}
  disabled={wait > 0}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  onclick={refresh}
/>
