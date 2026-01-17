<script lang="ts">
  import type { Tick } from '../engine';

  interface Props {
    minTick?: Tick;
    maxTick?: Tick;
    currentTick?: Tick;
    width?: number;
    height?: number;
    showLabels?: boolean;
    highlightRange?: { start: Tick; end: Tick } | null;
  }

  let {
    minTick = 0,
    maxTick = 20,
    currentTick = 0,
    width = 600,
    height = 60,
    showLabels = true,
    highlightRange = null,
  }: Props = $props();

  const padding = { left: 40, right: 20, top: 10, bottom: 25 };
  const innerWidth = $derived(width - padding.left - padding.right);
  const innerHeight = $derived(height - padding.top - padding.bottom);

  // Scale tick to x position
  function tickToX(tick: Tick): number {
    const range = maxTick - minTick;
    if (range === 0) return padding.left;
    return padding.left + ((tick - minTick) / range) * innerWidth;
  }

  // Generate tick marks
  const tickMarks = $derived.by(() => {
    const marks: Tick[] = [];
    const step = Math.max(1, Math.ceil((maxTick - minTick) / 10));
    for (let t = minTick; t <= maxTick; t += step) {
      marks.push(t);
    }
    return marks;
  });
</script>

<svg {width} {height} class="timeline">
  <!-- Highlight range -->
  {#if highlightRange}
    <rect
      x={tickToX(highlightRange.start)}
      y={padding.top}
      width={tickToX(highlightRange.end) - tickToX(highlightRange.start)}
      height={innerHeight}
      fill="var(--color-present)"
      opacity="0.1"
    />
  {/if}

  <!-- Main axis line -->
  <line
    x1={padding.left}
    y1={padding.top + innerHeight / 2}
    x2={width - padding.right}
    y2={padding.top + innerHeight / 2}
    stroke="var(--text-muted)"
    stroke-width="2"
  />

  <!-- Tick marks -->
  {#each tickMarks as tick}
    <line
      x1={tickToX(tick)}
      y1={padding.top + innerHeight / 2 - 6}
      x2={tickToX(tick)}
      y2={padding.top + innerHeight / 2 + 6}
      stroke="var(--text-muted)"
      stroke-width="1"
    />
    {#if showLabels}
      <text
        x={tickToX(tick)}
        y={height - 5}
        text-anchor="middle"
        font-size="11"
        fill="var(--text-muted)"
      >
        {tick}
      </text>
    {/if}
  {/each}

  <!-- Current tick indicator -->
  <circle
    cx={tickToX(currentTick)}
    cy={padding.top + innerHeight / 2}
    r="6"
    fill="var(--color-present)"
  />

  <!-- "t=" label -->
  {#if showLabels}
    <text x="5" y={padding.top + innerHeight / 2 + 4} font-size="12" fill="var(--text-secondary)">
      t=
    </text>
  {/if}
</svg>

<style>
  .timeline {
    display: block;
  }
</style>
