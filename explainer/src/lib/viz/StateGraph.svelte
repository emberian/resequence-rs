<script lang="ts">
  import { LifecycleState, lifecycleStateColors, lifecycleStateNames } from '../engine';

  interface Props {
    activeState?: LifecycleState | null;
    highlightTransition?: { from: LifecycleState; to: LifecycleState } | null;
    width?: number;
    height?: number;
    interactive?: boolean;
    onStateClick?: (state: LifecycleState) => void;
  }

  let {
    activeState = null,
    highlightTransition = null,
    width = 500,
    height = 300,
    interactive = true,
    onStateClick,
  }: Props = $props();

  // State positions (arranged in a logical flow)
  const statePositions: Record<LifecycleState, { x: number; y: number }> = {
    [LifecycleState.Unborn]: { x: 80, y: 150 },
    [LifecycleState.Prebirth]: { x: 200, y: 80 },
    [LifecycleState.Born]: { x: 320, y: 150 },
    [LifecycleState.Chronoporting]: { x: 200, y: 220 },
    [LifecycleState.Dead]: { x: 440, y: 150 },
  };

  // Transitions between states
  const transitions: { from: LifecycleState; to: LifecycleState; label: string }[] = [
    { from: LifecycleState.Unborn, to: LifecycleState.Prebirth, label: 'chronoport target' },
    { from: LifecycleState.Unborn, to: LifecycleState.Born, label: 'spawn()' },
    { from: LifecycleState.Prebirth, to: LifecycleState.Born, label: 'wave reaches' },
    { from: LifecycleState.Born, to: LifecycleState.Chronoporting, label: 'chronoport()' },
    { from: LifecycleState.Born, to: LifecycleState.Dead, label: 'destroy()' },
    { from: LifecycleState.Chronoporting, to: LifecycleState.Dead, label: 'departure' },
  ];

  const stateRadius = 35;

  function isTransitionHighlighted(from: LifecycleState, to: LifecycleState): boolean {
    return highlightTransition?.from === from && highlightTransition?.to === to;
  }

  function handleStateClick(state: LifecycleState) {
    if (interactive && onStateClick) {
      onStateClick(state);
    }
  }

  // Calculate arrow path between two states
  function getArrowPath(from: LifecycleState, to: LifecycleState): string {
    const p1 = statePositions[from];
    const p2 = statePositions[to];

    // Calculate direction
    const dx = p2.x - p1.x;
    const dy = p2.y - p1.y;
    const dist = Math.sqrt(dx * dx + dy * dy);
    const nx = dx / dist;
    const ny = dy / dist;

    // Start and end points (on circle edge)
    const x1 = p1.x + nx * stateRadius;
    const y1 = p1.y + ny * stateRadius;
    const x2 = p2.x - nx * (stateRadius + 8);
    const y2 = p2.y - ny * (stateRadius + 8);

    // Curve control point (offset perpendicular to line)
    const cx = (x1 + x2) / 2 - ny * 20;
    const cy = (y1 + y2) / 2 + nx * 20;

    return `M${x1},${y1} Q${cx},${cy} ${x2},${y2}`;
  }

  // Get label position for a transition
  function getLabelPos(from: LifecycleState, to: LifecycleState): { x: number; y: number } {
    const p1 = statePositions[from];
    const p2 = statePositions[to];
    const dx = p2.x - p1.x;
    const dy = p2.y - p1.y;
    const dist = Math.sqrt(dx * dx + dy * dy);
    const ny = dy / dist;
    const nx = dx / dist;

    return {
      x: (p1.x + p2.x) / 2 - ny * 30,
      y: (p1.y + p2.y) / 2 + nx * 30,
    };
  }
</script>

<svg {width} {height} class="state-graph">
  <defs>
    <!-- Arrow marker -->
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="var(--text-muted)" />
    </marker>
    <marker id="arrowhead-highlight" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-present)" />
    </marker>
  </defs>

  <!-- Transitions (arrows) -->
  {#each transitions as t}
    {@const highlighted = isTransitionHighlighted(t.from, t.to)}
    {@const labelPos = getLabelPos(t.from, t.to)}

    <path
      d={getArrowPath(t.from, t.to)}
      fill="none"
      stroke={highlighted ? 'var(--color-present)' : 'var(--text-muted)'}
      stroke-width={highlighted ? 2 : 1}
      marker-end={highlighted ? 'url(#arrowhead-highlight)' : 'url(#arrowhead)'}
      opacity={highlighted ? 1 : 0.5}
    />

    <text
      x={labelPos.x}
      y={labelPos.y}
      text-anchor="middle"
      font-size="10"
      fill={highlighted ? 'var(--color-present)' : 'var(--text-muted)'}
    >
      {t.label}
    </text>
  {/each}

  <!-- States (circles) -->
  {#each Object.entries(statePositions) as [stateStr, pos]}
    {@const state = Number(stateStr) as LifecycleState}
    {@const isActive = activeState === state}
    {@const color = lifecycleStateColors[state]}

    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <g
      class="state-node"
      class:active={isActive}
      class:interactive
      transform="translate({pos.x}, {pos.y})"
      onclick={() => handleStateClick(state)}
      onkeydown={(e) => e.key === 'Enter' && handleStateClick(state)}
      role={interactive ? 'button' : 'presentation'}
      tabindex={interactive ? 0 : -1}
    >
      <circle
        r={stateRadius}
        fill={isActive ? color : 'var(--bg-secondary)'}
        stroke={color}
        stroke-width={isActive ? 3 : 2}
      />
      <text
        y="4"
        text-anchor="middle"
        font-size="11"
        font-weight={isActive ? 600 : 400}
        fill={isActive ? 'var(--bg-primary)' : color}
      >
        {lifecycleStateNames[state]}
      </text>
    </g>
  {/each}
</svg>

<style>
  .state-graph {
    display: block;
  }

  .state-node {
    transition: transform 0.2s ease;
  }

  .state-node.interactive {
    cursor: pointer;
  }

  .state-node.interactive:hover circle {
    filter: brightness(1.2);
  }

  .state-node.active circle {
    filter: drop-shadow(0 0 8px currentColor);
  }
</style>
