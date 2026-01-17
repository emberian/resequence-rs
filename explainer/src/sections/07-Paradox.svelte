<script lang="ts">
  interface EntityCopy {
    id: number;
    position: number;
    state: 'Born' | 'Dead';
    label: string;
  }

  let copies = $state<EntityCopy[]>([
    { id: 0, position: 0, state: 'Born', label: 'Original' },
  ]);

  let selectedEntity = $state<number | null>(null);
  let showLinks = $state(true);

  function chronoport() {
    if (copies.length >= 5) return;

    const newId = copies.length;
    const targetPos = 5 + copies.length * 3;

    copies = [
      ...copies,
      { id: newId, position: targetPos, state: 'Born', label: `Copy ${newId}` }
    ];
  }

  function killEntity(id: number) {
    copies = copies.map(c =>
      c.id === id ? { ...c, state: 'Dead' } : c
    );
  }

  function reset() {
    copies = [{ id: 0, position: 0, state: 'Born', label: 'Original' }];
    selectedEntity = null;
  }

  const sameNameGroup = $derived(copies.map(c => c.id));
  const livingCopies = $derived(copies.filter(c => c.state === 'Born'));
  const deadCopies = $derived(copies.filter(c => c.state === 'Dead'));
</script>

<section class="paradox-section">
  <div class="content-wrapper">
    <h2>Paradox Detection</h2>

    <p>
      When an entity chronoports multiple times, it creates several copies that all share
      the same NameId. The engine tracks these as a "same-name group"—enabling scripts
      to detect and respond to paradoxes.
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

        <!-- Links between copies -->
        {#if showLinks && copies.length > 1}
          {#each copies.slice(0, -1) as copy, i}
            {@const nextCopy = copies[i + 1]}
            <path
              d="M{50 + copy.position * 30},{80} Q{50 + (copy.position + nextCopy.position) / 2 * 30},{40} {50 + nextCopy.position * 30},{80}"
              fill="none"
              stroke="var(--color-present)"
              stroke-width="2"
              stroke-dasharray="4,4"
              opacity="0.5"
            />
          {/each}
        {/if}

        <!-- Entity copies -->
        {#each copies as copy}
          {@const x = 50 + copy.position * 30}
          {@const color = copy.state === 'Born' ? 'var(--state-born)' : 'var(--state-dead)'}
          {@const isSelected = selectedEntity === copy.id}

          <g
            class="entity-copy"
            class:selected={isSelected}
            onclick={() => selectedEntity = isSelected ? null : copy.id}
            onkeydown={(e) => e.key === 'Enter' && (selectedEntity = isSelected ? null : copy.id)}
            role="button"
            tabindex="0"
          >
            <circle
              cx={x}
              cy="100"
              r={isSelected ? 18 : 14}
              fill={color}
              stroke={isSelected ? 'white' : 'none'}
              stroke-width="2"
            />
            <text x={x} y="104" text-anchor="middle" font-size="10" fill="var(--bg-primary)">
              E{copy.id}
            </text>
            <text x={x} y="140" text-anchor="middle" font-size="9" fill="var(--text-secondary)">
              {copy.label}
            </text>
            <text x={x} y="155" text-anchor="middle" font-size="8" fill={color}>
              {copy.state}
            </text>
          </g>
        {/each}
      </svg>

      <div class="controls">
        <button onclick={chronoport} disabled={copies.length >= 5} class="primary">
          Chronoport Again
        </button>
        {#if selectedEntity !== null}
          <button
            onclick={() => killEntity(selectedEntity!)}
            disabled={copies.find(c => c.id === selectedEntity)?.state === 'Dead'}
          >
            Kill Selected
          </button>
        {/if}
        <button onclick={reset}>Reset</button>

        <label class="toggle">
          <input type="checkbox" bind:checked={showLinks} />
          Show Links
        </label>
      </div>

      <div class="same-name-info">
        <h4>Same-Name Group (NameId: 0)</h4>
        <div class="group-display">
          <span class="label">Entities:</span>
          <span class="ids">[{sameNameGroup.join(', ')}]</span>
        </div>
        <div class="group-display">
          <span class="label">Living:</span>
          <span class="ids living">{livingCopies.length}</span>
          <span class="label">Dead:</span>
          <span class="ids dead">{deadCopies.length}</span>
        </div>
      </div>
    </div>

    <div class="code-example">
      <h4>Detecting Paradoxes</h4>
      <pre><code>{`// Get all temporal copies of an entity
const copies = engine.getSameNameEntities(entityId);

// Check for conflicts
for (const copyId of copies) {
  const state = engine.getState(copyId);
  if (state.lifecycle === 'Dead') {
    // One of our copies died!
    // Respond to paradox...
  }
}`}</code></pre>
    </div>

    <div class="insight">
      <div class="insight-label">Key Insight</div>
      <p>
        The same-name system lets game scripts detect when temporal copies interact
        in problematic ways. If one copy kills another, or if past-you prevents
        future-you from existing, the game can detect and handle it.
      </p>
    </div>
  </div>
</section>

<style>
  .paradox-section {
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
    margin: 0 auto;
  }

  .entity-copy {
    cursor: pointer;
    transition: transform 0.2s ease;
  }

  .entity-copy:hover circle,
  .entity-copy.selected circle {
    filter: brightness(1.2);
  }

  .controls {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    margin: 1.5rem 0;
    flex-wrap: wrap;
  }

  .toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .same-name-info {
    background: var(--bg-tertiary);
    border-radius: 6px;
    padding: 1rem;
    margin-top: 1.5rem;
  }

  .same-name-info h4 {
    font-size: 0.9rem;
    margin-bottom: 0.75rem;
    color: var(--color-present);
  }

  .group-display {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .group-display .label {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .group-display .ids {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-primary);
  }

  .group-display .living {
    color: var(--state-born);
  }

  .group-display .dead {
    color: var(--state-dead);
  }

  .code-example {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
    margin: 2rem 0;
  }

  .code-example h4 {
    font-size: 0.9rem;
    margin-bottom: 1rem;
    color: var(--text-secondary);
  }

  .code-example pre {
    margin: 0;
    overflow-x: auto;
  }

  .code-example code {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    background: none;
    padding: 0;
  }
</style>
