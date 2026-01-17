<script lang="ts">
  import { onMount } from 'svelte';

  type Phase = 'initial' | 'clicking' | 'traveling' | 'arrived' | 'linked';

  let phase = $state<Phase>('initial');
  let animationProgress = $state(0);

  const sourcePosition = 10;
  const targetPosition = 5;

  function startChronoport() {
    if (phase !== 'initial') return;
    phase = 'clicking';

    setTimeout(() => {
      phase = 'traveling';
      animateTravel();
    }, 500);
  }

  function animateTravel() {
    const duration = 1500;
    const start = performance.now();

    function animate(now: number) {
      const elapsed = now - start;
      animationProgress = Math.min(1, elapsed / duration);

      if (animationProgress < 1) {
        requestAnimationFrame(animate);
      } else {
        phase = 'arrived';
        setTimeout(() => {
          phase = 'linked';
        }, 800);
      }
    }

    requestAnimationFrame(animate);
  }

  function reset() {
    phase = 'initial';
    animationProgress = 0;
  }

  // Arrow position during travel animation
  const arrowX = $derived(
    phase === 'traveling'
      ? 50 + sourcePosition * 30 - (sourcePosition - targetPosition - 1) * 30 * animationProgress
      : phase === 'arrived' || phase === 'linked'
        ? 50 + (targetPosition + 1) * 30
        : 50 + sourcePosition * 30
  );
</script>

<section class="chronoport-section">
  <div class="content-wrapper">
    <h2>Chronoport: Time Travel</h2>

    <p>
      This is the "money shot"—the core mechanic that makes time-travel games possible.
      When an entity chronoports, it doesn't teleport. It <em>duplicates</em>.
    </p>

    <div class="viz-container">
      <svg width="700" height="250">
        <!-- Timeline axis -->
        <line x1="50" y1="120" x2="650" y2="120" stroke="var(--text-muted)" stroke-width="2" />

        <!-- Tick marks -->
        {#each Array(21) as _, i}
          <line x1={50 + i * 30} y1="115" x2={50 + i * 30} y2="125" stroke="var(--text-muted)" />
          {#if i % 5 === 0}
            <text x={50 + i * 30} y="145" text-anchor="middle" font-size="10" fill="var(--text-muted)">
              t={i}
            </text>
          {/if}
        {/each}

        <!-- Source entity at t=10 -->
        <g transform="translate({50 + sourcePosition * 30}, 120)">
          {#if phase === 'initial'}
            <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
            <circle r="15" fill="var(--state-born)" class="clickable" onclick={startChronoport} role="button" tabindex="0" />
            <text y="5" text-anchor="middle" font-size="11" fill="var(--bg-primary)">E0</text>
            <text y="40" text-anchor="middle" font-size="10" fill="var(--text-secondary)">Born</text>
            <text y="-25" text-anchor="middle" font-size="9" fill="var(--color-chronoport)">
              Click to chronoport!
            </text>
          {:else if phase === 'clicking'}
            <circle r="15" fill="var(--color-chronoport)" />
            <text y="5" text-anchor="middle" font-size="11" fill="var(--bg-primary)">E0</text>
            <text y="40" text-anchor="middle" font-size="10" fill="var(--color-chronoport)">
              Initiating...
            </text>
          {:else}
            <circle r="15" fill="var(--state-chronoporting)" opacity="0.6" />
            <text y="5" text-anchor="middle" font-size="11" fill="var(--bg-primary)">E0</text>
            <text y="40" text-anchor="middle" font-size="10" fill="var(--state-chronoporting)">
              Chronoporting
            </text>
          {/if}
        </g>

        <!-- Travel arrow -->
        {#if phase === 'traveling'}
          <g class="travel-arrow">
            <line
              x1={50 + sourcePosition * 30}
              y1="80"
              x2={arrowX}
              y2="80"
              stroke="var(--color-chronoport)"
              stroke-width="3"
              stroke-dasharray="8,4"
            />
            <circle cx={arrowX} cy="80" r="8" fill="var(--color-chronoport)" />
            <text x={arrowX} y="65" text-anchor="middle" font-size="9" fill="var(--color-chronoport)">
              ← to t=5
            </text>
          </g>
        {/if}

        <!-- Destination entity at t=6 (target+1) -->
        {#if phase === 'arrived' || phase === 'linked'}
          <g transform="translate({50 + (targetPosition + 1) * 30}, 120)">
            <circle
              r="15"
              fill={phase === 'arrived' ? 'var(--state-prebirth)' : 'var(--state-born)'}
            />
            <text y="5" text-anchor="middle" font-size="11" fill="var(--bg-primary)">E1</text>
            <text y="40" text-anchor="middle" font-size="10" fill={phase === 'arrived' ? 'var(--state-prebirth)' : 'var(--state-born)'}>
              {phase === 'arrived' ? 'Prebirth' : 'Born'}
            </text>
            <text y="-25" text-anchor="middle" font-size="9" fill="var(--text-muted)">
              New entity!
            </text>
          </g>
        {/if}

        <!-- Link between entities (same NameId) -->
        {#if phase === 'linked'}
          <g class="name-link">
            <path
              d="M{50 + (targetPosition + 1) * 30 + 20},120 Q{50 + ((targetPosition + 1 + sourcePosition) / 2) * 30},60 {50 + sourcePosition * 30 - 20},120"
              fill="none"
              stroke="var(--color-present)"
              stroke-width="2"
              stroke-dasharray="4,4"
            />
            <text
              x={50 + ((targetPosition + 1 + sourcePosition) / 2) * 30}
              y="50"
              text-anchor="middle"
              font-size="9"
              fill="var(--color-present)"
            >
              Same NameId
            </text>
          </g>
        {/if}
      </svg>

      <div class="phase-indicator">
        <div class="phase-step" class:active={phase === 'initial'}>1. Initial</div>
        <div class="phase-arrow">→</div>
        <div class="phase-step" class:active={phase === 'clicking' || phase === 'traveling'}>2. Traveling</div>
        <div class="phase-arrow">→</div>
        <div class="phase-step" class:active={phase === 'arrived'}>3. Arrived</div>
        <div class="phase-arrow">→</div>
        <div class="phase-step" class:active={phase === 'linked'}>4. Linked</div>
      </div>

      <div class="controls">
        <button onclick={reset}>Reset</button>
      </div>
    </div>

    <div class="explanation-cards">
      <div class="exp-card">
        <h4>Before</h4>
        <p>Entity E0 exists at t=10 in <span class="state born">Born</span> state.</p>
      </div>
      <div class="exp-card">
        <h4>Chronoport</h4>
        <p>E0 calls chronoport(target=5). It transitions to <span class="state chronoporting">Chronoporting</span>.</p>
      </div>
      <div class="exp-card">
        <h4>After</h4>
        <p>New entity E1 appears at t=6 as <span class="state prebirth">Prebirth</span>. Both share the same NameId.</p>
      </div>
    </div>

    <div class="insight">
      <div class="insight-label">Key Insight</div>
      <p>
        Chronoport creates <strong>duplicates</strong>, not teleportation. The original entity
        stays at its departure time (marked Chronoporting→Dead), while a copy appears at the
        destination. They share a NameId, linking them as the "same" entity across time.
      </p>
    </div>
  </div>
</section>

<style>
  .chronoport-section {
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

  .clickable {
    cursor: pointer;
    transition: filter 0.2s ease;
  }

  .clickable:hover {
    filter: brightness(1.2);
  }

  .phase-indicator {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 0.5rem;
    margin: 1.5rem 0;
  }

  .phase-step {
    padding: 0.5rem 1rem;
    background: var(--bg-tertiary);
    border-radius: 4px;
    font-size: 0.875rem;
    color: var(--text-muted);
    transition: all 0.3s ease;
  }

  .phase-step.active {
    background: var(--color-present);
    color: var(--bg-primary);
  }

  .phase-arrow {
    color: var(--text-muted);
  }

  .controls {
    display: flex;
    justify-content: center;
    margin-top: 1rem;
  }

  .explanation-cards {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    margin: 2rem 0;
  }

  .exp-card {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 6px;
    padding: 1rem;
  }

  .exp-card h4 {
    color: var(--color-present);
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
  }

  .exp-card p {
    font-size: 0.875rem;
    margin: 0;
  }

  .state {
    font-weight: 600;
    padding: 0.1em 0.3em;
    border-radius: 3px;
  }

  .state.born {
    color: var(--state-born);
  }

  .state.chronoporting {
    color: var(--state-chronoporting);
  }

  .state.prebirth {
    color: var(--state-prebirth);
  }

  .travel-arrow line {
    animation: dash 0.5s linear infinite;
  }

  @keyframes dash {
    to {
      stroke-dashoffset: -24;
    }
  }

  .name-link path {
    animation: dash 1s linear infinite;
  }
</style>
