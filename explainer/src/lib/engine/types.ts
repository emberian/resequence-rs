/**
 * Core types for the resequence time-travel simulation engine.
 * Mirrors the Rust implementation for WASM validation compatibility.
 */

/** Unique identifier for each entity */
export type EntityId = number;

/** Links temporal duplicates together (entities created via chronoport) */
export type NameId = number;

/** Discrete simulation time units */
export type Tick = number;

/** Unique identifier for each timewave */
export type TimewaveId = number;

/**
 * Entity lifecycle states - tracks an entity through its lifetime.
 * Values match the original Achron engine's high-nibble flags.
 */
export enum LifecycleState {
  /** Empty slot / not yet created */
  Unborn = 0,
  /** Scheduled to spawn (destination of chronoport) */
  Prebirth = 3,
  /** Destroyed/killed */
  Dead = 4,
  /** Mid-time-travel state */
  Chronoporting = 5,
  /** Normal active state */
  Born = 6,
}

/**
 * Check if a lifecycle state is "active" (entity exists and can act)
 */
export function isActiveState(state: LifecycleState): boolean {
  return state === LifecycleState.Born || state === LifecycleState.Prebirth;
}

/**
 * Human-readable names for lifecycle states
 */
export const lifecycleStateNames: Record<LifecycleState, string> = {
  [LifecycleState.Unborn]: 'Unborn',
  [LifecycleState.Prebirth]: 'Prebirth',
  [LifecycleState.Dead]: 'Dead',
  [LifecycleState.Chronoporting]: 'Chronoporting',
  [LifecycleState.Born]: 'Born',
};

/**
 * CSS color class names for lifecycle states
 */
export const lifecycleStateColors: Record<LifecycleState, string> = {
  [LifecycleState.Unborn]: 'var(--state-unborn)',
  [LifecycleState.Prebirth]: 'var(--state-prebirth)',
  [LifecycleState.Dead]: 'var(--state-dead)',
  [LifecycleState.Chronoporting]: 'var(--state-chronoporting)',
  [LifecycleState.Born]: 'var(--state-born)',
};

/** User-defined state data - just needs to be cloneable */
export interface EntityState {
  clone(): EntityState;
}

/** Simple default state for examples */
export class SimpleState implements EntityState {
  constructor(public data: Record<string, unknown> = {}) {}

  clone(): SimpleState {
    return new SimpleState({ ...this.data });
  }

  static default(): SimpleState {
    return new SimpleState();
  }
}
