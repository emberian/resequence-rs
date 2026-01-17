//! Basic usage example for the resequence engine
//!
//! Run with: cargo run --example basic

use resequence::{Engine, EntityState, TimewaveId};

/// A simple unit state for demonstration
#[derive(Clone, Default, Debug)]
struct Unit {
    name: String,
    hp: i32,
    x: f32,
    y: f32,
}

impl EntityState for Unit {}

fn main() {
    println!("Resequence Engine - Basic Example");
    println!("==================================\n");

    // Create the engine
    let mut engine = Engine::<Unit>::new();
    println!("Created engine at tick 0");

    // Spawn a unit
    let soldier = engine.spawn(Unit {
        name: "Soldier".to_string(),
        hp: 100,
        x: 0.0,
        y: 0.0,
    });
    println!("Spawned soldier (ID: {:?})", soldier);

    // Advance time and update state
    for i in 1..=10 {
        engine.tick();

        // Move the soldier
        engine
            .set_state(
                soldier,
                Unit {
                    name: "Soldier".to_string(),
                    hp: 100,
                    x: i as f32 * 10.0,
                    y: i as f32 * 5.0,
                },
            )
            .unwrap();
    }
    println!("Advanced to tick {}", engine.current_tick);

    // Check current state
    let state = engine.get_state(soldier).unwrap();
    println!(
        "Soldier at tick {}: pos=({}, {})",
        engine.current_tick, state.state.x, state.state.y
    );

    // Chronoport! Send the soldier back to tick 5
    println!("\nChronoporting soldier to tick 5...");
    let duplicate = engine.chronoport(soldier, 5).unwrap();
    println!("Created temporal duplicate (ID: {:?})", duplicate);

    // Check same-name entities
    let same_name = engine.get_same_name_entities(soldier);
    println!("Same-name entities: {:?}", same_name);

    // Add a timewave at the past to observe the duplicate
    let past_wave = engine.add_timewave_at(6, 0.0); // Stationary at t=6
    println!("\nCreated past observation wave at tick 6");

    // View both entities from different times
    if let Some(original_now) = engine.get_state(soldier) {
        println!(
            "Original soldier (present, t={}): pos=({}, {}), lifecycle={:?}",
            engine.current_tick, original_now.state.x, original_now.state.y, original_now.lifecycle
        );
    }

    if let Some(duplicate_past) = engine.get_state_at_wave(duplicate, past_wave) {
        println!(
            "Duplicate soldier (past wave, t=6): pos=({}, {}), lifecycle={:?}",
            duplicate_past.state.x, duplicate_past.state.y, duplicate_past.lifecycle
        );
    }

    // Advance more and watch the waves
    println!("\nAdvancing simulation with multiple waves...");
    let fast_wave = engine.add_timewave(2.0); // 2x speed

    for _ in 0..5 {
        engine.tick();
    }

    println!(
        "Present wave position: {:?}",
        engine.get_wave_position(TimewaveId(0))
    );
    println!(
        "Past wave position: {:?}",
        engine.get_wave_position(past_wave)
    );
    println!(
        "Fast wave position: {:?}",
        engine.get_wave_position(fast_wave)
    );

    println!("\nActive entities at current tick:");
    for (id, event) in engine.entities() {
        println!(
            "  {:?}: {} at ({}, {})",
            id, event.state.name, event.state.x, event.state.y
        );
    }

    println!("\nDone!");
}
