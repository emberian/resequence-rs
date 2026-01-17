<script lang="ts">
  import type { Engine, EntityState } from '../engine';

  interface Props {
    engine: Engine<EntityState>;
    currentTick: number;
  }

  let { engine, currentTick }: Props = $props();

  const stats = $derived.by(() => {
    const entities = Array.from(engine.timeline.allEntities());
    const waves = engine.getAllTimewaves();

    let totalEvents = 0;
    let bornCount = 0;
    let prebirthCount = 0;
    let deadCount = 0;
    let chronoportCount = 0;

    for (const entity of entities) {
      const events = entity.allEvents();
      totalEvents += events.length;

      const currentEvent = entity.getEventAt(currentTick);
      if (currentEvent) {
        switch (currentEvent.lifecycle) {
          case 6: bornCount++; break;    // Born
          case 3: prebirthCount++; break; // Prebirth
          case 4: deadCount++; break;     // Dead
          case 5: chronoportCount++; break; // Chronoporting
        }
      }
    }

    return {
      entityCount: entities.length,
      waveCount: waves.length,
      totalEvents,
      bornCount,
      prebirthCount,
      deadCount,
      chronoportCount,
      tick: currentTick,
    };
  });
</script>

<div class="stats-panel">
  <div class="stats-header">
    <span class="stats-title">Statistics</span>
    <span class="stats-tick">t = {stats.tick}</span>
  </div>

  <div class="stats-grid">
    <div class="stat">
      <span class="stat-value">{stats.entityCount}</span>
      <span class="stat-label">Entities</span>
    </div>
    <div class="stat">
      <span class="stat-value">{stats.waveCount}</span>
      <span class="stat-label">Waves</span>
    </div>
    <div class="stat">
      <span class="stat-value">{stats.totalEvents}</span>
      <span class="stat-label">Events</span>
    </div>
  </div>

  <div class="lifecycle-breakdown">
    <div class="lifecycle-row">
      <span class="lifecycle-dot" style="background: var(--state-born)"></span>
      <span class="lifecycle-name">Born</span>
      <span class="lifecycle-count">{stats.bornCount}</span>
    </div>
    <div class="lifecycle-row">
      <span class="lifecycle-dot" style="background: var(--state-prebirth)"></span>
      <span class="lifecycle-name">Prebirth</span>
      <span class="lifecycle-count">{stats.prebirthCount}</span>
    </div>
    <div class="lifecycle-row">
      <span class="lifecycle-dot" style="background: var(--state-chronoporting)"></span>
      <span class="lifecycle-name">Chronoporting</span>
      <span class="lifecycle-count">{stats.chronoportCount}</span>
    </div>
    <div class="lifecycle-row">
      <span class="lifecycle-dot" style="background: var(--state-dead)"></span>
      <span class="lifecycle-name">Dead</span>
      <span class="lifecycle-count">{stats.deadCount}</span>
    </div>
  </div>
</div>

<style>
  .stats-panel {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1rem;
  }

  .stats-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--bg-tertiary);
  }

  .stats-title {
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .stats-tick {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--color-present);
    font-weight: 600;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .stat {
    text-align: center;
    padding: 0.75rem 0.5rem;
    background: var(--bg-tertiary);
    border-radius: 6px;
  }

  .stat-value {
    display: block;
    font-family: var(--font-mono);
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
  }

  .stat-label {
    display: block;
    font-size: 0.7rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-top: 0.35rem;
  }

  .lifecycle-breakdown {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .lifecycle-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.8rem;
  }

  .lifecycle-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .lifecycle-name {
    flex: 1;
    color: var(--text-secondary);
  }

  .lifecycle-count {
    font-family: var(--font-mono);
    color: var(--text-primary);
    font-weight: 500;
  }
</style>
