//! Timeline storage for entities and events

use crate::{Entity, EntityId, EntityState, Error, Event, NameId, Result, Tick};
#[cfg(test)]
use crate::LifecycleState;
use std::collections::HashMap;

/// The timeline stores all entity state across time
///
/// This implements the core data structure from the original engine:
/// - Entities stored with their event histories
/// - Same-name groups for linking temporal duplicates
/// - Time window constraints for chronoport validation
#[derive(Debug)]
pub struct Timeline<S: EntityState> {
    /// All entities indexed by ID
    entities: HashMap<EntityId, Entity<S>>,
    /// Same-name groups for time travelers
    /// Maps NameId -> list of EntityIds sharing that name
    same_name_groups: HashMap<NameId, Vec<EntityId>>,
    /// Next entity ID to assign
    next_id: u32,
    /// Next name ID to assign
    next_name_id: u32,
    /// Maximum entities allowed
    max_entities: usize,
    /// How far back in time chronoport can travel (relative to current tick)
    /// None means unlimited. From original engine: windowSize at offset 0x140008
    pub window_past: Option<Tick>,
    /// How far forward in time chronoport can travel (relative to current tick)
    /// None means unlimited. From original engine: windowOffset at offset 0x140010
    pub window_future: Option<Tick>,
}

impl<S: EntityState> Timeline<S> {
    /// Create a new timeline with default capacity
    pub fn new() -> Self {
        Self::with_capacity(0x10000) // 64K entities like original
    }

    /// Create a timeline with custom capacity
    pub fn with_capacity(max_entities: usize) -> Self {
        Self {
            entities: HashMap::new(),
            same_name_groups: HashMap::new(),
            next_id: 0,
            next_name_id: 0,
            max_entities,
            window_past: None,
            window_future: None,
        }
    }

    /// Set the time window constraints for chronoport
    ///
    /// - `past`: How many ticks back from current time chronoport can target (None = unlimited)
    /// - `future`: How many ticks forward from current time (None = unlimited)
    pub fn set_time_window(&mut self, past: Option<Tick>, future: Option<Tick>) {
        self.window_past = past;
        self.window_future = future;
    }

    /// Spawn a new entity at the given time
    pub fn spawn(&mut self, state: S, tick: Tick) -> EntityId {
        let id = EntityId(self.next_id);
        let name_id = NameId(self.next_name_id);
        self.next_id += 1;
        self.next_name_id += 1;

        let entity = Entity::new(id, name_id, tick, state);
        self.entities.insert(id, entity);

        id
    }

    /// Spawn an entity with a specific name ID (for same-name linking)
    fn spawn_with_name(&mut self, state: S, tick: Tick, name_id: NameId) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;

        let entity = Entity::prebirth(id, name_id, tick, state);
        self.entities.insert(id, entity);

        // Add to same-name group
        self.same_name_groups
            .entry(name_id)
            .or_insert_with(Vec::new)
            .push(id);

        id
    }

    /// Get an entity by ID
    pub fn get(&self, id: EntityId) -> Option<&Entity<S>> {
        self.entities.get(&id)
    }

    /// Get a mutable reference to an entity
    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity<S>> {
        self.entities.get_mut(&id)
    }

    /// Get an entity's event at a specific time
    pub fn get_event_at(&self, id: EntityId, tick: Tick) -> Option<&Event<S>> {
        self.entities.get(&id)?.get_event_at(tick)
    }

    /// Add an event to an entity
    pub fn add_event(&mut self, id: EntityId, tick: Tick, state: S) -> Result<()> {
        let entity = self
            .entities
            .get_mut(&id)
            .ok_or(Error::EntityNotFound(id))?;
        entity.set_state(tick, state);
        Ok(())
    }

    /// Destroy an entity at the given time
    pub fn destroy(&mut self, id: EntityId, tick: Tick) -> Result<()> {
        let entity = self
            .entities
            .get_mut(&id)
            .ok_or(Error::EntityNotFound(id))?;
        entity.destroy(tick);
        Ok(())
    }

    /// Execute a chronoport (time travel)
    ///
    /// This creates a temporal duplicate at the target time, linked via
    /// the same-name system for paradox detection.
    ///
    /// # Time Window Validation
    ///
    /// If `window_past` is set, target must be >= current_tick - window_past.
    /// If `window_future` is set, target must be <= current_tick + window_future.
    pub fn chronoport(
        &mut self,
        id: EntityId,
        current_tick: Tick,
        target_tick: Tick,
    ) -> Result<EntityId> {
        // Check capacity
        if self.entities.len() >= self.max_entities {
            return Err(Error::TimelineFull(self.max_entities));
        }

        // Validate time window constraints
        if let Some(window_past) = self.window_past {
            let min_allowed = current_tick.saturating_sub(window_past);
            if target_tick < min_allowed {
                return Err(Error::ChronoportPastWindow {
                    target: target_tick,
                    min_allowed,
                });
            }
        }

        if let Some(window_future) = self.window_future {
            let max_allowed = current_tick.saturating_add(window_future);
            if target_tick > max_allowed {
                return Err(Error::ChronoportFutureWindow {
                    target: target_tick,
                    max_allowed,
                });
            }
        }

        // Get source entity
        let source = self.entities.get(&id).ok_or(Error::EntityNotFound(id))?;
        let name_id = source.name_id;

        // Get current state to copy
        let source_event = source
            .latest_event()
            .ok_or(Error::InvalidEntityState(id))?;
        let state = source_event.state.clone();

        // Ensure source is in same-name group
        let group = self.same_name_groups.entry(name_id).or_insert_with(Vec::new);
        if !group.contains(&id) {
            group.push(id);
        }

        // Create the temporal duplicate
        let new_id = self.spawn_with_name(state, target_tick + 1, name_id);

        // Mark source as chronoporting
        if let Some(source) = self.entities.get_mut(&id) {
            source.mark_chronoporting(current_tick);
        }

        Ok(new_id)
    }

    /// Get all entities with the same name (temporal duplicates)
    pub fn get_same_name_entities(&self, id: EntityId) -> Vec<EntityId> {
        let Some(entity) = self.entities.get(&id) else {
            return vec![];
        };

        self.same_name_groups
            .get(&entity.name_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get the next entity with the same name (Rescript VM opcode 0x36)
    pub fn get_next_same_name(&self, id: EntityId) -> Option<EntityId> {
        let entity = self.entities.get(&id)?;
        let group = self.same_name_groups.get(&entity.name_id)?;

        let idx = group.iter().position(|&eid| eid == id)?;
        group.get(idx + 1).copied()
    }

    /// Iterate over all entities with their current state at a given tick
    pub fn entities_at(&self, tick: Tick) -> impl Iterator<Item = (EntityId, &Event<S>)> {
        self.entities.iter().filter_map(move |(&id, entity)| {
            entity
                .get_event_at(tick)
                .filter(|e| e.is_active())
                .map(|e| (id, e))
        })
    }

    /// Get total entity count (including dead/unborn)
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Iterate over all entities
    pub fn iter(&self) -> impl Iterator<Item = (&EntityId, &Entity<S>)> {
        self.entities.iter()
    }

    /// Iterate over all entities mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&EntityId, &mut Entity<S>)> {
        self.entities.iter_mut()
    }

    /// Activate all prebirth entities that should become active at or before the given tick
    ///
    /// This transitions entities from Prebirth → Born when a timewave reaches them.
    /// In the original engine, this happens during timewave processing.
    pub fn activate_prebirth_entities(&mut self, tick: Tick) {
        // Collect IDs that need activation first (to avoid borrow issues)
        let to_activate: Vec<EntityId> = self
            .entities
            .iter()
            .filter(|(_, e)| e.needs_activation_at(tick))
            .map(|(&id, _)| id)
            .collect();

        // Now activate them
        for id in to_activate {
            if let Some(entity) = self.entities.get_mut(&id) {
                entity.activate_at(tick);
            }
        }
    }
}

impl<S: EntityState> Default for Timeline<S> {
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
        let mut timeline = Timeline::<TestState>::new();
        let id = timeline.spawn(TestState { value: 42 }, 0);

        let event = timeline.get_event_at(id, 0).unwrap();
        assert_eq!(event.state.value, 42);
        assert_eq!(event.lifecycle, LifecycleState::Born);
    }

    #[test]
    fn test_chronoport_links_entities() {
        let mut timeline = Timeline::<TestState>::new();
        let id = timeline.spawn(TestState { value: 100 }, 0);

        // Chronoport to tick 5
        let new_id = timeline.chronoport(id, 10, 5).unwrap();

        // Both should share the same name
        let same_name = timeline.get_same_name_entities(id);
        assert_eq!(same_name.len(), 2);
        assert!(same_name.contains(&id));
        assert!(same_name.contains(&new_id));

        // Source should be marked as chronoporting
        let source_event = timeline.get_event_at(id, 10).unwrap();
        assert_eq!(source_event.lifecycle, LifecycleState::Chronoporting);

        // Duplicate should be prebirth at target time
        let dup_event = timeline.get_event_at(new_id, 6).unwrap();
        assert_eq!(dup_event.lifecycle, LifecycleState::Prebirth);
        assert_eq!(dup_event.state.value, 100);
    }

    #[test]
    fn test_next_same_name() {
        let mut timeline = Timeline::<TestState>::new();
        let id1 = timeline.spawn(TestState { value: 1 }, 0);
        let id2 = timeline.chronoport(id1, 10, 5).unwrap();
        let id3 = timeline.chronoport(id1, 15, 8).unwrap();

        // Should be able to traverse the same-name chain
        assert_eq!(timeline.get_next_same_name(id1), Some(id2));
        assert_eq!(timeline.get_next_same_name(id2), Some(id3));
        assert_eq!(timeline.get_next_same_name(id3), None);
    }
}
