<script lang="ts">
  let currentTick = $state(10);
  let windowPast = $state<number | null>(5);
  let windowFuture = $state<number | null>(5);
  let targetTick = $state(8);
  let lastError = $state<string | null>(null);

  const minAllowed = $derived(
    windowPast !== null ? Math.max(0, currentTick - windowPast) : 0
  );
  const maxAllowed = $derived(
    windowFuture !== null ? currentTick + windowFuture : 20
  );

  const isValidTarget = $derived(
    targetTick >= minAllowed && targetTick <= maxAllowed
  );

  function attemptChronoport() {
    if (targetTick < minAllowed) {
      lastError = `Cannot chronoport to t=${targetTick}: before allowed window (min: t=${minAllowed})`;
    } else if (targetTick > maxAllowed) {
      lastError = `Cannot chronoport to t=${targetTick}: after allowed window (max: t=${maxAllowed})`;
    } else {
      lastError = null;
      // Success animation could go here
    }
  }

  function clearWindow(which: 'past' | 'future') {
    if (which === 'past') windowPast = null;
    else windowFuture = null;
    lastError = null;
  }
</script>

<section class="time-windows-section">
  <div class="content-wrapper">
    <h2>Time Windows</h2>

    <p>
      Time windows constrain how far entities can chronoport. Without limits, entities
      could travel to any point in history, which might break game balance or cause
      memory issues with very long timelines.
    </p>

    <div class="viz-container">
      <svg width="700" height="150">
        <!-- Timeline axis -->
        <line x1="50" y1="75" x2="650" y2="75" stroke="var(--text-muted)" stroke-width="2" />

        <!-- Tick marks -->
        {#each Array(21) as _, i}
          <line x1={50 + i * 30} y1="70" x2={50 + i * 30} y2="80" stroke="var(--text-muted)" />
          <text x={50 + i * 30} y="100" text-anchor="middle" font-size="10" fill="var(--text-muted)">
            {i}
          </text>
        {/each}

        <!-- Forbidden zone (before window) -->
        {#if windowPast !== null && minAllowed > 0}
          <rect
            x="50"
            y="55"
            width={minAllowed * 30}
            height="40"
            fill="var(--state-dead)"
            opacity="0.2"
          />
          <text
            x={50 + (minAllowed * 30) / 2}
            y="45"
            text-anchor="middle"
            font-size="9"
            fill="var(--state-dead)"
          >
            Too far past
          </text>
        {/if}

        <!-- Forbidden zone (after window) -->
        {#if windowFuture !== null && maxAllowed < 20}
          <rect
            x={50 + maxAllowed * 30}
            y="55"
            width={(20 - maxAllowed) * 30}
            height="40"
            fill="var(--state-dead)"
            opacity="0.2"
          />
          <text
            x={50 + maxAllowed * 30 + ((20 - maxAllowed) * 30) / 2}
            y="45"
            text-anchor="middle"
            font-size="9"
            fill="var(--state-dead)"
          >
            Too far future
          </text>
        {/if}

        <!-- Valid zone -->
        <rect
          x={50 + minAllowed * 30}
          y="55"
          width={(maxAllowed - minAllowed) * 30}
          height="40"
          fill="var(--state-born)"
          opacity="0.15"
        />

        <!-- Current tick marker -->
        <g transform="translate({50 + currentTick * 30}, 75)">
          <circle r="12" fill="var(--color-present)" />
          <text y="4" text-anchor="middle" font-size="10" fill="var(--bg-primary)">
            now
          </text>
        </g>

        <!-- Target tick marker -->
        <g transform="translate({50 + targetTick * 30}, 75)">
          <circle
            r="10"
            fill="none"
            stroke={isValidTarget ? 'var(--state-born)' : 'var(--state-dead)'}
            stroke-width="3"
            stroke-dasharray="5,3"
          />
          <text
            y="-20"
            text-anchor="middle"
            font-size="9"
            fill={isValidTarget ? 'var(--state-born)' : 'var(--state-dead)'}
          >
            target
          </text>
        </g>
      </svg>

      <div class="controls-grid">
        <div class="control-group">
          <label>Current Tick:</label>
          <input type="range" min="0" max="20" bind:value={currentTick} />
          <span>{currentTick}</span>
        </div>

        <div class="control-group">
          <label>Target Tick:</label>
          <input type="range" min="0" max="20" bind:value={targetTick} />
          <span>{targetTick}</span>
        </div>

        <div class="control-group">
          <label>Window Past:</label>
          {#if windowPast !== null}
            <input type="range" min="1" max="20" bind:value={windowPast} />
            <span>{windowPast}</span>
            <button class="small" onclick={() => clearWindow('past')}>Remove</button>
          {:else}
            <span class="unlimited">Unlimited</span>
            <button class="small" onclick={() => windowPast = 5}>Set Limit</button>
          {/if}
        </div>

        <div class="control-group">
          <label>Window Future:</label>
          {#if windowFuture !== null}
            <input type="range" min="1" max="20" bind:value={windowFuture} />
            <span>{windowFuture}</span>
            <button class="small" onclick={() => clearWindow('future')}>Remove</button>
          {:else}
            <span class="unlimited">Unlimited</span>
            <button class="small" onclick={() => windowFuture = 5}>Set Limit</button>
          {/if}
        </div>
      </div>

      <div class="action-area">
        <button class="primary" onclick={attemptChronoport}>
          Attempt Chronoport to t={targetTick}
        </button>

        {#if lastError}
          <div class="error-message">
            {lastError}
          </div>
        {:else if isValidTarget}
          <div class="success-message">
            Valid target! Chronoport would succeed.
          </div>
        {/if}
      </div>
    </div>

    <div class="window-info">
      <div class="info-card">
        <h4>Past Window</h4>
        <p>Limits how far back in time an entity can travel. Prevents accessing ancient history.</p>
        <code>min_target = current_tick - window_past</code>
      </div>
      <div class="info-card">
        <h4>Future Window</h4>
        <p>Limits how far forward an entity can travel. Usually prevents "time jumping" exploits.</p>
        <code>max_target = current_tick + window_future</code>
      </div>
    </div>

    <div class="insight">
      <div class="insight-label">Key Insight</div>
      <p>
        Time windows are optional. Set them to <code>null</code> for unlimited travel.
        In practice, most games limit past travel more strictly than future travel,
        since the past accumulates events that need to be stored.
      </p>
    </div>
  </div>
</section>

<style>
  .time-windows-section {
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

  .controls-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .control-group {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .control-group label {
    font-size: 0.875rem;
    color: var(--text-secondary);
    min-width: 100px;
  }

  .control-group input[type="range"] {
    flex: 1;
    max-width: 150px;
  }

  .control-group span {
    font-family: var(--font-mono);
    min-width: 30px;
    color: var(--text-primary);
  }

  .control-group .unlimited {
    color: var(--text-muted);
    font-style: italic;
  }

  button.small {
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
  }

  .action-area {
    text-align: center;
    margin-top: 1.5rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--bg-tertiary);
  }

  .error-message {
    margin-top: 1rem;
    padding: 0.75rem 1rem;
    background: rgba(248, 81, 73, 0.15);
    border: 1px solid var(--state-dead);
    border-radius: 6px;
    color: var(--state-dead);
    font-size: 0.9rem;
  }

  .success-message {
    margin-top: 1rem;
    padding: 0.75rem 1rem;
    background: rgba(63, 185, 80, 0.15);
    border: 1px solid var(--state-born);
    border-radius: 6px;
    color: var(--state-born);
    font-size: 0.9rem;
  }

  .window-info {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
    margin: 2rem 0;
  }

  .info-card {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .info-card h4 {
    color: var(--color-present);
    margin-bottom: 0.5rem;
  }

  .info-card p {
    font-size: 0.9rem;
    margin-bottom: 0.75rem;
  }

  .info-card code {
    display: block;
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  @media (max-width: 700px) {
    .controls-grid {
      grid-template-columns: 1fr;
    }

    .window-info {
      grid-template-columns: 1fr;
    }
  }
</style>
