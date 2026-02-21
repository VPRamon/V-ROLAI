//! Optional spatial extension for tasks with a position in some coordinate system.
//!
//! This trait is **not required** by core scheduling algorithms — it provides
//! an extension point for domain-specific spatial reasoning (e.g., slew time,
//! sky coverage, positional constraints).
//!
//! The coordinate type `C` is generic so each domain can bring its own:
//! - Astronomy: `ICRS` (ra/dec)
//! - Robotics: `(f64, f64)` (x, y)
//! - Logistics: `GeoCoord` (lat, lon)
//!
//! # Example
//!
//! ```ignore
//! use virolai::scheduling_block::SpatialTask;
//!
//! struct MyTask {
//!     name: String,
//!     position: (f64, f64),
//! }
//!
//! impl SpatialTask<(f64, f64)> for MyTask {
//!     fn position(&self) -> &(f64, f64) {
//!         &self.position
//!     }
//! }
//! ```

use std::fmt::Debug;

/// A task that has an associated spatial position.
///
/// `C` is the coordinate type, chosen by the domain (e.g., `ICRS` for astronomy).
/// This trait is intentionally separate from [`Task`](super::Task) so that:
/// - Core scheduling algorithms remain coordinate-agnostic.
/// - Only domains with spatial information need to implement it.
/// - It can be used as an additional trait bound where spatial reasoning is needed.
pub trait SpatialTask<C: Debug> {
    /// Returns a reference to this task's spatial position.
    fn position(&self) -> &C;
}
