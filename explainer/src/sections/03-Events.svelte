<script lang="ts">
  import Scrolly from '../lib/scroll/Scrolly.svelte';
  import EntityViz from '../lib/viz/EntityViz.svelte';
  import { Entity, SimpleState, bornEvent, type Event, type EntityState } from '../lib/engine';

  // Demo entity with events at specific times
  const demoEntity = Entity.spawn(0, 0, 0, new SimpleState({ health: 100 }));
  demoEntity.setState(5, new SimpleState({ health: 75 }));
  demoEntity.setState(10, new SimpleState({ health: 50 }));
  demoEntity.setState(15, new SimpleState({ health: 25 }));

  let queryTick = $state(7);
  let showBinarySearch = $state(false);
  let currentStep = $state(0);

  const events = $derived(demoEntity.allEvents());
  const foundEvent = $derived(demoEntity.getEventAt(queryTick));

  const steps = [
    {
      title: 'Events, Not Snapshots',
      content: 'Instead of storing complete state at every tick, we store only the moments when state changes.',
    },
    {
      title: 'Timestamped History',
      content: 'Each event records: when it happened, the lifecycle state, and the entity data.',
    },
    {
      title: 'Query Any Time',
      content: 'To find state at tick T, we find the most recent event at or before T.',
    },
    {
      title: 'Binary Search',
      content: "Since events are sorted by time, we can use binary search for O(log n) lookups—even with millions of events.",
    },
  ];

  function handleStepEnter(index: number) {
    currentStep = index;
    showBinarySearch = index >= 3;
  }
</script>

<section class="events-section">
  <Scrolly
    offset={0.5}
    onStepEnter={handleStepEnter}
  >
    {#snippet sticky()}
      <div class="viz-panel">
        <h3>Entity Event History</h3>

        <EntityViz
          {events}
          minTick={0}
          maxTick={20}
          {queryTick}
          width={600}
          height={100}
          label="Marine"
          {showBinarySearch}
        />

        <div class="query-controls">
          <div class="slider-container">
            <label for="query-tick-slider">Query at t=</label>
            <input id="query-tick-slider" type="range" min="0" max="20" bind:value={queryTick} />
            <span class="query-value">{queryTick}</span>
          </div>

          {#if foundEvent}
            <div class="query-result">
              <span class="result-label">Found:</span>
              Event at t={foundEvent.timestamp}
              <span class="state-badge" style="background: var(--state-born)">
                health: {(foundEvent.state as SimpleState).data.health}
              </span>
            </div>
          {:else}
            <div class="query-result no-result">
              No event found before t={queryTick}
            </div>
          {/if}
        </div>

        {#if showBinarySearch}
          <div class="binary-search-viz">
            <p class="search-caption">
              Binary search: log₂({events.length}) = {Math.ceil(Math.log2(events.length))} steps max
            </p>
          </div>
        {/if}
      </div>
    {/snippet}

    {#each steps as step, i}
      <div class="scrolly-step">
        <div class="scrolly-step-content" class:active={currentStep === i}>
          <h4>{step.title}</h4>
          <p>{step.content}</p>
        </div>
      </div>
    {/each}
  </Scrolly>

  <div class="content-wrapper">
    <div class="insight">
      <div class="insight-label">Key Insight</div>
      <p>
        By storing events instead of snapshots, we get <strong>O(log n)</strong> historical queries
        with minimal memory overhead. The past becomes as cheap to access as the present.
      </p>
    </div>

    <div class="code-example">
      <pre><code>{`// Query state at any tick - O(log n)
const event = entity.getEventAt(tick);

// Events are sorted by timestamp
// Binary search finds the right one
[t=0, t=5, t=10, t=15]
       ↑
  query t=7 → finds t=5`}</code></pre>
    </div>
  </div>
</section>

<style>
  .events-section {
    min-height: 100vh;
  }

  .viz-panel {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 12px;
    padding: 2rem;
    max-width: 700px;
    width: 90%;
  }

  .viz-panel h3 {
    text-align: center;
    margin-bottom: 1.5rem;
  }

  .query-controls {
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 1px solid var(--bg-tertiary);
  }

  .query-value {
    font-family: var(--font-mono);
    font-weight: 600;
    color: var(--color-present);
    min-width: 30px;
    text-align: right;
  }

  .query-result {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-top: 1rem;
    font-size: 0.9rem;
  }

  .result-label {
    color: var(--text-muted);
  }

  .state-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    color: var(--bg-primary);
  }

  .no-result {
    color: var(--text-muted);
    font-style: italic;
  }

  .binary-search-viz {
    margin-top: 1rem;
    padding: 0.75rem;
    background: rgba(88, 166, 255, 0.1);
    border-radius: 6px;
  }

  .search-caption {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--color-present);
    margin: 0;
    text-align: center;
  }

  .content-wrapper {
    max-width: 700px;
    margin: 4rem auto;
    padding: 0 2rem;
  }

  .code-example {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
    margin-top: 2rem;
  }

  .code-example pre {
    margin: 0;
    overflow-x: auto;
  }

  .code-example code {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-primary);
    background: none;
    padding: 0;
  }

  .scrolly-step-content {
    opacity: 0.5;
    transition: opacity 0.3s ease;
  }

  .scrolly-step-content.active {
    opacity: 1;
  }

  .scrolly-step-content h4 {
    color: var(--color-present);
    margin-bottom: 0.5rem;
  }
</style>
