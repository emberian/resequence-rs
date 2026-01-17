<script lang="ts">
  import type { Tick, TimewaveId } from '../engine';

  interface Props {
    id: TimewaveId;
    position: Tick;
    speed: number;
    minTick?: Tick;
    maxTick?: Tick;
    width?: number;
    height?: number;
    color?: string;
    label?: string;
  }

  let {
    id,
    position,
    speed,
    minTick = 0,
    maxTick = 20,
    width = 600,
    height = 100,
    color = 'var(--wave-normal)',
    label,
  }: Props = $props();

  const padding = { left: 40, right: 20, top: 30, bottom: 20 };
  const innerWidth = $derived(width - padding.left - padding.right);
  const innerHeight = $derived(height - padding.top - padding.bottom);

  function tickToX(tick: Tick): number {
    const range = maxTick - minTick;
    if (range === 0) return padding.left;
    return padding.left + ((tick - minTick) / range) * innerWidth;
  }

  const x = $derived(tickToX(position));
  const displayLabel = $derived(label ?? `Wave ${id}`);
  const speedLabel = $derived(
    speed === 0 ? 'paused' :
    speed === 1 ? '1x' :
    speed < 0 ? `${speed}x (reverse)` :
    `${speed}x`
  );
</script>

<g class="timewave-viz" style="--wave-color: {color}">
  <!-- Wave cursor line -->
  <line
    x1={x}
    y1={padding.top}
    x2={x}
    y2={height - padding.bottom}
    stroke={color}
    stroke-width="3"
  />

  <!-- Arrow head pointing down (indicates direction of observation) -->
  <polygon
    points="{x},{height - padding.bottom} {x - 6},{height - padding.bottom - 10} {x + 6},{height - padding.bottom - 10}"
    fill={color}
  />

  <!-- Label at top -->
  <text
    x={x}
    y={padding.top - 12}
    text-anchor="middle"
    font-size="11"
    font-weight="600"
    fill={color}
  >
    {displayLabel}
  </text>

  <!-- Speed indicator -->
  <text
    x={x}
    y={padding.top - 2}
    text-anchor="middle"
    font-size="9"
    fill="var(--text-muted)"
  >
    {speedLabel}
  </text>

  <!-- Position indicator -->
  <rect
    x={x - 18}
    y={height - padding.bottom + 5}
    width="36"
    height="16"
    fill={color}
    rx="3"
  />
  <text
    x={x}
    y={height - padding.bottom + 16}
    text-anchor="middle"
    font-size="10"
    font-weight="500"
    fill="var(--bg-primary)"
  >
    t={position}
  </text>

  <!-- Speed direction arrow (small indicator) -->
  {#if speed !== 0}
    <g transform="translate({x + (speed > 0 ? 25 : -25)}, {padding.top + innerHeight / 2})">
      {#if speed > 0}
        <path d="M0,-4 L6,0 L0,4" fill={color} opacity="0.7" />
      {:else}
        <path d="M0,-4 L-6,0 L0,4" fill={color} opacity="0.7" />
      {/if}
    </g>
  {/if}
</g>

<style>
  .timewave-viz {
    transition: transform 0.3s ease;
  }

  .timewave-viz line {
    filter: drop-shadow(0 0 4px var(--wave-color));
  }
</style>
