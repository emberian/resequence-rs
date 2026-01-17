//! Error types for the resequence engine

use crate::{EntityId, Tick};
use thiserror::Error;

/// Errors that can occur in the resequence engine
#[derive(Debug, Error)]
pub enum Error {
    /// Entity not found in timeline
    #[error("Entity {0:?} not found")]
    EntityNotFound(EntityId),

    /// Timeline capacity exceeded
    #[error("Timeline is full (max entities: {0})")]
    TimelineFull(usize),

    /// Invalid chronoport target (generic)
    #[error("Invalid chronoport target time: {0}")]
    InvalidChronoport(Tick),

    /// Chronoport target is too far in the past
    #[error("Chronoport target {target} exceeds past window limit (minimum: {min_allowed})")]
    ChronoportPastWindow {
        /// The requested target time
        target: Tick,
        /// The minimum allowed time (current - window_past)
        min_allowed: Tick,
    },

    /// Chronoport target is too far in the future
    #[error("Chronoport target {target} exceeds future window limit (maximum: {max_allowed})")]
    ChronoportFutureWindow {
        /// The requested target time
        target: Tick,
        /// The maximum allowed time (current + window_future)
        max_allowed: Tick,
    },

    /// Entity is not in a valid state for the operation
    #[error("Entity {0:?} is in invalid state for operation")]
    InvalidEntityState(EntityId),
}

/// Result type alias for resequence operations
pub type Result<T> = std::result::Result<T, Error>;
