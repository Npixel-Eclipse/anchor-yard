[English](#anchor-yard)

# Anchor Yard

[![Crates.io](https://img.shields.io/crates/v/anchor-yard.svg)](https://crates.io/crates/anchor-yard)
[![Docs.rs](https://docs.rs/anchor-yard/badge.svg)](https://docs.rs/anchor-yard)

**⚓ `anchor-yard` is a simple and effective performance profiling and world snapshot tool for the `shipyard` ECS.**

It helps you identify slow systems and capture the world state at the time of execution, making it easy to debug and analyze bottlenecks.

## 🤔 What Problem Does It Solve?

When developing complex games or simulations with `shipyard`, it can be difficult to figure out why a particular system is slowing down. `anchor-yard` solves this problem by:

- **Automatic Snapshots**: Automatically saves a snapshot of the `World` state when a system's execution time exceeds a set threshold.
- **State Analysis**: Load saved snapshots to precisely analyze all entity and component data at the moment a performance drop occurred.
- **Minimal Code Changes**: Apply profiling and snapshot capabilities to your existing systems with a single `#[snapshot_system]` attribute.

## ✨ Key Features

- **Threshold-based Snapshots**: Automatically captures a snapshot of systems that exceed a specified execution time in milliseconds.
- **Easy Integration**: Simply add the `#[snapshot_system]` attribute to your system functions.
- **Save and Restore World State**: Save the entire state of the `World` to a file and restore it perfectly later.
- **Flexible Configuration**: Selectively register which components to include in the snapshot.

## 🚀 Getting Started

### 1. Add Dependency

Add `anchor-yard` to your `Cargo.toml` file.

```toml
[dependencies]
anchor-yard = "0.1.0" # Use your desired version
shipyard = "0.6"
serde = { version = "1.0", features = ["derive"] }
```

### 2. Apply Attribute to System

Add the `#[snapshot_system]` attribute to the system you want to profile. You can set `threshold_ms` to specify the execution time that triggers a snapshot.

```rust
use anchor_yard::snapshot_system;
use shipyard::{View, ViewMut};

#[snapshot_system(threshold_ms = 10)] // Creates a snapshot if it takes longer than 10ms
fn slow_combat_system(mut healths: ViewMut<Health>, positions: View<Position>) {
    // ... system logic ...
    std::thread::sleep(std::time::Duration::from_millis(15));
}

fn fast_movement_system(mut positions: ViewMut<Position>, velocities: View<Velocity>) {
    // ... system logic
}
```

### 3. Run the World

When running your system, use `run_default_workload_with_snapshot`.

```rust
use anchor_yard::WorldSnapshotExt;

let mut world = World::new();
// ... add entities and components ...

// Add workload
let workload = (slow_combat_system, fast_movement_system).into_workload();
world.add_workload(workload);

// Run workload
world.run_default_workload_with_snapshot().unwrap();
```

Now, if `slow_combat_system` takes longer than 10ms to execute, a `slow_combat_system_TIMESTAMP.snapshot` file will be created in the `snapshots/` directory!

## 📦 Crate Structure

`anchor-yard` follows a modular design:

- `anchor-yard`: The main crate that integrates all features for the easiest use.
- `anchor-yard-core`: Contains the core logic for creating, saving, and restoring snapshots.
- `anchor-yard-macros`: Provides the `#[snapshot_system]` procedural macro.
