<script lang="ts" module>
  export interface ToastMessage {
    id: number;
    message: string;
    type: 'info' | 'success' | 'warning' | 'error';
    duration?: number;
  }

  let toasts = $state<ToastMessage[]>([]);
  let nextId = 0;

  export function showToast(message: string, type: ToastMessage['type'] = 'info', duration = 4000) {
    const id = nextId++;
    const toast: ToastMessage = { id, message, type, duration };
    toasts = [...toasts, toast];

    if (duration > 0) {
      setTimeout(() => {
        dismissToast(id);
      }, duration);
    }

    return id;
  }

  export function dismissToast(id: number) {
    toasts = toasts.filter(t => t.id !== id);
  }
</script>

<script lang="ts">
  const typeIcons = {
    info: '💡',
    success: '✓',
    warning: '⚠',
    error: '✕',
  };

  const typeColors = {
    info: 'var(--color-present)',
    success: 'var(--state-born)',
    warning: 'var(--color-chronoport)',
    error: 'var(--color-paradox)',
  };
</script>

{#if toasts.length > 0}
  <div class="toast-container" role="region" aria-label="Notifications">
    {#each toasts as toast (toast.id)}
      <div
        class="toast toast-{toast.type}"
        style="--toast-color: {typeColors[toast.type]}"
        role="alert"
      >
        <span class="toast-icon">{typeIcons[toast.type]}</span>
        <span class="toast-message">{toast.message}</span>
        <button
          class="toast-dismiss"
          onclick={() => dismissToast(toast.id)}
          aria-label="Dismiss notification"
        >
          ✕
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .toast-container {
    position: fixed;
    bottom: 1.5rem;
    right: 1.5rem;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    pointer-events: none;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.875rem 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--toast-color);
    border-left: 4px solid var(--toast-color);
    border-radius: 8px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    pointer-events: auto;
    animation: toast-slide 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    max-width: 400px;
  }

  @keyframes toast-slide {
    from {
      opacity: 0;
      transform: translateX(100%);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .toast-icon {
    flex-shrink: 0;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--toast-color);
    color: var(--bg-primary);
    font-size: 0.8rem;
    font-weight: 700;
  }

  .toast-message {
    flex: 1;
    font-size: 0.9rem;
    color: var(--text-primary);
    line-height: 1.4;
  }

  .toast-dismiss {
    flex-shrink: 0;
    padding: 0.25rem;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 0.875rem;
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.15s ease;
  }

  .toast-dismiss:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  @media (max-width: 600px) {
    .toast-container {
      left: 1rem;
      right: 1rem;
      bottom: 1rem;
    }

    .toast {
      max-width: 100%;
    }
  }
</style>
