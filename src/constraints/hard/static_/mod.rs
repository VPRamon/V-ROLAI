//! Hard + Static constraints.
//!
//! Feasibility windows fully determined before the scheduling loop.
//! Produces a binary accept/reject (hard) decision from fixed (static) data.
//!
//! The [`Constraint`] trait and the built-in [`IntervalConstraint`] live here.

pub mod constraint;

pub use constraint::Constraint;
pub use constraint::IntervalConstraint;
