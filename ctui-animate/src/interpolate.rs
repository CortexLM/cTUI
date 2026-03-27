//! Interpolation primitives for animations.
//!
//! This module provides the `Interpolator` trait and implementations for common
//! types used in terminal animations. Interpolation is the foundation of smooth
//! transitions, allowing values to blend seamlessly from one state to another.
//!
//! # Linear Interpolation (Lerp)
//!
//! The core operation is linear interpolation (lerp), which computes an
//! intermediate value between two endpoints based on a progress parameter `t`:
//!
//! ```text
//! result = start + (end - start) * t
//! ```
//!
//! When `t = 0.0`, the result is `start`. When `t = 1.0`, the result is `end`.
//! Values of `t` outside `[0.0, 1.0]` are clamped to this range.
//!
//! # Examples
//!
//! ```
//! use ctui_animate::interpolate::{Interpolator, lerp};
//!
//! // Using the lerp free function for numbers
//! assert_eq!(lerp(0.0_f32, 100.0, 0.5), 50.0);
//!
//! // Using the Interpolator trait for types
//! use ctui_core::geometry::Position;
//!
//! let start = Position::new(0, 0);
//! let end = Position::new(100, 200);
//! let mid = start.interpolate(&end, 0.5);
//! assert_eq!(mid.x, 50);
//! assert_eq!(mid.y, 100);
//! ```

use ctui_core::geometry::{Position, Size};
use ctui_core::style::Color;

/// Trait for types that can be interpolated between two values.
///
/// Implementations provide smooth transitions between start and end values
/// based on a progress parameter `t` in the range `[0.0, 1.0]`.
///
/// # Type Parameters
///
/// Types implementing this trait should be `Copy` for efficiency in animation loops.
///
/// # Examples
///
/// ```
/// use ctui_animate::interpolate::Interpolator;
///
/// let a = 0.0_f32;
/// let b = 100.0_f32;
///
/// // At t=0, result equals start
/// assert_eq!(a.interpolate(&b, 0.0), 0.0);
///
/// // At t=1, result equals end
/// assert_eq!(a.interpolate(&b, 1.0), 100.0);
///
/// // At t=0.5, result is halfway
/// assert_eq!(a.interpolate(&b, 0.5), 50.0);
/// ```
pub trait Interpolator: Copy + Sized {
    /// Interpolate between this value and another.
    ///
    /// # Arguments
    ///
    /// * `other` - The target value to interpolate towards
    /// * `t` - Progress parameter in range `[0.0, 1.0]` (values are clamped)
    ///
    /// # Returns
    ///
    /// The interpolated value. When `t = 0.0`, returns `self`.
    /// When `t = 1.0`, returns `other`.
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

/// Performs linear interpolation between two values.
///
/// This is the core interpolation function used by all `Interpolator` implementations
/// for numeric types.
///
/// # Arguments
///
/// * `a` - Start value
/// * `b` - End value
/// * `t` - Progress parameter (clamped to `[0.0, 1.0]`)
///
/// # Returns
///
/// The interpolated value: `a + (b - a) * t`
///
/// # Examples
///
/// ```
/// use ctui_animate::interpolate::lerp;
///
/// // Basic interpolation
/// assert_eq!(lerp(0.0_f32, 10.0, 0.0), 0.0);
/// assert_eq!(lerp(0.0_f32, 10.0, 0.5), 5.0);
/// assert_eq!(lerp(0.0_f32, 10.0, 1.0), 10.0);
///
/// // Values outside [0, 1] are clamped
/// assert_eq!(lerp(0.0_f32, 10.0, -0.5), 0.0);
/// assert_eq!(lerp(0.0_f32, 10.0, 1.5), 10.0);
/// ```
#[inline]
#[must_use]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    (b - a).mul_add(t, a)
}

// ============================================================================
// Numeric Implementations
// ============================================================================

impl Interpolator for f32 {
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        lerp(*self, *other, t)
    }
}

impl Interpolator for f64 {
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        let t = Self::from(t.clamp(0.0, 1.0));
        self + (other - self) * t
    }
}

impl Interpolator for u16 {
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let a = f32::from(*self);
        let b = f32::from(*other);
        (b - a).mul_add(t, a).round() as Self
    }
}

impl Interpolator for i32 {
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let a = *self as f32;
        let b = *other as f32;
        (b - a).mul_add(t, a).round() as Self
    }
}

// ============================================================================
// Color Implementation
// ============================================================================

/// Converts a Color to its RGB components.
///
/// Named colors are mapped to their approximate RGB values.
/// `Reset` and `Indexed` colors return black (0, 0, 0).
#[inline]
const fn color_to_rgb(color: &Color) -> (u8, u8, u8) {
    match color {
        Color::Reset => (0, 0, 0), // Default to black for terminal default
        Color::Black => (0, 0, 0),
        Color::Red => (205, 0, 0),
        Color::Green => (0, 205, 0),
        Color::Yellow => (205, 205, 0),
        Color::Blue => (0, 0, 238),
        Color::Magenta => (205, 0, 205),
        Color::Cyan => (0, 205, 205),
        Color::White => (229, 229, 229),
        Color::DarkGray => (127, 127, 127),
        Color::LightRed => (255, 0, 0),
        Color::LightGreen => (0, 255, 0),
        Color::LightYellow => (255, 255, 0),
        Color::LightBlue => (92, 92, 255),
        Color::LightMagenta => (255, 0, 255),
        Color::LightCyan => (0, 255, 255),
        Color::Gray => (229, 229, 229),
        Color::Indexed(_) => (0, 0, 0), // Default to black for indexed colors
        Color::Rgb(r, g, b) => (*r, *g, *b),
    }
}

impl Interpolator for Color {
    /// Interpolate between two colors using RGB component-wise linear interpolation.
    ///
    /// Named colors are converted to their RGB equivalents before interpolation.
    /// The result is always an `Rgb` color.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctui_animate::interpolate::Interpolator;
    /// use ctui_core::style::Color;
    ///
    /// let red = Color::Red;
    /// let blue = Color::Blue;
    ///
    /// // Interpolate halfway between red and blue
    /// let purple = red.interpolate(&blue, 0.5);
    /// assert!(matches!(purple, Color::Rgb(_, _, _)));
    /// ```
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);

        let (r1, g1, b1) = color_to_rgb(self);
        let (r2, g2, b2) = color_to_rgb(other);

        let r = lerp(f32::from(r1), f32::from(r2), t).round() as u8;
        let g = lerp(f32::from(g1), f32::from(g2), t).round() as u8;
        let b = lerp(f32::from(b1), f32::from(b2), t).round() as u8;

        Self::Rgb(r, g, b)
    }
}

// ============================================================================
// Geometry Implementations
// ============================================================================

impl Interpolator for Position {
    /// Interpolate between two positions.
    ///
    /// Both x and y coordinates are interpolated independently.
    /// The result is rounded to the nearest integer coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctui_animate::interpolate::Interpolator;
    /// use ctui_core::geometry::Position;
    ///
    /// let start = Position::new(0, 100);
    /// let end = Position::new(100, 0);
    ///
    /// // Quarter way through
    /// let quarter = start.interpolate(&end, 0.25);
    /// assert_eq!(quarter.x, 25);
    /// assert_eq!(quarter.y, 75);
    /// ```
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x.interpolate(&other.x, t),
            y: self.y.interpolate(&other.y, t),
        }
    }
}

impl Interpolator for Size {
    /// Interpolate between two sizes.
    ///
    /// Both width and height are interpolated independently.
    /// The result is rounded to the nearest integer values.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctui_animate::interpolate::Interpolator;
    /// use ctui_core::geometry::Size;
    ///
    /// let small = Size::new(10, 20);
    /// let large = Size::new(100, 200);
    ///
    /// // Halfway through
    /// let mid = small.interpolate(&large, 0.5);
    /// assert_eq!(mid.width, 55);
    /// assert_eq!(mid.height, 110);
    /// ```
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self {
            width: self.width.interpolate(&other.width, t),
            height: self.height.interpolate(&other.height, t),
        }
    }
}

// ============================================================================
// Multi-value Interpolation (Arrays)
// ============================================================================

impl Interpolator for [f32; 2] {
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        [
            self[0].interpolate(&other[0], t),
            self[1].interpolate(&other[1], t),
        ]
    }
}

impl Interpolator for [f32; 3] {
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        [
            self[0].interpolate(&other[0], t),
            self[1].interpolate(&other[1], t),
            self[2].interpolate(&other[2], t),
        ]
    }
}

impl Interpolator for [f32; 4] {
    #[inline]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        [
            self[0].interpolate(&other[0], t),
            self[1].interpolate(&other[1], t),
            self[2].interpolate(&other[2], t),
            self[3].interpolate(&other[3], t),
        ]
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // lerp function tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_lerp_basic() {
        assert_eq!(lerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 100.0, 1.0), 100.0);
        assert_eq!(lerp(0.0, 100.0, 0.5), 50.0);
    }

    #[test]
    fn test_lerp_negative_values() {
        assert_eq!(lerp(-100.0, 100.0, 0.0), -100.0);
        assert_eq!(lerp(-100.0, 100.0, 1.0), 100.0);
        assert_eq!(lerp(-100.0, 100.0, 0.5), 0.0);
    }

    #[test]
    fn test_lerp_reverse() {
        // Interpolating from higher to lower value
        assert_eq!(lerp(100.0, 0.0, 0.0), 100.0);
        assert_eq!(lerp(100.0, 0.0, 1.0), 0.0);
        assert_eq!(lerp(100.0, 0.0, 0.5), 50.0);
    }

    #[test]
    fn test_lerp_clamp_negative_t() {
        // t < 0 should clamp to 0
        assert_eq!(lerp(0.0, 100.0, -0.5), 0.0);
        assert_eq!(lerp(0.0, 100.0, -1.0), 0.0);
        assert_eq!(lerp(0.0, 100.0, -100.0), 0.0);
    }

    #[test]
    fn test_lerp_clamp_gt_one() {
        // t > 1 should clamp to 1
        assert_eq!(lerp(0.0, 100.0, 1.5), 100.0);
        assert_eq!(lerp(0.0, 100.0, 2.0), 100.0);
        assert_eq!(lerp(0.0, 100.0, 100.0), 100.0);
    }

    // -------------------------------------------------------------------------
    // f32 Interpolator tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_f32_interpolate() {
        let a = 0.0_f32;
        let b = 100.0_f32;

        assert_eq!(a.interpolate(&b, 0.0), 0.0);
        assert_eq!(a.interpolate(&b, 1.0), 100.0);
        approx::assert_relative_eq!(a.interpolate(&b, 0.5), 50.0);
    }

    // -------------------------------------------------------------------------
    // f64 Interpolator tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_f64_interpolate() {
        let a = 0.0_f64;
        let b = 100.0_f64;

        assert_eq!(a.interpolate(&b, 0.0), 0.0);
        assert_eq!(a.interpolate(&b, 1.0), 100.0);
        approx::assert_relative_eq!(a.interpolate(&b, 0.5), 50.0);
    }

    // -------------------------------------------------------------------------
    // u16 Interpolator tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_u16_interpolate() {
        let a: u16 = 0;
        let b: u16 = 100;

        assert_eq!(a.interpolate(&b, 0.0), 0);
        assert_eq!(a.interpolate(&b, 1.0), 100);
        assert_eq!(a.interpolate(&b, 0.5), 50);
    }

    #[test]
    fn test_u16_interpolate_rounding() {
        // Test rounding behavior
        let a: u16 = 0;
        let b: u16 = 99;

        // 0 + (99 - 0) * 0.5 = 49.5 -> rounds to 50
        assert_eq!(a.interpolate(&b, 0.5), 50);
    }

    // -------------------------------------------------------------------------
    // i32 Interpolator tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_i32_interpolate() {
        let a: i32 = -100;
        let b: i32 = 100;

        assert_eq!(a.interpolate(&b, 0.0), -100);
        assert_eq!(a.interpolate(&b, 1.0), 100);
        assert_eq!(a.interpolate(&b, 0.5), 0);
    }

    // -------------------------------------------------------------------------
    // Color Interpolator tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_color_interpolate_rgb() {
        let red = Color::Rgb(255, 0, 0);
        let blue = Color::Rgb(0, 0, 255);

        let purple = red.interpolate(&blue, 0.5);
        match purple {
            Color::Rgb(r, g, b) => {
                assert_eq!(r, 128); // (255 + 0) / 2 = 127.5 -> 128
                assert_eq!(g, 0);
                assert_eq!(b, 128); // (0 + 255) / 2 = 127.5 -> 128
            }
            _ => panic!("Expected Rgb color"),
        }
    }

    #[test]
    fn test_color_interpolate_named() {
        let red = Color::Red;
        let green = Color::Green;

        // Should convert to RGB and interpolate
        let mid = red.interpolate(&green, 0.5);
        assert!(matches!(mid, Color::Rgb(_, _, _)));
    }

    #[test]
    fn test_color_interpolate_boundaries() {
        let a = Color::Rgb(100, 100, 100);
        let b = Color::Rgb(200, 200, 200);

        // t = 0: returns a
        let result = a.interpolate(&b, 0.0);
        assert_eq!(result, Color::Rgb(100, 100, 100));

        // t = 1: returns b
        let result = a.interpolate(&b, 1.0);
        assert_eq!(result, Color::Rgb(200, 200, 200));
    }

    // -------------------------------------------------------------------------
    // Position Interpolator tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_position_interpolate() {
        let start = Position::new(0, 100);
        let end = Position::new(100, 0);

        // t = 0
        let result = start.interpolate(&end, 0.0);
        assert_eq!(result, Position::new(0, 100));

        // t = 1
        let result = start.interpolate(&end, 1.0);
        assert_eq!(result, Position::new(100, 0));

        // t = 0.5
        let result = start.interpolate(&end, 0.5);
        assert_eq!(result, Position::new(50, 50));
    }

    #[test]
    fn test_position_interpolate_clamped() {
        let start = Position::new(0, 0);
        let end = Position::new(100, 100);

        // t < 0
        let result = start.interpolate(&end, -1.0);
        assert_eq!(result, Position::new(0, 0));

        // t > 1
        let result = start.interpolate(&end, 2.0);
        assert_eq!(result, Position::new(100, 100));
    }

    // -------------------------------------------------------------------------
    // Size Interpolator tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_size_interpolate() {
        let small = Size::new(10, 20);
        let large = Size::new(100, 200);

        // t = 0
        let result = small.interpolate(&large, 0.0);
        assert_eq!(result, Size::new(10, 20));

        // t = 1
        let result = small.interpolate(&large, 1.0);
        assert_eq!(result, Size::new(100, 200));

        // t = 0.5
        let result = small.interpolate(&large, 0.5);
        // (10 + 100) / 2 = 55, (20 + 200) / 2 = 110
        assert_eq!(result, Size::new(55, 110));
    }

    // -------------------------------------------------------------------------
    // Array Interpolator tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_array_2_interpolate() {
        let a = [0.0_f32, 100.0];
        let b = [100.0_f32, 0.0];

        let result = a.interpolate(&b, 0.5);
        approx::assert_relative_eq!(result[0], 50.0);
        approx::assert_relative_eq!(result[1], 50.0);
    }

    #[test]
    fn test_array_3_interpolate() {
        let a = [0.0_f32, 100.0, 200.0];
        let b = [300.0_f32, 0.0, 100.0];

        let result = a.interpolate(&b, 0.5);
        approx::assert_relative_eq!(result[0], 150.0);
        approx::assert_relative_eq!(result[1], 50.0);
        approx::assert_relative_eq!(result[2], 150.0);
    }

    #[test]
    fn test_array_4_interpolate() {
        let a = [1.0_f32, 2.0, 3.0, 4.0];
        let b = [5.0_f32, 6.0, 7.0, 8.0];

        let result = a.interpolate(&b, 0.5);
        approx::assert_relative_eq!(result[0], 3.0);
        approx::assert_relative_eq!(result[1], 4.0);
        approx::assert_relative_eq!(result[2], 5.0);
        approx::assert_relative_eq!(result[3], 6.0);
    }

    // -------------------------------------------------------------------------
    // Edge Cases
    // -------------------------------------------------------------------------

    #[test]
    fn test_same_values() {
        // Interpolating between same values should return that value
        let a = 42.0_f32;
        assert_eq!(a.interpolate(&a, 0.0), 42.0);
        assert_eq!(a.interpolate(&a, 0.5), 42.0);
        assert_eq!(a.interpolate(&a, 1.0), 42.0);

        let pos = Position::new(50, 50);
        let result = pos.interpolate(&pos, 0.5);
        assert_eq!(result, Position::new(50, 50));
    }

    #[test]
    fn test_float_precision() {
        // Ensure float operations don't introduce significant errors
        let a = 0.0_f32;
        let b = 1_000_000.0_f32;

        for i in 0..=100 {
            let t = i as f32 / 100.0;
            let result = a.interpolate(&b, t);
            let expected = t * b;
            approx::assert_relative_eq!(result, expected, epsilon = 0.001);
        }
    }
}
