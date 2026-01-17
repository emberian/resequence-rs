<script lang="ts">
  import EngineLoopViz from '../lib/viz/EngineLoop.svelte';

  let currentStep = $state(0);

  const stepDetails = [
    {
      title: 'Step 1: Advance Waves',
      description: 'Each timewave moves forward by its speed multiplied by the delta time. A wave with speed 2.0 moves twice as fast as one with speed 1.0.',
      code: `for (const wave of timewaves) {
  wave.advance(delta);
  // position += speed * delta
}`,
    },
    {
      title: 'Step 2: Sort by Position',
      description: 'Waves are sorted by their current position in time. This ordering is essential for propagation—earlier waves must propagate to later ones.',
      code: `const sorted = [...timewaves].sort(
  (a, b) => a.position - b.position
);`,
    },
    {
      title: 'Step 3: Link Chain',
      description: 'Each wave gets prev/next pointers to form a chain. This enables efficient traversal during propagation.',
      code: `for (let i = 0; i < sorted.length; i++) {
  sorted[i].prev = sorted[i - 1] ?? null;
  sorted[i].next = sorted[i + 1] ?? null;
}`,
    },
    {
      title: 'Step 4: Activate Prebirth',
      description: 'When a wave reaches an entity in Prebirth state, that entity transitions to Born. This is how chronoported entities "appear" in time.',
      code: `for (const wave of sorted) {
  timeline.activatePrebirthEntities(wave.position);
  // Prebirth → Born
}`,
    },
    {
      title: 'Step 5: Propagate Views',
      description: 'The key causality step. When a faster wave passes a slower wave, it inherits the slower wave\'s view of entities. This ensures consistent observations.',
      code: `for (let i = 1; i < sorted.length; i++) {
  const prev = sorted[i - 1];
  const curr = sorted[i];

  if (curr.crossedOver(prev)) {
    curr.inheritViewsFrom(prev);
  }
}`,
    },
    {
      title: 'Step 6: Clear Dirty Flags',
      description: 'Reset change tracking for the next tick. Dirty flags mark which entity views changed, enabling efficient sync with external systems.',
      code: `for (const wave of timewaves) {
  wave.clearDirty();
  wave.needsSync = false;
}`,
    },
  ];

  function handleStepChange(step: number) {
    currentStep = step;
  }
</script>

<section class="engine-loop-section">
  <div class="content-wrapper">
    <h2>The Engine Loop</h2>

    <p>
      Every simulation tick runs through six carefully ordered steps. This loop is
      the heart of the engine—it advances time, maintains causality, and keeps
      all observers synchronized.
    </p>

    <div class="main-viz">
      <EngineLoopViz
        {currentStep}
        width={700}
        height={350}
        onStepChange={handleStepChange}
      />
    </div>

    <div class="step-detail">
      <h3>{stepDetails[currentStep].title}</h3>
      <p>{stepDetails[currentStep].description}</p>

      <div class="code-block">
        <pre><code>{stepDetails[currentStep].code}</code></pre>
      </div>
    </div>

    <div class="insight">
      <div class="insight-label">Key Insight</div>
      <p>
        The order matters. Waves must be sorted before propagation. Prebirth activation
        must happen before view refresh. This deterministic sequence ensures that
        given the same inputs, the simulation always produces the same results.
      </p>
    </div>

    <div class="complexity-note">
      <h4>Complexity</h4>
      <ul>
        <li><code>Advance</code>: O(W) where W = number of waves</li>
        <li><code>Sort</code>: O(W log W)</li>
        <li><code>Link</code>: O(W)</li>
        <li><code>Activate</code>: O(W × E) where E = entities</li>
        <li><code>Refresh Views</code>: O(W × E × log H) where H = events per entity</li>
        <li><code>Propagate</code>: O(W × E) worst case</li>
      </ul>
      <p>
        Total: <strong>O(W × E × log H)</strong> per tick—dominated by view refresh
        using binary search through event history.
      </p>
    </div>
  </div>
</section>

<style>
  .engine-loop-section {
    padding: 4rem 2rem;
  }

  .content-wrapper {
    max-width: 900px;
    margin: 0 auto;
  }

  .main-viz {
    margin: 2rem 0;
  }

  .step-detail {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
    margin: 2rem 0;
  }

  .step-detail h3 {
    color: var(--color-present);
    margin-bottom: 0.75rem;
  }

  .step-detail p {
    margin-bottom: 1rem;
  }

  .code-block {
    background: var(--bg-tertiary);
    border-radius: 6px;
    padding: 1rem;
    overflow-x: auto;
  }

  .code-block pre {
    margin: 0;
  }

  .code-block code {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    background: none;
    padding: 0;
  }

  .complexity-note {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
    margin-top: 2rem;
  }

  .complexity-note h4 {
    color: var(--text-primary);
    margin-bottom: 1rem;
  }

  .complexity-note ul {
    margin: 0 0 1rem 1.5rem;
  }

  .complexity-note li {
    margin-bottom: 0.5rem;
    color: var(--text-secondary);
  }

  .complexity-note li code {
    color: var(--color-present);
  }

  .complexity-note p {
    margin: 0;
    color: var(--text-secondary);
  }

  .complexity-note strong {
    color: var(--text-primary);
  }
</style>
