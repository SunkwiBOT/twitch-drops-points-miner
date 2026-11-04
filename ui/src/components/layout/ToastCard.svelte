<script lang="ts">
  import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
  import InfoIcon from "@lucide/svelte/icons/info";
  import OctagonXIcon from "@lucide/svelte/icons/octagon-x";

  import TextButton from "../ui/TextButton.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import type { ToastActionType, ToastImageType } from "../../lib/state/state.svelte";
  import Avatar from "../ui/Avatar.svelte";
  import ChannelAvatar from "../ui/ChannelAvatar.svelte";
  import RichText from "../ui/RichText.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import { AvatarSize, ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  type ToastCardPropsType = {
    kind: ToastKind;
    text: string;
    image: ToastImageType | null;
    title: string | null;
    action: ToastActionType | null;
    onclose: () => void;
  };

  let { kind, text, image, title, action, onclose }: ToastCardPropsType = $props();
</script>

<div
  class="relative flex w-toast max-w-[calc(100vw-2rem)] items-start gap-3 rounded-lg border bg-popover p-3 pr-10 text-sm text-popover-foreground shadow-lg"
  role={kind === ToastKind.ERROR ? "alert" : "status"}
>
  {#if image?.channel}
    <ChannelAvatar url={image.url} name={image.name} size={AvatarSize.SM} live={false} />
  {:else if image}
    <Avatar url={image.url} name={image.name} size={AvatarSize.SM} />
  {:else if kind === ToastKind.ERROR}
    <OctagonXIcon class="mt-0.5 size-4 shrink-0 text-destructive" />
  {:else if kind === ToastKind.SUCCESS}
    <CircleCheckIcon class="mt-0.5 size-4 shrink-0 text-success-emphasis" />
  {:else}
    <InfoIcon class="mt-0.5 size-4 shrink-0 text-info-emphasis" />
  {/if}
  <div class="flex min-w-0 flex-1 flex-col gap-2">
    <p class="m-0 flex flex-col leading-snug break-words">
      {#if title}<span class="font-semibold">{title}</span>{/if}
      <span><RichText {text} /></span>
    </p>
    {#if action}
      <TextButton
        label={action.label}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        class="w-full"
        onclick={() => {
          // ! The action first: what it opens decides whether the end of the
          // toast still has to clean up.
          action.run();
          onclose();
        }}
      />
    {/if}
  </div>
  <div class="absolute top-1.5 right-1.5">
    <IconButton
      icon={IconName.X}
      label={t.common_close()}
      busy={false}
      disabled={false}
      variant={ButtonVariant.GHOST}
      size={ButtonSize.SM}
      onclick={onclose}
    />
  </div>
</div>
