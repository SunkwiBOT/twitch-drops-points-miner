<script lang="ts">
  import type { Snippet } from "svelte";

  import { Field, FieldDescription, FieldError, FieldLabel } from "$lib/components/ui/field";
  import type { TextPersonType } from "../../lib/text.utils";
  import RichText from "./RichText.svelte";

  type FormFieldPropsType = {
    id: string;
    label: string;
    hint?: string;
    people?: TextPersonType[];
    error?: string;
    children: Snippet;
  };

  let { id, label, hint, people, error, children }: FormFieldPropsType = $props();
</script>

<Field class="gap-1.5" data-invalid={error ? "true" : undefined}>
  <FieldLabel for={id}>{label}</FieldLabel>
  {@render children()}
  {#if hint}<FieldDescription class="text-hint"><RichText text={hint} {people} /></FieldDescription
    >{/if}
  {#if error}<FieldError>{error}</FieldError>{/if}
</Field>
