<script lang="ts">
  import type { Tick } from '../engine';

  interface WavePoint {
    tick: Tick;
    position: number;
  }

  interface WaveHistory {
    id: number;
    speed: number;
    color: string;
    history: WavePoint[];
  }

  interface Props {
    waves: WaveHistory[];
    currentTick: Tick;
    width?: number;
    height?: number;
    maxHistory?: number;
  }

  let {
    waves,
    currentTick,
    width = 400,
    height = 150,
    maxHistory = 30,
  }: Props = $props();

  const padding = { left: 40, right: 20, top: 20, bottom: 30 };
  const innerWidth = $derived(width - padding.left - padding.right);
  const innerHeight = $derived(height - padding.top - padding.bottom);

  // Scale functions
  function tickToX(tick: Tick): number {
    const startTick = Math.max(0, currentTick - maxHistory);
    const endTick = currentTick;
    if (endTick === startTick) return padding.left;
    return padding.left + ((tick - startTick) / (endTick - startTick)) * innerWidth;
  }

  function posToY(pos: number): number {
    const maxPos = Math.max(20, ...waves.flatMap(w => w.history.map(h => h.position)));
    return padding.top + innerHeight - (pos / maxPos) * innerHeight;
  }

  // Generate path for a wave's history
  function getWavePath(wave: WaveHistory): string {
    const startTick = Math.max(0, currentTick - maxHistory);
    const points = wave.history.filter(p => p.tick >= startTick);

    if (points.length === 0) return '';

    return points
      .map((p, i) => `${i === 0 ? 'M' : 'L'}${tickToX(p.tick)},${posToY(p.position)}`)
      .join(' ');
  }

  // X-axis ticks
  const xTicks = $derived.by(() => {
    const startTick = Math.max(0, currentTick - maxHistory);
    const ticks: number[] = [];
    const step = Math.ceil(maxHistory / 5);
    for (let t = startTick; t <= currentTick; t += step) {
      ticks.push(t);
    }
    return ticks;
  });
</script>

<div class="waveform-container">
  <div class="waveform-header">
    <span class="waveform-title">Wave Positions Over Time</span>
  </div>

  <svg {width} {height}>
    <!-- Grid lines -->
    {#each [0.25, 0.5, 0.75] as ratio}
      <line
        x1={padding.left}
        y1={padding.top + innerHeight * ratio}
        x2={width - padding.right}
        y2={padding.top + innerHeight * ratio}
        stroke="var(--bg-tertiary)"
        stroke-width="1"
        stroke-dasharray="4,4"
      />
    {/each}

    <!-- X-axis -->
    <line
      x1={padding.left}
      y1={height - padding.bottom}
      x2={width - padding.right}
      y2={height - padding.bottom}
      stroke="var(--text-muted)"
      stroke-width="1"
    />

    <!-- X-axis ticks -->
    {#each xTicks as tick}
      <g transform="translate({tickToX(tick)}, {height - padding.bottom})">
        <line y1="0" y2="4" stroke="var(--text-muted)" stroke-width="1" />
        <text y="16" text-anchor="middle" font-size="9" fill="var(--text-muted)">
          t={tick}
        </text>
      </g>
    {/each}

    <!-- Y-axis -->
    <line
      x1={padding.left}
      y1={padding.top}
      x2={padding.left}
      y2={height - padding.bottom}
      stroke="var(--text-muted)"
      stroke-width="1"
    />

    <!-- Y-axis label -->
    <text
      x={12}
      y={padding.top + innerHeight / 2}
      text-anchor="middle"
      font-size="9"
      fill="var(--text-muted)"
      transform="rotate(-90, 12, {padding.top + innerHeight / 2})"
    >
      Position
    </text>

    <!-- Wave paths -->
    {#each waves as wave}
      <path
        d={getWavePath(wave)}
        fill="none"
        stroke={wave.color}
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        opacity="0.8"
      />

      <!-- Current position dot -->
      {#if wave.history.length > 0}
        {@const lastPoint = wave.history[wave.history.length - 1]}
        <circle
          cx={tickToX(lastPoint.tick)}
          cy={posToY(lastPoint.position)}
          r="5"
          fill={wave.color}
        />
      {/if}
    {/each}
  </svg>

  <!-- Legend -->
  <div class="waveform-legend">
    {#each waves as wave}
      <div class="legend-item">
        <span class="legend-color" style="background: {wave.color}"></span>
        <span class="legend-label">W{wave.id} ({wave.speed}x)</span>
      </div>
    {/each}
  </div>
</div>

<style>
  .waveform-container {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1rem;
  }

  .waveform-header {
    margin-bottom: 0.75rem;
  }

  .waveform-title {
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  svg {
    display: block;
  }

  .waveform-legend {
    display: flex;
    gap: 1rem;
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--bg-tertiary);
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .legend-color {
    width: 12px;
    height: 4px;
    border-radius: 2px;
  }
</style>
