use anchor_yard::{REGISTRY, WorldSnapshotExt, snapshot_system};
use serde::{Deserialize, Serialize};
use shipyard::{Component, IntoIter, View, ViewMut, Workload, World};

#[derive(Component, Serialize, Deserialize, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Serialize, Deserialize, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Serialize, Deserialize, Debug)]
struct Health {
    points: i32,
}

#[snapshot_system(threshold_ms = 0)]
fn fast_physics_system(mut positions: ViewMut<Position>, velocities: View<Velocity>) {
    for (pos, vel) in (&mut positions, &velocities).iter() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

#[snapshot_system(threshold_ms = 0)]
fn slow_combat_system(mut healths: ViewMut<Health>, positions: View<Position>) {
    for (health, pos) in (&mut healths, &positions).iter() {
        std::thread::sleep(std::time::Duration::from_millis(5));

        if pos.x > 50.0 {
            health.points -= 1;
        }
    }
}

#[snapshot_system(threshold_ms = 0)]
fn heavy_ai_system(positions: View<Position>, healths: View<Health>) {
    std::thread::sleep(std::time::Duration::from_millis(10));

    let mut calculations = 0;
    for (pos, health) in (&positions, &healths).iter() {
        calculations += (pos.x * pos.y + health.points as f32) as i32;
    }
    println!("AI calculations completed: {}", calculations);
}

fn main() {
    println!("=== Anchor Yard Snapshot System Demo ===\n");

    let mut world = World::new();

    let mut registry = REGISTRY.lock().unwrap();
    registry.register::<Position>();
    registry.register::<Velocity>();
    registry.register::<Health>();
    drop(registry);

    // 테스트 엔티티 생성
    println!("Creating test entities...");
    for i in 0..100 {
        world.add_entity((
            Position {
                x: i as f32,
                y: (i % 10) as f32,
            },
            Velocity { x: 1.0, y: 0.5 },
            Health {
                points: 100 - (i % 20),
            },
        ));
    }

    println!("Created 100 entities\n");

    // 개별 시스템 실행 테스트
    println!("=== Testing Individual Systems ===");

    println!("Running fast_physics_system (should NOT create snapshot)...");
    world.run_with_snapshot(|| world.run(fast_physics_system));

    println!("Running slow_combat_system (should create snapshot)...");
    world.run_with_snapshot(|| world.run(slow_combat_system));

    println!("Running heavy_ai_system (should create snapshot)...");
    world.run_with_snapshot(|| world.run(heavy_ai_system));

    println!("\n=== Testing Workload ===");

    // Workload 생성 및 실행
    Workload::new("game_loop")
        .with_system(fast_physics_system)
        .with_system(slow_combat_system)
        .with_system(heavy_ai_system)
        .add_to_world(&world)
        .unwrap();

    println!("Running complete workload...");
    if let Err(e) = world.run_workload_with_snapshot("game_loop") {
        eprintln!("Workload failed: {}", e);
    }

    println!("\n=== Snapshot Files Created ===");

    // 생성된 스냅샷 파일들 확인
    if let Ok(entries) = std::fs::read_dir("snapshots") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "snapshot") {
                    if let Some(filename) = path.file_name() {
                        println!("📁 {}", filename.to_string_lossy());
                    }
                }
            }
        }
    } else {
        println!("No snapshot directory found - no slow systems detected!");
    }

    println!("\n✅ Demo completed! Check the 'snapshots' folder for captured system states.");
    println!("💡 Tip: Run with 'cargo run --example basic_usage' to see this demo");
}

// #[test]
// fn load_snapshot() {
//     let mut registry = REGISTRY.lock().unwrap();
//     registry.register::<Position>();
//     registry.register::<Velocity>();
//     registry.register::<Health>();
//     drop(registry);

//     let snapshot = anchor_yard::SystemSnapshot::load_from_file(std::path::Path::new(
//         "snapshots/slow_combat_system_1754386748.snapshot",
//     ))
//     .unwrap();
//     let world = snapshot.restore_world().unwrap();

//     let positions = world.borrow::<View<Position>>().unwrap();
//     let velocities: View<'_, Velocity, shipyard::track::Untracked> =
//         world.borrow::<View<Velocity>>().unwrap();
//     let healths = world.borrow::<View<Health>>().unwrap();
//     println!("positions: {:?}", positions);
//     println!("velocities: {:?}", velocities);
//     println!("healths: {:?}", healths);
//     for (pos, vel, health) in (&positions, &velocities, &healths).iter() {
//         println!(
//             "Position: {:?}, Velocity: {:?}, Health: {:?}",
//             pos, vel, health
//         );
//     }
// }
