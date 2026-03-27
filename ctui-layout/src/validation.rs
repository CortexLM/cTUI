use ctui_core::Rect;

use crate::{Constraint, FlexLayout};

/// Result of layout validation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidationResult {
    /// Whether the layout is valid
    pub is_valid: bool,
    /// Total size required by children
    pub required_size: u32,
    /// Available size in the container
    pub available_size: u32,
    /// Whether constraints would overflow
    pub overflows: bool,
}

impl ValidationResult {
    /// Creates a valid result
    #[must_use]
    pub const fn valid() -> Self {
        Self {
            is_valid: true,
            required_size: 0,
            available_size: 0,
            overflows: false,
        }
    }

    /// Creates an invalid result with details
    #[must_use]
    pub const fn invalid(required: u32, available: u32) -> Self {
        Self {
            is_valid: false,
            required_size: required,
            available_size: available,
            overflows: required > available,
        }
    }
}

/// Error type for layout validation failures
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LayoutValidationError {
    /// The error message
    pub message: &'static str,
    /// The index of the problematic child (if applicable)
    pub child_index: Option<usize>,
}

impl std::fmt::Display for LayoutValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(idx) = self.child_index {
            write!(f, "Child {}: {}", idx, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for LayoutValidationError {}

/// Trait for validating layout constraints
pub trait LayoutValidator {
    /// Validates that constraints fit within the given size
    fn validate_constraints(&self, constraints: &[Constraint], available: u32) -> ValidationResult;

    /// Validates that children fit within the parent container
    fn validate_children(&self, area: Rect, children_count: usize) -> ValidationResult;

    /// Checks if the layout would overflow
    fn check_overflow(
        &self,
        constraints: &[Constraint],
        available: u32,
    ) -> Option<LayoutValidationError>;

    /// Returns the minimum size needed for the given constraints
    fn min_required_size(&self, constraints: &[Constraint]) -> u32;

    /// Returns the maximum possible size for the given constraints
    fn max_possible_size(&self, constraints: &[Constraint]) -> u32;
}

impl LayoutValidator for FlexLayout {
    fn validate_constraints(&self, constraints: &[Constraint], available: u32) -> ValidationResult {
        let min_size = self.min_required_size(constraints);
        let max_size = self.max_possible_size(constraints);

        if min_size > available && max_size > available {
            ValidationResult::invalid(min_size, available)
        } else {
            ValidationResult {
                is_valid: true,
                required_size: min_size,
                available_size: available,
                overflows: min_size > available,
            }
        }
    }

    fn validate_children(&self, area: Rect, children_count: usize) -> ValidationResult {
        if children_count == 0 {
            return ValidationResult::valid();
        }

        let available = match self.direction {
            crate::FlexDirection::Row => area.width as u32,
            crate::FlexDirection::Column => area.height as u32,
        };

        let total_gap = self.gap as u32 * children_count.saturating_sub(1) as u32;
        let available_for_content = available.saturating_sub(total_gap);

        ValidationResult {
            is_valid: available_for_content > 0,
            required_size: total_gap,
            available_size: available,
            overflows: total_gap >= available,
        }
    }

    fn check_overflow(
        &self,
        constraints: &[Constraint],
        available: u32,
    ) -> Option<LayoutValidationError> {
        let min_size = self.min_required_size(constraints);
        if min_size > available {
            Some(LayoutValidationError {
                message: "Layout would overflow container",
                child_index: None,
            })
        } else {
            None
        }
    }

    fn min_required_size(&self, constraints: &[Constraint]) -> u32 {
        let total_gap = self.gap as u32 * constraints.len().saturating_sub(1) as u32;

        let min_content: u32 = constraints
            .iter()
            .map(|c| match c {
                Constraint::Length(n) => *n as u32,
                Constraint::Min(n) => *n as u32,
                Constraint::Max(_) => 0,
                Constraint::Percentage(_) => 0,
                Constraint::Ratio(_, _) => 0,
                Constraint::Fill => 0,
                Constraint::Range { min, .. } => *min as u32,
                Constraint::Portion(_) => 0,
            })
            .sum();

        min_content.saturating_add(total_gap)
    }

    fn max_possible_size(&self, constraints: &[Constraint]) -> u32 {
        let total_gap = self.gap as u32 * constraints.len().saturating_sub(1) as u32;

        let max_content: u32 = constraints
            .iter()
            .map(|c| match c {
                Constraint::Length(n) => *n as u32,
                Constraint::Min(_) => u16::MAX as u32,
                Constraint::Max(n) => *n as u32,
                Constraint::Percentage(p) => ((*p as u32).min(100) * u16::MAX as u32) / 100,
                Constraint::Ratio(_, _) => u16::MAX as u32,
                Constraint::Fill => u16::MAX as u32,
                Constraint::Range { max, .. } => *max as u32,
                Constraint::Portion(_) => u16::MAX as u32,
            })
            .sum();

        max_content.saturating_add(total_gap)
    }
}

/// Validates that a single rect fits within a container
#[must_use]
pub fn rect_fits_in_container(rect: Rect, container: Rect) -> bool {
    rect.x >= container.x
        && rect.y >= container.y
        && rect.x + rect.width <= container.x + container.width
        && rect.y + rect.height <= container.y + container.height
}

/// Validates that a vector of rects don't overlap
#[must_use]
pub fn no_overlapping_rects(rects: &[Rect]) -> bool {
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            if rects_overlap(rects[i], rects[j]) {
                return false;
            }
        }
    }
    true
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Layout;

    fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect::new(x, y, width, height)
    }

    #[test]
    fn validation_result_valid() {
        let result = ValidationResult::valid();
        assert!(result.is_valid);
        assert!(!result.overflows);
    }

    #[test]
    fn validation_result_invalid() {
        let result = ValidationResult::invalid(100, 50);
        assert!(!result.is_valid);
        assert!(result.overflows);
        assert_eq!(result.required_size, 100);
        assert_eq!(result.available_size, 50);
    }

    #[test]
    fn validate_empty_constraints() {
        let layout = Layout::row();
        let result = layout.validate_constraints(&[], 100);
        assert!(result.is_valid);
    }

    #[test]
    fn validate_fixed_constraints() {
        let layout = Layout::row();
        let constraints = vec![Constraint::Length(30), Constraint::Length(30)];
        let result = layout.validate_constraints(&constraints, 100);
        assert!(result.is_valid);
    }

    #[test]
    fn validate_overflow_constraints() {
        let layout = Layout::row();
        let constraints = vec![Constraint::Length(60), Constraint::Length(60)];
        let result = layout.validate_constraints(&constraints, 100);
        assert!(result.overflows);
    }

    #[test]
    fn min_required_size() {
        let layout = Layout::row().gap(2);
        let constraints = vec![Constraint::Length(10), Constraint::Length(20)];
        let min_size = layout.min_required_size(&constraints);
        assert_eq!(min_size, 32);
    }

    #[test]
    fn max_possible_size() {
        let layout = Layout::row();
        let constraints = vec![Constraint::Length(10), Constraint::Max(20)];
        let max_size = layout.max_possible_size(&constraints);
        assert_eq!(max_size, 10 + 20);
    }

    #[test]
    fn check_overflow_none() {
        let layout = Layout::row();
        let constraints = vec![Constraint::Length(30)];
        let error = layout.check_overflow(&constraints, 100);
        assert!(error.is_none());
    }

    #[test]
    fn check_overflow_some() {
        let layout = Layout::row();
        let constraints = vec![Constraint::Length(200)];
        let error = layout.check_overflow(&constraints, 100);
        assert!(error.is_some());
    }

    #[test]
    fn validate_children() {
        let layout = Layout::row().gap(2);
        let area = rect(0, 0, 100, 24);
        let result = layout.validate_children(area, 5);
        assert!(result.is_valid);

        let small_area = rect(0, 0, 5, 24);
        let result = layout.validate_children(small_area, 10);
        assert!(result.overflows);
    }

    #[test]
    fn rect_fits_in_container_true() {
        let container = rect(0, 0, 100, 100);
        let inner = rect(10, 10, 50, 50);
        assert!(rect_fits_in_container(inner, container));
    }

    #[test]
    fn rect_fits_in_container_false() {
        let container = rect(0, 0, 100, 100);
        let overflow = rect(80, 80, 30, 30);
        assert!(!rect_fits_in_container(overflow, container));
    }

    #[test]
    fn no_overlapping_rects_true() {
        let rects = vec![rect(0, 0, 10, 10), rect(15, 0, 10, 10), rect(0, 15, 10, 10)];
        assert!(no_overlapping_rects(&rects));
    }

    #[test]
    fn no_overlapping_rects_false() {
        let rects = vec![rect(0, 0, 15, 15), rect(10, 10, 15, 15)];
        assert!(!no_overlapping_rects(&rects));
    }

    #[test]
    fn layout_validation_error_display() {
        let error = LayoutValidationError {
            message: "test error",
            child_index: Some(5),
        };
        assert_eq!(format!("{}", error), "Child 5: test error");

        let error_no_child = LayoutValidationError {
            message: "test error",
            child_index: None,
        };
        assert_eq!(format!("{}", error_no_child), "test error");
    }

    #[test]
    fn range_constraint_validation() {
        let layout = Layout::row();
        let constraints = vec![Constraint::Range { min: 10, max: 30 }];
        let min_size = layout.min_required_size(&constraints);
        let max_size = layout.max_possible_size(&constraints);
        assert_eq!(min_size, 10);
        assert_eq!(max_size, 30);
    }

    #[test]
    fn portion_constraint_validation() {
        let layout = Layout::row();
        let constraints = vec![Constraint::Portion(2)];
        let min_size = layout.min_required_size(&constraints);
        assert_eq!(min_size, 0);
    }
}
