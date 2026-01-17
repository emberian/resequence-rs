<script lang="ts">
  import StateGraph from '../lib/viz/StateGraph.svelte';
  import { LifecycleState, lifecycleStateNames, lifecycleStateColors } from '../lib/engine';

  let activeState = $state<LifecycleState | null>(null);
  let highlightTransition = $state<{ from: LifecycleState; to: LifecycleState } | null>(null);

  const stateDescriptions: Record<LifecycleState, string> = {
    [LifecycleState.Unborn]: 'The entity does not exist yet. This is the default state for empty slots or entities that haven\'t been created.',
    [LifecycleState.Prebirth]: 'The entity is scheduled to appear—typically as the destination of a chronoport. It exists but isn\'t fully active yet.',
    [LifecycleState.Born]: 'The normal active state. The entity exists, can act, and can be interacted with.',
    [LifecycleState.Chronoporting]: 'The entity is in the middle of time travel. It\'s departing from its current time.',
    [LifecycleState.Dead]: 'The entity has been destroyed or has completed its chronoport departure. It no longer exists at this time.',
  };

  const transitions = [
    { from: LifecycleState.Unborn, to: LifecycleState.Born, label: 'spawn() creates a new entity directly' },
    { from: LifecycleState.Unborn, to: LifecycleState.Prebirth, label: 'chronoport target appears as Prebirth' },
    { from: LifecycleState.Prebirth, to: LifecycleState.Born, label: 'when a timewave reaches the entity' },
    { from: LifecycleState.Born, to: LifecycleState.Chronoporting, label: 'chronoport() initiates time travel' },
    { from: LifecycleState.Born, to: LifecycleState.Dead, label: 'destroy() kills the entity' },
    { from: LifecycleState.Chronoporting, to: LifecycleState.Dead, label: 'after departure completes' },
  ];

  function handleStateClick(state: LifecycleState) {
    activeState = activeState === state ? null : state;
    highlightTransition = null;
  }

  function handleTransitionHover(from: LifecycleState, to: LifecycleState) {
    highlightTransition = { from, to };
  }

  function clearTransitionHover() {
    highlightTransition = null;
  }
</script>

<section class="lifecycle-section">
  <div class="content-wrapper">
    <h2>Lifecycle States</h2>
    <p>
      Every entity has a lifecycle state that tracks where it is in its existence.
      These five states handle everything from normal gameplay to time travel.
    </p>

    <div class="viz-container">
      <StateGraph
        {activeState}
        {highlightTransition}
        width={500}
        height={300}
        interactive={true}
        onStateClick={handleStateClick}
      />

      {#if activeState !== null}
        <div class="state-description" style="border-color: {lifecycleStateColors[activeState]}">
          <h4 style="color: {lifecycleStateColors[activeState]}">
            {lifecycleStateNames[activeState]}
          </h4>
          <p>{stateDescriptions[activeState]}</p>
        </div>
      {:else}
        <p class="click-hint">Click a state to learn more</p>
      {/if}
    </div>

    <h3>State Transitions</h3>

    <div class="transitions-list">
      {#each transitions as t}
        <button
          class="transition-item"
          class:highlighted={highlightTransition?.from === t.from && highlightTransition?.to === t.to}
          onmouseenter={() => handleTransitionHover(t.from, t.to)}
          onmouseleave={clearTransitionHover}
          onfocus={() => handleTransitionHover(t.from, t.to)}
          onblur={clearTransitionHover}
        >
          <span class="from" style="color: {lifecycleStateColors[t.from]}">
            {lifecycleStateNames[t.from]}
          </span>
          <span class="arrow">→</span>
          <span class="to" style="color: {lifecycleStateColors[t.to]}">
            {lifecycleStateNames[t.to]}
          </span>
          <span class="label">{t.label}</span>
        </button>
      {/each}
    </div>

    <div class="insight">
      <div class="insight-label">Key Insight</div>
      <p>
        The <strong>Prebirth</strong> state is crucial for time travel. When you chronoport,
        the destination entity appears as Prebirth until a timewave reaches it—then it
        transitions to Born and becomes fully active.
      </p>
    </div>
  </div>
</section>

<style>
  .lifecycle-section {
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
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .state-description {
    margin-top: 1.5rem;
    padding: 1rem 1.25rem;
    background: var(--bg-tertiary);
    border-radius: 6px;
    border-left: 3px solid;
    max-width: 400px;
  }

  .state-description h4 {
    margin-bottom: 0.5rem;
  }

  .state-description p {
    margin: 0;
    font-size: 0.9rem;
  }

  .click-hint {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin-top: 1rem;
    margin-bottom: 0;
  }

  .transitions-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin: 1.5rem 0;
  }

  .transition-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    text-align: left;
    width: 100%;
  }

  .transition-item:hover,
  .transition-item.highlighted {
    background: var(--bg-tertiary);
    border-color: var(--color-present);
  }

  .transition-item .from,
  .transition-item .to {
    font-weight: 600;
    min-width: 100px;
  }

  .transition-item .arrow {
    color: var(--text-muted);
  }

  .transition-item .label {
    flex: 1;
    color: var(--text-secondary);
    font-size: 0.9rem;
  }
</style>
