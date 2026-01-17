import { Entity, prebirthEvent } from './entity';
import type { EntityId, NameId, Tick, EntityState } from './types';
import { LifecycleState } from './types';

/**
 * Error types for timeline operations.
 */
export class TimelineError extends Error {
  constructor(
    message: string,
    public readonly code:
      | 'ENTITY_NOT_FOUND'
      | 'TIMELINE_FULL'
      | 'INVALID_CHRONOPORT'
      | 'CHRONOPORT_PAST_WINDOW'
      | 'CHRONOPORT_FUTURE_WINDOW'
  ) {
    super(message);
    this.name = 'TimelineError';
  }
}

/**
 * The timeline manages all entities and their temporal relationships.
 *
 * Key responsibilities:
 * - Entity creation and lifecycle management
 * - Same-name groups for temporal duplicates (chronoport links)
 * - Time window constraints for chronoport
 */
export class Timeline<S extends EntityState> {
  private entities: Map<EntityId, Entity<S>> = new Map();
  private sameNameGroups: Map<NameId, EntityId[]> = new Map();
  private nextId: number = 0;
  private nextNameId: number = 0;
  private maxEntities: number;

  /** Time window constraint - how far back chronoport can go */
  windowPast: Tick | null = null;

  /** Time window constraint - how far forward chronoport can go */
  windowFuture: Tick | null = null;

  constructor(maxEntities: number = 10000) {
    this.maxEntities = maxEntities;
  }

  /**
   * Spawn a new entity at the given tick with a unique NameId.
   */
  spawn(state: S, tick: Tick): EntityId {
    if (this.entities.size >= this.maxEntities) {
      throw new TimelineError(`Timeline full (max ${this.maxEntities})`, 'TIMELINE_FULL');
    }

    const id = this.nextId++ as EntityId;
    const nameId = this.nextNameId++ as NameId;

    const entity = Entity.spawn(id, nameId, tick, state);
    this.entities.set(id, entity);

    // Create new same-name group
    this.sameNameGroups.set(nameId, [id]);

    return id;
  }

  /**
   * Spawn a temporal duplicate with an existing NameId (used by chronoport).
   * The duplicate starts in Prebirth state.
   */
  spawnWithName(state: S, tick: Tick, nameId: NameId): EntityId {
    if (this.entities.size >= this.maxEntities) {
      throw new TimelineError(`Timeline full (max ${this.maxEntities})`, 'TIMELINE_FULL');
    }

    const id = this.nextId++ as EntityId;
    const entity = Entity.spawnPrebirth(id, nameId, tick, state);
    this.entities.set(id, entity);

    // Add to existing same-name group
    const group = this.sameNameGroups.get(nameId);
    if (group) {
      group.push(id);
    } else {
      this.sameNameGroups.set(nameId, [id]);
    }

    return id;
  }

  /**
   * Get an entity by ID.
   */
  get(id: EntityId): Entity<S> | undefined {
    return this.entities.get(id);
  }

  /**
   * Get an entity's event at the given tick.
   */
  getEventAt(id: EntityId, tick: Tick) {
    const entity = this.entities.get(id);
    return entity?.getEventAt(tick) ?? null;
  }

  /**
   * Get all entities that share the same NameId (temporal duplicates).
   */
  getSameNameEntities(id: EntityId): EntityId[] {
    const entity = this.entities.get(id);
    if (!entity) return [];

    return this.sameNameGroups.get(entity.nameId) ?? [];
  }

  /**
   * Add a state change event to an entity.
   */
  addEvent(id: EntityId, tick: Tick, state: S): void {
    const entity = this.entities.get(id);
    if (!entity) {
      throw new TimelineError(`Entity ${id} not found`, 'ENTITY_NOT_FOUND');
    }
    entity.setState(tick, state);
  }

  /**
   * Mark an entity as dead at the given tick.
   */
  destroy(id: EntityId, tick: Tick): void {
    const entity = this.entities.get(id);
    if (!entity) {
      throw new TimelineError(`Entity ${id} not found`, 'ENTITY_NOT_FOUND');
    }
    entity.destroy(tick);
  }

  /**
   * Set time window constraints for chronoport.
   */
  setTimeWindow(past: Tick | null, future: Tick | null): void {
    this.windowPast = past;
    this.windowFuture = future;
  }

  /**
   * Perform a chronoport: send an entity from currentTick to targetTick.
   *
   * This creates a temporal duplicate:
   * - Source entity is marked as Chronoporting
   * - New entity (Prebirth) appears at targetTick + 1 with same NameId
   *
   * Returns the new entity's ID.
   */
  chronoport(id: EntityId, currentTick: Tick, targetTick: Tick): EntityId {
    const entity = this.entities.get(id);
    if (!entity) {
      throw new TimelineError(`Entity ${id} not found`, 'ENTITY_NOT_FOUND');
    }

    if (this.entities.size >= this.maxEntities) {
      throw new TimelineError(`Timeline full (max ${this.maxEntities})`, 'TIMELINE_FULL');
    }

    // Validate time window constraints
    if (this.windowPast !== null) {
      const minAllowed = currentTick > this.windowPast ? currentTick - this.windowPast : 0;
      if (targetTick < minAllowed) {
        throw new TimelineError(
          `Chronoport target ${targetTick} is before allowed window (min: ${minAllowed})`,
          'CHRONOPORT_PAST_WINDOW'
        );
      }
    }

    if (this.windowFuture !== null) {
      const maxAllowed = currentTick + this.windowFuture;
      if (targetTick > maxAllowed) {
        throw new TimelineError(
          `Chronoport target ${targetTick} is after allowed window (max: ${maxAllowed})`,
          'CHRONOPORT_FUTURE_WINDOW'
        );
      }
    }

    // Get current state
    const currentState = entity.getStateAt(currentTick);
    if (!currentState) {
      throw new TimelineError(`Entity ${id} has no state at tick ${currentTick}`, 'ENTITY_NOT_FOUND');
    }

    // Mark source as chronoporting
    entity.markChronoporting(currentTick);

    // Create temporal duplicate at target + 1 (Prebirth state)
    const duplicateId = this.spawnWithName(currentState.clone() as S, targetTick + 1, entity.nameId);

    return duplicateId;
  }

  /**
   * Activate all Prebirth entities at the given tick.
   * Called when a timewave reaches entities scheduled to spawn.
   */
  activatePrebirthEntities(tick: Tick): void {
    for (const entity of this.entities.values()) {
      if (entity.needsActivationAt(tick)) {
        entity.activateAt(tick);
      }
    }
  }

  /**
   * Get all entities (for iteration).
   */
  allEntities(): IterableIterator<Entity<S>> {
    return this.entities.values();
  }

  /**
   * Get entity count.
   */
  get size(): number {
    return this.entities.size;
  }

  /**
   * Get all entity IDs.
   */
  allEntityIds(): EntityId[] {
    return Array.from(this.entities.keys());
  }
}
