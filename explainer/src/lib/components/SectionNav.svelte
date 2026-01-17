<script lang="ts">
  import { onMount } from 'svelte';

  interface Section {
    id: string;
    title: string;
    shortTitle?: string;
  }

  interface Props {
    sections: Section[];
  }

  let { sections }: Props = $props();

  let activeIndex = $state(0);
  let isVisible = $state(true);
  let isExpanded = $state(false);
  let lastScrollY = 0;

  onMount(() => {
    // Create intersection observers for each section
    const observers: IntersectionObserver[] = [];

    sections.forEach((section, index) => {
      const element = document.getElementById(section.id);
      if (!element) return;

      const observer = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting && entry.intersectionRatio > 0.3) {
              activeIndex = index;
            }
          });
        },
        { threshold: [0.3], rootMargin: '-20% 0px -60% 0px' }
      );

      observer.observe(element);
      observers.push(observer);
    });

    // Hide nav when scrolling down, show when scrolling up
    const handleScroll = () => {
      const currentY = window.scrollY;
      isVisible = currentY < lastScrollY || currentY < 100;
      lastScrollY = currentY;
    };

    window.addEventListener('scroll', handleScroll, { passive: true });

    return () => {
      observers.forEach(o => o.disconnect());
      window.removeEventListener('scroll', handleScroll);
    };
  });

  function scrollToSection(index: number) {
    const section = sections[index];
    const element = document.getElementById(section.id);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth' });
      isExpanded = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'ArrowUp' && activeIndex > 0) {
      e.preventDefault();
      scrollToSection(activeIndex - 1);
    } else if (e.key === 'ArrowDown' && activeIndex < sections.length - 1) {
      e.preventDefault();
      scrollToSection(activeIndex + 1);
    }
  }

  const progress = $derived((activeIndex / (sections.length - 1)) * 100);
</script>

<svelte:window onkeydown={handleKeydown} />

<nav class="section-nav" class:visible={isVisible} class:expanded={isExpanded} aria-label="Section navigation">
  <!-- Progress bar -->
  <div class="progress-track">
    <div class="progress-fill" style="height: {progress}%"></div>
  </div>

  <!-- Compact view: dots -->
  <div class="nav-dots">
    {#each sections as section, i}
      <button
        class="nav-dot"
        class:active={i === activeIndex}
        class:completed={i < activeIndex}
        onclick={() => scrollToSection(i)}
        onmouseenter={() => isExpanded = true}
        title={section.title}
        aria-label="Go to {section.title}"
        aria-current={i === activeIndex ? 'step' : undefined}
      >
        <span class="dot-inner"></span>
      </button>
    {/each}
  </div>

  <!-- Expanded view: full titles -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="nav-expanded" onmouseleave={() => isExpanded = false} role="menu" tabindex="-1">
    <div class="nav-header">
      <span class="nav-title">Contents</span>
      <span class="nav-progress">{activeIndex + 1}/{sections.length}</span>
    </div>
    <ul class="nav-list">
      {#each sections as section, i}
        <li>
          <button
            class="nav-item"
            class:active={i === activeIndex}
            class:completed={i < activeIndex}
            onclick={() => scrollToSection(i)}
          >
            <span class="nav-number">{String(i + 1).padStart(2, '0')}</span>
            <span class="nav-label">{section.shortTitle || section.title}</span>
          </button>
        </li>
      {/each}
    </ul>
  </div>
</nav>

<style>
  .section-nav {
    position: fixed;
    right: 1.5rem;
    top: 50%;
    transform: translateY(-50%);
    z-index: 100;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: opacity 0.3s ease, transform 0.3s ease;
  }

  .section-nav:not(.visible) {
    opacity: 0;
    transform: translateY(-50%) translateX(20px);
    pointer-events: none;
  }

  .progress-track {
    width: 3px;
    height: 200px;
    background: var(--bg-tertiary);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    width: 100%;
    background: linear-gradient(180deg, var(--color-present), var(--color-future));
    border-radius: 2px;
    transition: height 0.3s ease;
  }

  .nav-dots {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0.5rem;
  }

  .nav-dot {
    width: 12px;
    height: 12px;
    padding: 0;
    border: none;
    background: transparent;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .dot-inner {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--bg-tertiary);
    border: 2px solid var(--text-muted);
    transition: all 0.2s ease;
  }

  .nav-dot:hover .dot-inner {
    transform: scale(1.3);
    border-color: var(--color-present);
  }

  .nav-dot.completed .dot-inner {
    background: var(--state-born);
    border-color: var(--state-born);
  }

  .nav-dot.active .dot-inner {
    background: var(--color-present);
    border-color: var(--color-present);
    transform: scale(1.5);
    box-shadow: 0 0 8px var(--color-present);
  }

  .nav-expanded {
    position: absolute;
    right: 100%;
    top: 50%;
    transform: translateY(-50%);
    margin-right: 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--bg-tertiary);
    border-radius: 12px;
    padding: 1rem;
    min-width: 220px;
    opacity: 0;
    visibility: hidden;
    transition: all 0.2s ease;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }

  .section-nav.expanded .nav-expanded {
    opacity: 1;
    visibility: visible;
  }

  .nav-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.75rem;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid var(--bg-tertiary);
  }

  .nav-title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-muted);
  }

  .nav-progress {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--color-present);
  }

  .nav-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: none;
    background: transparent;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: left;
  }

  .nav-item:hover {
    background: var(--bg-tertiary);
  }

  .nav-number {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    color: var(--text-muted);
    min-width: 1.5rem;
  }

  .nav-label {
    font-size: 0.85rem;
    color: var(--text-secondary);
    transition: color 0.15s ease;
  }

  .nav-item:hover .nav-label {
    color: var(--text-primary);
  }

  .nav-item.active {
    background: rgba(88, 166, 255, 0.1);
  }

  .nav-item.active .nav-number {
    color: var(--color-present);
  }

  .nav-item.active .nav-label {
    color: var(--color-present);
    font-weight: 500;
  }

  .nav-item.completed .nav-number {
    color: var(--state-born);
  }

  @media (max-width: 900px) {
    .section-nav {
      right: 0.75rem;
    }

    .nav-expanded {
      display: none;
    }
  }
</style>
