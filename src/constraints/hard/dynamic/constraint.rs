//! Core trait for hard+dynamic constraints.
//!
//! Dynamic constraints depend on **runtime state** (the partial schedule built
//! so far, resource utilisation, etc.) to compute feasibility windows.  Their
//! `compute_intervals` method therefore receives a [`SchedulingContext`] in
//! addition to the query range.
//!
//! See also: [`Constraint`](crate::constraints::Constraint) for the static
//! counterpart whose windows are fixed before the scheduling loop.

use crate::schedule::Schedule;
use crate::solution_space::{Interval, IntervalSet, SolutionSpace};
use qtty::Unit;
use std::fmt::Debug;

/// Runtime state available to dynamic constraints during evaluation.
///
/// Bundled into a struct so the trait signature stays stable when new
/// context fields are added (e.g. resource utilisation, iteration count).
///
/// All references are immutable borrows — dynamic constraints **read** state
/// but never mutate it.
#[derive(Debug)]
pub struct SchedulingContext<'a, U: Unit> {
    /// Current partial schedule (tasks already placed).
    pub schedule: &'a Schedule<U>,
    /// Static solution space (pre-computed from static constraints).
    pub solution_space: &'a SolutionSpace<U>,
}

impl<'a, U: Unit> SchedulingContext<'a, U> {
    /// Creates a new context from the current schedule and solution space.
    pub fn new(schedule: &'a Schedule<U>, solution_space: &'a SolutionSpace<U>) -> Self {
        Self {
            schedule,
            solution_space,
        }
    }
}

/// Computes intervals where a dynamic scheduling condition is satisfied.
///
/// Unlike the static [`Constraint`](crate::constraints::Constraint) trait,
/// dynamic constraints receive:
/// - `ref_task_id` — the task on the *other* end of the dependency edge
///   (resolved from graph topology, **not** stored inside the constraint).
/// - `ctx` — immutable view of the current scheduling state.
///
/// # Contract
///
/// Implementations should:
/// - Return non-overlapping, sorted intervals within `[start, end]`
/// - Be deterministic for identical `(range, ref_task_id, ctx)` inputs
/// - Handle empty results gracefully (return empty `IntervalSet`)
pub trait DynamicConstraint<U: Unit>: Send + Sync + Debug {
    /// Computes intervals where this constraint is satisfied within `range`,
    /// given a reference task `ref_task_id` and the current scheduling context.
    fn compute_intervals(
        &self,
        range: Interval<U>,
        ref_task_id: &str,
        ctx: &SchedulingContext<U>,
    ) -> IntervalSet<U>;

    /// Returns a human-readable description of this constraint.
    fn stringify(&self) -> String;

    /// Prints this constraint to stdout.
    fn print(&self) {
        println!("{}", self.stringify());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qtty::Second;

    #[test]
    fn scheduling_context_construction() {
        let schedule = Schedule::<Second>::new();
        let solution_space = SolutionSpace::<Second>::new();
        let ctx = SchedulingContext::new(&schedule, &solution_space);
        assert!(ctx.schedule.is_empty());
        assert!(ctx.solution_space.is_empty());
    }
}
