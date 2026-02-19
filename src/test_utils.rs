//! Shared test utilities for virolai tests.
//!
//! Provides reusable mock types and helper functions used across multiple test modules.

use crate::constraints::{ConstraintExpr, IntervalConstraint};
use crate::scheduling_block::Task;
use crate::solution_space::Interval;
use qtty::{Quantity, Second};

/// Convenience helper: creates an `Interval<Second>` from two `f64` values.
pub fn iv(start: f64, end: f64) -> Interval<Second> {
    Interval::from_f64(start, end)
}

/// Convenience helper: creates a `Quantity<Second>` from an `f64` value.
pub fn q(value: f64) -> Quantity<Second> {
    Quantity::new(value)
}

/// A configurable mock task for testing scheduling logic.
///
/// Supports setting name, size, priority, gap_after, and optional constraints.
#[derive(Debug, Clone)]
pub struct TestTask {
    pub name: String,
    pub size: Quantity<Second>,
    pub priority: i32,
    pub delay: Quantity<Second>,
    pub constraints: Option<ConstraintExpr<IntervalConstraint<Second>>>,
}

impl TestTask {
    /// Creates a simple test task with the given name and size (seconds).
    /// Priority defaults to 0, delay to 0, no constraints.
    pub fn new(name: &str, size: f64) -> Self {
        Self {
            name: name.to_string(),
            size: Quantity::new(size),
            priority: 0,
            delay: Quantity::new(0.0),
            constraints: None,
        }
    }

    /// Sets the priority and returns self (builder pattern).
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Sets the gap_after and returns self (builder pattern).
    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Quantity::new(delay);
        self
    }

    /// Sets constraints and returns self (builder pattern).
    pub fn with_constraints(
        mut self,
        constraints: ConstraintExpr<IntervalConstraint<Second>>,
    ) -> Self {
        self.constraints = Some(constraints);
        self
    }
}

impl Task<Second> for TestTask {
    type SizeUnit = Second;
    type ConstraintLeaf = IntervalConstraint<Second>;

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> Quantity<Second> {
        self.size
    }

    fn priority(&self) -> i32 {
        self.priority
    }

    fn gap_after(&self) -> Quantity<Second> {
        self.delay
    }

    fn constraints(&self) -> Option<&ConstraintExpr<IntervalConstraint<Second>>> {
        self.constraints.as_ref()
    }

    fn compute_gap_after(&self, _previous_task: &Self) -> Quantity<Second> {
        self.delay
    }
}
