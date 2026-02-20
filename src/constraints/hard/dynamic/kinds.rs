//! Built-in hard+dynamic constraint kinds.
//!
//! Each variant represents a different inter-task relationship that depends
//! on the current partial schedule at evaluation time.
//!
//! # Variants
//!
//! | Kind          | Meaning                                                      |
//! |---------------|--------------------------------------------------------------|
//! | `Dependence`  | Target task schedulable **only if** reference task is placed |
//! | `Consecutive` | Target task schedulable **only after** reference task ends   |
//! | `Exclusive`   | Target task schedulable **only if** reference task is absent |

use super::constraint::{DynamicConstraint, SchedulingContext};
use crate::solution_space::{Interval, IntervalSet};
use qtty::Unit;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enumeration of built-in hard+dynamic constraint kinds.
///
/// Attached as edge data `D` in a [`SchedulingBlock`](crate::scheduling_block::SchedulingBlock),
/// each variant describes the relationship from the edge's **source** (reference task)
/// to the edge's **target** (constrained task).
///
/// # Examples
///
/// ```ignore
/// use virolai::constraints::hard::dynamic::DynConstraintKind;
///
/// // "task B can only be scheduled if task A is scheduled"
/// block.add_dependency(node_a, node_b, DynConstraintKind::Dependence);
///
/// // "task B must come after task A"
/// block.add_dependency(node_a, node_b, DynConstraintKind::Consecutive);
///
/// // "task B can only be scheduled if task A is NOT scheduled"
/// block.add_dependency(node_a, node_b, DynConstraintKind::Exclusive);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum DynConstraintKind {
    /// Target is schedulable **only if** the reference task has been placed.
    ///
    /// - Reference task scheduled → full `range` is valid
    /// - Reference task absent → empty (target cannot be scheduled)
    Dependence,

    /// Target is schedulable **only after** the reference task finishes.
    ///
    /// Implies dependence: if the reference task is absent, the target cannot
    /// be scheduled either.
    ///
    /// - Reference task scheduled at `[a_start, a_end)` →
    ///   valid window is `[max(range.start, a_end), range.end)`
    /// - Reference task absent → empty
    Consecutive,

    /// Target is schedulable **only if** the reference task is **not** placed.
    ///
    /// - Reference task absent → full `range` is valid
    /// - Reference task scheduled → empty (target is excluded)
    Exclusive,
}

impl<U: Unit> DynamicConstraint<U> for DynConstraintKind {
    fn compute_intervals(
        &self,
        range: Interval<U>,
        ref_task_id: &str,
        ctx: &SchedulingContext<U>,
    ) -> IntervalSet<U> {
        match self {
            Self::Dependence => {
                if ctx.schedule.contains_task(ref_task_id) {
                    IntervalSet::from(range)
                } else {
                    IntervalSet::new()
                }
            }

            Self::Consecutive => {
                if let Some(ref_interval) = ctx.schedule.get_interval(ref_task_id) {
                    let effective_start = if range.start().value() >= ref_interval.end().value() {
                        range.start()
                    } else {
                        ref_interval.end()
                    };
                    if effective_start.value() < range.end().value() {
                        IntervalSet::from(Interval::new(effective_start, range.end()))
                    } else {
                        IntervalSet::new()
                    }
                } else {
                    // Reference task not scheduled → target cannot be scheduled
                    IntervalSet::new()
                }
            }

            Self::Exclusive => {
                if ctx.schedule.contains_task(ref_task_id) {
                    IntervalSet::new()
                } else {
                    IntervalSet::from(range)
                }
            }
        }
    }

    fn stringify(&self) -> String {
        match self {
            Self::Dependence => "Dependence".to_string(),
            Self::Consecutive => "Consecutive".to_string(),
            Self::Exclusive => "Exclusive".to_string(),
        }
    }
}

impl std::fmt::Display for DynConstraintKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dependence => write!(f, "Dependence"),
            Self::Consecutive => write!(f, "Consecutive"),
            Self::Exclusive => write!(f, "Exclusive"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schedule::Schedule;
    use crate::solution_space::{Interval, SolutionSpace};
    use qtty::Second;

    fn iv(start: f64, end: f64) -> Interval<Second> {
        Interval::from_f64(start, end)
    }

    fn ctx_with_schedule(schedule: &Schedule<Second>) -> SchedulingContext<Second> {
        let ss = SolutionSpace::new();
        // We need a reference that outlives the ctx, so use a leaked ref for tests.
        // Instead, build it properly:
        SchedulingContext {
            schedule,
            solution_space: Box::leak(Box::new(ss)),
        }
    }

    fn empty_ctx() -> (Schedule<Second>, SolutionSpace<Second>) {
        (Schedule::new(), SolutionSpace::new())
    }

    // ── Dependence ────────────────────────────────────────────────────

    #[test]
    fn dependence_ref_scheduled_returns_full_range() {
        let mut schedule = Schedule::new();
        schedule.add("task-a", iv(10.0, 30.0)).unwrap();
        let ss = SolutionSpace::new();
        let ctx = SchedulingContext::new(&schedule, &ss);

        let result = DynConstraintKind::Dependence.compute_intervals(iv(0.0, 100.0), "task-a", &ctx);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], iv(0.0, 100.0));
    }

    #[test]
    fn dependence_ref_absent_returns_empty() {
        let (schedule, ss) = empty_ctx();
        let ctx = SchedulingContext::new(&schedule, &ss);

        let result = DynConstraintKind::Dependence.compute_intervals(iv(0.0, 100.0), "task-a", &ctx);
        assert!(result.is_empty());
    }

    // ── Consecutive ───────────────────────────────────────────────────

    #[test]
    fn consecutive_ref_scheduled_returns_after_ref_end() {
        let mut schedule = Schedule::new();
        schedule.add("task-a", iv(10.0, 30.0)).unwrap();
        let ss = SolutionSpace::new();
        let ctx = SchedulingContext::new(&schedule, &ss);

        let result =
            DynConstraintKind::Consecutive.compute_intervals(iv(0.0, 100.0), "task-a", &ctx);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], iv(30.0, 100.0));
    }

    #[test]
    fn consecutive_range_already_after_ref_end() {
        let mut schedule = Schedule::new();
        schedule.add("task-a", iv(10.0, 30.0)).unwrap();
        let ss = SolutionSpace::new();
        let ctx = SchedulingContext::new(&schedule, &ss);

        // Range starts after ref ends — full range is valid
        let result =
            DynConstraintKind::Consecutive.compute_intervals(iv(50.0, 100.0), "task-a", &ctx);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], iv(50.0, 100.0));
    }

    #[test]
    fn consecutive_range_ends_before_ref_end() {
        let mut schedule = Schedule::new();
        schedule.add("task-a", iv(10.0, 80.0)).unwrap();
        let ss = SolutionSpace::new();
        let ctx = SchedulingContext::new(&schedule, &ss);

        // Range [0, 50) but ref ends at 80 → no valid window
        let result =
            DynConstraintKind::Consecutive.compute_intervals(iv(0.0, 50.0), "task-a", &ctx);
        assert!(result.is_empty());
    }

    #[test]
    fn consecutive_ref_absent_returns_empty() {
        let (schedule, ss) = empty_ctx();
        let ctx = SchedulingContext::new(&schedule, &ss);

        let result =
            DynConstraintKind::Consecutive.compute_intervals(iv(0.0, 100.0), "task-a", &ctx);
        assert!(result.is_empty());
    }

    // ── Exclusive ─────────────────────────────────────────────────────

    #[test]
    fn exclusive_ref_absent_returns_full_range() {
        let (schedule, ss) = empty_ctx();
        let ctx = SchedulingContext::new(&schedule, &ss);

        let result = DynConstraintKind::Exclusive.compute_intervals(iv(0.0, 100.0), "task-a", &ctx);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], iv(0.0, 100.0));
    }

    #[test]
    fn exclusive_ref_scheduled_returns_empty() {
        let mut schedule = Schedule::new();
        schedule.add("task-a", iv(10.0, 30.0)).unwrap();
        let ss = SolutionSpace::new();
        let ctx = SchedulingContext::new(&schedule, &ss);

        let result = DynConstraintKind::Exclusive.compute_intervals(iv(0.0, 100.0), "task-a", &ctx);
        assert!(result.is_empty());
    }

    // ── Display / stringify ───────────────────────────────────────────

    #[test]
    fn stringify_variants() {
        use crate::constraints::hard::dynamic::DynamicConstraint;
        assert_eq!(
            DynamicConstraint::<Second>::stringify(&DynConstraintKind::Dependence),
            "Dependence"
        );
        assert_eq!(
            DynamicConstraint::<Second>::stringify(&DynConstraintKind::Consecutive),
            "Consecutive"
        );
        assert_eq!(
            DynamicConstraint::<Second>::stringify(&DynConstraintKind::Exclusive),
            "Exclusive"
        );
    }

    #[test]
    fn display_matches_stringify() {
        use crate::constraints::hard::dynamic::DynamicConstraint;
        for kind in [
            DynConstraintKind::Dependence,
            DynConstraintKind::Consecutive,
            DynConstraintKind::Exclusive,
        ] {
            assert_eq!(
                format!("{kind}"),
                DynamicConstraint::<Second>::stringify(&kind),
            );
        }
    }
}
