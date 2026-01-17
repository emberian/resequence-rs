import {
  type EntityId,
  type NameId,
  type Tick,
  type EntityState,
  LifecycleState,
  isActiveState,
} from './types';

/**
 * A timestamped state change for an entity.
 * The core building block - we store events, not snapshots.
 */
export interface Event<S extends EntityState> {
  timestamp: Tick;
  lifecycle: LifecycleState;
  state: S;
}

/**
 * Create a new event with the Born lifecycle state
 */
export function bornEvent<S extends EntityState>(timestamp: Tick, state: S): Event<S> {
  return { timestamp, lifecycle: LifecycleState.Born, state };
}

/**
 * Create a new event with the Prebirth lifecycle state
 */
export function prebirthEvent<S extends EntityState>(timestamp: Tick, state: S): Event<S> {
  return { timestamp, lifecycle: LifecycleState.Prebirth, state };
}

/**
 * Create a new event with the Dead lifecycle state
 */
export function deadEvent<S extends EntityState>(timestamp: Tick, state: S): Event<S> {
  return { timestamp, lifecycle: LifecycleState.Dead, state };
}

/**
 * Create a new event with the Chronoporting lifecycle state
 */
export function chronoportingEvent<S extends EntityState>(timestamp: Tick, state: S): Event<S> {
  return { timestamp, lifecycle: LifecycleState.Chronoporting, state };
}

/**
 * An entity with its complete event history.
 *
 * Key design: State is stored as a list of timestamped events, not full snapshots.
 * This enables O(log n) historical queries via binary search.
 */
export class Entity<S extends EntityState> {
  /** Unique identifier */
  readonly id: EntityId;

  /** Links temporal duplicates (same NameId = same "person" across time) */
  readonly nameId: NameId;

  /** When this entity was first created */
  readonly createdAt: Tick;

  /** Event history, always sorted by timestamp */
  private events: Event<S>[];

  constructor(id: EntityId, nameId: NameId, createdAt: Tick, initialEvent: Event<S>) {
    this.id = id;
    this.nameId = nameId;
    this.createdAt = createdAt;
    this.events = [initialEvent];
  }

  /**
   * Create a new entity that starts as Born at the given tick.
   */
  static spawn<S extends EntityState>(
    id: EntityId,
    nameId: NameId,
    tick: Tick,
    state: S
  ): Entity<S> {
    return new Entity(id, nameId, tick, bornEvent(tick, state));
  }

  /**
   * Create a new entity that starts as Prebirth (scheduled to appear).
   */
  static spawnPrebirth<S extends EntityState>(
    id: EntityId,
    nameId: NameId,
    tick: Tick,
    state: S
  ): Entity<S> {
    return new Entity(id, nameId, tick, prebirthEvent(tick, state));
  }

  /**
   * Get the event at or before the given tick.
   * Uses binary search for O(log n) complexity.
   */
  getEventAt(tick: Tick): Event<S> | null {
    if (this.events.length === 0) return null;
    if (tick < this.events[0].timestamp) return null;

    // Binary search for the rightmost event <= tick
    let left = 0;
    let right = this.events.length - 1;

    while (left < right) {
      const mid = Math.ceil((left + right + 1) / 2);
      if (this.events[mid].timestamp <= tick) {
        left = mid;
      } else {
        right = mid - 1;
      }
    }

    return this.events[left];
  }

  /**
   * Get the state at a specific tick.
   */
  getStateAt(tick: Tick): S | null {
    const event = this.getEventAt(tick);
    return event?.state ?? null;
  }

  /**
   * Get the lifecycle state at a specific tick.
   */
  getLifecycleAt(tick: Tick): LifecycleState {
    const event = this.getEventAt(tick);
    return event?.lifecycle ?? LifecycleState.Unborn;
  }

  /**
   * Check if the entity is active (Born or Prebirth) at the given tick.
   */
  isActiveAt(tick: Tick): boolean {
    return isActiveState(this.getLifecycleAt(tick));
  }

  /**
   * Get the most recent event.
   */
  latestEvent(): Event<S> {
    return this.events[this.events.length - 1];
  }

  /**
   * Get all events (for visualization).
   */
  allEvents(): readonly Event<S>[] {
    return this.events;
  }

  /**
   * Add an event, maintaining sorted order.
   */
  addEvent(event: Event<S>): void {
    // Find insertion point
    let insertIndex = this.events.length;
    for (let i = this.events.length - 1; i >= 0; i--) {
      if (this.events[i].timestamp <= event.timestamp) {
        insertIndex = i + 1;
        break;
      }
      if (i === 0) {
        insertIndex = 0;
      }
    }
    this.events.splice(insertIndex, 0, event);
  }

  /**
   * Set the entity's state at the given tick (creates a Born event).
   */
  setState(tick: Tick, state: S): void {
    this.addEvent(bornEvent(tick, state));
  }

  /**
   * Mark the entity as dead at the given tick.
   */
  destroy(tick: Tick): void {
    const currentState = this.getStateAt(tick);
    if (currentState) {
      this.addEvent(deadEvent(tick, currentState.clone() as S));
    }
  }

  /**
   * Mark the entity as chronoporting at the given tick.
   */
  markChronoporting(tick: Tick): void {
    const currentState = this.getStateAt(tick);
    if (currentState) {
      this.addEvent(chronoportingEvent(tick, currentState.clone() as S));
    }
  }

  /**
   * Check if the entity needs activation at the given tick.
   * Returns true if the entity is in Prebirth state at this tick.
   */
  needsActivationAt(tick: Tick): boolean {
    const event = this.getEventAt(tick);
    return event !== null && event.lifecycle === LifecycleState.Prebirth;
  }

  /**
   * Activate a Prebirth entity at the given tick (transition to Born).
   */
  activateAt(tick: Tick): void {
    const currentState = this.getStateAt(tick);
    if (currentState && this.needsActivationAt(tick)) {
      this.addEvent(bornEvent(tick, currentState.clone() as S));
    }
  }

  /**
   * Serialize the entity to a plain object for storage.
   */
  serialize(): {
    id: EntityId;
    nameId: NameId;
    createdAt: Tick;
    events: { timestamp: Tick; lifecycle: LifecycleState; state: unknown }[];
  } {
    return {
      id: this.id,
      nameId: this.nameId,
      createdAt: this.createdAt,
      events: this.events.map(e => ({
        timestamp: e.timestamp,
        lifecycle: e.lifecycle,
        state: e.state,
      })),
    };
  }
}
