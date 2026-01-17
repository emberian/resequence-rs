<script lang="ts">
  import { onMount } from 'svelte';

  interface Shortcut {
    key: string;
    description: string;
    action: () => void;
    ctrl?: boolean;
    shift?: boolean;
  }

  interface Props {
    shortcuts?: Shortcut[];
  }

  let { shortcuts = [] }: Props = $props();

  let showHelp = $state(false);

  const defaultShortcuts: Shortcut[] = [
    { key: '?', description: 'Show keyboard shortcuts', action: () => { showHelp = !showHelp; } },
    { key: 'Escape', description: 'Close dialogs', action: () => { showHelp = false; } },
  ];

  const allShortcuts = $derived<Shortcut[]>([...defaultShortcuts, ...shortcuts]);

  onMount(() => {
    function handleKeydown(e: KeyboardEvent) {
      // Ignore when typing in inputs
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
        return;
      }

      for (const shortcut of allShortcuts) {
        const keyMatches = e.key.toLowerCase() === shortcut.key.toLowerCase() ||
                          e.code === shortcut.key;
        const ctrlMatches = !shortcut.ctrl || (e.ctrlKey || e.metaKey);
        const shiftMatches = !shortcut.shift || e.shiftKey;

        if (keyMatches && ctrlMatches && shiftMatches) {
          e.preventDefault();
          shortcut.action();
          return;
        }
      }
    }

    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });

  function formatKey(shortcut: Shortcut): string {
    const parts: string[] = [];
    if (shortcut.ctrl) parts.push('⌘');
    if (shortcut.shift) parts.push('⇧');
    parts.push(shortcut.key === ' ' ? 'Space' : shortcut.key);
    return parts.join(' + ');
  }
</script>

{#if showHelp}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="shortcuts-backdrop" onclick={() => showHelp = false} role="presentation">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="shortcuts-modal" onclick={(e) => e.stopPropagation()} role="dialog" aria-label="Keyboard shortcuts" tabindex="-1">
      <div class="shortcuts-header">
        <h3>Keyboard Shortcuts</h3>
        <button class="close-btn" onclick={() => showHelp = false} aria-label="Close">✕</button>
      </div>

      <div class="shortcuts-list">
        {#each allShortcuts as shortcut}
          <div class="shortcut-row">
            <kbd class="shortcut-key">{formatKey(shortcut)}</kbd>
            <span class="shortcut-desc">{shortcut.description}</span>
          </div>
        {/each}
      </div>

      <div class="shortcuts-footer">
        <span>Press <kbd>?</kbd> to toggle this help</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .shortcuts-backdrop {
    position: fixed;
    inset: 0;
    z-index: 9999;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    animation: backdrop-fade 0.2s ease;
  }

  @keyframes backdrop-fade {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .shortcuts-modal {
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 16px;
    padding: 1.5rem;
    min-width: 320px;
    max-width: 90vw;
    max-height: 80vh;
    overflow-y: auto;
    animation: modal-pop 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 24px 48px rgba(0, 0, 0, 0.4);
  }

  @keyframes modal-pop {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .shortcuts-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.25rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--bg-tertiary);
  }

  .shortcuts-header h3 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .close-btn {
    padding: 0.25rem 0.5rem;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 1rem;
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.15s ease;
  }

  .close-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .shortcut-key {
    min-width: 80px;
    padding: 0.4rem 0.6rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--text-muted);
    border-radius: 6px;
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-primary);
    text-align: center;
    box-shadow: 0 2px 0 var(--text-muted);
  }

  .shortcut-desc {
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .shortcuts-footer {
    margin-top: 1.25rem;
    padding-top: 1rem;
    border-top: 1px solid var(--bg-tertiary);
    font-size: 0.8rem;
    color: var(--text-muted);
    text-align: center;
  }

  .shortcuts-footer kbd {
    padding: 0.15rem 0.4rem;
    background: var(--bg-tertiary);
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 0.75rem;
  }
</style>
