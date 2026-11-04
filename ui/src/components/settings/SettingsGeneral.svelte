<script lang="ts">
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import { Theme } from "../../lib/bindings/Theme";
  import { Language, LANGUAGE_NAMES, RESOLVED_LANGUAGES } from "../../lib/i18n/i18n.constants";
  import { t } from "../../lib/i18n/i18n.svelte";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import Select from "../ui/Select.svelte";
  import { MenuAlign } from "../ui/ui.constants";

  type SettingsGeneralPropsType = {
    draft: GlobalSettingsType;
  };

  let { draft = $bindable() }: SettingsGeneralPropsType = $props();

  const languages = $derived([
    { value: Language.AUTO, label: t.language_auto() },
    ...RESOLVED_LANGUAGES.map((tag) => ({ value: tag, label: LANGUAGE_NAMES[tag] })),
  ]);

  const themes = $derived([
    { value: Theme.DARK, label: t.theme_dark() },
    { value: Theme.LIGHT, label: t.theme_light() },
  ]);
</script>

<Panel title={t.settings_general()}>
  <div class="grid grid-cols-[repeat(auto-fit,minmax(min(200px,100%),1fr))] gap-3">
    <FormField id="set-lang" label={t.settings_language()}>
      <Select
        id="set-lang"
        value={draft.language}
        options={languages}
        label={t.settings_language()}
        align={MenuAlign.LEFT}
        onchange={(language) => (draft.language = language)}
      />
    </FormField>
    <FormField id="set-theme" label={t.settings_theme()}>
      <Select
        id="set-theme"
        value={draft.theme}
        options={themes}
        label={t.settings_theme()}
        align={MenuAlign.LEFT}
        onchange={(theme) => (draft.theme = theme)}
      />
    </FormField>
  </div>
</Panel>
