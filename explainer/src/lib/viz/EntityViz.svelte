<script lang="ts">
  import type { Event, EntityState, Tick } from '../engine';
  import { LifecycleState, lifecycleStateColors, lifecycleStateNames } from '../engine';

  interface Props {
    events: readonly Event<EntityState>[];
    minTick?: Tick;
    maxTick?: Tick;
    queryTick?: Tick | null;
    width?: number;
    height?: number;
    label?: string;
    showBinarySearch?: boolean;
  }

  let {
    events,
    minTick = 0,
    maxTick = 20,
    queryTick = null,
    width = 600,
    height = 80,
    label = 'Entity',
    showBinarySearch = false,
  }: Props = $props();

  const padding = { left: 60, right: 20, top: 15, bottom: 15 };
  const innerWidth = $derived(width - padding.left - padding.right);
  const innerHeight = $derived(height - padding.top - padding.bottom);

  function tickToX(tick: Tick): number {
    const range = maxTick - minTick;
    if (range === 0) return padding.left;
    return padding.left + ((tick - minTick) / range) * innerWidth;
  }

  // Find the event at or before queryTick using binary search
  const activeEvent = $derived.by(() => {
    if (queryTick === null || events.length === 0) return null;
    if (queryTick < events[0].timestamp) return null;

    let left = 0;
    let right = events.length - 1;

    while (left < right) {
      const mid = Math.ceil((left + right + 1) / 2);
      if (events[mid].timestamp <= queryTick) {
        left = mid;
      } else {
        right = mid - 1;
      }
    }

    return events[left];
  });

  // For binary search animation
  const searchSteps = $derived.by(() => {
    if (!showBinarySearch || queryTick === null || events.length === 0) return [];

    const steps: { left: number; right: number; mid: number; comparison: 'less' | 'equal' | 'greater' }[] = [];

    if (queryTick < events[0].timestamp) return steps;

    let left = 0;
    let right = events.length - 1;

    while (left < right) {
      const mid = Math.ceil((left + right + 1) / 2);
      const comparison = events[mid].timestamp <= queryTick ? 'less' : 'greater';
      steps.push({ left, right, mid, comparison });

      if (events[mid].timestamp <= queryTick) {
        left = mid;
      } else {
        right = mid - 1;
      }
    }

    steps.push({ left, right: left, mid: left, comparison: 'equal' });
    return steps;
  });
</script>

<svg {width} {height} class="entity-viz">
  <!-- Label -->
  <text x="10" y={padding.top + innerHeight / 2 + 4} font-size="12" fill="var(--text-secondary)">
    {label}
  </text>

  <!-- Timeline bar background -->
  <rect
    x={padding.left}
    y={padding.top + innerHeight / 2 - 3}
    width={innerWidth}
    height="6"
    fill="var(--bg-tertiary)"
    rx="3"
  />

  <!-- Event spans (colored regions between events) -->
  {#each events as event, i}
    {@const nextEvent = events[i + 1]}
    {@const startX = tickToX(event.timestamp)}
    {@const endX = nextEvent ? tickToX(nextEvent.timestamp) : width - padding.right}
    {@const color = lifecycleStateColors[event.lifecycle]}

    <rect
      x={startX}
      y={padding.top + innerHeight / 2 - 3}
      width={Math.max(0, endX - startX)}
      height="6"
      fill={color}
      opacity="0.6"
      rx="3"
    />
  {/each}

  <!-- Event markers -->
  {#each events as event, i}
    {@const x = tickToX(event.timestamp)}
    {@const isActive = activeEvent === event}

    <g class="event-marker" class:active={isActive}>
      <circle
        cx={x}
        cy={padding.top + innerHeight / 2}
        r={isActive ? 8 : 5}
        fill={lifecycleStateColors[event.lifecycle]}
        stroke={isActive ? 'white' : 'none'}
        stroke-width="2"
      />

      <!-- Event label -->
      <text
        x={x}
        y={padding.top - 2}
        text-anchor="middle"
        font-size="9"
        fill="var(--text-muted)"
      >
        t={event.timestamp}
      </text>

      <!-- State label below -->
      <text
        x={x}
        y={height - 2}
        text-anchor="middle"
        font-size="9"
        fill={lifecycleStateColors[event.lifecycle]}
      >
        {lifecycleStateNames[event.lifecycle]}
      </text>
    </g>
  {/each}

  <!-- Query tick indicator -->
  {#if queryTick !== null}
    <line
      x1={tickToX(queryTick)}
      y1={padding.top}
      x2={tickToX(queryTick)}
      y2={height - padding.bottom}
      stroke="var(--color-present)"
      stroke-width="2"
      stroke-dasharray="4,4"
      opacity="0.8"
    />
  {/if}
</svg>

<style>
  .entity-viz {
    display: block;
  }

  .event-marker {
    transition: transform 0.2s ease;
  }

  .event-marker.active circle {
    filter: drop-shadow(0 0 4px var(--color-present));
  }
</style>
