//! Soft constraints — preference / scoring functions.
//!
//! Subdivided by data lifetime:
//! - [`static_`] — parameters fixed before the scheduling loop.
//! - [`dynamic`] — evaluated at runtime against mutable state.

pub mod dynamic;
#[allow(non_snake_case)]
pub mod static_;
