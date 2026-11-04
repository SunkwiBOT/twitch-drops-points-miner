export type TextLinkType = { label: string; url: string };

export const LINK_DELIMITER = "\uE000";

export const linkMarker = (index: number): string => `${LINK_DELIMITER}${index}${LINK_DELIMITER}`;

export type TextPersonType = {
  name: string;
  avatar: string | undefined;
  channel: string | null;
};

export const PERSON_DELIMITER = "\uE001";

export const personMarker = (index: number): string =>
  `${PERSON_DELIMITER}${index}${PERSON_DELIMITER}`;

export const RICH_TEXT_TOKENS = /(\*\*|\uE000\d+\uE000|\uE001\d+\uE001)/;

export const escapeEmphasis = (value: string): string =>
  value.replaceAll("*", "∗").replaceAll(LINK_DELIMITER, "").replaceAll(PERSON_DELIMITER, "");

export const strong = (value: string | number): string => `**${escapeEmphasis(String(value))}**`;
