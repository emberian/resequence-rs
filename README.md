# resequence

A time-travel simulation engine based on patterns reverse-engineered from Achron's Resequence engine (2011). This crate preserves the intellectual heritage of Achron's innovative time manipulation mechanics for future game development and research.

## Features

- **Event-Linked Entities**: Entities store history as timestamped events, not full snapshots. O(log n) historical queries via binary search.
- **Timewaves**: Execution contexts that move through time at variable speeds. Multiple waves can view the same timeline at different positions.
- **Identity Linking**: When entities time-travel (chronoport), they create linked duplicates sharing a "name ID" for paradox detection.
- **Sorted Wave Processing**: Timewaves are sorted by position each tick. Earlier waves propagate state changes to later waves for causal consistency.

## Quick Start

```rust
use resequence::{Engine, EntityState};

#[derive(Clone, Default, Debug)]
struct Unit {
    hp: i32,
    x: f32,
    y: f32,
}

impl EntityState for Unit {}

fn main() {
    let mut engine = Engine::<Unit>::new();

    // Spawn a unit
    let unit = engine.spawn(Unit { hp: 100, x: 0.0, y: 0.0 });

    // Advance time
    for _ in 0..10 {
        engine.tick();
    }

    // Time travel! Create a duplicate at t=5
    engine.chronoport(unit, 5).unwrap();

    // Now there are two linked copies of the unit
    let duplicates = engine.get_same_name_entities(unit);
    assert_eq!(duplicates.len(), 2);
}
```

## Lifecycle States

Entities transition through lifecycle states matching the original engine:

- `Unborn`: Empty slot or not yet scheduled
- `Prebirth`: Scheduled to appear (e.g., destination of chronoport)
- `Born`: Normal active state
- `Chronoporting`: Mid-time-travel
- `Dead`: Destroyed

## Time Window Constraints

Chronoport operations can be constrained to a time window:

```rust
// Allow chronoport up to 100 ticks in the past, 50 ticks in the future
engine.set_time_window(Some(100), Some(50));
```

## Applications

Beyond games, these patterns are useful for:

- **Simulation debugging**: Add past observation waves to inspect state
- **What-if analysis**: Fork timelines, apply different decisions, compare
- **Undo/redo systems**: Position-based state views without explicit snapshots
- **Distributed consensus**: Multiple nodes as waves at different timestamps

## Examples

Run the examples with:

```bash
cargo run --example basic
cargo run --example paradox
cargo run --example timewave_sync
cargo run --example time_window
```

## License

MIT
