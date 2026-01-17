<script lang="ts">
  let snapshotCount = $state(5);
  let tickCount = $state(100);
  let entitySize = $state(256); // bytes

  const snapshotMemory = $derived(snapshotCount * tickCount * entitySize);
  const eventMemory = $derived(snapshotCount * 10 * entitySize); // ~10 events per entity

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<section class="problem-section">
  <div class="content-wrapper">
    <h2>Why Time-Travel Simulation Is Hard</h2>

    <p>
      In a normal game, state flows forward. You update positions, health, resources—each tick
      replaces the previous state. But what happens when you need to <em>query the past</em>?
    </p>

    <div class="approach-cards">
      <div class="approach-card bad">
        <h3>Naive Approach: Full Snapshots</h3>
        <p>Store a complete copy of every entity at every tick.</p>
        <ul>
          <li>100 ticks × 1000 entities × 256 bytes = <strong>25 MB</strong></li>
          <li>Memory grows linearly with simulation length</li>
          <li>Works, but doesn't scale</li>
        </ul>
      </div>

      <div class="approach-card better">
        <h3>Better: Deltas</h3>
        <p>Store only what changed between ticks.</p>
        <ul>
          <li>Lower memory when entities change rarely</li>
          <li>Reconstructing state requires replaying deltas</li>
          <li>Random access is O(n) in worst case</li>
        </ul>
      </div>
    </div>

    <h3>Interactive: Memory Comparison</h3>

    <div class="viz-container">
      <div class="slider-container">
        <label>Entities:</label>
        <input type="range" min="1" max="100" bind:value={snapshotCount} />
        <span>{snapshotCount}</span>
      </div>

      <div class="slider-container">
        <label>Ticks:</label>
        <input type="range" min="10" max="1000" step="10" bind:value={tickCount} />
        <span>{tickCount}</span>
      </div>

      <div class="memory-comparison">
        <div class="memory-bar">
          <div class="bar-label">Snapshots</div>
          <div class="bar-fill snapshot" style="width: {Math.min(100, snapshotMemory / 10000)}%">
            {formatBytes(snapshotMemory)}
          </div>
        </div>
        <div class="memory-bar">
          <div class="bar-label">Events</div>
          <div class="bar-fill events" style="width: {Math.min(100, eventMemory / 10000)}%">
            {formatBytes(eventMemory)}
          </div>
        </div>
      </div>

      <p class="comparison-note">
        Event-based storage uses ~{Math.round((1 - eventMemory / snapshotMemory) * 100)}% less memory
      </p>
    </div>

    <div class="insight">
      <div class="insight-label">The Challenge</div>
      <p>
        We need O(1) memory growth with simulation length, while maintaining O(log n) random
        access to any point in history. Events give us that.
      </p>
    </div>
  </div>
</section>

<style>
  .problem-section {
    padding: 4rem 2rem;
  }

  .content-wrapper {
    max-width: 900px;
    margin: 0 auto;
  }

  .approach-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    margin: 2rem 0;
  }

  .approach-card {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .approach-card.bad {
    border-left: 3px solid var(--state-dead);
  }

  .approach-card.better {
    border-left: 3px solid var(--color-present);
  }

  .approach-card h3 {
    font-size: 1rem;
    margin-bottom: 0.75rem;
  }

  .approach-card ul {
    margin: 0.75rem 0 0 1.25rem;
    color: var(--text-secondary);
  }

  .approach-card li {
    margin-bottom: 0.25rem;
  }

  .viz-container {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
    margin: 2rem 0;
  }

  .memory-comparison {
    margin-top: 1.5rem;
  }

  .memory-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 0.75rem;
  }

  .bar-label {
    min-width: 80px;
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .bar-fill {
    height: 24px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    padding: 0 0.75rem;
    font-size: 0.75rem;
    font-weight: 500;
    transition: width 0.3s ease;
    min-width: 80px;
  }

  .bar-fill.snapshot {
    background: var(--state-dead);
    color: white;
  }

  .bar-fill.events {
    background: var(--state-born);
    color: var(--bg-primary);
  }

  .comparison-note {
    text-align: center;
    font-size: 0.875rem;
    color: var(--text-muted);
    margin-top: 1rem;
    margin-bottom: 0;
  }
</style>
