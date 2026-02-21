//! Hard + Dynamic constraints.
//!
//! Binary accept/reject constraints whose feasibility depends on runtime
//! state (e.g., current partial schedule, resource utilisation).
//!
//! # Overview
//!
//! Dynamic constraints are attached as **edge data** on a
//! [`SchedulingBlock`](crate::scheduling_block::SchedulingBlock) graph.
//! Each edge encodes a relationship from a *source* (reference) task to a
//! *target* (constrained) task.
//!
//! At each scheduling iteration the algorithm evaluates incoming edges for
//! each candidate task via [`DynamicConstraintIndex`] and intersects the
//! result with the pre-computed static solution space.
//!
//! # Built-in kinds
//!
//! | Kind          | Meaning                                               |
//! |---------------|-------------------------------------------------------|
//! | `Dependence`  | Target schedulable only if reference is placed         |
//! | `Consecutive` | Target schedulable only after reference finishes       |
//! | `Exclusive`   | Target schedulable only if reference is **not** placed |
//!
//! Custom kinds can be added by implementing [`DynamicConstraint`] for a
//! new type.

pub mod coalition;
pub mod constraint;
pub mod evaluate;
pub mod kinds;

pub use coalition::CoalitionConstraint;
pub use constraint::{DynamicConstraint, SchedulingContext};
pub use evaluate::DynamicConstraintIndex;
pub use kinds::DynConstraintKind;
