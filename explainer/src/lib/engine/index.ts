// Core types
export {
  type EntityId,
  type NameId,
  type Tick,
  type TimewaveId,
  type EntityState,
  LifecycleState,
  isActiveState,
  lifecycleStateNames,
  lifecycleStateColors,
  SimpleState,
} from './types';

// Entity and events
export {
  Entity,
  type Event,
  bornEvent,
  prebirthEvent,
  deadEvent,
  chronoportingEvent,
} from './entity';

// Timeline
export { Timeline, TimelineError } from './timeline';

// Timewave
export { Timewave } from './timewave';

// Engine
export { Engine, PRESENT_WAVE_ID } from './engine';

// Note: WASM bindings are in './wasm.ts' but not exported by default
// since the WASM module may not be built. Import directly if needed:
// import { loadWasm, WasmEngine } from '$lib/engine/wasm';
