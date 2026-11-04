import { BenefitKind } from "../../lib/bindings/BenefitKind";
import { t } from "../../lib/i18n/i18n.svelte";

export const BENEFIT_KIND_LABELS: Record<BenefitKind, (() => string) | null> = {
  [BenefitKind.BADGE]: () => t.reward_kind_badge(),
  [BenefitKind.IN_GAME_ITEM]: () => t.reward_kind_item(),
  [BenefitKind.OTHER]: null,
};
