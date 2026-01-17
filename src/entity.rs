//! Entity and event types

use crate::{LifecycleState, Tick};

/// Unique identifier for an entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

/// Identifier linking temporal duplicates ("same name" in original engine)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NameId(pub u32);

/// Trait for entity state data
///
/// Implement this for your game's unit/entity state.
pub trait EntityState: Clone + Default {}

/// A single event (state snapshot) in an entity's timeline
#[derive(Debug, Clone)]
pub struct Event<S: EntityState> {
    /// When this event occurs
    pub timestamp: Tick,
    /// Entity lifecycle state
    pub lifecycle: LifecycleState,
    /// The actual state data
    pub state: S,
}

impl<S: EntityState> Event<S> {
    /// Create a new event
    pub fn new(timestamp: Tick, lifecycle: LifecycleState, state: S) -> Self {
        Self {
            timestamp,
            lifecycle,
            state,
        }
    }

    /// Create a "born" event (normal spawn)
    pub fn born(timestamp: Tick, state: S) -> Self {
        Self::new(timestamp, LifecycleState::Born, state)
    }

    /// Create a "prebirth" event (scheduled to appear)
    pub fn prebirth(timestamp: Tick, state: S) -> Self {
        Self::new(timestamp, LifecycleState::Prebirth, state)
    }

    /// Create a "dead" event
    pub fn dead(timestamp: Tick, state: S) -> Self {
        Self::new(timestamp, LifecycleState::Dead, state)
    }

    /// Create a "chronoporting" event
    pub fn chronoporting(timestamp: Tick, state: S) -> Self {
        Self::new(timestamp, LifecycleState::Chronoporting, state)
    }

    /// Check if entity is active at this event
    pub fn is_active(&self) -> bool {
        matches!(
            self.lifecycle,
            LifecycleState::Born | LifecycleState::Prebirth
        )
    }
}

/// An entity with its event history
#[derive(Debug)]
pub struct Entity<S: EntityState> {
    /// Unique identifier
    pub id: EntityId,
    /// Name ID for same-name linking (temporal duplicates share this)
    pub name_id: NameId,
    /// Creation time
    pub created_at: Tick,
    /// Event history (sorted by timestamp)
    events: Vec<Event<S>>,
}

impl<S: EntityState> Entity<S> {
    /// Create a new entity
    pub fn new(id: EntityId, name_id: NameId, created_at: Tick, initial_state: S) -> Self {
        let initial_event = Event::born(created_at, initial_state);
        Self {
            id,
            name_id,
            created_at,
            events: vec![initial_event],
        }
    }

    /// Create an entity scheduled to appear at a future time
    pub fn prebirth(id: EntityId, name_id: NameId, appear_at: Tick, state: S) -> Self {
        let event = Event::prebirth(appear_at, state);
        Self {
            id,
            name_id,
            created_at: appear_at,
            events: vec![event],
        }
    }

    /// Get the most recent event at or before the given time
    pub fn get_event_at(&self, tick: Tick) -> Option<&Event<S>> {
        // Binary search for efficiency with large histories
        let idx = self
            .events
            .partition_point(|e| e.timestamp <= tick)
            .saturating_sub(1);

        self.events.get(idx).filter(|e| e.timestamp <= tick)
    }

    /// Get the most recent event
    pub fn latest_event(&self) -> Option<&Event<S>> {
        self.events.last()
    }

    /// Add a new event to the history
    pub fn add_event(&mut self, event: Event<S>) {
        // Insert in sorted order by timestamp
        let pos = self.events.partition_point(|e| e.timestamp < event.timestamp);
        self.events.insert(pos, event);
    }

    /// Add a state change at the given time
    pub fn set_state(&mut self, tick: Tick, state: S) {
        self.add_event(Event::born(tick, state));
    }

    /// Mark entity as dead at the given time
    pub fn destroy(&mut self, tick: Tick) {
        if let Some(last) = self.events.last() {
            self.add_event(Event::dead(tick, last.state.clone()));
        }
    }

    /// Mark entity as chronoporting
    pub fn mark_chronoporting(&mut self, tick: Tick) {
        if let Some(last) = self.events.last() {
            self.add_event(Event::chronoporting(tick, last.state.clone()));
        }
    }

    /// Check if entity needs activation at the given tick
    ///
    /// Returns true if:
    /// - Entity has a Prebirth event at or before tick
    /// - No Born event exists after that Prebirth event
    pub fn needs_activation_at(&self, tick: Tick) -> bool {
        if let Some(event) = self.get_event_at(tick) {
            if event.lifecycle == LifecycleState::Prebirth {
                // Check if there's a subsequent Born event
                let has_born_after = self.events.iter().any(|e| {
                    e.timestamp > event.timestamp && e.lifecycle == LifecycleState::Born
                });
                return !has_born_after;
            }
        }
        false
    }

    /// Activate the entity (transition Prebirth → Born) at the given tick
    ///
    /// This is called when a timewave reaches/passes an entity's prebirth time.
    /// In the original engine, this transitions units from scheduled to active.
    pub fn activate_at(&mut self, tick: Tick) {
        if let Some(event) = self.get_event_at(tick) {
            if event.lifecycle == LifecycleState::Prebirth {
                // Clone state and create Born event
                self.add_event(Event::born(tick, event.state.clone()));
            }
        }
    }

    /// Get the number of events in history
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Iterate over all events
    pub fn events(&self) -> impl Iterator<Item = &Event<S>> {
        self.events.iter()
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
    fn test_event_at_time() {
        let mut entity = Entity::new(
            EntityId(0),
            NameId(0),
            0,
            TestState { value: 0 },
        );

        entity.set_state(5, TestState { value: 50 });
        entity.set_state(10, TestState { value: 100 });

        // At t=0, should see value=0
        assert_eq!(entity.get_event_at(0).unwrap().state.value, 0);

        // At t=3, should still see value=0 (before t=5 change)
        assert_eq!(entity.get_event_at(3).unwrap().state.value, 0);

        // At t=7, should see value=50
        assert_eq!(entity.get_event_at(7).unwrap().state.value, 50);

        // At t=15, should see value=100
        assert_eq!(entity.get_event_at(15).unwrap().state.value, 100);
    }
}
