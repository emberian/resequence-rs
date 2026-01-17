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
  import { showToast } from '../lib/components/Toast.svelte';
  import { Tooltip, StatsPanel, WaveformView } from '../lib/components';
  import { getEntityName, getWaveInfo } from '../lib/utils/entityNames';

  // Engine instance
  let engine = $state(new Engine<SimpleState>());
  let playing = $state(false);
  let selectedEntity = $state<EntityId | null>(null);
  let chronoportTarget = $state(0);

  // New features
  let playbackSpeed = $state(1);
  let breakpointTick = $state<number | null>(null);
  let history = $state<string[]>([]);
  let historyIndex = $state(-1);
  let showAdvanced = $state(false);
  let waveHistory = $state<{ id: number; speed: number; color: string; history: { tick: number; position: number }[] }[]>([]);

  // Entity counter for naming
  let entityCounter = $state(0);

  // Initialize with demo entities
  onMount(() => {
    const e1 = engine.spawn(new SimpleState({ name: getEntityName(entityCounter++) }));
    const e2 = engine.spawnAt(new SimpleState({ name: getEntityName(entityCounter++) }), 3);
    selectedEntity = e1;
    saveState();
    engine = engine;
    updateWaveHistory();
  });

  // Animation loop with variable speed
  onMount(() => {
    let lastTime = 0;
    let accumulated = 0;

    const animate = (time: number) => {
      if (playing) {
        const delta = time - lastTime;
        accumulated += delta * playbackSpeed;

        // Tick every 500ms base rate
        while (accumulated >= 500) {
          accumulated -= 500;
          engine.tick();

          // Check breakpoint
          if (breakpointTick !== null && engine.currentTick >= breakpointTick) {
            playing = false;
            showToast(`Breakpoint reached at t=${engine.currentTick}`, 'info');
            breakpointTick = null;
          }

          updateWaveHistory();
          engine = engine;
        }
      }
      lastTime = time;
      requestAnimationFrame(animate);
    };

    requestAnimationFrame(animate);
  });

  function updateWaveHistory() {
    const waves = engine.getAllTimewaves();
    const tick = engine.currentTick;

    waveHistory = waves.map(wave => {
      const info = getWaveInfo(wave.id, wave.speed);
      const existing = waveHistory.find(w => w.id === wave.id);
      const history = existing ? [...existing.history] : [];

      // Add current position
      history.push({ tick, position: wave.position });

      // Keep only last 50 points
      if (history.length > 50) history.shift();

      return {
        id: wave.id,
        speed: wave.speed,
        color: info.color,
        history,
      };
    });
  }

  // History management for undo/redo
  function saveState() {
    const state = JSON.stringify({
      timeline: serializeEngine(),
      entityCounter,
    });

    // Truncate forward history on new action
    if (historyIndex < history.length - 1) {
      history = history.slice(0, historyIndex + 1);
    }

    history = [...history, state];
    historyIndex = history.length - 1;

    // Limit history size
    if (history.length > 50) {
      history = history.slice(-50);
      historyIndex = history.length - 1;
    }
  }

  function serializeEngine() {
    const entities = Array.from(engine.timeline.allEntities());
    const waves = engine.getAllTimewaves();
    return { entities: entities.map(e => e.serialize()), waves, tick: engine.currentTick };
  }

  function undo() {
    if (historyIndex > 0) {
      historyIndex--;
      restoreState(history[historyIndex]);
      showToast('Undo', 'info', 1500);
    }
  }

  function redo() {
    if (historyIndex < history.length - 1) {
      historyIndex++;
      restoreState(history[historyIndex]);
      showToast('Redo', 'info', 1500);
    }
  }

  function restoreState(state: string) {
    try {
      const data = JSON.parse(state);
      entityCounter = data.entityCounter;
      // Note: Full state restoration would require engine serialization support
      // For now, this is a simplified version
      engine = engine;
    } catch (e) {
      showToast('Failed to restore state', 'error');
    }
  }

  // LocalStorage save/load
  function saveToStorage() {
    try {
      const state = {
        engine: serializeEngine(),
        entityCounter,
        timestamp: Date.now(),
      };
      localStorage.setItem('resequence-playground', JSON.stringify(state));
      showToast('Simulation saved', 'success');
    } catch (e) {
      showToast('Failed to save: ' + (e as Error).message, 'error');
    }
  }

  function loadFromStorage() {
    try {
      const saved = localStorage.getItem('resequence-playground');
      if (!saved) {
        showToast('No saved simulation found', 'warning');
        return;
      }

      const data = JSON.parse(saved);
      const timestamp = new Date(data.timestamp).toLocaleString();
      showToast(`Loaded simulation from ${timestamp}`, 'success');

      // For a full implementation, we'd deserialize the engine state
      // This is a simplified notification
    } catch (e) {
      showToast('Failed to load: ' + (e as Error).message, 'error');
    }
  }

  function spawnEntity() {
    const name = getEntityName(entityCounter++);
    const id = engine.spawn(new SimpleState({ name, health: 100 }));
    selectedEntity = id;
    saveState();
    engine = engine;
    showToast(`Spawned ${name}`, 'success', 2000);
  }

  function addTimewave() {
    const speed = playbackSpeed;
    engine.addTimewave(speed);
    saveState();
    updateWaveHistory();
    engine = engine;
    const info = getWaveInfo(engine.getAllTimewaves().length - 1, speed);
    showToast(`Added ${info.name} wave (${speed}x)`, 'success', 2000);
  }

  function doChronoport() {
    if (selectedEntity === null) return;

    try {
      const oldEntity = engine.timeline.get(selectedEntity);
      const oldName = (oldEntity?.getEventAt(engine.currentTick)?.state as SimpleState)?.data.name || 'Entity';

      const newId = engine.chronoport(selectedEntity, chronoportTarget);
      selectedEntity = newId;
      saveState();
      engine = engine;
      showToast(`${oldName} chronoported to t=${chronoportTarget}`, 'success');
    } catch (e) {
      showToast((e as Error).message, 'error');
    }
  }

  function destroySelected() {
    if (selectedEntity === null) return;

    const entity = engine.timeline.get(selectedEntity);
    const name = (entity?.getEventAt(engine.currentTick)?.state as SimpleState)?.data.name || 'Entity';

    engine.destroy(selectedEntity);
    saveState();
    engine = engine;
    showToast(`${name} destroyed`, 'warning', 2000);
  }

  function reset() {
    engine = new Engine<SimpleState>();
    entityCounter = 0;
    const name = getEntityName(entityCounter++);
    engine.spawn(new SimpleState({ name, health: 100 }));
    selectedEntity = 0;
    playing = false;
    history = [];
    historyIndex = -1;
    waveHistory = [];
    saveState();
    showToast('Simulation reset', 'info');
  }

  function setBreakpoint() {
    const tick = prompt('Pause at tick:', String(engine.currentTick + 10));
    if (tick !== null) {
      const t = parseInt(tick);
      if (!isNaN(t) && t > engine.currentTick) {
        breakpointTick = t;
        showToast(`Breakpoint set at t=${t}`, 'info');
      }
    }
  }

  // Derived state
  const entities = $derived(Array.from(engine.timeline.allEntities()));
  const waves = $derived(engine.getAllTimewaves());
  const currentTick = $derived(engine.currentTick);
  const canUndo = $derived(historyIndex > 0);
  const canRedo = $derived(historyIndex < history.length - 1);

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
    <div class="section-header">
      <h2>Playground</h2>
      <div class="header-actions">
        <Tooltip text="Save simulation to browser">
          <button onclick={saveToStorage}>Save</button>
        </Tooltip>
        <Tooltip text="Load saved simulation">
          <button onclick={loadFromStorage}>Load</button>
        </Tooltip>
        <Tooltip text="Toggle advanced features">
          <button onclick={() => showAdvanced = !showAdvanced} class:active={showAdvanced}>
            Advanced
          </button>
        </Tooltip>
      </div>
    </div>
    <p>
      Experiment freely with the simulation engine. Create entities, add waves,
      chronoport through time, and watch how everything interacts.
    </p>

    <div class="playground-layout" class:show-advanced={showAdvanced}>
      <!-- Timeline visualization -->
      <div class="timeline-panel">
        <div class="panel-header">
          <h3>Timeline</h3>
          <div class="undo-redo">
            <Tooltip text="Undo (Ctrl+Z)">
              <button onclick={undo} disabled={!canUndo} class="icon-btn">↶</button>
            </Tooltip>
            <Tooltip text="Redo (Ctrl+Y)">
              <button onclick={redo} disabled={!canRedo} class="icon-btn">↷</button>
            </Tooltip>
          </div>
        </div>

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

          <!-- Breakpoint indicator -->
          {#if breakpointTick !== null && breakpointTick <= 20}
            <g class="breakpoint">
              <line
                x1={50 + breakpointTick * 30}
                y1="20"
                x2={50 + breakpointTick * 30}
                y2="240"
                stroke="var(--color-paradox)"
                stroke-width="2"
                stroke-dasharray="6,4"
                opacity="0.6"
              />
              <circle
                cx={50 + breakpointTick * 30}
                cy="15"
                r="6"
                fill="var(--color-paradox)"
              />
            </g>
          {/if}

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
                    class="event-marker"
                  />
                {/if}
              {/each}

              <!-- Current position marker -->
              <g
                class="entity-marker interactive"
                class:selected={selectedEntity === entity.id}
                onclick={() => selectedEntity = entity.id}
                onkeydown={(e) => e.key === 'Enter' && (selectedEntity = entity.id)}
                role="button"
                tabindex="0"
              >
                <circle
                  cx={50 + Math.min(20, currentTick) * 30}
                  cy={y}
                  r={selectedEntity === entity.id ? 12 : 8}
                  fill={color}
                  stroke={selectedEntity === entity.id ? 'white' : 'none'}
                  stroke-width="2"
                />
              </g>

              <!-- Label with proper name -->
              <text x="20" y={y + 4} font-size="9" fill="var(--text-secondary)">
                {display.state?.data.name || `E${entity.id}`}
              </text>
            {/if}
          {/each}

          <!-- Timewaves -->
          {#each waves as wave}
            {@const x = 50 + Math.min(20, wave.position) * 30}
            {@const info = getWaveInfo(wave.id, wave.speed)}

            <g class="wave-cursor">
              <line
                x1={x}
                y1="20"
                x2={x}
                y2="240"
                stroke={info.color}
                stroke-width="2"
                opacity="0.7"
              />
              <text x={x} y="15" text-anchor="middle" font-size="9" fill={info.color}>
                {info.name} ({wave.speed}x)
              </text>
            </g>
          {/each}
        </svg>

        <div class="playback-controls">
          <div class="control-group">
            <Tooltip text={playing ? 'Pause (Space)' : 'Play (Space)'}>
              <button onclick={() => playing = !playing} class:primary={playing}>
                {playing ? '⏸ Pause' : '▶ Play'}
              </button>
            </Tooltip>
            <Tooltip text="Single step">
              <button onclick={() => { engine.tick(); updateWaveHistory(); engine = engine; }} disabled={playing}>
                Step
              </button>
            </Tooltip>
            <Tooltip text="Reset simulation">
              <button onclick={reset}>Reset</button>
            </Tooltip>
          </div>

          <div class="speed-control">
            <label>
              <span>Speed:</span>
              <input
                type="range"
                min="0.25"
                max="3"
                step="0.25"
                bind:value={playbackSpeed}
              />
              <span class="speed-value">{playbackSpeed}x</span>
            </label>
          </div>

          <span class="tick-display">t = {currentTick}</span>
        </div>
      </div>

      <!-- Control panel -->
      <div class="control-panel">
        <div class="panel-section">
          <h4>Entities</h4>
          <button onclick={spawnEntity} class="primary">+ Spawn Entity</button>

          {#if selectedEntity !== null}
            {@const display = getEntityDisplay(selectedEntity)}
            {#if display}
              <div class="selected-info">
                <div class="info-row">
                  <span>Selected:</span>
                  <span class="value entity-name">{display.state?.data.name || `E${display.id}`}</span>
                </div>
                <div class="info-row">
                  <span>ID / NameID:</span>
                  <span class="value">{display.id} / {display.nameId}</span>
                </div>
                <div class="info-row">
                  <span>State:</span>
                  <span class="value lifecycle-badge" style="--badge-color: {lifecycleStateColors[display.lifecycle]}">
                    {lifecycleStateNames[display.lifecycle]}
                  </span>
                </div>

                <div class="same-name">
                  <span>Temporal group:</span>
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
              class="primary chronoport-btn"
            >
              ⚡ Chronoport
            </button>
          </div>
        </div>

        <div class="panel-section">
          <h4>Timewaves ({waves.length})</h4>
          <button onclick={addTimewave}>+ Add Wave</button>

          <div class="wave-list">
            {#each waves as wave}
              {@const info = getWaveInfo(wave.id, wave.speed)}
              <div class="wave-item">
                <span class="wave-color" style="background: {info.color}"></span>
                <span class="wave-name">{info.name}</span>
                <span class="speed">{wave.speed}x</span>
                <span class="position">t={wave.position.toFixed(1)}</span>
              </div>
            {/each}
          </div>
        </div>

        <div class="panel-section">
          <h4>Actions</h4>
          <div class="action-buttons">
            <button onclick={destroySelected} disabled={selectedEntity === null}>
              Destroy Selected
            </button>
            <button onclick={setBreakpoint}>
              Set Breakpoint
            </button>
          </div>
        </div>
      </div>

      <!-- Advanced panel (stats & waveform) -->
      {#if showAdvanced}
        <div class="advanced-panel">
          <StatsPanel {engine} {currentTick} />

          <WaveformView
            waves={waveHistory}
            {currentTick}
            width={380}
            height={160}
          />
        </div>
      {/if}
    </div>

    <div class="insight">
      <div class="insight-label">Try This</div>
      <p>
        Spawn an entity, wait a few ticks, then chronoport it to tick 2. Watch it
        appear in the past as a Prebirth entity, then transition to Born when the
        waves reach it. Check the temporal group to see both copies linked together.
      </p>
    </div>

    <div class="keyboard-hint">
      Press <kbd>?</kbd> for keyboard shortcuts
    </div>
  </div>
</section>

<style>
  .playground-section {
    padding: 4rem 2rem;
    background: linear-gradient(180deg, var(--bg-primary), var(--bg-secondary));
  }

  .content-wrapper {
    max-width: 1200px;
    margin: 0 auto;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .header-actions button.active {
    background: var(--color-present);
    border-color: var(--color-present);
    color: var(--bg-primary);
  }

  .playground-layout {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 1.5rem;
    margin: 2rem 0;
  }

  .playground-layout.show-advanced {
    grid-template-columns: 1fr 300px 400px;
  }

  .timeline-panel {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .panel-header h3 {
    margin: 0;
  }

  .undo-redo {
    display: flex;
    gap: 0.25rem;
  }

  .icon-btn {
    width: 32px;
    height: 32px;
    padding: 0;
    font-size: 1.1rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .timeline-panel svg {
    display: block;
    margin-bottom: 1rem;
  }

  .entity-marker circle {
    transition: r 0.2s ease, filter 0.2s ease;
  }

  .entity-marker:hover circle,
  .entity-marker.selected circle {
    filter: drop-shadow(0 0 8px currentColor);
  }

  .event-marker {
    opacity: 0.8;
  }

  .wave-cursor line {
    filter: drop-shadow(0 0 4px currentColor);
  }

  .breakpoint line {
    animation: pulse-opacity 1s ease-in-out infinite;
  }

  @keyframes pulse-opacity {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 0.8; }
  }

  .playback-controls {
    display: flex;
    gap: 1rem;
    align-items: center;
    padding-top: 1rem;
    border-top: 1px solid var(--bg-tertiary);
    flex-wrap: wrap;
  }

  .control-group {
    display: flex;
    gap: 0.5rem;
  }

  .speed-control {
    flex: 1;
    min-width: 150px;
  }

  .speed-control label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .speed-control input[type="range"] {
    flex: 1;
    min-width: 80px;
  }

  .speed-value {
    font-family: var(--font-mono);
    color: var(--color-present);
    min-width: 3ch;
  }

  .tick-display {
    font-family: var(--font-mono);
    color: var(--color-present);
    font-weight: 600;
    font-size: 1.1rem;
    margin-left: auto;
    padding: 0.25rem 0.75rem;
    background: rgba(88, 166, 255, 0.1);
    border-radius: 4px;
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
    font-size: 0.8rem;
    color: var(--text-muted);
    margin-bottom: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .panel-section button {
    width: 100%;
    margin-bottom: 0.5rem;
  }

  .panel-section button:last-child {
    margin-bottom: 0;
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
    margin-bottom: 0.35rem;
  }

  .info-row span:first-child {
    color: var(--text-muted);
  }

  .info-row .value {
    font-family: var(--font-mono);
    color: var(--text-primary);
  }

  .entity-name {
    color: var(--color-present) !important;
    font-weight: 500;
  }

  .lifecycle-badge {
    padding: 0.15rem 0.5rem;
    background: var(--badge-color);
    border-radius: 4px;
    font-size: 0.75rem;
  }

  .same-name {
    margin-top: 0.5rem;
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .same-name .ids {
    font-family: var(--font-mono);
    color: var(--color-past);
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

  .chronoport-btn {
    background: var(--color-chronoport) !important;
    border-color: var(--color-chronoport) !important;
  }

  .wave-list {
    margin-top: 0.75rem;
    max-height: 140px;
    overflow-y: auto;
  }

  .wave-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--bg-tertiary);
    border-radius: 4px;
    margin-bottom: 0.25rem;
    font-size: 0.8rem;
  }

  .wave-color {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .wave-name {
    flex: 1;
    color: var(--text-primary);
  }

  .wave-item .speed {
    color: var(--color-present);
    font-family: var(--font-mono);
  }

  .wave-item .position {
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .action-buttons {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .advanced-panel {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .keyboard-hint {
    text-align: center;
    font-size: 0.8rem;
    color: var(--text-muted);
    margin-top: 2rem;
  }

  .keyboard-hint kbd {
    padding: 0.2rem 0.5rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--text-muted);
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 0.75rem;
    box-shadow: 0 2px 0 var(--text-muted);
  }

  @media (max-width: 1200px) {
    .playground-layout.show-advanced {
      grid-template-columns: 1fr 300px;
    }

    .advanced-panel {
      grid-column: 1 / -1;
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 1rem;
    }
  }

  @media (max-width: 900px) {
    .playground-layout {
      grid-template-columns: 1fr;
    }

    .playground-layout.show-advanced {
      grid-template-columns: 1fr;
    }

    .control-panel {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
    }

    .advanced-panel {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 600px) {
    .control-panel {
      grid-template-columns: 1fr;
    }

    .section-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }
  }
</style>
