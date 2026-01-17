<script lang="ts">
  import { onMount } from 'svelte';
  import scrollama from 'scrollama';

  interface Props {
    offset?: number;
    debug?: boolean;
    children?: import('svelte').Snippet;
    sticky?: import('svelte').Snippet<[{ progress: number; direction: 'up' | 'down' }]>;
    onStepEnter?: (index: number, direction: 'up' | 'down') => void;
    onStepExit?: (index: number, direction: 'up' | 'down') => void;
    onStepProgress?: (index: number, progress: number) => void;
  }

  let {
    offset = 0.5,
    debug = false,
    children,
    sticky,
    onStepEnter,
    onStepExit,
    onStepProgress,
  }: Props = $props();

  let container: HTMLElement;
  let currentIndex = $state(0);
  let progress = $state(0);
  let direction = $state<'up' | 'down'>('down');

  onMount(() => {
    const scroller = scrollama();

    scroller
      .setup({
        step: '.scrolly-step',
        offset,
        progress: true,
        debug,
      })
      .onStepEnter((response) => {
        currentIndex = response.index;
        direction = response.direction as 'up' | 'down';
        onStepEnter?.(response.index, direction);
      })
      .onStepExit((response) => {
        onStepExit?.(response.index, response.direction as 'up' | 'down');
      })
      .onStepProgress((response) => {
        progress = response.progress;
        onStepProgress?.(response.index, response.progress);
      });

    const handleResize = () => scroller.resize();
    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      scroller.destroy();
    };
  });
</script>

<div class="scrolly-container" bind:this={container}>
  <div class="scrolly-sticky">
    {#if sticky}
      {@render sticky({ progress, direction })}
    {/if}
  </div>
  <div class="scrolly-steps">
    {@render children?.()}
  </div>
</div>

<style>
  .scrolly-container {
    position: relative;
  }

  .scrolly-sticky {
    position: sticky;
    top: 0;
    height: 100vh;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .scrolly-steps {
    position: relative;
    pointer-events: none;
  }

  :global(.scrolly-step) {
    min-height: 80vh;
    padding: 2rem;
    display: flex;
    align-items: center;
    pointer-events: auto;
  }
</style>
