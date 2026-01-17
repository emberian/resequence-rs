<script lang="ts">
  interface Props {
    currentStep?: number;
    playing?: boolean;
    width?: number;
    height?: number;
    onStepChange?: (step: number) => void;
  }

  let {
    currentStep = 0,
    playing = false,
    width = 600,
    height = 400,
    onStepChange,
  }: Props = $props();

  const steps = [
    {
      title: '1. Advance Waves',
      description: 'Each wave moves by speed × delta',
      visual: 'advance',
    },
    {
      title: '2. Sort by Position',
      description: 'Waves sorted for propagation order',
      visual: 'sort',
    },
    {
      title: '3. Link Chain',
      description: 'prev/next pointers connect waves',
      visual: 'link',
    },
    {
      title: '4. Activate Prebirth',
      description: 'Entities transition Prebirth → Born',
      visual: 'activate',
    },
    {
      title: '5. Propagate Views',
      description: 'Faster waves inherit from slower',
      visual: 'propagate',
    },
    {
      title: '6. Clear Dirty',
      description: 'Reset change flags for next tick',
      visual: 'clear',
    },
  ];

  // Example wave data for visualization
  const waveData = $derived.by(() => {
    // Initial positions
    const base = [
      { id: 0, pos: 5, speed: 1.0, label: 'Present', color: 'var(--wave-normal)' },
      { id: 1, pos: 3, speed: 0.5, label: 'Slow', color: 'var(--wave-slow)' },
      { id: 2, pos: 8, speed: 2.0, label: 'Fast', color: 'var(--wave-fast)' },
    ];

    if (currentStep >= 1) {
      // After advance: positions changed
      base[0].pos = 6;
      base[1].pos = 3; // 0.5 speed, so didn't advance full tick
      base[2].pos = 10;
    }

    // After sort (step 2+): reorder by position
    if (currentStep >= 2) {
      return [...base].sort((a, b) => a.pos - b.pos);
    }

    return base;
  });

  // Entity data for prebirth activation
  const entityData = $derived.by(() => {
    return {
      id: 0,
      lifecycle: currentStep >= 4 ? 'Born' : 'Prebirth',
      color: currentStep >= 4 ? 'var(--state-born)' : 'var(--state-prebirth)',
    };
  });

  function nextStep() {
    const next = (currentStep + 1) % steps.length;
    onStepChange?.(next);
  }

  function prevStep() {
    const prev = (currentStep - 1 + steps.length) % steps.length;
    onStepChange?.(prev);
  }

  function goToStep(step: number) {
    onStepChange?.(step);
  }

  // Visualization positioning
  const vizPadding = { left: 50, right: 30, top: 80, bottom: 100 };
  const vizWidth = width - vizPadding.left - vizPadding.right;

  function posToX(pos: number): number {
    return vizPadding.left + (pos / 15) * vizWidth;
  }
</script>

<div class="engine-loop" style="width: {width}px">
  <!-- Step indicator dots -->
  <div class="step-indicators">
    {#each steps as step, i}
      <button
        class="step-dot"
        class:active={i === currentStep}
        class:completed={i < currentStep}
        onclick={() => goToStep(i)}
        title={step.title}
      >
        {i + 1}
      </button>
    {/each}
  </div>

  <!-- Current step info -->
  <div class="step-info">
    <h4>{steps[currentStep].title}</h4>
    <p>{steps[currentStep].description}</p>
  </div>

  <!-- Visualization area -->
  <svg {width} height={height - 160}>
    <!-- Timeline axis -->
    <line
      x1={vizPadding.left}
      y1={height / 2 - 40}
      x2={width - vizPadding.right}
      y2={height / 2 - 40}
      stroke="var(--text-muted)"
      stroke-width="2"
    />

    <!-- Tick marks -->
    {#each Array(16) as _, i}
      <line
        x1={posToX(i)}
        y1={height / 2 - 45}
        x2={posToX(i)}
        y2={height / 2 - 35}
        stroke="var(--text-muted)"
        stroke-width="1"
      />
      {#if i % 5 === 0}
        <text
          x={posToX(i)}
          y={height / 2 - 20}
          text-anchor="middle"
          font-size="10"
          fill="var(--text-muted)"
        >
          t={i}
        </text>
      {/if}
    {/each}

    <!-- Waves -->
    {#each waveData as wave, i}
      <g class="wave" style="--wave-color: {wave.color}">
        <!-- Wave cursor -->
        <line
          x1={posToX(wave.pos)}
          y1={60}
          x2={posToX(wave.pos)}
          y2={height / 2 - 50}
          stroke={wave.color}
          stroke-width="3"
        />

        <!-- Arrow head -->
        <polygon
          points="{posToX(wave.pos)},{height / 2 - 50} {posToX(wave.pos) - 5},{height / 2 - 60} {posToX(wave.pos) + 5},{height / 2 - 60}"
          fill={wave.color}
        />

        <!-- Label -->
        <text
          x={posToX(wave.pos)}
          y={50}
          text-anchor="middle"
          font-size="11"
          fill={wave.color}
        >
          {wave.label} ({wave.speed}x)
        </text>

        <!-- Chain links (step 3+) -->
        {#if currentStep >= 3 && i < waveData.length - 1}
          {@const nextWave = waveData[i + 1]}
          <line
            x1={posToX(wave.pos) + 10}
            y1={80}
            x2={posToX(nextWave.pos) - 10}
            y2={80}
            stroke="var(--text-muted)"
            stroke-width="1"
            stroke-dasharray="4,4"
          />
          <text
            x={(posToX(wave.pos) + posToX(nextWave.pos)) / 2}
            y={75}
            text-anchor="middle"
            font-size="9"
            fill="var(--text-muted)"
          >
            →
          </text>
        {/if}
      </g>
    {/each}

    <!-- Entity (for activation visualization) -->
    {#if currentStep >= 3}
      <g class="entity" transform="translate({posToX(4)}, {height / 2})">
        <circle r="12" fill={entityData.color} />
        <text y="4" text-anchor="middle" font-size="10" fill="var(--bg-primary)">
          E0
        </text>
        <text y="28" text-anchor="middle" font-size="9" fill={entityData.color}>
          {entityData.lifecycle}
        </text>
      </g>
    {/if}

    <!-- Propagation arrows (step 5) -->
    {#if currentStep === 5}
      <g class="propagation">
        <path
          d="M{posToX(3) + 15},{height / 2 - 70} Q{posToX(5)},{height / 2 - 90} {posToX(6) - 15},{height / 2 - 70}"
          fill="none"
          stroke="var(--color-present)"
          stroke-width="2"
          marker-end="url(#arrow)"
        />
        <text
          x={posToX(4.5)}
          y={height / 2 - 95}
          text-anchor="middle"
          font-size="9"
          fill="var(--color-present)"
        >
          inherit views
        </text>
      </g>
    {/if}

    <!-- Checkmarks (step 6) -->
    {#if currentStep === 6}
      {#each waveData as wave}
        <text
          x={posToX(wave.pos)}
          y={height / 2 - 55}
          text-anchor="middle"
          font-size="14"
          fill="var(--state-born)"
        >
          ✓
        </text>
      {/each}
    {/if}

    <defs>
      <marker id="arrow" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
        <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-present)" />
      </marker>
    </defs>
  </svg>

  <!-- Controls -->
  <div class="controls">
    <button onclick={prevStep}>← Previous</button>
    <span class="step-counter">Step {currentStep + 1} of {steps.length}</span>
    <button onclick={nextStep}>Next →</button>
  </div>
</div>

<style>
  .engine-loop {
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .step-indicators {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .step-dot {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: 2px solid var(--text-muted);
    background: var(--bg-tertiary);
    color: var(--text-muted);
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .step-dot.active {
    border-color: var(--color-present);
    background: var(--color-present);
    color: var(--bg-primary);
  }

  .step-dot.completed {
    border-color: var(--state-born);
    background: var(--state-born);
    color: var(--bg-primary);
  }

  .step-info {
    text-align: center;
    margin-bottom: 1rem;
  }

  .step-info h4 {
    font-size: 1.1rem;
    color: var(--text-primary);
    margin-bottom: 0.25rem;
  }

  .step-info p {
    font-size: 0.875rem;
    color: var(--text-secondary);
    margin: 0;
  }

  .controls {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    margin-top: 1rem;
  }

  .step-counter {
    font-size: 0.875rem;
    color: var(--text-muted);
  }

  .wave {
    transition: transform 0.3s ease;
  }

  .wave line {
    filter: drop-shadow(0 0 4px var(--wave-color));
  }
</style>
