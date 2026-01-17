//! Time window constraints example
//!
//! This demonstrates how to set and enforce time window limits on chronoport,
//! preventing entities from traveling too far into the past or future.
//!
//! Run with: cargo run --example time_window

use resequence::{Engine, EntityState, Error};

#[derive(Clone, Default, Debug)]
struct TimeTraveler {
    name: String,
    energy: i32,
}

impl EntityState for TimeTraveler {}

fn main() {
    println!("Resequence Engine - Time Window Constraints Example");
    println!("====================================================\n");

    let mut engine = Engine::<TimeTraveler>::new();

    // Spawn a time traveler
    let traveler = engine.spawn(TimeTraveler {
        name: "Chrononaut".to_string(),
        energy: 100,
    });
    println!("Spawned Chrononaut at t=0");

    // Advance to tick 100
    for _ in 0..100 {
        engine.tick();
    }
    println!("Advanced to t={}", engine.current_tick);

    // Set time window: 30 ticks past, 20 ticks future
    engine.set_time_window(Some(30), Some(20));
    let (past, future) = engine.get_time_window();
    println!(
        "\nSet time window: {} ticks past, {} ticks future",
        past.unwrap(),
        future.unwrap()
    );
    println!("Valid chronoport range: t={} to t={}", 100 - 30, 100 + 20);

    println!("\n--- Testing Chronoport Constraints ---\n");

    // Test 1: Valid chronoport within window
    println!("Test 1: Chronoport to t=80 (within window)");
    match engine.chronoport(traveler, 80) {
        Ok(new_id) => println!("  SUCCESS: Created duplicate {:?} at t=80", new_id),
        Err(e) => println!("  FAILED: {}", e),
    }

    // Test 2: Valid chronoport to future
    println!("\nTest 2: Chronoport to t=110 (within future window)");
    match engine.chronoport(traveler, 110) {
        Ok(new_id) => println!("  SUCCESS: Created duplicate {:?} at t=110", new_id),
        Err(e) => println!("  FAILED: {}", e),
    }

    // Test 3: Invalid - too far in past
    println!("\nTest 3: Chronoport to t=50 (outside past window)");
    match engine.chronoport(traveler, 50) {
        Ok(new_id) => println!("  SUCCESS: Created duplicate {:?}", new_id),
        Err(Error::ChronoportPastWindow { target, min_allowed }) => {
            println!("  BLOCKED: Cannot travel to t={}", target);
            println!("           Minimum allowed: t={}", min_allowed);
        }
        Err(e) => println!("  ERROR: {}", e),
    }

    // Test 4: Invalid - too far in future
    println!("\nTest 4: Chronoport to t=150 (outside future window)");
    match engine.chronoport(traveler, 150) {
        Ok(new_id) => println!("  SUCCESS: Created duplicate {:?}", new_id),
        Err(Error::ChronoportFutureWindow { target, max_allowed }) => {
            println!("  BLOCKED: Cannot travel to t={}", target);
            println!("           Maximum allowed: t={}", max_allowed);
        }
        Err(e) => println!("  ERROR: {}", e),
    }

    println!("\n--- Dynamic Window Adjustment ---\n");

    // Windows can be adjusted dynamically (e.g., based on in-game resources)
    println!("Expanding window: 50 ticks past, 50 ticks future");
    engine.set_time_window(Some(50), Some(50));

    // Now t=50 should work
    println!("\nRetrying: Chronoport to t=50");
    match engine.chronoport(traveler, 50) {
        Ok(new_id) => println!("  SUCCESS: Created duplicate {:?} at t=50", new_id),
        Err(e) => println!("  FAILED: {}", e),
    }

    // Show all temporal duplicates
    println!("\n--- Temporal Duplicates ---\n");
    let duplicates = engine.get_same_name_entities(traveler);
    println!("Total temporal copies of Chrononaut: {}", duplicates.len());
    for (i, id) in duplicates.iter().enumerate() {
        if let Some(event) = engine.get_state(*id) {
            println!(
                "  Copy {}: {:?} - {} (lifecycle: {:?})",
                i + 1,
                id,
                event.state.name,
                event.lifecycle
            );
        }
    }

    // Disable window (unlimited time travel)
    println!("\n--- Unlimited Mode ---\n");
    engine.set_time_window(None, None);
    println!("Disabled time window constraints");

    println!("Chronoport to t=0 (beginning of time)");
    match engine.chronoport(traveler, 0) {
        Ok(new_id) => println!("  SUCCESS: Created duplicate {:?} at t=0", new_id),
        Err(e) => println!("  FAILED: {}", e),
    }

    println!("\nDone!");
}
