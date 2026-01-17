<script lang="ts">
  import { onMount } from 'svelte';
  import TimewaveViz from '../lib/viz/TimewaveViz.svelte';

  interface WaveState {
    id: number;
    position: number;
    speed: number;
    label: string;
    color: string;
  }

  let waves = $state<WaveState[]>([
    { id: 0, position: 0, speed: 1.0, label: 'Present', color: 'var(--wave-normal)' },
    { id: 1, position: 0, speed: 0.5, label: 'Slow', color: 'var(--wave-slow)' },
    { id: 2, position: 0, speed: 2.0, label: 'Fast', color: 'var(--wave-fast)' },
  ]);

  let playing = $state(false);
  let tick = $state(0);

  function tickWaves() {
    waves = waves.map(w => ({
      ...w,
      position: Math.min(20, w.position + w.speed)
    }));
    tick++;
  }

  function reset() {
    waves = waves.map(w => ({ ...w, position: 0 }));
    tick = 0;
    playing = false;
  }

  function togglePlay() {
    playing = !playing;
  }

  onMount(() => {
    const interval = setInterval(() => {
      if (playing && waves.some(w => w.position < 20)) {
        tickWaves();
      } else if (playing) {
        playing = false;
      }
    }, 500);

    return () => clearInterval(interval);
  });

  // What each wave "sees" for an entity that spawned at t=5
  const entitySpawnTick = 5;
  const waveViews = $derived(waves.map(w => ({
    ...w,
    seesEntity: w.position >= entitySpawnTick,
    entityState: w.position >= entitySpawnTick ? 'Born' : 'Unborn',
  })));
</script>

<section class="timewaves-section">
  <div class="content-wrapper">
    <h2>Timewaves: Multiple Observers</h2>

    <p>
      A timewave is an observer moving through time. The "present" wave moves at normal speed,
      but you can create waves that move slower, faster, or even backward.
    </p>

    <div class="viz-container">
      <svg width="700" height="200">
        <!-- Timeline axis -->
        <line x1="50" y1="100" x2="650" y2="100" stroke="var(--text-muted)" stroke-width="2" />

        <!-- Tick marks -->
        {#each Array(21) as _, i}
          <line x1={50 + i * 30} y1="95" x2={50 + i * 30} y2="105" stroke="var(--text-muted)" />
          {#if i % 5 === 0}
            <text x={50 + i * 30} y="125" text-anchor="middle" font-size="10" fill="var(--text-muted)">
              t={i}
            </text>
          {/if}
        {/each}

        <!-- Entity at t=5 -->
        <g transform="translate({50 + entitySpawnTick * 30}, 100)">
          <circle r="10" fill="var(--state-born)" />
          <text y="30" text-anchor="middle" font-size="10" fill="var(--text-secondary)">
            Entity (t=5)
          </text>
        </g>

        <!-- Waves -->
        {#each waves as wave}
          <TimewaveViz
            id={wave.id}
            position={wave.position}
            speed={wave.speed}
            minTick={0}
            maxTick={20}
            width={700}
            height={200}
            color={wave.color}
            label={wave.label}
          />
        {/each}
      </svg>

      <div class="controls">
        <button onclick={togglePlay} class:primary={playing}>
          {playing ? 'Pause' : 'Play'}
        </button>
        <button onclick={tickWaves} disabled={playing}>Step</button>
        <button onclick={reset}>Reset</button>
        <span class="tick-display">Tick: {tick}</span>
      </div>

      <div class="wave-views">
        <h4>What each wave sees:</h4>
        <div class="view-cards">
          {#each waveViews as view}
            <div class="view-card" style="border-color: {view.color}">
              <div class="view-header" style="color: {view.color}">{view.label}</div>
              <div class="view-position">Position: t={view.position.toFixed(1)}</div>
              <div class="view-entity" class:visible={view.seesEntity}>
                Entity: {view.entityState}
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <div class="insight">
      <div class="insight-label">Key Insight</div>
      <p>
        Same timeline, different views. Each wave sees entities based on its own position
        in time. A slow wave might still see an entity as Unborn while the fast wave
        already sees it as Born.
      </p>
    </div>

    <div class="use-cases">
      <h3>Use Cases</h3>
      <div class="use-case-grid">
        <div class="use-case">
          <h4>Present (1x)</h4>
          <p>Normal gameplay simulation</p>
        </div>
        <div class="use-case">
          <h4>Slow (0.5x)</h4>
          <p>Slow-motion replay, debugging</p>
        </div>
        <div class="use-case">
          <h4>Fast (2x+)</h4>
          <p>Fast-forward, prediction</p>
        </div>
        <div class="use-case">
          <h4>Paused (0x)</h4>
          <p>Frozen observation point</p>
        </div>
      </div>
    </div>
  </div>
</section>

<style>
  .timewaves-section {
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

  .controls {
    display: flex;
    justify-content: center;
    gap: 1rem;
    align-items: center;
    margin-bottom: 2rem;
  }

  .tick-display {
    font-family: var(--font-mono);
    color: var(--text-secondary);
    min-width: 80px;
  }

  .wave-views h4 {
    text-align: center;
    margin-bottom: 1rem;
  }

  .view-cards {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
  }

  .view-card {
    background: var(--bg-tertiary);
    border: 2px solid;
    border-radius: 8px;
    padding: 1rem;
    text-align: center;
  }

  .view-header {
    font-weight: 600;
    margin-bottom: 0.5rem;
  }

  .view-position {
    font-family: var(--font-mono);
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .view-entity {
    margin-top: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-muted);
    transition: color 0.3s ease;
  }

  .view-entity.visible {
    color: var(--state-born);
    font-weight: 500;
  }

  .use-cases {
    margin-top: 3rem;
  }

  .use-case-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
  }

  .use-case {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 6px;
    padding: 1rem;
  }

  .use-case h4 {
    color: var(--color-present);
    font-size: 0.9rem;
    margin-bottom: 0.25rem;
  }

  .use-case p {
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin: 0;
  }
</style>
