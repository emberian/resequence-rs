<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Engine,
    SimpleState,
    type EntityId,
    type Tick,
  } from '../lib/engine';

  // Create a demo engine for the teaser
  let engine = $state(new Engine<SimpleState>());
  let entities: EntityId[] = $state([]);
  let animating = $state(true);

  onMount(() => {
    // Setup demo entities
    const e1 = engine.spawnAt(new SimpleState({ name: 'Marine Alpha' }), 0);
    const e2 = engine.spawnAt(new SimpleState({ name: 'Marine Beta' }), 5);
    entities = [e1, e2];

    // Add some additional waves
    engine.addTimewaveAt(0, 0.5); // Slow wave
    engine.addTimewaveAt(0, 2.0); // Fast wave

    // Animation loop
    let frame: number;
    const animate = () => {
      if (animating) {
        engine.tick();
        engine = engine; // Trigger reactivity
      }
      frame = requestAnimationFrame(animate);
    };

    frame = requestAnimationFrame(animate);

    return () => {
      cancelAnimationFrame(frame);
    };
  });

  const waves = $derived(engine.getAllTimewaves());
</script>

<section class="section-intro hook">
  <div class="hook-content">
    <h1>What if you could see the past and future at the same time?</h1>
    <p class="lead">
      An interactive exploration of <strong>resequence</strong>, a time-travel simulation engine
      inspired by <em>Achron</em>, the 2011 RTS where you could send units back in time.
    </p>
  </div>

  <div class="teaser-viz">
    <svg width="700" height="200">
      <!-- Timeline axis -->
      <line x1="50" y1="100" x2="650" y2="100" stroke="var(--text-muted)" stroke-width="2" />

      <!-- Tick marks -->
      {#each Array(21) as _, i}
        <line
          x1={50 + i * 30}
          y1="95"
          x2={50 + i * 30}
          y2="105"
          stroke="var(--text-muted)"
        />
        {#if i % 5 === 0}
          <text x={50 + i * 30} y="125" text-anchor="middle" font-size="10" fill="var(--text-muted)">
            t={i}
          </text>
        {/if}
      {/each}

      <!-- Timewaves -->
      {#each waves as wave, i}
        {@const x = 50 + (wave.position % 20) * 30}
        {@const colors = ['var(--wave-normal)', 'var(--wave-slow)', 'var(--wave-fast)']}
        {@const labels = ['Present', 'Slow', 'Fast']}

        <g class="wave-cursor">
          <line x1={x} y1="40" x2={x} y2="90" stroke={colors[i]} stroke-width="3" />
          <polygon points="{x},{90} {x-5},{80} {x+5},{80}" fill={colors[i]} />
          <text x={x} y="30" text-anchor="middle" font-size="11" fill={colors[i]}>
            {labels[i]}
          </text>
        </g>
      {/each}

      <!-- Entity markers -->
      {#each entities as id, i}
        {@const entity = engine.timeline.get(id)}
        {#if entity}
          {@const x = 50 + entity.createdAt * 30}
          <g class="entity-marker">
            <circle cx={x} cy="100" r="8" fill="var(--state-born)" />
            <text x={x} y="155" text-anchor="middle" font-size="10" fill="var(--text-secondary)">
              Entity {i}
            </text>
          </g>
        {/if}
      {/each}
    </svg>

    <p class="viz-caption">
      Three timewaves moving at different speeds, each observing the same timeline
    </p>
  </div>

  <div class="scroll-hint">
    <span>Scroll to explore</span>
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M12 5v14M19 12l-7 7-7-7" />
    </svg>
  </div>
</section>

<style>
  .hook {
    background: radial-gradient(ellipse at center, var(--bg-secondary) 0%, var(--bg-primary) 70%);
  }

  .hook-content {
    max-width: 800px;
    text-align: center;
    margin-bottom: 3rem;
  }

  .hook-content h1 {
    font-size: 2.5rem;
    margin-bottom: 1.5rem;
  }

  .lead {
    font-size: 1.25rem;
    color: var(--text-secondary);
  }

  .lead strong {
    color: var(--color-present);
  }

  .teaser-viz {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 12px;
    padding: 2rem;
    margin-bottom: 3rem;
  }

  .teaser-viz svg {
    display: block;
    margin: 0 auto;
  }

  .viz-caption {
    text-align: center;
    font-size: 0.875rem;
    color: var(--text-muted);
    margin-top: 1rem;
    margin-bottom: 0;
  }

  .wave-cursor line {
    filter: drop-shadow(0 0 6px currentColor);
  }

  .scroll-hint {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    color: var(--text-muted);
    font-size: 0.875rem;
    animation: bounce 2s infinite;
  }

  @keyframes bounce {
    0%, 20%, 50%, 80%, 100% {
      transform: translateY(0);
    }
    40% {
      transform: translateY(-10px);
    }
    60% {
      transform: translateY(-5px);
    }
  }
</style>
