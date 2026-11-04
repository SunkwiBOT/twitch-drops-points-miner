import { BetResult } from "../bindings/BetResult";
import { EventCategory } from "../bindings/EventCategory";
import { EventKind } from "../bindings/EventKind";
import { MinerKind } from "../bindings/MinerKind";
import { PointsReason } from "../bindings/PointsReason";
import type { MinerEventType } from "../bindings/MinerEventType";
import { number } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { escapeEmphasis, linkMarker, strong, type TextLinkType } from "../text.utils";
import { BadgeTone, IconName } from "../../components/ui/ui.constants";
import { TWITCH_WATCH_STREAK_REASON } from "./events.constants";

export const EVENT_KIND_LABELS: Record<EventKind, () => string> = {
  [EventKind.INFO]: () => t.event_kind_info(),
  [EventKind.ERROR]: () => t.event_kind_error(),
  [EventKind.WATCH_STARTED]: () => t.event_kind_watch_started(),
  [EventKind.WATCH_STOPPED]: () => t.event_kind_watch_stopped(),
  [EventKind.DROP_PROGRESS]: () => t.event_kind_drop_progress(),
  [EventKind.DROP_CLAIMED]: () => t.event_kind_drop_claimed(),
  [EventKind.CAMPAIGN_FINISHED]: () => t.event_kind_campaign_finished(),
  [EventKind.POINTS_EARNED]: () => t.event_kind_points_earned(),
  [EventKind.POINTS_SPENT]: () => t.event_kind_points_spent(),
  [EventKind.BONUS_CLAIMED]: () => t.event_kind_bonus_claimed(),
  [EventKind.PREDICTION_PLACED]: () => t.event_kind_prediction_placed(),
  [EventKind.PREDICTION_RESULT]: () => t.event_kind_prediction_result(),
  [EventKind.STREAM_ONLINE]: () => t.event_kind_stream_online(),
  [EventKind.STREAM_OFFLINE]: () => t.event_kind_stream_offline(),
  [EventKind.RAID_JOINED]: () => t.event_kind_raid_joined(),
  [EventKind.MOMENT_CLAIMED]: () => t.event_kind_moment_claimed(),
  [EventKind.STREAK_RECOVERED]: () => t.event_kind_streak_recovered(),
  [EventKind.GOAL_CONTRIBUTED]: () => t.event_kind_goal_contributed(),
  [EventKind.CHAT_MENTION]: () => t.event_kind_chat_mention(),
  [EventKind.SESSION_EXPIRED]: () => t.event_kind_session_expired(),
  [EventKind.LOGIN_COMPLETED]: () => t.event_kind_login_completed(),
  [EventKind.UPDATE_AVAILABLE]: () => t.event_kind_update_available(),
};

export const EVENT_TONES: Record<EventKind, BadgeTone> = {
  [EventKind.INFO]: BadgeTone.NEUTRAL,
  [EventKind.ERROR]: BadgeTone.DANGER,
  [EventKind.WATCH_STARTED]: BadgeTone.NEUTRAL,
  [EventKind.WATCH_STOPPED]: BadgeTone.NEUTRAL,
  [EventKind.DROP_PROGRESS]: BadgeTone.NEUTRAL,
  [EventKind.DROP_CLAIMED]: BadgeTone.ACCENT,
  [EventKind.CAMPAIGN_FINISHED]: BadgeTone.ACCENT,
  [EventKind.POINTS_EARNED]: BadgeTone.SUCCESS,
  [EventKind.POINTS_SPENT]: BadgeTone.NEUTRAL,
  [EventKind.BONUS_CLAIMED]: BadgeTone.SUCCESS,
  [EventKind.PREDICTION_PLACED]: BadgeTone.NEUTRAL,
  [EventKind.PREDICTION_RESULT]: BadgeTone.WARNING,
  [EventKind.STREAM_ONLINE]: BadgeTone.NEUTRAL,
  [EventKind.STREAM_OFFLINE]: BadgeTone.NEUTRAL,
  [EventKind.RAID_JOINED]: BadgeTone.NEUTRAL,
  [EventKind.MOMENT_CLAIMED]: BadgeTone.NEUTRAL,
  [EventKind.STREAK_RECOVERED]: BadgeTone.NEUTRAL,
  [EventKind.GOAL_CONTRIBUTED]: BadgeTone.NEUTRAL,
  [EventKind.CHAT_MENTION]: BadgeTone.INFO,
  [EventKind.SESSION_EXPIRED]: BadgeTone.DANGER,
  [EventKind.LOGIN_COMPLETED]: BadgeTone.NEUTRAL,
  [EventKind.UPDATE_AVAILABLE]: BadgeTone.INFO,
};

export const EVENT_ICONS: Record<EventKind, IconName> = {
  [EventKind.INFO]: IconName.BELL,
  [EventKind.ERROR]: IconName.ALERT,
  [EventKind.WATCH_STARTED]: IconName.PLAY,
  [EventKind.WATCH_STOPPED]: IconName.PAUSE,
  [EventKind.DROP_PROGRESS]: IconName.CLOCK,
  [EventKind.DROP_CLAIMED]: IconName.GIFT,
  [EventKind.CAMPAIGN_FINISHED]: IconName.CHECK,
  [EventKind.POINTS_EARNED]: IconName.CHANNEL_POINTS,
  [EventKind.POINTS_SPENT]: IconName.CHANNEL_POINTS,
  [EventKind.BONUS_CLAIMED]: IconName.CHANNEL_POINTS,
  [EventKind.PREDICTION_PLACED]: IconName.TARGET,
  [EventKind.PREDICTION_RESULT]: IconName.TROPHY,
  [EventKind.STREAM_ONLINE]: IconName.RADIO,
  [EventKind.STREAM_OFFLINE]: IconName.TV,
  [EventKind.RAID_JOINED]: IconName.ZAP,
  [EventKind.MOMENT_CLAIMED]: IconName.STAR,
  [EventKind.STREAK_RECOVERED]: IconName.FLAME,
  [EventKind.GOAL_CONTRIBUTED]: IconName.TARGET,
  [EventKind.CHAT_MENTION]: IconName.AT,
  [EventKind.SESSION_EXPIRED]: IconName.LOGOUT,
  [EventKind.LOGIN_COMPLETED]: IconName.LOG_IN,
  [EventKind.UPDATE_AVAILABLE]: IconName.IMPORT,
};

export const eventIcon = (event: MinerEventType): IconName => {
  const streakBonus =
    event.kind === EventKind.POINTS_EARNED && event.context.reason === TWITCH_WATCH_STREAK_REASON;

  return streakBonus || event.kind === EventKind.STREAK_RECOVERED
    ? IconName.WATCH_STREAK
    : EVENT_ICONS[event.kind];
};

const WATCH_POINTS_TEXTS: Record<PointsReason, (args: { channel: string }) => string> = {
  [PointsReason.STREAK_BONUS]: (args) => t.ev_watch_points_streak_bonus(args),
  [PointsReason.STREAK_AT_RISK]: (args) => t.ev_watch_points_streak_at_risk(args),
  [PointsReason.SUBSCRIBED]: (args) => t.ev_watch_points_subscribed(args),
  [PointsReason.LIST_ORDER]: (args) => t.ev_watch_points_list_order(args),
};

export const channelUrl = (login: string): string => `https://www.twitch.tv/${login}`;
export const campaignUrl = (campaignId: string): string =>
  `https://www.twitch.tv/drops/campaigns?dropID=${encodeURIComponent(campaignId)}`;

export type EventTextType = { text: string; links: TextLinkType[] };

export const eventText = (event: MinerEventType, linked: boolean): EventTextType => {
  const context = event.context;
  const links: TextLinkType[] = [];

  const named = (label: string | undefined, url: string | null): string | undefined => {
    if (label === undefined) {
      return undefined;
    }

    if (linked && url) {
      links.push({ label, url });

      return linkMarker(links.length - 1);
    }

    return strong(label);
  };

  const bold = (value: string | number | undefined): string | undefined =>
    value === undefined ? undefined : strong(typeof value === "number" ? number(value) : value);

  const campaignLink = context.campaign_id ? campaignUrl(context.campaign_id) : null;
  const channel = named(
    context.channel_name ?? context.channel,
    context.channel ? channelUrl(context.channel) : null,
  );
  const drop = named(context.drop_name, campaignLink);
  const campaign = drop ? bold(context.campaign_name) : named(context.campaign_name, campaignLink);
  const fallback = escapeEmphasis(event.title);

  const text = ((): string => {
    switch (event.kind) {
      case EventKind.WATCH_STARTED:
        if (channel && campaign) {
          return t.ev_watch_drop({ channel, campaign });
        }

        return channel && context.watch_reason
          ? WATCH_POINTS_TEXTS[context.watch_reason]({ channel })
          : fallback;
      case EventKind.WATCH_STOPPED:
        return channel ? t.ev_watch_stopped({ channel }) : fallback;
      case EventKind.DROP_CLAIMED:
        return drop && campaign ? t.ev_drop_claimed({ drop, campaign }) : fallback;
      case EventKind.CAMPAIGN_FINISHED:
        return campaign ? t.ev_campaign_finished({ campaign }) : fallback;
      case EventKind.DROP_PROGRESS:
        if (context.current === undefined || context.required === undefined) {
          return fallback;
        }

        return drop
          ? t.ev_drop_progress({
              drop,
              current: bold(context.current) ?? "",
              required: bold(context.required) ?? "",
            })
          : t.ev_drop_progress_unknown({
              current: bold(context.current) ?? "",
              required: bold(context.required) ?? "",
            });
      case EventKind.BONUS_CLAIMED:
        return channel ? t.ev_bonus({ channel }) : fallback;
      case EventKind.POINTS_EARNED:
        return channel && context.earned !== undefined
          ? t.ev_points({ channel, points: bold(context.earned) ?? "" })
          : fallback;
      case EventKind.POINTS_SPENT:
        if (channel && context.spent !== undefined) {
          return t.ev_points_spent({ channel, points: bold(context.spent) ?? "" });
        }

        return channel && context.balance !== undefined
          ? t.ev_points_spent_balance({ channel, balance: bold(context.balance) ?? "" })
          : fallback;
      case EventKind.STREAM_ONLINE:
        return channel ? t.ev_online({ channel }) : fallback;
      case EventKind.STREAM_OFFLINE:
        return channel ? t.ev_offline({ channel }) : fallback;
      case EventKind.RAID_JOINED:
        return channel ? t.ev_raid({ channel }) : fallback;
      case EventKind.MOMENT_CLAIMED:
        return channel ? t.ev_moment({ channel }) : fallback;
      case EventKind.PREDICTION_PLACED:
        return channel && context.points !== undefined
          ? t.ev_bet_placed({
              channel,
              points: bold(context.points) ?? "",
              outcome: bold(context.outcome) ?? "",
            })
          : fallback;
      case EventKind.PREDICTION_RESULT:
        if (context.bet_result === BetResult.WIN) {
          return t.ev_bet_won({ points: bold(context.points_won ?? 0) ?? "" });
        }

        if (context.bet_result === BetResult.LOSE) {
          return t.ev_bet_lost({ points: bold(context.staked ?? 0) ?? "" });
        }

        return context.bet_result === BetResult.REFUND ? t.ev_bet_refund() : fallback;
      case EventKind.LOGIN_COMPLETED:
        return t.ev_login({ login: strong(context.account_name ?? event.account_login) });
      case EventKind.SESSION_EXPIRED:
        return t.ev_session_expired();
      case EventKind.UPDATE_AVAILABLE:
        return context.version
          ? t.ev_update_available({ version: strong(context.version) })
          : fallback;
      case EventKind.STREAK_RECOVERED:
        return channel ? t.ev_streak_recovered({ channel }) : fallback;
      case EventKind.GOAL_CONTRIBUTED:
        return channel && context.points !== undefined
          ? t.ev_goal_contributed({
              points: bold(context.points) ?? "",
              goal: bold(context.goal) ?? "",
              channel,
            })
          : fallback;
      case EventKind.CHAT_MENTION:
        return channel && context.chatter
          ? t.ev_chat_mention({ chatter: strong(context.chatter), channel })
          : fallback;
      case EventKind.INFO:
      case EventKind.ERROR:
        return fallback;
    }
  })();

  return { text, links };
};

export const MINER_LABELS: Record<MinerKind, () => string> = {
  [MinerKind.DROPS]: () => t.miner_drops(),
  [MinerKind.POINTS]: () => t.miner_points(),
};

export const MINER_TONES: Record<MinerKind, BadgeTone> = {
  [MinerKind.DROPS]: BadgeTone.ACCENT,
  [MinerKind.POINTS]: BadgeTone.SUCCESS,
};

export const MINER_ICONS: Record<MinerKind, IconName> = {
  [MinerKind.DROPS]: IconName.GIFT,
  [MinerKind.POINTS]: IconName.CHANNEL_POINTS,
};

export const JOURNAL_ALL = "ALL";

export type JournalFilterType = EventCategory | typeof JOURNAL_ALL;

export const JOURNAL_FILTER_SLUGS: Record<JournalFilterType, string> = {
  [JOURNAL_ALL]: "all",
  [EventCategory.DROPS]: "drops",
  [EventCategory.POINTS]: "points",
  [EventCategory.OTHER]: "other",
};

export const JOURNAL_FILTERS: {
  value: JournalFilterType;
  label: () => string;
  icon?: IconName;
}[] = [
  { value: JOURNAL_ALL, label: () => t.journal_filter_all() },
  { value: EventCategory.DROPS, label: () => t.journal_filter_drops(), icon: IconName.GIFT },
  {
    value: EventCategory.POINTS,
    label: () => t.journal_filter_points(),
    icon: IconName.CHANNEL_POINTS,
  },
  { value: EventCategory.OTHER, label: () => t.journal_filter_other() },
];

export enum KindGroup {
  DROPS = "DROPS",
  POINTS = "POINTS",
  WATCH = "WATCH",
  OTHER = "OTHER",
}

export const KIND_GROUP_LABELS: Record<KindGroup, () => string> = {
  [KindGroup.DROPS]: () => t.journal_filter_drops(),
  [KindGroup.POINTS]: () => t.journal_filter_points(),
  [KindGroup.WATCH]: () => t.journal_group_watch(),
  [KindGroup.OTHER]: () => t.journal_filter_other(),
};

export const KIND_GROUPS: { group: KindGroup; kinds: EventKind[]; categories: EventCategory[] }[] =
  [
    {
      group: KindGroup.DROPS,
      kinds: [EventKind.DROP_PROGRESS, EventKind.DROP_CLAIMED, EventKind.CAMPAIGN_FINISHED],
      categories: [EventCategory.DROPS],
    },
    {
      group: KindGroup.POINTS,
      kinds: [
        EventKind.POINTS_EARNED,
        EventKind.POINTS_SPENT,
        EventKind.BONUS_CLAIMED,
        EventKind.PREDICTION_PLACED,
        EventKind.PREDICTION_RESULT,
        EventKind.RAID_JOINED,
        EventKind.MOMENT_CLAIMED,
        EventKind.STREAK_RECOVERED,
        EventKind.GOAL_CONTRIBUTED,
        EventKind.STREAM_ONLINE,
        EventKind.STREAM_OFFLINE,
        EventKind.CHAT_MENTION,
      ],
      categories: [EventCategory.POINTS],
    },
    {
      group: KindGroup.WATCH,
      kinds: [EventKind.WATCH_STARTED, EventKind.WATCH_STOPPED],
      categories: [EventCategory.DROPS, EventCategory.POINTS],
    },
    {
      group: KindGroup.OTHER,
      kinds: [
        EventKind.INFO,
        EventKind.ERROR,
        EventKind.SESSION_EXPIRED,
        EventKind.LOGIN_COMPLETED,
        EventKind.UPDATE_AVAILABLE,
      ],
      categories: [EventCategory.OTHER],
    },
  ];

export const kindGroupsOf = (filter: JournalFilterType) =>
  KIND_GROUPS.filter((entry) => filter === JOURNAL_ALL || entry.categories.includes(filter));

export const POINTS_ROUTINE: ReadonlySet<EventKind> = new Set([
  EventKind.POINTS_EARNED,
  EventKind.POINTS_SPENT,
]);
