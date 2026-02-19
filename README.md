# VIROLAI
Virtual Resource Optimization Leveraging AI

A Rust library for DAG-based task scheduling using `petgraph`, designed for flexibility, type safety, and extensibility.

## Features

- **Trait-based Task Abstraction**: Define custom task types by implementing the `Task` trait
- **Generic DAG Structure**: Type-safe scheduling blocks with any task and dependency types
- **Cycle Prevention**: Proactive cycle detection ensures DAG invariants
- **Rich Query APIs**: Topological ordering, critical path analysis, root/leaf detection
- **Thread-Safe**: All tasks are `Send + Sync + 'static` for concurrent scheduling
- **Object-Safe**: Can use trait objects (`Box<dyn Task>`) when needed

## Quick Start

### 1. Define Your Task Types

```rust
use qtty::{Quantity, Second};
use virolai::constraints::IntervalConstraint;
use virolai::scheduling_block::Task;

#[derive(Debug, Clone)]
struct MyTask {
    name: String,
    duration: Quantity<Second>,
}

impl Task<Second> for MyTask {
    type SizeUnit = Second;
    type ConstraintLeaf = IntervalConstraint<Second>;

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> Quantity<Second> {
        self.duration
    }
}
```

### 2. Create a Scheduling Block

```rust
use qtty::{Quantity, Second};
use virolai::scheduling_block::{SchedulingBlock, SchedulingError};

fn main() -> Result<(), SchedulingError> {
    let mut schedule = SchedulingBlock::<MyTask, Second>::new();

    // Add tasks – each returns a unique String ID
    let id1 = schedule.add_task(MyTask { name: "Task 1".into(), duration: Quantity::new(1.0) });
    let id2 = schedule.add_task(MyTask { name: "Task 2".into(), duration: Quantity::new(2.0) });
    let id3 = schedule.add_task(MyTask { name: "Task 3".into(), duration: Quantity::new(1.5) });

    // Resolve IDs to NodeIndex handles for dependency edges
    let n1 = schedule.node_of(&id1).unwrap();
    let n2 = schedule.node_of(&id2).unwrap();
    let n3 = schedule.node_of(&id3).unwrap();

    // Define dependencies (task1 must complete before task2 and task3)
    schedule.add_dependency(n1, n2, ())?;
    schedule.add_dependency(n1, n3, ())?;

    // Get execution order
    let order = schedule.topo_order()?;
    for node in order {
        let task = schedule.get_task(node).unwrap();
        println!("Execute: {}", task.name());
    }

    // Compute critical path (duration in axis units – seconds here)
    let (duration, _path) = schedule.critical_path()?;
    println!("Total duration: {:.1}s", duration);

    Ok(())
}
```

### 3. Mix Multiple Task Types

Use enums to combine different task types:

```rust
use qtty::{Quantity, Second};
use virolai::constraints::IntervalConstraint;

enum TaskType {
    Observation(ObservationTask),
    Calibration(CalibrationTask),
}

impl Task<Second> for TaskType {
    type SizeUnit = Second;
    type ConstraintLeaf = IntervalConstraint<Second>;

    fn name(&self) -> &str {
        match self {
            TaskType::Observation(t) => t.name(),
            TaskType::Calibration(t) => t.name(),
        }
    }

    fn size(&self) -> Quantity<Second> {
        match self {
            TaskType::Observation(t) => t.size(),
            TaskType::Calibration(t) => t.size(),
        }
    }
}

let mut schedule = SchedulingBlock::<TaskType, Second, DependencyKind>::new();
```

## Examples

Astronomy-specific examples now live in `astro_scheduler`.

```bash
# Astronomical observation task scheduling
cargo run -p astro_scheduler --example astro_observation

# Altitude constraint demos
cargo run -p astro_scheduler --example altitude_constraint_demo
cargo run -p astro_scheduler --example altitude_multi_target

# Demo application using VIROLAI + astronomy modules
cargo run -p astro_scheduler
```

See `astro_scheduler/examples/README.md`.

## Documentation

- **[API Docs](https://docs.rs/virolai)**: Generated API documentation

## API Overview

### Core Trait

```rust
pub trait Task<A: Unit>: Send + Sync + Debug + 'static {
    /// Unit in which the task's size is expressed (must share dimension with `A`).
    type SizeUnit: SameDim<A>;
    /// Leaf type used in constraint trees.
    type ConstraintLeaf: Constraint<A>;

    fn name(&self) -> &str;
    /// Duration in the task's natural unit.
    fn size(&self) -> Quantity<Self::SizeUnit>;
    /// Duration converted to the scheduling axis unit (default: `size().to::<A>()`).
    fn size_on_axis(&self) -> Quantity<A> { self.size().to::<A>() }
    fn priority(&self) -> i32 { 0 }
    fn constraints(&self) -> Option<&ConstraintExpr<Self::ConstraintLeaf>> { None }
    /// Gap inserted after this task in the timeline (default: zero).
    fn gap_after(&self) -> Quantity<A> { Quantity::new(0.0) }
}
```

### SchedulingBlock Methods

- `new()` - Create empty scheduling block
- `add_task(task: T) -> Id` - Add a task, returns a unique String ID
- `add_task_with_id(task, Option<Id>) -> Result<Id, SchedulingError>` - Add with a caller-supplied ID
- `node_of(id: &str) -> Option<NodeIndex>` - Resolve a String ID to its graph node handle
- `task_by_id(id: &str) -> Option<&T>` - Look up a task by ID
- `tasks() -> impl Iterator<(Id, &T)>` - Iterate all `(id, task)` pairs
- `remove_task(id: &str) -> Option<T>` - Remove task and all incident edges by ID
- `add_dependency(from: NodeIndex, to: NodeIndex, dep: D)` - Add ordering edge with cycle detection
- `topo_order()` - Get topological ordering (returns `Vec<NodeIndex>`)
- `critical_path()` - Compute longest path; returns `(f64, Vec<NodeIndex>)` in axis units
- `roots()` - Get tasks with no predecessors
- `leaves()` - Get tasks with no successors
- `graph()` - Access underlying `petgraph::StableGraph` for advanced queries

### Error Handling

```rust
pub enum SchedulingError {
    CycleDetected,
    InvalidNodeIndex(NodeIndex),
    GraphContainsCycle,
    EmptyGraph,
    DuplicateId(String),
}
```

## Design Philosophy

VIROLAI uses **generic types** (`SchedulingBlock<T: Task, D>`) rather than trait objects (`Box<dyn Task>`) as the primary pattern because:

- ✅ Zero-cost abstraction (no heap allocation or virtual dispatch)
- ✅ Type safety (access task-specific fields without downcasting)
- ✅ Better performance (compiler optimizations, cache locality)
- ✅ Natural enum-based composition in Rust

Trait objects are still supported for dynamic use cases (plugins, runtime loading).

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please open an issue or PR.
