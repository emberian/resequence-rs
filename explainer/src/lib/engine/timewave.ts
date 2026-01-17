import type { EntityId, Tick, TimewaveId } from './types';

/**
 * A timewave represents an observer moving through the timeline.
 *
 * Multiple waves can exist simultaneously, each at different positions
 * and moving at different speeds. This enables:
 * - Present-time simulation (speed 1.0)
 * - Slow-motion replay (speed 0.5)
 * - Fast-forward (speed 2.0+)
 * - Backward viewing (negative speed)
 * - Paused observation (speed 0)
 */
export class Timewave {
  readonly id: TimewaveId;

  /** Current time position */
  position: Tick;

  /** Position before last advance (for delta calculations) */
  previousPosition: Tick;

  /** Speed multiplier - can be negative for backward travel */
  speed: number;

  /** Base speed multiplier (usually 1.0) */
  baseSpeed: number = 1.0;

  /** Accumulator for sub-tick movement */
  private fractional: number = 0;

  /** Whether this wave is actively processing */
  isActive: boolean = true;

  /** Whether this wave needs synchronization after propagation */
  needsSync: boolean = false;

  /** Cache: maps EntityId → event timestamp this wave sees */
  private entityViews: Map<EntityId, Tick> = new Map();

  /** Entities that changed since last sync */
  private dirtyEntities: Set<EntityId> = new Set();

  /** Link to previous wave in sorted chain (earlier in time) */
  prevWave: TimewaveId | null = null;

  /** Link to next wave in sorted chain (later in time) */
  nextWave: TimewaveId | null = null;

  constructor(id: TimewaveId, position: Tick, speed: number = 1.0) {
    this.id = id;
    this.position = position;
    this.previousPosition = position;
    this.speed = speed;
  }

  /**
   * Create a stationary wave (speed 0).
   */
  static stationary(id: TimewaveId, position: Tick): Timewave {
    return new Timewave(id, position, 0);
  }

  /**
   * Create a reverse wave (negative speed).
   */
  static reverse(id: TimewaveId, position: Tick, speed: number = -1.0): Timewave {
    return new Timewave(id, position, speed);
  }

  /**
   * Advance the wave by delta ticks, accounting for speed.
   *
   * Handles:
   * - Fractional speeds (0.5 takes 2 ticks to advance 1 position)
   * - Negative speeds (backward movement)
   */
  advance(delta: number): void {
    if (!this.isActive || delta === 0) return;

    this.previousPosition = this.position;
    const advancement = this.speed * this.baseSpeed * delta;
    const newFractional = this.fractional + advancement;

    if (advancement >= 0) {
      // Forward movement
      const wholeTicks = Math.floor(newFractional);
      this.position += wholeTicks;
      this.fractional = newFractional - wholeTicks;
    } else {
      // Backward movement
      if (newFractional >= 0) {
        const wholeTicks = Math.floor(newFractional);
        this.position += wholeTicks;
        this.fractional = newFractional - wholeTicks;
      } else {
        const negativeTicks = Math.ceil(-newFractional);
        this.position = Math.max(0, this.position - negativeTicks);
        this.fractional = newFractional + negativeTicks;
      }
    }

    // Ensure position never goes negative
    if (this.position < 0) {
      this.position = 0;
      this.fractional = 0;
    }
  }

  /**
   * Set wave speed.
   */
  setSpeed(speed: number): void {
    this.speed = speed;
  }

  /**
   * Pause the wave.
   */
  pause(): void {
    this.isActive = false;
  }

  /**
   * Resume the wave.
   */
  resume(): void {
    this.isActive = true;
  }

  /**
   * Instantly jump to a new position.
   */
  jumpTo(position: Tick): void {
    this.previousPosition = this.position;
    this.position = position;
    this.fractional = 0;
  }

  // ============ Entity View Cache Methods ============

  /**
   * Update the cached view of an entity.
   * @param entityId The entity being viewed
   * @param eventTick The timestamp of the event this wave sees
   */
  updateEntityView(entityId: EntityId, eventTick: Tick): void {
    const oldTick = this.entityViews.get(entityId);
    this.entityViews.set(entityId, eventTick);

    if (oldTick !== eventTick) {
      this.dirtyEntities.add(entityId);
    }
  }

  /**
   * Get the cached event timestamp for an entity.
   */
  getEntityView(entityId: EntityId): Tick | undefined {
    return this.entityViews.get(entityId);
  }

  /**
   * Check if an entity's view has changed.
   */
  isDirty(entityId: EntityId): boolean {
    return this.dirtyEntities.has(entityId);
  }

  /**
   * Mark an entity as dirty (view changed).
   */
  markDirty(entityId: EntityId): void {
    this.dirtyEntities.add(entityId);
  }

  /**
   * Clear all dirty flags.
   */
  clearDirty(): void {
    this.dirtyEntities.clear();
    this.needsSync = false;
  }

  /**
   * Get all dirty entity IDs.
   */
  getDirtyEntities(): EntityId[] {
    return Array.from(this.dirtyEntities);
  }

  /**
   * Get all entity views.
   */
  getAllEntityViews(): Map<EntityId, Tick> {
    return new Map(this.entityViews);
  }

  /**
   * Inherit views from another wave (propagation).
   * Used when a faster wave passes a slower wave.
   */
  inheritViewsFrom(other: Timewave): void {
    for (const [entityId, eventTick] of other.entityViews) {
      const myTick = this.entityViews.get(entityId);
      if (myTick === undefined || myTick < eventTick) {
        this.entityViews.set(entityId, eventTick);
        this.dirtyEntities.add(entityId);
      }
    }
  }

  /**
   * Clear all views (for reset).
   */
  clearViews(): void {
    this.entityViews.clear();
    this.dirtyEntities.clear();
  }
}
