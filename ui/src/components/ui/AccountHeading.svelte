<script lang="ts">
  import type { Snippet } from "svelte";

  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import Avatar from "./Avatar.svelte";
  import AccountStatusBadges from "./AccountStatusBadges.svelte";
  import { AvatarSize } from "./ui.constants";

  type AccountHeadingPropsType = {
    account: AccountSnapshotType;
    subtitle: string;
    actions?: Snippet;
  };

  let { account, subtitle, actions }: AccountHeadingPropsType = $props();
</script>

<div class="flex flex-wrap items-center gap-x-3 gap-y-2.5">
  <Avatar url={account.avatar_url} name={account.display_name} size={AvatarSize.LG} />
  <div class="flex min-w-0 flex-[1_1_12rem] flex-col gap-1">
    <span class="flex min-w-0 flex-wrap items-center gap-2">
      <span class="truncate text-base font-semibold">{account.display_name}</span>
      <AccountStatusBadges {account} />
    </span>
    <span class="text-hint text-muted-foreground">{subtitle}</span>
  </div>
  {#if actions}<div class="flex max-w-full shrink-0 flex-wrap items-center gap-2">
      {@render actions()}
    </div>{/if}
</div>
