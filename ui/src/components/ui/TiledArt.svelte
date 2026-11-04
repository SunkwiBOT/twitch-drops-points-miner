<script lang="ts">
  import { ART_TILES, type ThumbShape } from "./ui.constants";

  type TiledArtPropsType = {
    url: string;
    shape: ThumbShape;
    grey: boolean;
  };

  let { url, shape, grey }: TiledArtPropsType = $props();

  const uid = $props.id();
  const pattern = `tiles-${uid}`;

  const tile = $derived(ART_TILES[shape]);

  const spots = $derived([
    { x: 0, y: 0 },
    { x: tile.stepX, y: tile.stepY / 2 },
    { x: tile.stepX, y: tile.stepY / 2 - tile.stepY },
  ]);
</script>

<svg
  class={[
    "pointer-events-none absolute inset-0 -z-10 size-full opacity-20 [mask-image:linear-gradient(to_right,black,transparent_80%)] dark:opacity-30",
    grey && "grayscale",
  ]}
  aria-hidden="true"
>
  <defs>
    <pattern id={pattern} width={tile.stepX * 2} height={tile.stepY} patternUnits="userSpaceOnUse">
      {#each spots as spot (spot.y)}
        <image
          href={url}
          x={spot.x}
          y={spot.y}
          width={tile.width}
          height={tile.height}
          preserveAspectRatio="xMidYMid slice"
        />
      {/each}
    </pattern>
  </defs>
  <rect width="100%" height="100%" fill="url(#{pattern})" />
</svg>
