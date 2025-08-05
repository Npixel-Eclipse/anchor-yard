<!--
This README is available in English and Korean.
이 README는 영어와 한국어로 제공됩니다.
-->
[English](#anchor-yard) | [한국어](#anchor-yard-korean)

# Anchor Yard

[![Build Status](https://img.shields.io/github/actions/workflow/status/your-repo/rust.yml?branch=main)](https://github.com/your-repo/actions)
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

### 2. Register Components

At your application's entry point (e.g., in the `main` function), register the components you want to include in snapshots.

```rust
use anchor_yard::REGISTRY;
use shipyard::{Component, World};
use serde::{Serialize, Deserialize};

#[derive(Component, Serialize, Deserialize)]
struct Position { x: f32, y: f32 }

#[derive(Component, Serialize, Deserialize)]
struct Velocity { x: f32, y: f32 }

fn main() {
    let mut registry = REGISTRY.lock().unwrap();
    registry.register::<Position>();
    registry.register::<Velocity>();
}
```

### 3. Apply Attribute to System

Add the `#[snapshot_system]` attribute to the system you want to profile. You can set `threshold_ms` to specify the execution time that triggers a snapshot.

```rust
use anchor_yard::snapshot_system;
use shipyard::{View, ViewMut};

#[snapshot_system(threshold_ms = 10)] // Creates a snapshot if it takes longer than 10ms
fn slow_combat_system(mut healths: ViewMut<Health>, positions: View<Position>) {
    // ... system logic ...
    std::thread::sleep(std::time::Duration::from_millis(15));
}
```

### 4. Run the World

When running your system, use `run_with_snapshot` or `run_workload_with_snapshot`.

```rust
use anchor_yard::WorldSnapshotExt;

let mut world = World::new();
// ... add entities and components ...

// Run an individual system
world.run_with_snapshot(|| world.run(slow_combat_system));

// Run a workload
world.add_workload("game_loop", slow_combat_system);
world.run_workload_with_snapshot("game_loop").unwrap();
```

Now, if `slow_combat_system` takes longer than 10ms to execute, a `slow_combat_system_TIMESTAMP.snapshot` file will be created in the `snapshots/` directory!

## 📦 Crate Structure

`anchor-yard` follows a modular design:

- `anchor-yard`: The main crate that integrates all features for the easiest use.
- `anchor-yard-core`: Contains the core logic for creating, saving, and restoring snapshots.
- `anchor-yard-macros`: Provides the `#[snapshot_system]` procedural macro.

---

# Anchor Yard (Korean)

**⚓ `anchor-yard`는 `shipyard` ECS를 위한 간단하고 효과적인 성능 프로파일링 및 월드 스냅샷 도구입니다.**

느린 시스템을 식별하고, 실행 시점의 월드 상태를 캡처하여 병목 현상을 쉽게 디버깅하고 분석할 수 있도록 도와줍니다.

## 🤔 무엇을 해결하나요?

`shipyard`를 사용하여 복잡한 게임이나 시뮬레이션을 개발할 때, 특정 시스템이 왜 느려지는지 파악하기 어려울 수 있습니다. `anchor-yard`는 이 문제를 다음과 같이 해결합니다:

- **자동 스냅샷**: 시스템 실행 시간이 설정된 임계값을 초과하면 자동으로 해당 시점의 `World` 상태를 스냅샷으로 저장합니다.
- **상태 분석**: 저장된 스냅샷을 로드하여, 성능 저하가 발생했을 때의 모든 엔티티와 컴포넌트 데이터를 정밀하게 분석할 수 있습니다.
- **최소한의 코드 변경**: 간단한 `#[snapshot_system]` 어트리뷰트 하나로 기존 시스템에 프로파일링 및 스냅샷 기능을 적용할 수 있습니다.

## ✨ 주요 기능

- **임계값 기반 스냅샷**: 지정한 시간(ms)을 초과하는 시스템의 스냅샷을 자동으로 캡처합니다.
- **간편한 통합**: `#[snapshot_system]` 어트리뷰트를 시스템 함수에 추가하기만 하면 됩니다.
- **월드 상태 저장 및 복원**: `World`의 전체 상태를 파일로 저장하고, 나중에 다시 로드하여 완벽하게 복원할 수 있습니다.
- **유연한 설정**: 어떤 컴포넌트를 스냅샷에 포함할지 선택적으로 등록할 수 있습니다.

## 🚀 시작하기

### 1. 의존성 추가

`Cargo.toml` 파일에 `anchor-yard`를 추가합니다.

```toml
[dependencies]
anchor-yard = "0.1.0" # 원하는 버전을 사용하세요
shipyard = "0.6"
serde = { version = "1.0", features = ["derive"] }
```

### 2. 컴포넌트 등록

애플리케이션 시작 지점 (예: `main` 함수)에서 스냅샷에 포함할 컴포넌트들을 등록합니다.

```rust
use anchor_yard::REGISTRY;
use shipyard::{Component, World};
use serde::{Serialize, Deserialize};

#[derive(Component, Serialize, Deserialize)]
struct Position { x: f32, y: f32 }

#[derive(Component, Serialize, Deserialize)]
struct Velocity { x: f32, y: f32 }

fn main() {
    let mut registry = REGISTRY.lock().unwrap();
    registry.register::<Position>();
    registry.register::<Velocity>();
}
```

### 3. 시스템에 어트리뷰트 적용

프로파일링하고 싶은 시스템에 `#[snapshot_system]` 어트리뷰트를 추가합니다. `threshold_ms`를 설정하여 스냅샷을 트리거할 실행 시간을 지정할 수 있습니다.

```rust
use anchor_yard::snapshot_system;
use shipyard::{View, ViewMut};

#[snapshot_system(threshold_ms = 10)] // 10ms 이상 걸리면 스냅샷 생성
fn slow_combat_system(mut healths: ViewMut<Health>, positions: View<Position>) {
    // ... 시스템 로직 ...
    std::thread::sleep(std::time::Duration::from_millis(15));
}
```

### 4. 월드 실행

시스템을 실행할 때 `run_with_snapshot` 또는 `run_workload_with_snapshot`을 사용합니다.

```rust
use anchor_yard::WorldSnapshotExt;

let mut world = World::new();
// ... 엔티티와 컴포넌트 추가 ...

// 개별 시스템 실행
world.run_with_snapshot(|| world.run(slow_combat_system));

// 워크로드 실행
world.add_workload("game_loop", slow_combat_system);
world.run_workload_with_snapshot("game_loop").unwrap();
```

이제 `slow_combat_system`이 10ms 이상 실행되면 `snapshots/` 디렉토리에 `slow_combat_system_TIMESTAMP.snapshot` 파일이 생성됩니다!

## 📦 크레이트 구조

`anchor-yard`는 모듈식 설계를 따릅니다:

- `anchor-yard`: 핵심 기능을 통합하고 가장 쉽게 사용할 수 있는 메인 크레이트입니다.
- `anchor-yard-core`: 스냅샷 생성, 저장, 복원 등 핵심 로직을 포함합니다.
- `anchor-yard-macros`: `#[snapshot_system]` 절차적 매크로를 제공합니다.
