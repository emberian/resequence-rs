//! Timewave - an iterator moving through time

use crate::{EntityId, Tick};
use std::collections::{HashMap, HashSet};

/// Unique identifier for a timewave
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimewaveId(pub u32);

/// A timewave is an execution context moving through time
///
/// From the original engine:
/// - Each wave has a position (current time) and velocity (speed)
/// - Waves can move forward, backward, or stay stationary
/// - Multiple waves can view the same timeline at different times
///
/// The wave maintains a cache of entity "views" (which event each entity is at)
/// for efficient propagation between waves. When waves pass each other,
/// the faster wave inherits entity views from the slower one.
#[derive(Debug)]
pub struct Timewave {
    /// Unique identifier
    pub id: TimewaveId,
    /// Current time position (integer part)
    pub position: Tick,
    /// Previous position (for delta calculations)
    pub previous_position: Tick,
    /// Speed multiplier (can be negative for backward movement)
    pub speed: f64,
    /// Base speed (usually 1.0)
    pub base_speed: f64,
    /// Fractional time accumulator
    fractional: f64,
    /// Whether this wave is active
    pub is_active: bool,
    /// Whether this wave needs synchronization with others
    pub needs_sync: bool,
    /// Per-entity event timestamp cache (what event tick this wave "sees" for each entity)
    /// Maps EntityId -> timestamp of the event this wave sees
    /// From original engine: unitPointers array at offset 0x0044
    entity_views: HashMap<EntityId, Tick>,
    /// Entities modified since last sync
    /// From original engine: dirtyFlags array at offset 0x40044
    dirty_entities: HashSet<EntityId>,
    /// Previous wave in time-sorted chain (for propagation)
    pub prev_wave: Option<TimewaveId>,
    /// Next wave in time-sorted chain
    pub next_wave: Option<TimewaveId>,
}

impl Timewave {
    /// Create a new timewave at the given position
    pub fn new(id: TimewaveId, position: Tick, speed: f64) -> Self {
        Self {
            id,
            position,
            previous_position: position,
            speed,
            base_speed: 1.0,
            fractional: 0.0,
            is_active: true,
            needs_sync: false,
            entity_views: HashMap::new(),
            dirty_entities: HashSet::new(),
            prev_wave: None,
            next_wave: None,
        }
    }

    /// Create a stationary timewave (frozen in time)
    pub fn stationary(id: TimewaveId, position: Tick) -> Self {
        Self::new(id, position, 0.0)
    }

    /// Create a reverse timewave (moving backward)
    pub fn reverse(id: TimewaveId, position: Tick, speed: f64) -> Self {
        Self::new(id, position, -speed.abs())
    }

    /// Advance the wave by the given number of ticks
    pub fn advance(&mut self, delta: u64) {
        if !self.is_active {
            return;
        }

        self.previous_position = self.position;

        // Calculate advancement
        let advance = self.speed * self.base_speed * (delta as f64);
        let new_frac = self.fractional + advance;

        // Handle positive movement
        if new_frac >= 0.0 {
            let whole = new_frac.floor() as u64;
            self.position = self.position.saturating_add(whole);
            self.fractional = new_frac - (whole as f64);
        } else {
            // Handle negative movement (going backward)
            let whole = (-new_frac).ceil() as u64;
            self.position = self.position.saturating_sub(whole);
            self.fractional = new_frac + (whole as f64);
        }
    }

    /// Set the wave's speed
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    /// Pause the wave
    pub fn pause(&mut self) {
        self.is_active = false;
    }

    /// Resume the wave
    pub fn resume(&mut self) {
        self.is_active = true;
    }

    /// Jump to a specific time position
    pub fn jump_to(&mut self, position: Tick) {
        self.previous_position = self.position;
        self.position = position;
        self.fractional = 0.0;
    }

    /// Get the delta since last advance
    pub fn delta(&self) -> i64 {
        (self.position as i64) - (self.previous_position as i64)
    }

    /// Check if the wave is moving forward
    pub fn is_forward(&self) -> bool {
        self.speed > 0.0
    }

    /// Check if the wave is moving backward
    pub fn is_backward(&self) -> bool {
        self.speed < 0.0
    }

    /// Check if the wave is stationary
    pub fn is_stationary(&self) -> bool {
        self.speed == 0.0 || !self.is_active
    }

    // === Entity View Cache Methods ===
    // These implement the per-unit event pointer cache from the original engine

    /// Update this wave's view of an entity (what event tick it sees)
    pub fn update_entity_view(&mut self, id: EntityId, event_tick: Tick) {
        self.entity_views.insert(id, event_tick);
    }

    /// Get this wave's cached view of an entity
    pub fn get_entity_view(&self, id: EntityId) -> Option<Tick> {
        self.entity_views.get(&id).copied()
    }

    /// Mark an entity as dirty (modified since last sync)
    pub fn mark_dirty(&mut self, id: EntityId) {
        self.dirty_entities.insert(id);
    }

    /// Check if an entity is dirty
    pub fn is_dirty(&self, id: EntityId) -> bool {
        self.dirty_entities.contains(&id)
    }

    /// Clear all dirty flags (called after sync)
    pub fn clear_dirty(&mut self) {
        self.dirty_entities.clear();
    }

    /// Get all dirty entity IDs
    pub fn dirty_entities(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.dirty_entities.iter().copied()
    }

    /// Get all entity views
    pub fn entity_views(&self) -> impl Iterator<Item = (EntityId, Tick)> + '_ {
        self.entity_views.iter().map(|(&id, &tick)| (id, tick))
    }

    /// Copy entity views from another wave (for propagation)
    pub fn inherit_views_from(&mut self, other: &Timewave) {
        for (&id, &tick) in &other.entity_views {
            // Only inherit if we don't have a more recent view
            if let Some(&our_tick) = self.entity_views.get(&id) {
                if tick <= our_tick {
                    continue;
                }
            }
            self.entity_views.insert(id, tick);
            self.dirty_entities.insert(id);
        }
    }

    /// Clear entity views (for reset/cleanup)
    pub fn clear_views(&mut self) {
        self.entity_views.clear();
        self.dirty_entities.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_movement() {
        let mut wave = Timewave::new(TimewaveId(0), 0, 1.0);

        wave.advance(10);
        assert_eq!(wave.position, 10);
        assert_eq!(wave.delta(), 10);
    }

    #[test]
    fn test_fast_movement() {
        let mut wave = Timewave::new(TimewaveId(0), 0, 3.0);

        wave.advance(10);
        assert_eq!(wave.position, 30);
    }

    #[test]
    fn test_slow_movement() {
        let mut wave = Timewave::new(TimewaveId(0), 0, 0.5);

        // At 0.5 speed, need 2 ticks to advance 1 position
        wave.advance(1);
        assert_eq!(wave.position, 0);

        wave.advance(1);
        assert_eq!(wave.position, 1);
    }

    #[test]
    fn test_backward_movement() {
        let mut wave = Timewave::reverse(TimewaveId(0), 100, 1.0);

        wave.advance(10);
        assert_eq!(wave.position, 90);
        assert_eq!(wave.delta(), -10);
    }

    #[test]
    fn test_stationary() {
        let mut wave = Timewave::stationary(TimewaveId(0), 50);

        wave.advance(100);
        assert_eq!(wave.position, 50);
    }

    #[test]
    fn test_pause_resume() {
        let mut wave = Timewave::new(TimewaveId(0), 0, 1.0);

        wave.advance(5);
        assert_eq!(wave.position, 5);

        wave.pause();
        wave.advance(10);
        assert_eq!(wave.position, 5); // Should not have moved

        wave.resume();
        wave.advance(10);
        assert_eq!(wave.position, 15);
    }

    #[test]
    fn test_jump() {
        let mut wave = Timewave::new(TimewaveId(0), 0, 1.0);

        wave.advance(10);
        wave.jump_to(100);

        assert_eq!(wave.position, 100);
        assert_eq!(wave.previous_position, 10);
    }
}
