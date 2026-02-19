pub mod constraint;
pub mod error;
pub mod node;
pub mod operations;

pub use constraint::Constraint;
pub use constraint::IntervalConstraint;
pub use error::ConstraintError;
pub use node::ConstraintExpr;

use qtty::{Quantity, Unit};

/// Returns the minimum of two quantities.
pub fn quantity_min<U: Unit>(a: Quantity<U>, b: Quantity<U>) -> Quantity<U> {
    match a.value().partial_cmp(&b.value()) {
        Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal) => a,
        _ => b,
    }
}

/// Returns the maximum of two quantities.
pub fn quantity_max<U: Unit>(a: Quantity<U>, b: Quantity<U>) -> Quantity<U> {
    match a.value().partial_cmp(&b.value()) {
        Some(std::cmp::Ordering::Greater) | Some(std::cmp::Ordering::Equal) => a,
        _ => b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qtty::Second;

    #[test]
    fn quantity_min_returns_smaller() {
        let a = Quantity::<Second>::new(10.0);
        let b = Quantity::<Second>::new(20.0);
        assert_eq!(quantity_min(a, b).value(), 10.0);
        assert_eq!(quantity_min(b, a).value(), 10.0);
    }

    #[test]
    fn quantity_min_equal_values() {
        let a = Quantity::<Second>::new(5.0);
        let b = Quantity::<Second>::new(5.0);
        assert_eq!(quantity_min(a, b).value(), 5.0);
    }

    #[test]
    fn quantity_max_returns_larger() {
        let a = Quantity::<Second>::new(10.0);
        let b = Quantity::<Second>::new(20.0);
        assert_eq!(quantity_max(a, b).value(), 20.0);
        assert_eq!(quantity_max(b, a).value(), 20.0);
    }

    #[test]
    fn quantity_max_equal_values() {
        let a = Quantity::<Second>::new(7.0);
        let b = Quantity::<Second>::new(7.0);
        assert_eq!(quantity_max(a, b).value(), 7.0);
    }
}
