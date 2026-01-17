<script lang="ts">
  import Hook from './sections/01-Hook.svelte';
  import Problem from './sections/02-Problem.svelte';
  import Events from './sections/03-Events.svelte';
  import Lifecycle from './sections/04-Lifecycle.svelte';
  import Timewaves from './sections/05-Timewaves.svelte';
  import Chronoport from './sections/06-Chronoport.svelte';
  import Paradox from './sections/07-Paradox.svelte';
  import EngineLoop from './sections/08-EngineLoop.svelte';
  import Propagation from './sections/09-Propagation.svelte';
  import TimeWindows from './sections/10-TimeWindows.svelte';
  import Playground from './sections/11-Playground.svelte';

  import { SectionNav, Toast, KeyboardShortcuts } from './lib/components';

  const sections = [
    { id: 'hook', title: 'Introduction', shortTitle: 'Intro' },
    { id: 'problem', title: 'The Problem', shortTitle: 'Problem' },
    { id: 'events', title: 'Event History', shortTitle: 'Events' },
    { id: 'lifecycle', title: 'Entity Lifecycle', shortTitle: 'Lifecycle' },
    { id: 'timewaves', title: 'Timewaves', shortTitle: 'Waves' },
    { id: 'chronoport', title: 'Chronoporting', shortTitle: 'Chronoport' },
    { id: 'paradox', title: 'Paradoxes', shortTitle: 'Paradox' },
    { id: 'engine-loop', title: 'Engine Loop', shortTitle: 'Loop' },
    { id: 'propagation', title: 'Propagation', shortTitle: 'Propagate' },
    { id: 'time-windows', title: 'Time Windows', shortTitle: 'Windows' },
    { id: 'playground', title: 'Playground', shortTitle: 'Play' },
  ];

  function scrollToSection(direction: 'up' | 'down') {
    const currentY = window.scrollY;
    const windowHeight = window.innerHeight;

    for (let i = 0; i < sections.length; i++) {
      const el = document.getElementById(sections[i].id);
      if (!el) continue;

      const rect = el.getBoundingClientRect();

      if (direction === 'down' && rect.top > 100) {
        el.scrollIntoView({ behavior: 'smooth' });
        return;
      }

      if (direction === 'up' && rect.bottom > windowHeight * 0.5) {
        const prevIndex = Math.max(0, i - 1);
        document.getElementById(sections[prevIndex].id)?.scrollIntoView({ behavior: 'smooth' });
        return;
      }
    }
  }

  const shortcuts = [
    { key: 'j', description: 'Next section', action: () => scrollToSection('down') },
    { key: 'k', description: 'Previous section', action: () => scrollToSection('up') },
    { key: 'ArrowDown', description: 'Next section', action: () => scrollToSection('down') },
    { key: 'ArrowUp', description: 'Previous section', action: () => scrollToSection('up') },
    { key: 'Home', description: 'Go to start', action: () => window.scrollTo({ top: 0, behavior: 'smooth' }) },
    { key: 'End', description: 'Go to playground', action: () => document.getElementById('playground')?.scrollIntoView({ behavior: 'smooth' }) },
  ];
</script>

<SectionNav {sections} />
<KeyboardShortcuts {shortcuts} />
<Toast />

<main>
  <div id="hook"><Hook /></div>
  <div id="problem"><Problem /></div>
  <div id="events"><Events /></div>
  <div id="lifecycle"><Lifecycle /></div>
  <div id="timewaves"><Timewaves /></div>
  <div id="chronoport"><Chronoport /></div>
  <div id="paradox"><Paradox /></div>
  <div id="engine-loop"><EngineLoop /></div>
  <div id="propagation"><Propagation /></div>
  <div id="time-windows"><TimeWindows /></div>
  <div id="playground"><Playground /></div>
</main>

<style>
  main {
    width: 100%;
  }
</style>
