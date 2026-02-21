pub mod error;
pub mod hard;
pub mod node;
pub mod operations;
pub mod soft;

pub use error::ConstraintError;
pub use hard::Constraint;
pub use hard::IntervalConstraint;
pub use hard::ResourceConstraint;
pub use node::ConstraintExpr;

// Re-export dynamic constraint types at the `constraints` level.
pub use hard::{
    CoalitionConstraint, DynConstraintKind, DynamicConstraint, DynamicConstraintIndex,
    SchedulingContext,
};

use qtty::{Quantity, Unit};

/// Returns the minimum of two quantities.
pub fn quantity_min<U: Unit>(a: Quantity<U>, b: Quantity<U>) -> Quantity<U> {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal) => a,
        _ => b,
    }
}

/// Returns the maximum of two quantities.
pub fn quantity_max<U: Unit>(a: Quantity<U>, b: Quantity<U>) -> Quantity<U> {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Greater) | Some(std::cmp::Ordering::Equal) => a,
        _ => b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qtty::Seconds;

    #[test]
    fn quantity_min_returns_smaller() {
        let a = Seconds::new(10.0);
        let b = Seconds::new(20.0);
        assert_eq!(quantity_min(a, b), 10.0);
        assert_eq!(quantity_min(b, a), 10.0);
    }

    #[test]
    fn quantity_min_equal_values() {
        let a = Seconds::new(5.0);
        let b = Seconds::new(5.0);
        assert_eq!(quantity_min(a, b), 5.0);
    }

    #[test]
    fn quantity_max_returns_larger() {
        let a = Seconds::new(10.0);
        let b = Seconds::new(20.0);
        assert_eq!(quantity_max(a, b), 20.0);
        assert_eq!(quantity_max(b, a), 20.0);
    }

    #[test]
    fn quantity_max_equal_values() {
        let a = Seconds::new(7.0);
        let b = Seconds::new(7.0);
        assert_eq!(quantity_max(a, b), 7.0);
    }
}
