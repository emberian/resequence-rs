/**
 * WASM bindings for the resequence engine.
 *
 * This module provides optional WASM validation - the JS engine can run
 * independently, but when WASM is loaded, operations can be validated
 * against the Rust reference implementation.
 */

import type { EntityId, Tick, TimewaveId, EntityState } from './types';
import { LifecycleState } from './types';

/** WASM module type (matches wasm-bindgen output) */
interface WasmModule {
  WasmEngine: {
    new(): WasmEngineInstance;
  };
}

interface WasmEngineInstance {
  spawn(data: unknown): number;
  spawn_at(data: unknown, tick: bigint): number;
  current_tick(): bigint;
  tick_by(delta: bigint): void;
  tick(): void;
  get_state(entity_id: number): unknown;
  destroy(entity_id: number): void;
  chronoport(entity_id: number, target_tick: bigint): number;
  get_same_name_entities(entity_id: number): Uint32Array;
  set_time_window(past: bigint | undefined, future: bigint | undefined): void;
  add_timewave(speed: number): number;
  add_timewave_at(tick: bigint, speed: number): number;
  remove_timewave(wave_id: number): boolean;
  get_wave_position(wave_id: number): bigint | undefined;
  entity_count(): number;
  active_entities(): Uint32Array;
  get_lifecycle(entity_id: number): number;
}

/** Singleton WASM module instance */
let wasmModule: WasmModule | null = null;
let wasmLoadPromise: Promise<WasmModule | null> | null = null;

// WASM module path - use a variable to prevent static analysis
const WASM_MODULE_PATH = '../../../wasm/pkg/resequence_wasm';

/**
 * Load the WASM module.
 * Returns null if loading fails (e.g., WASM not built).
 */
export async function loadWasm(): Promise<WasmModule | null> {
  if (wasmModule) return wasmModule;

  if (!wasmLoadPromise) {
    wasmLoadPromise = (async () => {
      try {
        // Dynamic import of WASM package using variable path
        // This prevents Vite from trying to resolve it at build time
        const wasm = await import(/* @vite-ignore */ WASM_MODULE_PATH);
        wasmModule = wasm as unknown as WasmModule;
        console.log('[WASM] Loaded resequence WASM module');
        return wasmModule;
      } catch (e) {
        console.warn('[WASM] Failed to load WASM module:', e);
        return null;
      }
    })();
  }

  return wasmLoadPromise;
}

/**
 * Check if WASM is available.
 */
export function isWasmAvailable(): boolean {
  return wasmModule !== null;
}

/**
 * WASM-backed engine for validation.
 */
export class WasmEngine {
  private instance: WasmEngineInstance;

  private constructor(instance: WasmEngineInstance) {
    this.instance = instance;
  }

  static async create(): Promise<WasmEngine | null> {
    const module = await loadWasm();
    if (!module) return null;

    const instance = new module.WasmEngine();
    return new WasmEngine(instance);
  }

  spawn(state: EntityState): EntityId {
    return this.instance.spawn(state) as EntityId;
  }

  spawnAt(state: EntityState, tick: Tick): EntityId {
    return this.instance.spawn_at(state, BigInt(tick)) as EntityId;
  }

  get currentTick(): Tick {
    return Number(this.instance.current_tick()) as Tick;
  }

  tickBy(delta: number): void {
    this.instance.tick_by(BigInt(delta));
  }

  tick(): void {
    this.instance.tick();
  }

  getState(id: EntityId): EntityState | null {
    const state = this.instance.get_state(id);
    return state as EntityState | null;
  }

  destroy(id: EntityId): void {
    this.instance.destroy(id);
  }

  chronoport(id: EntityId, targetTick: Tick): EntityId {
    return this.instance.chronoport(id, BigInt(targetTick)) as EntityId;
  }

  getSameNameEntities(id: EntityId): EntityId[] {
    const arr = this.instance.get_same_name_entities(id);
    return Array.from(arr) as EntityId[];
  }

  setTimeWindow(past: Tick | null, future: Tick | null): void {
    this.instance.set_time_window(
      past !== null ? BigInt(past) : undefined,
      future !== null ? BigInt(future) : undefined
    );
  }

  addTimewave(speed: number): TimewaveId {
    return this.instance.add_timewave(speed) as TimewaveId;
  }

  addTimewaveAt(tick: Tick, speed: number): TimewaveId {
    return this.instance.add_timewave_at(BigInt(tick), speed) as TimewaveId;
  }

  removeTimewave(id: TimewaveId): boolean {
    return this.instance.remove_timewave(id);
  }

  getWavePosition(id: TimewaveId): Tick | undefined {
    const pos = this.instance.get_wave_position(id);
    return pos !== undefined ? (Number(pos) as Tick) : undefined;
  }

  get entityCount(): number {
    return this.instance.entity_count();
  }

  activeEntities(): EntityId[] {
    const arr = this.instance.active_entities();
    return Array.from(arr) as EntityId[];
  }

  getLifecycle(id: EntityId): LifecycleState {
    return this.instance.get_lifecycle(id) as LifecycleState;
  }
}

/**
 * Validation result comparing JS and WASM engines.
 */
export interface ValidationResult {
  match: boolean;
  differences: string[];
}

/**
 * Compare JS engine state with WASM engine state.
 */
export function validateEngineState(
  jsEntityCount: number,
  jsCurrentTick: Tick,
  jsActiveEntities: EntityId[],
  wasmEngine: WasmEngine
): ValidationResult {
  const differences: string[] = [];

  // Compare entity count
  if (jsEntityCount !== wasmEngine.entityCount) {
    differences.push(
      `Entity count: JS=${jsEntityCount}, WASM=${wasmEngine.entityCount}`
    );
  }

  // Compare current tick
  if (jsCurrentTick !== wasmEngine.currentTick) {
    differences.push(
      `Current tick: JS=${jsCurrentTick}, WASM=${wasmEngine.currentTick}`
    );
  }

  // Compare active entities
  const wasmActive = wasmEngine.activeEntities();
  const jsActiveSet = new Set(jsActiveEntities);
  const wasmActiveSet = new Set(wasmActive);

  const onlyInJs = jsActiveEntities.filter((id) => !wasmActiveSet.has(id));
  const onlyInWasm = wasmActive.filter((id) => !jsActiveSet.has(id));

  if (onlyInJs.length > 0) {
    differences.push(`Entities only in JS: [${onlyInJs.join(', ')}]`);
  }
  if (onlyInWasm.length > 0) {
    differences.push(`Entities only in WASM: [${onlyInWasm.join(', ')}]`);
  }

  return {
    match: differences.length === 0,
    differences,
  };
}
