//! Continuous interval representation for task scheduling.

use std::fmt::Display;

use qtty::{Quantity, Unit};

/// Continuous range `[start, end)` where a task may be scheduled.
///
/// The interval is **half-open**: `start` is inclusive, `end` is exclusive.
/// This avoids ambiguity at shared boundaries (two abutting intervals never
/// overlap) and eliminates the need for epsilon offsets in the scheduler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval<U: Unit> {
    start: Quantity<U>,
    end: Quantity<U>,
}

impl<U: Unit> Interval<U> {
    /// Creates interval `[start, end)`.
    ///
    /// # Panics
    ///
    /// Panics if `start > end`.
    pub const fn new(start: Quantity<U>, end: Quantity<U>) -> Self {
        assert!(
            start.value() <= end.value(),
            "Interval start must be <= end"
        );
        Self { start, end }
    }

    pub const fn from_f64(start: f64, end: f64) -> Self {
        Self::new(Quantity::<U>::new(start), Quantity::<U>::new(end))
    }

    pub const fn start(&self) -> Quantity<U> {
        self.start
    }

    pub const fn end(&self) -> Quantity<U> {
        self.end
    }

    pub fn duration(&self) -> Quantity<U> {
        self.end - self.start
    }

    /// Converts this interval to another unit of the same dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty::*;
    /// use virolai::solution_space::Interval;
    ///
    /// let interval_sec = Interval::<Second>::new(
    ///     Seconds::new(0.0),
    ///     Seconds::new(86400.0)
    /// );
    /// let interval_day: Interval<Day> = interval_sec.to();
    /// assert!((interval_day.start().value() - 0.0).abs() < 1e-12);
    /// assert!((interval_day.end().value() - 1.0).abs() < 1e-12);
    /// ```
    pub fn to<T: Unit<Dim = U::Dim>>(self) -> Interval<T> {
        Interval::new(self.start.to(), self.end.to())
    }

    /// Returns true if `position` ∈ `[start, end)` (start inclusive, end exclusive).
    pub const fn contains(&self, position: Quantity<U>) -> bool {
        self.start.value() <= position.value() && position.value() < self.end.value()
    }

    /// Returns true if this interval shares any interior point with `other`.
    ///
    /// Because intervals are half-open, abutting intervals (`self.end == other.start`)
    /// do **not** overlap — they are back-to-back with no shared point.
    pub const fn overlaps(&self, other: &Interval<U>) -> bool {
        self.start.value() < other.end.value() && other.start.value() < self.end.value()
    }

    pub fn intersection(&self, other: &Interval<U>) -> Option<Interval<U>> {
        if self.overlaps(other) {
            let start = if self.start.value() > other.start.value() {
                self.start
            } else {
                other.start
            };
            let end = if self.end.value() < other.end.value() {
                self.end
            } else {
                other.end
            };
            Some(Interval::new(start, end))
        } else {
            None
        }
    }

    /// Returns true if task of `size` fits starting at `start_position`.
    pub fn can_fit(&self, start_position: Quantity<U>, size: Quantity<U>) -> bool {
        self.contains(start_position) && (start_position + size).value() <= self.end.value()
    }
}

impl<U: Unit> Display for Interval<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.3}, {:.3}]", self.start.value(), self.end.value())
    }
}

// =============================================================================
// Interval Serde Support
// =============================================================================

#[cfg(feature = "serde")]
impl<U: Unit> serde::Serialize for Interval<U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("Interval", 2)?;
        s.serialize_field("start", &self.start.value())?;
        s.serialize_field("end", &self.end.value())?;
        s.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, U: Unit> serde::Deserialize<'de> for Interval<U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Raw {
            start: f64,
            end: f64,
        }

        let raw = Raw::deserialize(deserializer)?;
        Ok(Self::new(
            Quantity::<U>::new(raw.start),
            Quantity::<U>::new(raw.end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qtty::{Day, Second};

    #[test]
    fn test_interval_creation() {
        let interval = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(100.0));
        assert_eq!(interval.duration().value(), 100.0);
        assert_eq!(interval.start().value(), 0.0);
        assert_eq!(interval.end().value(), 100.0);
    }

    #[test]
    fn test_interval_to_conversion() {
        let interval_sec = Interval::new(
            Quantity::<Second>::new(0.0),
            Quantity::<Second>::new(86400.0),
        );
        let interval_day: Interval<Day> = interval_sec.to();
        assert!((interval_day.start().value() - 0.0).abs() < 1e-12);
        assert!((interval_day.end().value() - 1.0).abs() < 1e-12);
        assert!((interval_day.duration().value() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_interval_contains() {
        let interval = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(100.0));
        assert!(interval.contains(Quantity::<Second>::new(50.0)));
        assert!(interval.contains(Quantity::<Second>::new(0.0)));
        // end is exclusive — the boundary point itself is NOT contained
        assert!(!interval.contains(Quantity::<Second>::new(100.0)));
        assert!(!interval.contains(Quantity::<Second>::new(150.0)));
    }

    #[test]
    fn test_interval_can_fit() {
        let interval = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(100.0));
        assert!(interval.can_fit(Quantity::<Second>::new(0.0), Quantity::<Second>::new(50.0)));
        assert!(interval.can_fit(Quantity::<Second>::new(50.0), Quantity::<Second>::new(50.0)));
        assert!(!interval.can_fit(Quantity::<Second>::new(60.0), Quantity::<Second>::new(50.0)));
    }

    #[test]
    fn test_interval_overlaps() {
        let interval1 = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(100.0));
        let interval2 = Interval::new(
            Quantity::<Second>::new(50.0),
            Quantity::<Second>::new(150.0),
        );
        let interval3 = Interval::new(
            Quantity::<Second>::new(200.0),
            Quantity::<Second>::new(300.0),
        );

        assert!(interval1.overlaps(&interval2));
        assert!(interval2.overlaps(&interval1));
        assert!(!interval1.overlaps(&interval3));
    }

    // ── Gap coverage tests ────────────────────────────────────────────

    #[test]
    fn test_intersection_overlapping() {
        let a = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(100.0));
        let b = Interval::new(
            Quantity::<Second>::new(50.0),
            Quantity::<Second>::new(150.0),
        );
        let result = a.intersection(&b).unwrap();
        assert_eq!(result.start().value(), 50.0);
        assert_eq!(result.end().value(), 100.0);
    }

    #[test]
    fn test_intersection_non_overlapping() {
        let a = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(50.0));
        let b = Interval::new(
            Quantity::<Second>::new(60.0),
            Quantity::<Second>::new(100.0),
        );
        assert!(a.intersection(&b).is_none());
    }

    #[test]
    fn test_intersection_contained() {
        let a = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(100.0));
        let b = Interval::new(Quantity::<Second>::new(20.0), Quantity::<Second>::new(80.0));
        let result = a.intersection(&b).unwrap();
        assert_eq!(result.start().value(), 20.0);
        assert_eq!(result.end().value(), 80.0);
    }

    #[test]
    fn test_intersection_touching_endpoints() {
        // Half-open intervals that share only a boundary point do NOT overlap
        // and therefore have no intersection.
        let a = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(50.0));
        let b = Interval::new(
            Quantity::<Second>::new(50.0),
            Quantity::<Second>::new(100.0),
        );
        assert!(a.intersection(&b).is_none());
    }

    #[test]
    fn test_display_format() {
        let interval = Interval::new(Quantity::<Second>::new(1.5), Quantity::<Second>::new(99.25));
        let s = format!("{}", interval);
        assert!(s.contains("1.500"));
        assert!(s.contains("99.250"));
    }

    #[test]
    fn test_from_f64() {
        let interval = Interval::<Second>::from_f64(10.0, 20.0);
        assert_eq!(interval.start().value(), 10.0);
        assert_eq!(interval.end().value(), 20.0);
    }

    #[test]
    fn test_zero_width_interval() {
        let interval = Interval::new(Quantity::<Second>::new(5.0), Quantity::<Second>::new(5.0));
        assert_eq!(interval.duration().value(), 0.0);
        // A zero-width half-open interval [5, 5) is empty — contains nothing.
        assert!(!interval.contains(Quantity::<Second>::new(5.0)));
    }

    #[test]
    fn test_can_fit_at_boundary() {
        let interval = Interval::new(Quantity::<Second>::new(0.0), Quantity::<Second>::new(100.0));
        // Exactly fits from 0 to 100
        assert!(interval.can_fit(Quantity::<Second>::new(0.0), Quantity::<Second>::new(100.0)));
        // Does NOT fit starting at 1 with size 100
        assert!(!interval.can_fit(Quantity::<Second>::new(1.0), Quantity::<Second>::new(100.0)));
    }

    #[test]
    fn test_contains_boundary() {
        let interval = Interval::new(Quantity::<Second>::new(10.0), Quantity::<Second>::new(20.0));
        assert!(!interval.contains(Quantity::<Second>::new(9.999)));
        // start is inclusive
        assert!(interval.contains(Quantity::<Second>::new(10.0)));
        // end is exclusive
        assert!(!interval.contains(Quantity::<Second>::new(20.0)));
        assert!(!interval.contains(Quantity::<Second>::new(20.001)));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip() {
        let interval = Interval::<Second>::from_f64(10.0, 50.0);
        let json = serde_json::to_string(&interval).unwrap();
        let restored: Interval<Second> = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.start().value(), 10.0);
        assert_eq!(restored.end().value(), 50.0);
    }
}
