<script lang="ts">
  interface Props {
    text: string;
    position?: 'top' | 'bottom' | 'left' | 'right';
    children: import('svelte').Snippet;
  }

  let {
    text,
    position = 'top',
    children,
  }: Props = $props();

  let visible = $state(false);
  let element: HTMLElement;
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<span
  class="tooltip-wrapper"
  bind:this={element}
  onmouseenter={() => visible = true}
  onmouseleave={() => visible = false}
  onfocus={() => visible = true}
  onblur={() => visible = false}
  role="group"
>
  {@render children()}

  {#if visible}
    <span class="tooltip tooltip-{position}" role="tooltip">
      {text}
      <span class="tooltip-arrow"></span>
    </span>
  {/if}
</span>

<style>
  .tooltip-wrapper {
    position: relative;
    display: inline-flex;
  }

  .tooltip {
    position: absolute;
    z-index: 1000;
    padding: 0.5rem 0.75rem;
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--text-primary);
    background: var(--bg-tertiary);
    border: 1px solid var(--text-muted);
    border-radius: 6px;
    white-space: nowrap;
    pointer-events: none;
    animation: tooltip-fade 0.15s ease-out;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  @keyframes tooltip-fade {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .tooltip-arrow {
    position: absolute;
    width: 8px;
    height: 8px;
    background: var(--bg-tertiary);
    border: 1px solid var(--text-muted);
    transform: rotate(45deg);
  }

  .tooltip-top {
    bottom: calc(100% + 8px);
    left: 50%;
    transform: translateX(-50%);
  }

  .tooltip-top .tooltip-arrow {
    bottom: -5px;
    left: 50%;
    margin-left: -4px;
    border-top: none;
    border-left: none;
  }

  .tooltip-bottom {
    top: calc(100% + 8px);
    left: 50%;
    transform: translateX(-50%);
  }

  .tooltip-bottom .tooltip-arrow {
    top: -5px;
    left: 50%;
    margin-left: -4px;
    border-bottom: none;
    border-right: none;
  }

  .tooltip-left {
    right: calc(100% + 8px);
    top: 50%;
    transform: translateY(-50%);
  }

  .tooltip-left .tooltip-arrow {
    right: -5px;
    top: 50%;
    margin-top: -4px;
    border-left: none;
    border-bottom: none;
  }

  .tooltip-right {
    left: calc(100% + 8px);
    top: 50%;
    transform: translateY(-50%);
  }

  .tooltip-right .tooltip-arrow {
    left: -5px;
    top: 50%;
    margin-top: -4px;
    border-right: none;
    border-top: none;
  }
</style>
