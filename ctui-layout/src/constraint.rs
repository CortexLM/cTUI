//! Size constraints for layout children
//!
//! Constraints define how child elements should be sized within a layout.
//! They support fixed sizes, flexible ranges, percentages, ratios, and portions.

use std::fmt;

/// Size constraints for layout children
///
/// Constraints control how space is allocated to layout children.
/// They range from exact sizing (`Length`) to flexible sizing (`Fill`).
///
/// # Example
///
/// ```
/// use ctui_layout::Constraint;
///
/// // Fixed size of 20 cells
/// let fixed = Constraint::Length(20);
/// assert_eq!(fixed.apply(100), 20);
///
/// // At least 10 cells, grows to fill
/// let min = Constraint::Min(10);
/// // With 100 cells available, Min(10) might grow
/// assert!(min.apply(100) >= 10);
///
/// // Range constraint: between 10 and 30 cells
/// let range = Constraint::Range { min: 10, max: 30 };
///
/// // Portion constraint: like CSS fr units
/// let portion = Constraint::Portion(2); // 2fr
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Constraint {
    /// Fixed size in cells
    ///
    /// The element will always have exactly this size,
    /// regardless of available space.
    Length(u16),

    /// Minimum size, grows to fill available space
    ///
    /// The element will be at least this size, but can grow
    /// larger if there's extra space to distribute.
    Min(u16),

    /// Maximum size, shrinks if needed
    ///
    /// The element will be at most this size, but can shrink
    /// if there's not enough space available.
    Max(u16),

    /// Size as a percentage of available space (0-100)
    ///
    /// The element will take this percentage of the parent's space.
    /// Values over 100 are clamped.
    Percentage(u16),

    /// Ratio of available space after fixed constraints
    ///
    /// Two `Ratio(1, 3)` constraints will each take 1/3 of remaining space.
    /// Useful for proportional layouts.
    Ratio(u32, u32),

    /// Fill all available space
    ///
    /// The element will expand to fill any remaining space after
    /// other constraints are satisfied.
    Fill,

    /// Size constrained to a range (min to max)
    ///
    /// The element will be at least `min` cells but no more than `max` cells.
    /// If there's extra space to distribute, it will grow up to `max`.
    Range {
        /// Minimum size in cells
        min: u16,
        /// Maximum size in cells
        max: u16,
    },

    /// Portion of available space (like CSS fr units)
    ///
    /// Portion(1) takes 1 fraction of available space.
    /// Portion(2) takes 2 fractions (twice as much as Portion(1)).
    /// Named "Portion" to be distinct from Ratio which is numerator/denominator.
    Portion(u32),
}

impl Constraint {
    /// Creates a fixed-length constraint
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_layout::Constraint;
    /// let c = Constraint::length(20);
    /// assert_eq!(c, Constraint::Length(20));
    /// ```
    #[must_use]
    pub const fn length(n: u16) -> Self {
        Self::Length(n)
    }

    /// Creates a minimum-size constraint
    #[must_use]
    pub const fn min(n: u16) -> Self {
        Self::Min(n)
    }

    /// Creates a maximum-size constraint
    #[must_use]
    pub const fn max(n: u16) -> Self {
        Self::Max(n)
    }

    /// Creates a percentage constraint
    ///
    /// Panics if percentage > 100
    #[must_use]
    pub const fn percentage(p: u16) -> Self {
        Self::Percentage(p)
    }

    /// Creates a ratio constraint
    #[must_use]
    pub const fn ratio(num: u32, den: u32) -> Self {
        Self::Ratio(num, den)
    }

    /// Creates a fill constraint
    #[must_use]
    pub const fn fill() -> Self {
        Self::Fill
    }

    /// Creates a range constraint
    #[must_use]
    pub const fn range(min: u16, max: u16) -> Self {
        Self::Range { min, max }
    }

    /// Creates a portion constraint (CSS fr unit equivalent)
    #[must_use]
    pub const fn portion(n: u32) -> Self {
        Self::Portion(n)
    }

    /// Returns the minimum size this constraint allows
    ///
    /// For `Length`, returns the exact size.
    /// For `Min`, returns the minimum value.
    /// For flexible constraints, returns 0.
    #[must_use]
    pub fn min_size(&self) -> u16 {
        match self {
            Self::Length(n) => *n,
            Self::Min(n) => *n,
            Self::Max(_) => 0,
            Self::Percentage(_) => 0,
            Self::Ratio(_, _) => 0,
            Self::Fill => 0,
            Self::Range { min, .. } => *min,
            Self::Portion(_) => 0,
        }
    }

    /// Returns the maximum size this constraint allows, if any
    ///
    /// Returns `None` for flexible constraints that can grow indefinitely.
    #[must_use]
    pub fn max_size(&self) -> Option<u16> {
        match self {
            Self::Length(n) => Some(*n),
            Self::Min(_) => None,
            Self::Max(n) => Some(*n),
            Self::Percentage(_) => None,
            Self::Ratio(_, _) => None,
            Self::Fill => None,
            Self::Range { max, .. } => Some(*max),
            Self::Portion(_) => None,
        }
    }

    /// Applies this constraint to compute an actual size
    ///
    /// # Arguments
    /// * `space` - The total available space
    ///
    /// # Returns
    /// The computed size for this constraint.
    ///
    /// # Note
    /// For flexible constraints (`Min`, `Fill`, `Ratio`, `Portion`), this returns
    /// the minimum value. The actual layout algorithm handles flex
    /// distribution in `Layout::split()`.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_layout::Constraint;
    ///
    /// // Length always returns its fixed size
    /// assert_eq!(Constraint::Length(20).apply(100), 20);
    /// assert_eq!(Constraint::Length(50).apply(10), 50); // even if larger than space
    ///
    /// // Percentage calculates proportion of space
    /// assert_eq!(Constraint::Percentage(50).apply(100), 50);
    /// assert_eq!(Constraint::Percentage(25).apply(80), 20);
    ///
    /// // Min returns minimum when there's extra space
    /// assert_eq!(Constraint::Min(10).apply(100), 10);
    ///
    /// // Max returns at most the maximum
    /// assert_eq!(Constraint::Max(20).apply(100), 20);
    ///
    /// // Range returns minimum within bounds
    /// assert_eq!(Constraint::Range { min: 10, max: 30 }.apply(100), 10);
    ///
    /// // Portion returns 0 (actual size from flex distribution)
    /// assert_eq!(Constraint::Portion(2).apply(100), 0);
    /// ```
    #[must_use]
    pub fn apply(&self, space: u16) -> u16 {
        match self {
            Self::Length(n) => *n,
            Self::Min(n) => *n,
            Self::Max(n) => (*n).min(space),
            Self::Percentage(p) => {
                let pct = (*p).min(100) as u64;
                ((space as u64 * pct / 100) as u16).max(1)
            }
            Self::Ratio(num, den) => {
                if *den == 0 {
                    return 0;
                }
                let result = (space as u64 * *num as u64 / *den as u64) as u16;
                result.max(1)
            }
            Self::Fill => 0,
            Self::Range { min, .. } => *min,
            Self::Portion(_) => 0,
        }
    }

    /// Returns true if this constraint is flexible (can grow)
    #[must_use]
    pub fn is_flexible(&self) -> bool {
        matches!(
            self,
            Self::Min(_)
                | Self::Max(_)
                | Self::Ratio(_, _)
                | Self::Fill
                | Self::Range { .. }
                | Self::Portion(_)
        )
    }

    /// Returns true if this constraint is fixed (has exact size)
    #[must_use]
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Length(_))
    }

    /// Returns the portion value for Portion constraints
    #[must_use]
    pub fn portion_value(&self) -> Option<u32> {
        match self {
            Self::Portion(n) => Some(*n),
            _ => None,
        }
    }
}

impl Default for Constraint {
    fn default() -> Self {
        Self::Fill
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length(n) => write!(f, "Length({})", n),
            Self::Min(n) => write!(f, "Min({})", n),
            Self::Max(n) => write!(f, "Max({})", n),
            Self::Percentage(p) => write!(f, "Percentage({}%)", p),
            Self::Ratio(num, den) => write!(f, "Ratio({}/{})", num, den),
            Self::Fill => write!(f, "Fill"),
            Self::Range { min, max } => write!(f, "Range({}..{})", min, max),
            Self::Portion(n) => write!(f, "Portion({}fr)", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_applies_exact_size() {
        assert_eq!(Constraint::Length(20).apply(100), 20);
        assert_eq!(Constraint::Length(50).apply(200), 50);
        assert_eq!(Constraint::Length(10).apply(5), 10);
    }

    #[test]
    fn min_applies_minimum_size() {
        assert_eq!(Constraint::Min(10).apply(100), 10);
        assert_eq!(Constraint::Min(5).apply(50), 5);
    }

    #[test]
    fn max_caps_at_maximum() {
        assert_eq!(Constraint::Max(20).apply(100), 20);
        assert_eq!(Constraint::Max(50).apply(30), 30);
    }

    #[test]
    fn percentage_calculates_proportion() {
        assert_eq!(Constraint::Percentage(50).apply(100), 50);
        assert_eq!(Constraint::Percentage(25).apply(80), 20);
        assert_eq!(Constraint::Percentage(33).apply(99), 32);
    }

    #[test]
    fn percentage_clamps_to_100() {
        assert_eq!(Constraint::Percentage(150).apply(100), 100);
    }

    #[test]
    fn ratio_calculates_proportion() {
        assert_eq!(Constraint::Ratio(1, 2).apply(100), 50);
        assert_eq!(Constraint::Ratio(1, 4).apply(100), 25);
        assert_eq!(Constraint::Ratio(2, 3).apply(99), 66);
    }

    #[test]
    fn ratio_handles_zero_denominator() {
        assert_eq!(Constraint::Ratio(1, 0).apply(100), 0);
    }

    #[test]
    fn fill_returns_zero() {
        assert_eq!(Constraint::Fill.apply(100), 0);
    }

    #[test]
    fn range_applies_minimum_size() {
        assert_eq!(Constraint::Range { min: 10, max: 30 }.apply(100), 10);
        assert_eq!(Constraint::Range { min: 5, max: 20 }.apply(50), 5);
    }

    #[test]
    fn range_min_size() {
        assert_eq!(Constraint::Range { min: 10, max: 30 }.min_size(), 10);
    }

    #[test]
    fn range_max_size() {
        assert_eq!(Constraint::Range { min: 10, max: 30 }.max_size(), Some(30));
    }

    #[test]
    fn range_is_flexible() {
        assert!(Constraint::Range { min: 10, max: 30 }.is_flexible());
    }

    #[test]
    fn portion_returns_zero_for_apply() {
        assert_eq!(Constraint::Portion(2).apply(100), 0);
    }

    #[test]
    fn portion_is_flexible() {
        assert!(Constraint::Portion(1).is_flexible());
        assert!(Constraint::Portion(2).is_flexible());
    }

    #[test]
    fn portion_value() {
        assert_eq!(Constraint::Portion(3).portion_value(), Some(3));
        assert_eq!(Constraint::Length(10).portion_value(), None);
    }

    #[test]
    fn range_constructor() {
        assert_eq!(
            Constraint::range(10, 30),
            Constraint::Range { min: 10, max: 30 }
        );
    }

    #[test]
    fn portion_constructor() {
        assert_eq!(Constraint::portion(2), Constraint::Portion(2));
    }

    #[test]
    fn min_size_returns_correct_values() {
        assert_eq!(Constraint::Length(20).min_size(), 20);
        assert_eq!(Constraint::Min(10).min_size(), 10);
        assert_eq!(Constraint::Max(30).min_size(), 0);
        assert_eq!(Constraint::Percentage(50).min_size(), 0);
        assert_eq!(Constraint::Ratio(1, 2).min_size(), 0);
        assert_eq!(Constraint::Fill.min_size(), 0);
        assert_eq!(Constraint::Range { min: 10, max: 30 }.min_size(), 10);
        assert_eq!(Constraint::Portion(2).min_size(), 0);
    }

    #[test]
    fn max_size_returns_correct_values() {
        assert_eq!(Constraint::Length(20).max_size(), Some(20));
        assert_eq!(Constraint::Min(10).max_size(), None);
        assert_eq!(Constraint::Max(30).max_size(), Some(30));
        assert_eq!(Constraint::Percentage(50).max_size(), None);
        assert_eq!(Constraint::Ratio(1, 2).max_size(), None);
        assert_eq!(Constraint::Fill.max_size(), None);
        assert_eq!(Constraint::Range { min: 10, max: 30 }.max_size(), Some(30));
        assert_eq!(Constraint::Portion(2).max_size(), None);
    }

    #[test]
    fn is_flexible_correct() {
        assert!(!Constraint::Length(10).is_flexible());
        assert!(Constraint::Min(10).is_flexible());
        assert!(Constraint::Max(10).is_flexible());
        assert!(!Constraint::Percentage(50).is_flexible());
        assert!(Constraint::Ratio(1, 2).is_flexible());
        assert!(Constraint::Fill.is_flexible());
        assert!(Constraint::Range { min: 5, max: 10 }.is_flexible());
        assert!(Constraint::Portion(2).is_flexible());
    }

    #[test]
    fn is_fixed_correct() {
        assert!(Constraint::Length(10).is_fixed());
        assert!(!Constraint::Min(10).is_fixed());
        assert!(!Constraint::Fill.is_fixed());
        assert!(!Constraint::Range { min: 5, max: 10 }.is_fixed());
        assert!(!Constraint::Portion(2).is_fixed());
    }

    #[test]
    fn default_is_fill() {
        assert_eq!(Constraint::default(), Constraint::Fill);
    }

    #[test]
    fn display_formats_nicely() {
        assert_eq!(Constraint::Length(20).to_string(), "Length(20)");
        assert_eq!(Constraint::Min(10).to_string(), "Min(10)");
        assert_eq!(Constraint::Max(30).to_string(), "Max(30)");
        assert_eq!(Constraint::Percentage(50).to_string(), "Percentage(50%)");
        assert_eq!(Constraint::Ratio(1, 3).to_string(), "Ratio(1/3)");
        assert_eq!(Constraint::Fill.to_string(), "Fill");
        assert_eq!(
            Constraint::Range { min: 5, max: 15 }.to_string(),
            "Range(5..15)"
        );
        assert_eq!(Constraint::Portion(2).to_string(), "Portion(2fr)");
    }

    #[test]
    fn constructor_helpers() {
        assert_eq!(Constraint::length(20), Constraint::Length(20));
        assert_eq!(Constraint::min(10), Constraint::Min(10));
        assert_eq!(Constraint::max(30), Constraint::Max(30));
        assert_eq!(Constraint::percentage(50), Constraint::Percentage(50));
        assert_eq!(Constraint::ratio(1, 3), Constraint::Ratio(1, 3));
        assert_eq!(Constraint::fill(), Constraint::Fill);
        assert_eq!(
            Constraint::range(5, 15),
            Constraint::Range { min: 5, max: 15 }
        );
        assert_eq!(Constraint::portion(2), Constraint::Portion(2));
    }

    #[test]
    fn constraint_equality() {
        assert_eq!(Constraint::Length(10), Constraint::Length(10));
        assert_ne!(Constraint::Length(10), Constraint::Length(20));
        assert_eq!(Constraint::Ratio(1, 2), Constraint::Ratio(1, 2));
        assert_eq!(
            Constraint::Range { min: 5, max: 10 },
            Constraint::Range { min: 5, max: 10 }
        );
        assert_eq!(Constraint::Portion(2), Constraint::Portion(2));
    }

    #[test]
    fn constraint_clone() {
        let c = Constraint::Min(15);
        let cloned = c.clone();
        assert_eq!(c, cloned);

        let r = Constraint::Range { min: 5, max: 20 };
        let r_cloned = r.clone();
        assert_eq!(r, r_cloned);
    }

    #[test]
    fn constraint_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Constraint::Length(10));
        set.insert(Constraint::Length(10));
        set.insert(Constraint::Length(20));
        set.insert(Constraint::Range { min: 5, max: 10 });
        set.insert(Constraint::Range { min: 5, max: 10 });
        set.insert(Constraint::Portion(2));
        set.insert(Constraint::Portion(2));
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(Constraint::Length(0).apply(100), 0);
        assert_eq!(Constraint::Min(0).apply(100), 0);
        assert_eq!(Constraint::Max(0).apply(100), 0);
        assert_eq!(Constraint::Percentage(0).apply(100), 1);

        assert_eq!(Constraint::Percentage(100).apply(1000), 1000);

        assert_eq!(Constraint::Range { min: 0, max: 10 }.apply(100), 0);
        assert_eq!(Constraint::Portion(0).apply(100), 0);
    }
}
