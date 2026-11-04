import type { SpamRewardType } from "../bindings/SpamRewardType";
import type { RewardStatus } from "./spam.constants";

export type SpamHistoryRowType = {
  login: string;
  name: string;
  avatar: string | undefined;
  live: boolean;
  balance: number | undefined;
  in_points: boolean;
  own: boolean;
};

export type SpamChannelType = {
  login: string;
  name: string;
  avatar: string | undefined;
  live: boolean;
  game: string | undefined;
  viewers: number | undefined;
};

export type SpamRewardItemType = { reward: SpamRewardType; status: RewardStatus };
