import { Timeline } from './timeline';
import { Timewave } from './timewave';
import type { EntityId, NameId, Tick, TimewaveId, EntityState } from './types';
import { LifecycleState } from './types';

/** The present wave is always ID 0 */
export const PRESENT_WAVE_ID: TimewaveId = 0;

/**
 * The simulation engine - orchestrates timewaves and the timeline.
 *
 * Core responsibilities:
 * - Managing timewaves (observers at different time positions/speeds)
 * - Running the simulation tick loop
 * - Propagating views between waves (causality)
 */
export class Engine<S extends EntityState> {
  /** The timeline containing all entities */
  readonly timeline: Timeline<S>;

  /** All timewaves indexed by ID */
  private timewaves: Map<TimewaveId, Timewave> = new Map();

  /** Current simulation tick */
  currentTick: Tick = 0;

  /** Next wave ID to assign */
  private nextWaveId: TimewaveId = 0;

  constructor(maxEntities?: number) {
    this.timeline = new Timeline<S>(maxEntities);

    // Create the "present" wave at tick 0 with speed 1.0
    const presentWave = new Timewave(PRESENT_WAVE_ID, 0, 1.0);
    this.timewaves.set(PRESENT_WAVE_ID, presentWave);
    this.nextWaveId = 1;
  }

  // ============ Entity Management ============

  /**
   * Spawn a new entity at the current tick.
   */
  spawn(state: S): EntityId {
    return this.timeline.spawn(state, this.currentTick);
  }

  /**
   * Spawn a new entity at a specific tick.
   */
  spawnAt(state: S, tick: Tick): EntityId {
    return this.timeline.spawn(state, tick);
  }

  /**
   * Get an entity's current state (from present wave's perspective).
   */
  getState(id: EntityId): S | null {
    const entity = this.timeline.get(id);
    return entity?.getStateAt(this.currentTick) ?? null;
  }

  /**
   * Set an entity's state at the current tick.
   */
  setState(id: EntityId, state: S): void {
    this.timeline.addEvent(id, this.currentTick, state);
  }

  /**
   * Destroy an entity at the current tick.
   */
  destroy(id: EntityId): void {
    this.timeline.destroy(id, this.currentTick);
  }

  /**
   * Perform a chronoport at the current tick.
   */
  chronoport(id: EntityId, targetTick: Tick): EntityId {
    return this.timeline.chronoport(id, this.currentTick, targetTick);
  }

  /**
   * Get all temporal duplicates of an entity.
   */
  getSameNameEntities(id: EntityId): EntityId[] {
    return this.timeline.getSameNameEntities(id);
  }

  /**
   * Set time window constraints for chronoport.
   */
  setTimeWindow(past: Tick | null, future: Tick | null): void {
    this.timeline.setTimeWindow(past, future);
  }

  // ============ Timewave Management ============

  /**
   * Add a new timewave at the current tick.
   */
  addTimewave(speed: number): TimewaveId {
    return this.addTimewaveAt(this.currentTick, speed);
  }

  /**
   * Add a new timewave at a specific tick.
   */
  addTimewaveAt(tick: Tick, speed: number): TimewaveId {
    const id = this.nextWaveId++ as TimewaveId;
    const wave = new Timewave(id, tick, speed);
    this.timewaves.set(id, wave);
    return id;
  }

  /**
   * Remove a timewave.
   */
  removeTimewave(id: TimewaveId): boolean {
    if (id === PRESENT_WAVE_ID) return false; // Can't remove present
    return this.timewaves.delete(id);
  }

  /**
   * Get a timewave by ID.
   */
  getTimewave(id: TimewaveId): Timewave | undefined {
    return this.timewaves.get(id);
  }

  /**
   * Get the present wave.
   */
  get presentWave(): Timewave {
    return this.timewaves.get(PRESENT_WAVE_ID)!;
  }

  /**
   * Get all timewaves.
   */
  getAllTimewaves(): Timewave[] {
    return Array.from(this.timewaves.values());
  }

  /**
   * Get wave position.
   */
  getWavePosition(id: TimewaveId): Tick | undefined {
    return this.timewaves.get(id)?.position;
  }

  // ============ Core Simulation Loop ============

  /**
   * Advance the simulation by delta ticks.
   *
   * The 6-step process:
   * 1. Advance each wave by delta × speed
   * 2. Sort waves by position (for propagation order)
   * 3. Link waves into prev/next chain
   * 4. Activate Prebirth entities where waves reach them
   * 5. Refresh wave views and propagate between waves
   * 6. Clear dirty flags
   */
  tickBy(delta: number = 1): void {
    if (delta <= 0) return;

    // Step 1: Advance each wave
    for (const wave of this.timewaves.values()) {
      wave.advance(delta);
    }

    // Step 2: Sort waves by position
    const sortedWaves = Array.from(this.timewaves.values()).sort(
      (a, b) => a.position - b.position
    );

    // Step 3: Link waves into chain
    for (let i = 0; i < sortedWaves.length; i++) {
      sortedWaves[i].prevWave = i > 0 ? sortedWaves[i - 1].id : null;
      sortedWaves[i].nextWave = i < sortedWaves.length - 1 ? sortedWaves[i + 1].id : null;
    }

    // Step 4: Activate Prebirth entities
    const activationTicks = new Set(sortedWaves.map((w) => w.position));
    for (const tick of activationTicks) {
      this.timeline.activatePrebirthEntities(tick);
    }

    // Step 5: Refresh wave views
    for (const wave of sortedWaves) {
      for (const entity of this.timeline.allEntities()) {
        const event = entity.getEventAt(wave.position);
        if (event) {
          wave.updateEntityView(entity.id, event.timestamp);
        }
      }
    }

    // Step 5b: Propagate between waves (causality)
    for (let i = 1; i < sortedWaves.length; i++) {
      const prevWave = sortedWaves[i - 1];
      const currWave = sortedWaves[i];

      // Check if currWave passed prevWave this tick
      const prevPos = currWave.previousPosition;
      const currPos = currWave.position;
      const earlierPos = prevWave.position;

      if (prevPos <= earlierPos && currPos > earlierPos) {
        // Current wave crossed the previous wave's position
        // Inherit views from the slower wave
        currWave.inheritViewsFrom(prevWave);
        currWave.needsSync = true;
      }
    }

    // Step 6: Clear dirty flags
    for (const wave of this.timewaves.values()) {
      wave.clearDirty();
    }

    // Update current tick (present wave position)
    this.currentTick = this.presentWave.position;
  }

  /**
   * Single tick advance (convenience method).
   */
  tick(): void {
    this.tickBy(1);
  }

  // ============ Query Methods ============

  /**
   * Get an entity's state from a specific wave's perspective.
   */
  getStateAtWave(id: EntityId, waveId: TimewaveId): S | null {
    const wave = this.timewaves.get(waveId);
    if (!wave) return null;

    const entity = this.timeline.get(id);
    return entity?.getStateAt(wave.position) ?? null;
  }

  /**
   * Get all active entities at the current tick.
   */
  activeEntities(): EntityId[] {
    const result: EntityId[] = [];
    for (const entity of this.timeline.allEntities()) {
      if (entity.isActiveAt(this.currentTick)) {
        result.push(entity.id);
      }
    }
    return result;
  }

  /**
   * Get entity count.
   */
  get entityCount(): number {
    return this.timeline.size;
  }
}
