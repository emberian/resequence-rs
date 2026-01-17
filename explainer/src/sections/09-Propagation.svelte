<script lang="ts">
  import { onMount } from 'svelte';

  interface Wave {
    id: number;
    position: number;
    speed: number;
    color: string;
    label: string;
    views: Map<number, number>; // entityId -> eventTick
  }

  let tick = $state(0);
  let playing = $state(false);
  let propagationHappened = $state(false);

  // Two waves: slow and fast
  let waves = $state<Wave[]>([
    { id: 0, position: 5, speed: 0.5, color: 'var(--wave-slow)', label: 'Slow', views: new Map([[0, 0]]) },
    { id: 1, position: 3, speed: 2.0, color: 'var(--wave-fast)', label: 'Fast', views: new Map([[0, 0]]) },
  ]);

  // Entity with events at t=0 and t=4
  const entityEvents = [
    { tick: 0, label: 'spawn' },
    { tick: 4, label: 'move' },
  ];

  function tickSimulation() {
    tick++;

    // Store previous positions
    const prevPositions = waves.map(w => w.position);

    // Advance waves
    waves = waves.map(w => ({
      ...w,
      position: Math.min(15, w.position + w.speed)
    }));

    // Sort by position
    const sorted = [...waves].sort((a, b) => a.position - b.position);

    // Update views (binary search for event at position)
    for (const wave of sorted) {
      for (const event of entityEvents) {
        if (event.tick <= wave.position) {
          wave.views.set(0, event.tick);
        }
      }
    }

    // Check for propagation (fast overtakes slow)
    for (let i = 1; i < sorted.length; i++) {
      const slowWave = sorted[i - 1];
      const fastWave = sorted[i];

      const slowIdx = waves.findIndex(w => w.id === slowWave.id);
      const fastIdx = waves.findIndex(w => w.id === fastWave.id);

      const prevSlowPos = prevPositions[slowIdx];
      const prevFastPos = prevPositions[fastIdx];

      // Did fast wave cross slow wave?
      if (prevFastPos <= prevSlowPos && fastWave.position > slowWave.position) {
        // Propagation!
        propagationHappened = true;

        // Inherit views
        for (const [entityId, eventTick] of slowWave.views) {
          const currentView = fastWave.views.get(entityId) ?? -1;
          if (eventTick > currentView) {
            fastWave.views.set(entityId, eventTick);
          }
        }
      }
    }

    waves = waves;
  }

  function reset() {
    tick = 0;
    playing = false;
    propagationHappened = false;
    waves = [
      { id: 0, position: 5, speed: 0.5, color: 'var(--wave-slow)', label: 'Slow', views: new Map([[0, 0]]) },
      { id: 1, position: 3, speed: 2.0, color: 'var(--wave-fast)', label: 'Fast', views: new Map([[0, 0]]) },
    ];
  }

  function togglePlay() {
    playing = !playing;
  }

  onMount(() => {
    const interval = setInterval(() => {
      if (playing && waves.every(w => w.position < 15)) {
        tickSimulation();
      } else if (playing) {
        playing = false;
      }
    }, 800);

    return () => clearInterval(interval);
  });

  const sortedWaves = $derived([...waves].sort((a, b) => a.position - b.position));
</script>

<section class="propagation-section">
  <div class="content-wrapper">
    <h2>Wave Propagation Deep Dive</h2>

    <p>
      When a faster wave overtakes a slower wave, it inherits the slower wave's view
      of entities. This maintains causal consistency—you can't un-see something
      that a slower observer already saw.
    </p>

    <div class="viz-container">
      <svg width="700" height="220">
        <!-- Timeline axis -->
        <line x1="50" y1="120" x2="650" y2="120" stroke="var(--text-muted)" stroke-width="2" />

        <!-- Tick marks -->
        {#each Array(16) as _, i}
          <line x1={50 + i * 40} y1="115" x2={50 + i * 40} y2="125" stroke="var(--text-muted)" />
          <text x={50 + i * 40} y="145" text-anchor="middle" font-size="10" fill="var(--text-muted)">
            {i}
          </text>
        {/each}

        <!-- Entity events -->
        {#each entityEvents as event}
          <g transform="translate({50 + event.tick * 40}, 120)">
            <circle r="8" fill="var(--state-born)" />
            <text y="-15" text-anchor="middle" font-size="9" fill="var(--text-secondary)">
              {event.label}
            </text>
          </g>
        {/each}

        <!-- Waves -->
        {#each sortedWaves as wave, i}
          {@const x = 50 + wave.position * 40}

          <g class="wave-group">
            <!-- Wave line -->
            <line
              x1={x}
              y1="50"
              x2={x}
              y2="110"
              stroke={wave.color}
              stroke-width="4"
            />
            <polygon
              points="{x},{110} {x-6},{100} {x+6},{100}"
              fill={wave.color}
            />

            <!-- Label -->
            <text x={x} y="40" text-anchor="middle" font-size="11" fill={wave.color} font-weight="600">
              {wave.label} ({wave.speed}x)
            </text>

            <!-- View cache display -->
            <rect
              x={x - 35}
              y="160"
              width="70"
              height="40"
              fill="var(--bg-tertiary)"
              stroke={wave.color}
              stroke-width="1"
              rx="4"
            />
            <text x={x} y="175" text-anchor="middle" font-size="9" fill={wave.color}>
              View Cache
            </text>
            <text x={x} y="190" text-anchor="middle" font-size="10" fill="var(--text-primary)">
              E0: t={wave.views.get(0)}
            </text>
          </g>
        {/each}

        <!-- Propagation indicator -->
        {#if propagationHappened}
          <g class="propagation-indicator">
            <path
              d="M{50 + sortedWaves[0].position * 40 + 40},80 L{50 + sortedWaves[1].position * 40 - 40},80"
              fill="none"
              stroke="var(--color-present)"
              stroke-width="2"
              marker-end="url(#arrow)"
            />
            <text
              x={(50 + sortedWaves[0].position * 40 + 50 + sortedWaves[1].position * 40) / 2}
              y="70"
              text-anchor="middle"
              font-size="10"
              fill="var(--color-present)"
            >
              inherited!
            </text>
          </g>
        {/if}

        <defs>
          <marker id="arrow" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-present)" />
          </marker>
        </defs>
      </svg>

      <div class="controls">
        <button onclick={togglePlay} class:primary={playing}>
          {playing ? 'Pause' : 'Play'}
        </button>
        <button onclick={tickSimulation} disabled={playing}>Step</button>
        <button onclick={reset}>Reset</button>
        <span class="tick-display">Tick: {tick}</span>
      </div>

      {#if propagationHappened}
        <div class="propagation-alert">
          Propagation occurred! Fast wave inherited Slow wave's view.
        </div>
      {/if}
    </div>

    <div class="scenario">
      <h4>Watch what happens:</h4>
      <ol>
        <li>Slow wave (0.5x) starts ahead at t=5, Fast wave (2x) starts behind at t=3</li>
        <li>Fast wave catches up due to higher speed</li>
        <li>When Fast passes Slow, it inherits Slow's view of the entity</li>
        <li>Both waves now see the same event (consistency maintained)</li>
      </ol>
    </div>

    <div class="insight">
      <div class="insight-label">Key Insight</div>
      <p>
        Propagation prevents causality violations. If a slower wave has already observed
        an entity change, faster waves that pass it must also see that change. This
        ensures all observers eventually agree on what happened.
      </p>
    </div>
  </div>
</section>

<style>
  .propagation-section {
    padding: 4rem 2rem;
  }

  .content-wrapper {
    max-width: 900px;
    margin: 0 auto;
  }

  .viz-container {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 2rem;
    margin: 2rem 0;
  }

  .viz-container svg {
    display: block;
    margin: 0 auto 1.5rem;
  }

  .wave-group line {
    filter: drop-shadow(0 0 4px currentColor);
    transition: all 0.3s ease;
  }

  .controls {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .tick-display {
    font-family: var(--font-mono);
    color: var(--text-secondary);
    min-width: 80px;
  }

  .propagation-alert {
    margin-top: 1.5rem;
    padding: 1rem;
    background: rgba(88, 166, 255, 0.15);
    border: 1px solid var(--color-present);
    border-radius: 6px;
    text-align: center;
    color: var(--color-present);
    font-weight: 500;
    animation: pulse 1s ease-in-out;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
  }

  .scenario {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
    margin: 2rem 0;
  }

  .scenario h4 {
    margin-bottom: 1rem;
  }

  .scenario ol {
    margin: 0 0 0 1.5rem;
    color: var(--text-secondary);
  }

  .scenario li {
    margin-bottom: 0.5rem;
  }
</style>
