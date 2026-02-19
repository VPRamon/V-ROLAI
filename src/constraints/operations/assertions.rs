//! Interval set operations for constraint trees.

use crate::solution_space::Interval;
use qtty::Unit;

/// Returns true if `intervals` is canonical: each interval has start <= end,
/// intervals are sorted by start, and they do not overlap (previous end <= next start).
pub fn is_canonical<U: Unit>(intervals: &[Interval<U>]) -> bool {
    intervals.windows(2).all(|w| {
        let prev = &w[0];
        let curr = &w[1];
        !curr.overlaps(prev) && prev.end() <= curr.start()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use qtty::Second;

    fn iv(start: f64, end: f64) -> Interval<Second> {
        Interval::from_f64(start, end)
    }

    #[test]
    fn empty_is_canonical() {
        let intervals: Vec<Interval<Second>> = vec![];
        assert!(is_canonical(&intervals));
    }

    #[test]
    fn single_interval_is_canonical() {
        assert!(is_canonical(&[iv(0.0, 50.0)]));
    }

    #[test]
    fn sorted_non_overlapping_is_canonical() {
        assert!(is_canonical(&[
            iv(0.0, 10.0),
            iv(20.0, 30.0),
            iv(40.0, 50.0)
        ]));
    }

    #[test]
    fn overlapping_is_not_canonical() {
        assert!(!is_canonical(&[iv(0.0, 30.0), iv(20.0, 50.0)]));
    }

    #[test]
    fn unsorted_is_not_canonical() {
        assert!(!is_canonical(&[iv(20.0, 30.0), iv(0.0, 10.0)]));
    }

    #[test]
    fn adjacent_is_canonical() {
        // Half-open intervals [0, 10) and [10, 20) share no interior points,
        // so they are non-overlapping and correctly sorted → canonical.
        assert!(is_canonical(&[iv(0.0, 10.0), iv(10.0, 20.0)]));
    }
}
