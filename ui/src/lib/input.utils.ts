export const boundedInteger = (value: number, min: number, max: number): number =>
  Number.isNaN(value) ? min : Math.min(max, Math.max(min, Math.round(value)));

// ! Bounded back to the value already set, the setting does not change and the
// field would keep the number typed: it is written back.
export const boundNumberInput = (input: HTMLInputElement, min: number, max: number): number => {
  const value = boundedInteger(Number(input.value), min, max);

  input.value = String(value);

  return value;
};
