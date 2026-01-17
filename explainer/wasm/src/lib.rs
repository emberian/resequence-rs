//! WASM bindings for the resequence time-travel simulation engine.
//!
//! This module exposes the core engine functionality to JavaScript,
//! enabling validation of the JS implementation against the Rust reference.

use resequence::{Engine, EntityId, LifecycleState, NameId, Tick, TimewaveId};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Simple state for WASM demos - just stores arbitrary JSON data
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct WasmState {
    pub data: serde_json::Value,
}

impl resequence::EntityState for WasmState {}

/// WASM-exposed engine wrapper
#[wasm_bindgen]
pub struct WasmEngine {
    inner: Engine<WasmState>,
}

#[wasm_bindgen]
impl WasmEngine {
    /// Create a new engine instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            inner: Engine::new(),
        }
    }

    /// Spawn an entity at the current tick
    pub fn spawn(&mut self, data: JsValue) -> u32 {
        let state = WasmState {
            data: serde_wasm_bindgen::from_value(data).unwrap_or(serde_json::Value::Null),
        };
        self.inner.spawn(state).0
    }

    /// Spawn an entity at a specific tick
    pub fn spawn_at(&mut self, data: JsValue, tick: u64) -> u32 {
        let state = WasmState {
            data: serde_wasm_bindgen::from_value(data).unwrap_or(serde_json::Value::Null),
        };
        self.inner.spawn_at(state, tick).0
    }

    /// Get the current tick
    pub fn current_tick(&self) -> u64 {
        self.inner.current_tick
    }

    /// Advance simulation by delta ticks
    pub fn tick_by(&mut self, delta: u64) {
        self.inner.tick_by(delta);
    }

    /// Single tick advance
    pub fn tick(&mut self) {
        self.inner.tick();
    }

    /// Get entity state at current tick
    pub fn get_state(&self, entity_id: u32) -> JsValue {
        match self.inner.get_state(EntityId(entity_id)) {
            Some(state) => serde_wasm_bindgen::to_value(&state.data).unwrap_or(JsValue::NULL),
            None => JsValue::NULL,
        }
    }

    /// Destroy an entity at current tick
    pub fn destroy(&mut self, entity_id: u32) -> Result<(), JsValue> {
        self.inner
            .destroy(EntityId(entity_id))
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    /// Chronoport an entity to a target tick
    pub fn chronoport(&mut self, entity_id: u32, target_tick: u64) -> Result<u32, JsValue> {
        self.inner
            .chronoport(EntityId(entity_id), target_tick)
            .map(|id| id.0)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    /// Get same-name entities (temporal duplicates)
    pub fn get_same_name_entities(&self, entity_id: u32) -> Vec<u32> {
        self.inner
            .get_same_name_entities(EntityId(entity_id))
            .into_iter()
            .map(|id| id.0)
            .collect()
    }

    /// Set time window constraints
    pub fn set_time_window(&mut self, past: Option<u64>, future: Option<u64>) {
        self.inner.set_time_window(past, future);
    }

    /// Add a new timewave at current tick
    pub fn add_timewave(&mut self, speed: f64) -> u32 {
        self.inner.add_timewave(speed).0
    }

    /// Add a new timewave at specific tick
    pub fn add_timewave_at(&mut self, tick: u64, speed: f64) -> u32 {
        self.inner.add_timewave_at(tick, speed).0
    }

    /// Remove a timewave
    pub fn remove_timewave(&mut self, wave_id: u32) -> bool {
        self.inner.remove_timewave(TimewaveId(wave_id))
    }

    /// Get wave position
    pub fn get_wave_position(&self, wave_id: u32) -> Option<u64> {
        self.inner.get_wave_position(TimewaveId(wave_id))
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.inner.entity_count()
    }

    /// Get all active entity IDs at current tick
    pub fn active_entities(&self) -> Vec<u32> {
        self.inner.active_entities().into_iter().map(|id| id.0).collect()
    }

    /// Get entity lifecycle state at current tick
    pub fn get_lifecycle(&self, entity_id: u32) -> u8 {
        if let Some(entity) = self.inner.timeline.get(EntityId(entity_id)) {
            entity.get_lifecycle_at(self.inner.current_tick) as u8
        } else {
            LifecycleState::Unborn as u8
        }
    }
}

/// Initialize WASM module (called automatically)
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

// Ensure console_error_panic_hook is available
mod console_error_panic_hook {
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn set_once() {
        INIT.call_once(|| {
            std::panic::set_hook(Box::new(|info| {
                web_sys::console::error_1(&format!("{}", info).into());
            }));
        });
    }
}
