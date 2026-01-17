//! # Resequence
//!
//! A time-travel simulation engine based on patterns reverse-engineered from
//! Achron's Resequence engine (2011). This crate preserves the intellectual heritage
//! of Achron's innovative time manipulation mechanics for future game development
//! and research.
//!
//! ## Design Patterns
//!
//! The engine implements four core patterns from the original:
//!
//! ### Pattern 1: Event-Linked Entities
//!
//! Entities store their history as a list of timestamped events, not full snapshots.
//! This provides O(log n) historical queries via binary search while minimizing storage.
//!
//! ### Pattern 2: Iterator-Based Time Views (Timewaves)
//!
//! Timewaves are execution contexts that move through time at variable speeds.
//! Multiple waves can view the same timeline at different positions, enabling
//! parallel timeline observation, replay debugging, and what-if analysis.
//!
//! ### Pattern 3: Identity Linking (Same-Name Groups)
//!
//! When entities travel through time (chronoport), they create linked duplicates
//! sharing a "name ID". This enables paradox detection - scripts can iterate
//! through all temporal copies of an entity to detect conflicts.
//!
//! ### Pattern 4: Sorted Wave Processing with Propagation
//!
//! Timewaves are sorted by position each tick. Earlier waves propagate state
//! changes to later waves, ensuring causal consistency. Entity view caches
//! maintain what each wave "sees" for efficient lookups.
//!
//! ## Lifecycle States
//!
//! Entities transition through lifecycle states matching the original engine:
//!
//! - `Unborn`: Empty slot or not yet scheduled
//! - `Prebirth`: Scheduled to appear (e.g., destination of chronoport)
//! - `Born`: Normal active state
//! - `Chronoporting`: Mid-time-travel
//! - `Dead`: Destroyed
//!
//! Prebirth entities automatically transition to Born when a timewave reaches them.
//!
//! ## Time Window Constraints
//!
//! Chronoport operations can be constrained to a time window, preventing
//! entities from traveling too far into the past or future.
//!
//! ## Example
//!
//! ```rust
//! use resequence::{Engine, EntityState};
//!
//! #[derive(Clone, Default, Debug)]
//! struct UnitState {
//!     hp: i32,
//!     x: f32,
//!     y: f32,
//! }
//!
//! impl EntityState for UnitState {}
//!
//! let mut engine = Engine::<UnitState>::new();
//!
//! // Spawn a unit
//! let unit = engine.spawn(UnitState { hp: 100, x: 0.0, y: 0.0 });
//!
//! // Advance time
//! for _ in 0..10 {
//!     engine.tick();
//! }
//!
//! // Time travel! Create a duplicate at t=5
//! engine.chronoport(unit, 5).unwrap();
//!
//! // Now there are two linked copies of the unit
//! let duplicates = engine.get_same_name_entities(unit);
//! assert_eq!(duplicates.len(), 2);
//! ```
//!
//! ## Applications Beyond Games
//!
//! These patterns are useful for:
//!
//! - **Simulation debugging**: Add past observation waves to inspect state
//! - **What-if analysis**: Fork timelines, apply different decisions, compare
//! - **Undo/redo systems**: Position-based state views without explicit snapshots
//! - **Distributed consensus**: Multiple nodes as waves at different timestamps

mod entity;
mod error;
mod timeline;
mod timewave;

pub use entity::{Entity, EntityId, EntityState, Event, NameId};
pub use error::{Error, Result};
pub use timeline::Timeline;
pub use timewave::{Timewave, TimewaveId};

use std::collections::HashMap;

/// Simulation tick (discrete time unit)
pub type Tick = u64;

/// Entity lifecycle state (matches original engine's high-nibble flags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LifecycleState {
    /// Not yet created / empty slot
    Unborn = 0,
    /// Scheduled to spawn at this time
    Prebirth = 3,
    /// Destroyed / killed
    Dead = 4,
    /// Mid-time-travel (chronoporting)
    Chronoporting = 5,
    /// Normal active state
    Born = 6,
}

impl Default for LifecycleState {
    fn default() -> Self {
        Self::Unborn
    }
}

/// Main engine coordinating timeline and timewaves
pub struct Engine<S: EntityState> {
    /// The timeline storing all entity events
    pub timeline: Timeline<S>,
    /// Active timewaves (iterators through time)
    pub timewaves: HashMap<TimewaveId, Timewave>,
    /// Current simulation tick
    pub current_tick: Tick,
    /// Next timewave ID to assign
    next_wave_id: TimewaveId,
}

impl<S: EntityState> Engine<S> {
    /// Create a new engine with a default "present" timewave
    pub fn new() -> Self {
        let mut engine = Self {
            timeline: Timeline::new(),
            timewaves: HashMap::new(),
            current_tick: 0,
            next_wave_id: TimewaveId(1),
        };

        // Create the "present" wave that always moves at 1x speed
        let present = Timewave::new(TimewaveId(0), 0, 1.0);
        engine.timewaves.insert(TimewaveId(0), present);

        engine
    }

    /// Create a new engine with custom max entities
    pub fn with_capacity(max_entities: usize) -> Self {
        let mut engine = Self {
            timeline: Timeline::with_capacity(max_entities),
            timewaves: HashMap::new(),
            current_tick: 0,
            next_wave_id: TimewaveId(1),
        };

        let present = Timewave::new(TimewaveId(0), 0, 1.0);
        engine.timewaves.insert(TimewaveId(0), present);

        engine
    }

    /// Spawn a new entity with the given initial state
    pub fn spawn(&mut self, state: S) -> EntityId {
        self.spawn_at(state, self.current_tick)
    }

    /// Spawn a new entity at a specific time
    pub fn spawn_at(&mut self, state: S, tick: Tick) -> EntityId {
        self.timeline.spawn(state, tick)
    }

    /// Get an entity's current state (as seen by the present wave)
    pub fn get_state(&self, id: EntityId) -> Option<&Event<S>> {
        self.timeline.get_event_at(id, self.current_tick)
    }

    /// Update an entity's state at the current tick
    pub fn set_state(&mut self, id: EntityId, state: S) -> Result<()> {
        self.timeline.add_event(id, self.current_tick, state)
    }

    /// Destroy an entity at the current tick
    pub fn destroy(&mut self, id: EntityId) -> Result<()> {
        self.timeline.destroy(id, self.current_tick)
    }

    /// Execute a chronoport (time travel) for an entity
    ///
    /// This creates a temporal duplicate at the target time, linked via
    /// the same-name system for paradox detection.
    pub fn chronoport(&mut self, id: EntityId, target_tick: Tick) -> Result<EntityId> {
        self.timeline.chronoport(id, self.current_tick, target_tick)
    }

    /// Get all entities with the same "name" (temporal duplicates)
    pub fn get_same_name_entities(&self, id: EntityId) -> Vec<EntityId> {
        self.timeline.get_same_name_entities(id)
    }

    /// Add a new timewave with the given speed
    pub fn add_timewave(&mut self, speed: f64) -> TimewaveId {
        let id = self.next_wave_id;
        self.next_wave_id = TimewaveId(self.next_wave_id.0 + 1);

        let wave = Timewave::new(id, self.current_tick, speed);
        self.timewaves.insert(id, wave);
        id
    }

    /// Add a timewave starting at a specific time
    pub fn add_timewave_at(&mut self, tick: Tick, speed: f64) -> TimewaveId {
        let id = self.next_wave_id;
        self.next_wave_id = TimewaveId(self.next_wave_id.0 + 1);

        let wave = Timewave::new(id, tick, speed);
        self.timewaves.insert(id, wave);
        id
    }

    /// Remove a timewave
    pub fn remove_timewave(&mut self, id: TimewaveId) -> Option<Timewave> {
        if id.0 == 0 {
            return None; // Can't remove the present wave
        }
        self.timewaves.remove(&id)
    }

    /// Get a timewave's current position
    pub fn get_wave_position(&self, id: TimewaveId) -> Option<Tick> {
        self.timewaves.get(&id).map(|w| w.position)
    }

    /// Advance the simulation by one tick
    pub fn tick(&mut self) {
        self.tick_by(1);
    }

    /// Advance the simulation by multiple ticks
    ///
    /// This implements the core timewave processing loop from the original engine:
    /// 1. Advance each wave by delta ticks
    /// 2. Sort waves by position (bubble sort in original, stable sort here)
    /// 3. Link waves in chain (prev/next pointers)
    /// 4. Activate prebirth entities when waves reach them
    /// 5. Propagate events between waves (earlier affects later)
    /// 6. Clear dirty flags after sync
    pub fn tick_by(&mut self, delta: u64) {
        self.current_tick += delta;

        // Collect wave IDs to avoid borrow issues
        let wave_ids: Vec<TimewaveId> = self.timewaves.keys().copied().collect();

        // 1. Advance each wave
        for id in &wave_ids {
            if let Some(wave) = self.timewaves.get_mut(id) {
                wave.advance(delta);
            }
        }

        // 2. Sort waves by position for proper event propagation
        // Original uses bubble sort which is stable for nearly-sorted data
        let mut sorted: Vec<TimewaveId> = wave_ids;
        sorted.sort_by_key(|id| self.timewaves.get(id).map(|w| w.position).unwrap_or(0));

        // 3. Link waves in chain (from original FUN_0040ba40)
        self.link_wave_chain(&sorted);

        // 4. Activate prebirth entities for each wave's position
        // This transitions Prebirth → Born when waves reach scheduled entities
        for wave_id in &sorted {
            if let Some(wave) = self.timewaves.get(wave_id) {
                self.timeline.activate_prebirth_entities(wave.position);
            }
        }

        // 5. Refresh entity views and propagate between waves
        for i in 0..sorted.len() {
            let wave_id = sorted[i];

            // Refresh this wave's view of all entities
            self.refresh_wave_views(wave_id);

            // Propagate from previous wave if this wave passed it
            if i > 0 {
                let prev_id = sorted[i - 1];
                self.propagate_between_waves(prev_id, wave_id);
            }
        }

        // 6. Clear dirty flags after all propagation is done
        for wave_id in &sorted {
            if let Some(wave) = self.timewaves.get_mut(wave_id) {
                wave.clear_dirty();
                wave.needs_sync = false;
            }
        }
    }

    /// Link waves into a chain by time order (prev/next pointers)
    fn link_wave_chain(&mut self, sorted: &[TimewaveId]) {
        // Clear existing links
        for id in sorted {
            if let Some(wave) = self.timewaves.get_mut(id) {
                wave.prev_wave = None;
                wave.next_wave = None;
            }
        }

        // Set up chain
        for i in 0..sorted.len() {
            let current = sorted[i];

            if i > 0 {
                if let Some(wave) = self.timewaves.get_mut(&current) {
                    wave.prev_wave = Some(sorted[i - 1]);
                }
            }

            if i < sorted.len() - 1 {
                if let Some(wave) = self.timewaves.get_mut(&current) {
                    wave.next_wave = Some(sorted[i + 1]);
                }
            }
        }
    }

    /// Refresh a wave's view of all entities at its current position
    ///
    /// This updates the wave's entity_views cache to reflect what each entity
    /// looks like at the wave's current time position.
    fn refresh_wave_views(&mut self, wave_id: TimewaveId) {
        let wave_pos = match self.timewaves.get(&wave_id) {
            Some(w) => w.position,
            None => return,
        };

        // Collect entity views first (to avoid borrow issues)
        let views: Vec<(EntityId, Tick)> = self
            .timeline
            .iter()
            .filter_map(|(&id, entity)| {
                entity
                    .get_event_at(wave_pos)
                    .map(|event| (id, event.timestamp))
            })
            .collect();

        // Update wave's cache
        if let Some(wave) = self.timewaves.get_mut(&wave_id) {
            for (id, tick) in views {
                let old_tick = wave.get_entity_view(id);
                wave.update_entity_view(id, tick);

                // Mark dirty if view changed
                if old_tick != Some(tick) {
                    wave.mark_dirty(id);
                }
            }
        }
    }

    /// Propagate entity state from earlier wave to later wave
    ///
    /// From original engine FUN_00412950: when a wave passes another,
    /// it inherits entity views and command queues from the passed wave.
    fn propagate_between_waves(&mut self, earlier_id: TimewaveId, later_id: TimewaveId) {
        // Get positions to check if propagation needed
        let (earlier_pos, later_pos, later_prev_pos) = {
            let earlier = match self.timewaves.get(&earlier_id) {
                Some(w) => w,
                None => return,
            };
            let later = match self.timewaves.get(&later_id) {
                Some(w) => w,
                None => return,
            };

            // Only propagate if later wave has passed earlier wave's position
            // (comparing current to previous position)
            if later.position <= earlier.position {
                return;
            }

            (earlier.position, later.position, later.previous_position)
        };

        // Check if the later wave crossed the earlier wave this tick
        let crossed = later_prev_pos <= earlier_pos && later_pos > earlier_pos;

        if !crossed {
            return;
        }

        // Collect entity views to propagate (avoiding borrow issues)
        let views_to_propagate: Vec<(EntityId, Tick)> = {
            let earlier = match self.timewaves.get(&earlier_id) {
                Some(w) => w,
                None => return,
            };
            earlier.entity_views().collect()
        };

        // Apply propagation to later wave
        if let Some(later) = self.timewaves.get_mut(&later_id) {
            for (id, tick) in views_to_propagate {
                // Inherit view if we don't have one or ours is older
                if later.get_entity_view(id).map(|t| t < tick).unwrap_or(true) {
                    later.update_entity_view(id, tick);
                    later.mark_dirty(id);
                }
            }
            later.needs_sync = true;
        }
    }

    /// Get an entity's state as seen by a specific timewave
    pub fn get_state_at_wave(&self, id: EntityId, wave_id: TimewaveId) -> Option<&Event<S>> {
        let wave = self.timewaves.get(&wave_id)?;
        self.timeline.get_event_at(id, wave.position)
    }

    /// Iterate over all active entities at the current tick
    pub fn entities(&self) -> impl Iterator<Item = (EntityId, &Event<S>)> {
        self.timeline.entities_at(self.current_tick)
    }

    /// Get the number of entities (including dead/unborn)
    pub fn entity_count(&self) -> usize {
        self.timeline.entity_count()
    }

    /// Get the number of active entities at current tick
    pub fn active_entity_count(&self) -> usize {
        self.entities().count()
    }

    /// Set the time window constraints for chronoport
    ///
    /// - `past`: How many ticks back from current time chronoport can target (None = unlimited)
    /// - `future`: How many ticks forward from current time (None = unlimited)
    ///
    /// # Example
    ///
    /// ```rust
    /// use resequence::{Engine, EntityState};
    ///
    /// #[derive(Clone, Default)]
    /// struct State;
    /// impl EntityState for State {}
    ///
    /// let mut engine = Engine::<State>::new();
    /// // Allow chronoport up to 100 ticks in the past, 50 ticks in the future
    /// engine.set_time_window(Some(100), Some(50));
    /// ```
    pub fn set_time_window(&mut self, past: Option<Tick>, future: Option<Tick>) {
        self.timeline.set_time_window(past, future);
    }

    /// Get the current time window constraints
    pub fn get_time_window(&self) -> (Option<Tick>, Option<Tick>) {
        (self.timeline.window_past, self.timeline.window_future)
    }
}

impl<S: EntityState> Default for Engine<S> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default, Debug, PartialEq)]
    struct TestState {
        value: i32,
    }

    impl EntityState for TestState {}

    #[test]
    fn test_spawn_and_get() {
        let mut engine = Engine::<TestState>::new();
        let id = engine.spawn(TestState { value: 42 });

        let state = engine.get_state(id).unwrap();
        assert_eq!(state.state.value, 42);
        assert_eq!(state.lifecycle, LifecycleState::Born);
    }

    #[test]
    fn test_chronoport_creates_duplicate() {
        let mut engine = Engine::<TestState>::new();
        let id = engine.spawn(TestState { value: 100 });

        // Advance time
        for _ in 0..10 {
            engine.tick();
        }

        // Chronoport to t=5
        let new_id = engine.chronoport(id, 5).unwrap();

        // Should have two entities with same name
        let same_name = engine.get_same_name_entities(id);
        assert_eq!(same_name.len(), 2);
        assert!(same_name.contains(&id));
        assert!(same_name.contains(&new_id));
    }

    #[test]
    fn test_timewave_advancement() {
        let mut engine = Engine::<TestState>::new();

        // Add a fast wave (3x speed)
        let fast_wave = engine.add_timewave(3.0);

        // Tick 10 times
        for _ in 0..10 {
            engine.tick();
        }

        // Present wave should be at 10
        assert_eq!(engine.get_wave_position(TimewaveId(0)), Some(10));

        // Fast wave should be at 30
        assert_eq!(engine.get_wave_position(fast_wave), Some(30));
    }

    #[test]
    fn test_entity_state_history() {
        let mut engine = Engine::<TestState>::new();
        let id = engine.spawn(TestState { value: 0 });

        // Change state over time
        for i in 1..=5 {
            engine.tick();
            engine.set_state(id, TestState { value: i * 10 }).unwrap();
        }

        // Add a wave at t=3
        let past_wave = engine.add_timewave_at(3, 0.0); // Stationary

        // Present sees value=50 (t=5)
        assert_eq!(engine.get_state(id).unwrap().state.value, 50);

        // Past wave sees value=30 (t=3)
        assert_eq!(
            engine.get_state_at_wave(id, past_wave).unwrap().state.value,
            30
        );
    }

    #[test]
    fn test_prebirth_to_born_transition() {
        let mut engine = Engine::<TestState>::new();
        let original = engine.spawn(TestState { value: 100 });

        // Advance to tick 10
        for _ in 0..10 {
            engine.tick();
        }

        // Chronoport to tick 5 - creates a Prebirth entity at t=6
        let duplicate = engine.chronoport(original, 5).unwrap();

        // Add a stationary wave at tick 5 (before the duplicate's spawn time)
        let before_wave = engine.add_timewave_at(5, 0.0);

        // Add a stationary wave at tick 7 (after the duplicate's spawn time)
        let after_wave = engine.add_timewave_at(7, 0.0);

        // Trigger lifecycle transitions by ticking
        engine.tick();

        // Before spawn time: entity doesn't exist yet (or is Prebirth)
        // The duplicate spawns at t=6, so at t=5 it shouldn't be visible
        let before_state = engine.get_state_at_wave(duplicate, before_wave);
        assert!(
            before_state.is_none()
                || before_state.map(|e| e.lifecycle) == Some(LifecycleState::Prebirth)
        );

        // After spawn time: entity should be Born
        let after_state = engine.get_state_at_wave(duplicate, after_wave);
        assert!(after_state.is_some());
        assert_eq!(after_state.unwrap().lifecycle, LifecycleState::Born);
        assert_eq!(after_state.unwrap().state.value, 100);
    }

    #[test]
    fn test_wave_chaining() {
        let mut engine = Engine::<TestState>::new();

        // Add waves at different positions
        let wave_a = engine.add_timewave_at(10, 1.0);
        let wave_b = engine.add_timewave_at(20, 1.0);
        let wave_c = engine.add_timewave_at(30, 1.0);

        // Tick to trigger chain linking
        engine.tick();

        // Verify chain is linked correctly (sorted by position)
        // Present wave (0) is at position 1, wave_a at 11, wave_b at 21, wave_c at 31
        let present = engine.timewaves.get(&TimewaveId(0)).unwrap();
        assert!(present.prev_wave.is_none()); // First in chain
        assert_eq!(present.next_wave, Some(wave_a));

        let a = engine.timewaves.get(&wave_a).unwrap();
        assert_eq!(a.prev_wave, Some(TimewaveId(0)));
        assert_eq!(a.next_wave, Some(wave_b));

        let c = engine.timewaves.get(&wave_c).unwrap();
        assert_eq!(c.prev_wave, Some(wave_b));
        assert!(c.next_wave.is_none()); // Last in chain
    }

    #[test]
    fn test_wave_view_cache() {
        let mut engine = Engine::<TestState>::new();
        let entity = engine.spawn(TestState { value: 42 });

        // Advance and modify entity
        engine.tick();
        engine.set_state(entity, TestState { value: 100 }).unwrap();

        // Check that present wave's view cache has been updated
        let present = engine.timewaves.get(&TimewaveId(0)).unwrap();
        let view = present.get_entity_view(entity);
        assert!(view.is_some());
    }

    #[test]
    fn test_fast_wave_passes_slow_wave() {
        let mut engine = Engine::<TestState>::new();
        let entity = engine.spawn(TestState { value: 1 });

        // Create a slow wave at position 0
        let slow_wave = engine.add_timewave_at(0, 0.5); // Half speed

        // Create a fast wave also at position 0
        let fast_wave = engine.add_timewave(2.0); // 2x speed

        // Tick several times - fast wave will pass slow wave
        for i in 1..=10 {
            engine.tick();
            engine
                .set_state(entity, TestState { value: i * 10 })
                .unwrap();
        }

        // Fast wave should be at position 20, slow wave at position 5
        assert_eq!(engine.get_wave_position(fast_wave), Some(20));
        assert_eq!(engine.get_wave_position(slow_wave), Some(5));

        // Both waves should have entity view caches populated
        let fast = engine.timewaves.get(&fast_wave).unwrap();
        let slow = engine.timewaves.get(&slow_wave).unwrap();

        assert!(fast.get_entity_view(entity).is_some());
        assert!(slow.get_entity_view(entity).is_some());

        // Fast wave sees later state than slow wave
        let fast_state = engine.get_state_at_wave(entity, fast_wave).unwrap();
        let slow_state = engine.get_state_at_wave(entity, slow_wave).unwrap();

        assert!(fast_state.state.value > slow_state.state.value);
    }

    #[test]
    fn test_time_window_past_constraint() {
        let mut engine = Engine::<TestState>::new();
        let entity = engine.spawn(TestState { value: 42 });

        // Advance to tick 100
        for _ in 0..100 {
            engine.tick();
        }

        // Set window: can only go back 50 ticks
        engine.set_time_window(Some(50), None);

        // Chronoport to tick 60 should succeed (within window)
        let result = engine.chronoport(entity, 60);
        assert!(result.is_ok());

        // Chronoport to tick 40 should fail (outside window: 100 - 50 = 50 minimum)
        let result = engine.chronoport(entity, 40);
        assert!(result.is_err());

        // Check error type
        match result {
            Err(Error::ChronoportPastWindow { target, min_allowed }) => {
                assert_eq!(target, 40);
                assert_eq!(min_allowed, 50);
            }
            _ => panic!("Expected ChronoportPastWindow error"),
        }
    }

    #[test]
    fn test_time_window_future_constraint() {
        let mut engine = Engine::<TestState>::new();
        let entity = engine.spawn(TestState { value: 42 });

        // Advance to tick 50
        for _ in 0..50 {
            engine.tick();
        }

        // Set window: can only go forward 20 ticks
        engine.set_time_window(None, Some(20));

        // Chronoport to tick 60 should succeed (within window)
        let result = engine.chronoport(entity, 60);
        assert!(result.is_ok());

        // Chronoport to tick 80 should fail (outside window: 50 + 20 = 70 maximum)
        let result = engine.chronoport(entity, 80);
        assert!(result.is_err());

        // Check error type
        match result {
            Err(Error::ChronoportFutureWindow { target, max_allowed }) => {
                assert_eq!(target, 80);
                assert_eq!(max_allowed, 70);
            }
            _ => panic!("Expected ChronoportFutureWindow error"),
        }
    }

    #[test]
    fn test_time_window_both_constraints() {
        let mut engine = Engine::<TestState>::new();
        let entity = engine.spawn(TestState { value: 42 });

        // Advance to tick 100
        for _ in 0..100 {
            engine.tick();
        }

        // Set window: 30 ticks past, 30 ticks future
        engine.set_time_window(Some(30), Some(30));

        // Valid range is 70-130

        // Chronoport to tick 90 should succeed
        assert!(engine.chronoport(entity, 90).is_ok());

        // Chronoport to tick 50 should fail (too far past)
        assert!(matches!(
            engine.chronoport(entity, 50),
            Err(Error::ChronoportPastWindow { .. })
        ));

        // Chronoport to tick 150 should fail (too far future)
        assert!(matches!(
            engine.chronoport(entity, 150),
            Err(Error::ChronoportFutureWindow { .. })
        ));
    }
}
