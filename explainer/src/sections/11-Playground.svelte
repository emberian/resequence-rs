<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Engine,
    SimpleState,
    type EntityId,
    type TimewaveId,
    lifecycleStateNames,
    lifecycleStateColors,
    LifecycleState,
  } from '../lib/engine';

  // Engine instance
  let engine = $state(new Engine<SimpleState>());
  let playing = $state(false);
  let selectedEntity = $state<EntityId | null>(null);
  let chronoportTarget = $state(0);

  // Initialize with demo entities
  onMount(() => {
    const e1 = engine.spawn(new SimpleState({ name: 'Alpha', health: 100 }));
    const e2 = engine.spawnAt(new SimpleState({ name: 'Beta', health: 100 }), 3);
    selectedEntity = e1;
    engine = engine;
  });

  // Animation loop
  onMount(() => {
    const interval = setInterval(() => {
      if (playing) {
        engine.tick();
        engine = engine;
      }
    }, 500);
    return () => clearInterval(interval);
  });

  function spawnEntity() {
    const id = engine.spawn(new SimpleState({
      name: `Entity ${engine.entityCount}`,
      health: 100,
    }));
    selectedEntity = id;
    engine = engine;
  }

  function addTimewave() {
    const speed = parseFloat(prompt('Wave speed (e.g., 0.5, 1.0, 2.0):', '1.0') || '1.0');
    engine.addTimewave(speed);
    engine = engine;
  }

  function doChronoport() {
    if (selectedEntity === null) return;

    try {
      const newId = engine.chronoport(selectedEntity, chronoportTarget);
      selectedEntity = newId;
      engine = engine;
    } catch (e) {
      alert((e as Error).message);
    }
  }

  function destroySelected() {
    if (selectedEntity === null) return;
    engine.destroy(selectedEntity);
    engine = engine;
  }

  function reset() {
    engine = new Engine<SimpleState>();
    engine.spawn(new SimpleState({ name: 'Alpha', health: 100 }));
    selectedEntity = 0;
    playing = false;
  }

  // Derived state
  const entities = $derived(Array.from(engine.timeline.allEntities()));
  const waves = $derived(engine.getAllTimewaves());
  const currentTick = $derived(engine.currentTick);

  // Get entity state for display
  function getEntityDisplay(id: EntityId) {
    const entity = engine.timeline.get(id);
    if (!entity) return null;

    const event = entity.getEventAt(currentTick);
    return {
      id: entity.id,
      nameId: entity.nameId,
      lifecycle: event?.lifecycle ?? LifecycleState.Unborn,
      state: event?.state as SimpleState | null,
      createdAt: entity.createdAt,
      events: entity.allEvents(),
    };
  }
</script>

<section class="playground-section">
  <div class="content-wrapper">
    <h2>Playground</h2>
    <p>
      Experiment freely with the simulation engine. Create entities, add waves,
      chronoport through time, and watch how everything interacts.
    </p>

    <div class="playground-layout">
      <!-- Timeline visualization -->
      <div class="timeline-panel">
        <h3>Timeline</h3>
        <svg width="100%" height="300" viewBox="0 0 700 300">
          <!-- Timeline axis -->
          <line x1="50" y1="250" x2="650" y2="250" stroke="var(--text-muted)" stroke-width="2" />

          <!-- Tick marks -->
          {#each Array(21) as _, i}
            <line x1={50 + i * 30} y1="245" x2={50 + i * 30} y2="255" stroke="var(--text-muted)" />
            {#if i % 5 === 0}
              <text x={50 + i * 30} y="275" text-anchor="middle" font-size="10" fill="var(--text-muted)">
                t={i}
              </text>
            {/if}
          {/each}

          <!-- Entities -->
          {#each entities as entity, idx}
            {@const display = getEntityDisplay(entity.id)}
            {#if display}
              {@const y = 50 + idx * 45}
              {@const color = lifecycleStateColors[display.lifecycle]}

              <!-- Entity timeline bar -->
              <line
                x1={50 + display.createdAt * 30}
                y1={y}
                x2={50 + Math.min(20, currentTick) * 30}
                y2={y}
                stroke={color}
                stroke-width="4"
                opacity="0.5"
              />

              <!-- Event markers -->
              {#each display.events as event}
                {#if event.timestamp <= 20}
                  <circle
                    cx={50 + event.timestamp * 30}
                    cy={y}
                    r="6"
                    fill={lifecycleStateColors[event.lifecycle]}
                  />
                {/if}
              {/each}

              <!-- Current position marker -->
              <circle
                cx={50 + Math.min(20, currentTick) * 30}
                cy={y}
                r={selectedEntity === entity.id ? 12 : 8}
                fill={color}
                stroke={selectedEntity === entity.id ? 'white' : 'none'}
                stroke-width="2"
                class="entity-marker"
                onclick={() => selectedEntity = entity.id}
                onkeydown={(e) => e.key === 'Enter' && (selectedEntity = entity.id)}
                role="button"
                tabindex="0"
              />

              <!-- Label -->
              <text x="20" y={y + 4} font-size="10" fill="var(--text-secondary)">
                E{entity.id}
              </text>
            {/if}
          {/each}

          <!-- Timewaves -->
          {#each waves as wave}
            {@const x = 50 + Math.min(20, wave.position) * 30}
            {@const colors = ['var(--wave-normal)', 'var(--wave-slow)', 'var(--wave-fast)']}
            {@const color = wave.id === 0 ? colors[0] : wave.speed < 1 ? colors[1] : colors[2]}

            <line
              x1={x}
              y1="20"
              x2={x}
              y2="240"
              stroke={color}
              stroke-width="2"
              opacity="0.7"
            />
            <text x={x} y="15" text-anchor="middle" font-size="9" fill={color}>
              W{wave.id} ({wave.speed}x)
            </text>
          {/each}
        </svg>

        <div class="playback-controls">
          <button onclick={() => playing = !playing} class:primary={playing}>
            {playing ? 'Pause' : 'Play'}
          </button>
          <button onclick={() => { engine.tick(); engine = engine; }} disabled={playing}>
            Step
          </button>
          <button onclick={reset}>Reset</button>
          <span class="tick-display">t = {currentTick}</span>
        </div>
      </div>

      <!-- Control panel -->
      <div class="control-panel">
        <div class="panel-section">
          <h4>Entities</h4>
          <button onclick={spawnEntity}>Spawn Entity</button>

          {#if selectedEntity !== null}
            {@const display = getEntityDisplay(selectedEntity)}
            {#if display}
              <div class="selected-info">
                <div class="info-row">
                  <span>Selected:</span>
                  <span class="value">E{display.id}</span>
                </div>
                <div class="info-row">
                  <span>NameId:</span>
                  <span class="value">{display.nameId}</span>
                </div>
                <div class="info-row">
                  <span>State:</span>
                  <span class="value" style="color: {lifecycleStateColors[display.lifecycle]}">
                    {lifecycleStateNames[display.lifecycle]}
                  </span>
                </div>
                {#if display.state}
                  <div class="info-row">
                    <span>Name:</span>
                    <span class="value">{display.state.data.name}</span>
                  </div>
                {/if}

                <div class="same-name">
                  <span>Same-name group:</span>
                  <span class="ids">
                    [{engine.getSameNameEntities(selectedEntity).join(', ')}]
                  </span>
                </div>
              </div>
            {/if}
          {/if}
        </div>

        <div class="panel-section">
          <h4>Chronoport</h4>
          <div class="chronoport-controls">
            <label>
              Target tick:
              <input type="number" min="0" max="100" bind:value={chronoportTarget} />
            </label>
            <button
              onclick={doChronoport}
              disabled={selectedEntity === null}
              class="primary"
            >
              Chronoport
            </button>
          </div>
        </div>

        <div class="panel-section">
          <h4>Timewaves ({waves.length})</h4>
          <button onclick={addTimewave}>Add Wave</button>

          <div class="wave-list">
            {#each waves as wave}
              <div class="wave-item">
                <span>W{wave.id}</span>
                <span class="speed">{wave.speed}x</span>
                <span class="position">t={wave.position.toFixed(1)}</span>
              </div>
            {/each}
          </div>
        </div>

        <div class="panel-section">
          <h4>Actions</h4>
          <button onclick={destroySelected} disabled={selectedEntity === null}>
            Destroy Selected
          </button>
        </div>
      </div>
    </div>

    <div class="insight">
      <div class="insight-label">Try This</div>
      <p>
        Spawn an entity, wait a few ticks, then chronoport it to tick 2. Watch it
        appear in the past as a Prebirth entity, then transition to Born when the
        waves reach it. Check the same-name group to see both copies linked together.
      </p>
    </div>
  </div>
</section>

<style>
  .playground-section {
    padding: 4rem 2rem;
    background: linear-gradient(180deg, var(--bg-primary), var(--bg-secondary));
  }

  .content-wrapper {
    max-width: 1100px;
    margin: 0 auto;
  }

  .playground-layout {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 1.5rem;
    margin: 2rem 0;
  }

  .timeline-panel {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .timeline-panel h3 {
    margin-bottom: 1rem;
  }

  .timeline-panel svg {
    display: block;
    margin-bottom: 1rem;
  }

  .entity-marker {
    cursor: pointer;
    transition: r 0.2s ease;
  }

  .entity-marker:hover {
    filter: brightness(1.2);
  }

  .playback-controls {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    justify-content: center;
    padding-top: 1rem;
    border-top: 1px solid var(--bg-tertiary);
  }

  .tick-display {
    font-family: var(--font-mono);
    color: var(--color-present);
    font-weight: 600;
    margin-left: auto;
  }

  .control-panel {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .panel-section {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1rem;
  }

  .panel-section h4 {
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .panel-section button {
    width: 100%;
    margin-bottom: 0.5rem;
  }

  .selected-info {
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--bg-tertiary);
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    margin-bottom: 0.25rem;
  }

  .info-row span:first-child {
    color: var(--text-muted);
  }

  .info-row .value {
    font-family: var(--font-mono);
    color: var(--text-primary);
  }

  .same-name {
    margin-top: 0.5rem;
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .same-name .ids {
    font-family: var(--font-mono);
    color: var(--color-present);
  }

  .chronoport-controls {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .chronoport-controls label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .chronoport-controls input[type="number"] {
    width: 60px;
    padding: 0.25rem 0.5rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--text-muted);
    border-radius: 4px;
    color: var(--text-primary);
    font-family: var(--font-mono);
  }

  .wave-list {
    margin-top: 0.75rem;
    max-height: 120px;
    overflow-y: auto;
  }

  .wave-item {
    display: flex;
    gap: 0.75rem;
    padding: 0.5rem;
    background: var(--bg-tertiary);
    border-radius: 4px;
    margin-bottom: 0.25rem;
    font-size: 0.8rem;
  }

  .wave-item .speed {
    color: var(--color-present);
  }

  .wave-item .position {
    color: var(--text-muted);
    margin-left: auto;
  }

  @media (max-width: 900px) {
    .playground-layout {
      grid-template-columns: 1fr;
    }

    .control-panel {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 600px) {
    .control-panel {
      grid-template-columns: 1fr;
    }
  }
</style>
