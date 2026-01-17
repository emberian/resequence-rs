//! Paradox detection example - demonstrating same-name linking
//!
//! This shows how temporal duplicates are linked and can be enumerated,
//! which is the foundation for paradox detection in time-travel games.
//!
//! Run with: cargo run --example paradox

use resequence::{Engine, EntityState, LifecycleState};

/// A combat unit that can time travel
#[derive(Clone, Default, Debug)]
struct CombatUnit {
    name: String,
    hp: i32,
    damage: i32,
    owner: u8,
}

impl EntityState for CombatUnit {}

fn main() {
    println!("Resequence Engine - Paradox Detection Example");
    println!("==============================================\n");

    let mut engine = Engine::<CombatUnit>::new();

    // Spawn a unit for player 1
    let marine = engine.spawn(CombatUnit {
        name: "Marine".to_string(),
        hp: 100,
        damage: 10,
        owner: 1,
    });
    println!("Player 1 spawned Marine at t=0");

    // Advance time
    for _ in 0..20 {
        engine.tick();
    }
    println!("Advanced to t={}", engine.current_tick);

    // The marine takes damage during the battle
    engine
        .set_state(
            marine,
            CombatUnit {
                name: "Marine".to_string(),
                hp: 30, // Took heavy damage!
                damage: 10,
                owner: 1,
            },
        )
        .unwrap();
    println!("Marine took damage, now at 30 HP");

    // Player decides to send a copy back to help!
    println!("\nChronoporting Marine to t=5 to create reinforcement...");
    let marine_past = engine.chronoport(marine, 5).unwrap();

    // And again to t=10
    let marine_mid = engine.chronoport(marine, 10).unwrap();
    println!("Chronoported again to t=10");

    // Now we have 3 copies linked by same-name
    println!("\n--- Same-Name Chain (Temporal Duplicates) ---");
    let duplicates = engine.get_same_name_entities(marine);
    println!("Found {} temporal duplicates:", duplicates.len());

    for dup_id in &duplicates {
        if let Some(event) = engine.get_state(*dup_id) {
            println!(
                "  {:?}: HP={}, lifecycle={:?}, owner={}",
                dup_id, event.state.hp, event.lifecycle, event.state.owner
            );
        }
    }

    // Demonstrate paradox detection logic
    println!("\n--- Paradox Detection ---");
    println!("Checking for temporal conflicts...\n");

    // In a real game, this would be run by the Rescript VM
    // The NextUnitOfSameName opcode (0x36) enables this pattern
    for &check_id in &duplicates {
        let Some(check_event) = engine.get_state(check_id) else {
            continue;
        };

        // Skip if this unit is chronoporting (mid-travel)
        if check_event.lifecycle == LifecycleState::Chronoporting {
            println!(
                "{:?} is currently chronoporting, skipping",
                check_id
            );
            continue;
        }

        // Check against all other same-name units
        for &other_id in &duplicates {
            if other_id == check_id {
                continue;
            }

            let Some(other_event) = engine.get_state(other_id) else {
                continue;
            };

            // Different owners = captured/converted = PARADOX
            if check_event.state.owner != other_event.state.owner {
                println!(
                    "PARADOX! {:?} (owner {}) conflicts with {:?} (owner {})",
                    check_id, check_event.state.owner, other_id, other_event.state.owner
                );
            }
        }
    }

    // Simulate what happens when an enemy captures one of the duplicates
    println!("\n--- Simulating Enemy Capture ---");
    engine
        .set_state(
            marine_past,
            CombatUnit {
                name: "Marine".to_string(),
                hp: 100,
                damage: 10,
                owner: 2, // Now owned by player 2!
            },
        )
        .unwrap();
    println!("Enemy (player 2) captured the t=5 duplicate!");

    println!("\nRe-checking for paradoxes...\n");
    for &check_id in &duplicates {
        let Some(check_event) = engine.get_state(check_id) else {
            continue;
        };

        if check_event.lifecycle == LifecycleState::Chronoporting {
            continue;
        }

        for &other_id in &duplicates {
            if other_id == check_id {
                continue;
            }

            let Some(other_event) = engine.get_state(other_id) else {
                continue;
            };

            if other_event.lifecycle == LifecycleState::Chronoporting {
                continue;
            }

            if check_event.state.owner != other_event.state.owner {
                println!(
                    "PARADOX DETECTED! {:?} (owner {}) vs {:?} (owner {})",
                    check_id, check_event.state.owner, other_id, other_event.state.owner
                );
                println!("  -> In Achron, this would cause temporal damage!");
            }
        }
    }

    println!("\n--- Timeline Summary ---");
    println!("Total entities: {}", engine.entity_count());
    println!("Active entities: {}", engine.active_entity_count());

    println!("\nDone!");
}
