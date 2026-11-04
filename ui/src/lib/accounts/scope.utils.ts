import { DropsScope } from "../bindings/DropsScope";
import { t } from "../i18n/i18n.svelte";

export type ScopeChoiceType = { value: DropsScope; title: string; description: string };

export const DROPS_SCOPE_LABELS: Record<DropsScope, () => string> = {
  [DropsScope.ALL]: () => t.drops_scope_all(),
  [DropsScope.LINKED_ONLY]: () => t.drops_scope_linked_only(),
  [DropsScope.PRIORITY_ONLY]: () => t.drops_scope_priority_only(),
};

const DROPS_SCOPE_HINTS: Record<DropsScope, () => string> = {
  [DropsScope.ALL]: () => t.drops_scope_all_hint(),
  [DropsScope.LINKED_ONLY]: () => t.drops_scope_linked_hint(),
  [DropsScope.PRIORITY_ONLY]: () => t.drops_scope_priority_hint(),
};

export const dropsScopeChoices = (withPriority: boolean): ScopeChoiceType[] =>
  [DropsScope.ALL, DropsScope.LINKED_ONLY, ...(withPriority ? [DropsScope.PRIORITY_ONLY] : [])].map(
    (scope) => ({
      value: scope,
      title: DROPS_SCOPE_LABELS[scope](),
      description: DROPS_SCOPE_HINTS[scope](),
    }),
  );
