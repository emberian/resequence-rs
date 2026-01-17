//! Timewave synchronization example - demonstrating wave propagation
//!
//! This shows how timewaves maintain views of entities and propagate
//! state changes when faster waves pass slower ones.
//!
//! Run with: cargo run --example timewave_sync

use resequence::{Engine, EntityState, TimewaveId};

#[derive(Clone, Default, Debug)]
struct GameUnit {
    name: String,
    position: i32,
}

impl EntityState for GameUnit {}

fn main() {
    println!("Resequence Engine - Timewave Synchronization Example");
    println!("=====================================================\n");

    let mut engine = Engine::<GameUnit>::new();

    // Spawn a unit
    let unit = engine.spawn(GameUnit {
        name: "Scout".to_string(),
        position: 0,
    });
    println!("Spawned Scout at position 0, tick 0");

    // Create waves at different speeds
    let slow_wave = engine.add_timewave(0.5); // Half speed
    let fast_wave = engine.add_timewave(3.0); // 3x speed

    println!("\nCreated waves:");
    println!("  Present wave (id=0): speed 1.0x");
    println!("  Slow wave: speed 0.5x");
    println!("  Fast wave: speed 3.0x");

    println!("\n--- Advancing simulation ---\n");

    // Simulate movement over time
    for i in 1..=20 {
        engine.tick();

        // Move the unit each tick
        engine
            .set_state(
                unit,
                GameUnit {
                    name: "Scout".to_string(),
                    position: i * 10,
                },
            )
            .unwrap();

        // Print wave positions every 5 ticks
        if i % 5 == 0 {
            let present_pos = engine.get_wave_position(TimewaveId(0)).unwrap();
            let slow_pos = engine.get_wave_position(slow_wave).unwrap();
            let fast_pos = engine.get_wave_position(fast_wave).unwrap();

            println!("Tick {}:", i);
            println!(
                "  Present wave at t={}: Scout position = {}",
                present_pos,
                engine.get_state(unit).unwrap().state.position
            );
            println!(
                "  Slow wave at t={}: Scout position = {}",
                slow_pos,
                engine
                    .get_state_at_wave(unit, slow_wave)
                    .map(|e| e.state.position)
                    .unwrap_or(0)
            );
            println!(
                "  Fast wave at t={}: Scout position = {}",
                fast_pos,
                engine
                    .get_state_at_wave(unit, fast_wave)
                    .map(|e| e.state.position)
                    .unwrap_or(0)
            );

            // Show wave chain
            let present = engine.timewaves.get(&TimewaveId(0)).unwrap();
            println!(
                "  Wave chain: present -> {:?} -> {:?}",
                present.next_wave,
                present
                    .next_wave
                    .and_then(|id| engine.timewaves.get(&id))
                    .and_then(|w| w.next_wave)
            );
            println!();
        }
    }

    // Demonstrate view caching
    println!("--- Entity View Caches ---\n");

    let present = engine.timewaves.get(&TimewaveId(0)).unwrap();
    let slow = engine.timewaves.get(&slow_wave).unwrap();
    let fast = engine.timewaves.get(&fast_wave).unwrap();

    println!(
        "Present wave sees unit event at tick: {:?}",
        present.get_entity_view(unit)
    );
    println!(
        "Slow wave sees unit event at tick: {:?}",
        slow.get_entity_view(unit)
    );
    println!(
        "Fast wave sees unit event at tick: {:?}",
        fast.get_entity_view(unit)
    );

    println!("\n--- Backward Time Travel ---\n");

    // Create a wave going backward
    let reverse_wave = engine.add_timewave_at(50, -1.0); // Start at future, go backward
    println!("Created reverse wave at t=50, speed -1.0x");

    for i in 1..=10 {
        engine.tick();

        if i % 5 == 0 {
            let rev_pos = engine.get_wave_position(reverse_wave).unwrap();
            println!(
                "After {} ticks: Reverse wave at t={}, Scout position = {}",
                i,
                rev_pos,
                engine
                    .get_state_at_wave(unit, reverse_wave)
                    .map(|e| e.state.position)
                    .unwrap_or(0)
            );
        }
    }

    println!("\nDone!");
}
