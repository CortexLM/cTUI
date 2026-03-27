//! Easing functions for smooth animations.
//!
//! This module provides standard easing functions commonly used in animations.
//! Each easing type comes in three variants:
//! - `In`: Ease in (start slow)
//! - `Out`: Ease out (end slow)
//! - `InOut`: Ease in-out (both ends slow)
//!
//! All functions expect input `t` in the range `[0.0, 1.0]` and return values
//! in the range `[0.0, 1.0]`. Input values are clamped to this range for safety.
//!
//! Reference: <https://easings.net/>

// Clippy pedantic/nursery allowances for easing math
#![allow(clippy::missing_const_for_fn, clippy::suboptimal_flops)]

use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// Easing function types for animations.
///
/// Each variant represents a different mathematical function that maps
/// a progress value `t` in `[0, 1]` to an eased progress value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EasingFunction {
    /// Linear interpolation (no easing).
    Linear,
    /// Quadratic easing (t²).
    QuadIn,
    /// Quadratic easing (inverse t²).
    QuadOut,
    /// Quadratic easing (combined).
    QuadInOut,
    /// Cubic easing (t³).
    CubicIn,
    /// Cubic easing (inverse t³).
    CubicOut,
    /// Cubic easing (combined).
    CubicInOut,
    /// Quartic easing (t⁴).
    QuartIn,
    /// Quartic easing (inverse t⁴).
    QuartOut,
    /// Quartic easing (combined).
    QuartInOut,
    /// Quintic easing (t⁵).
    QuintIn,
    /// Quintic easing (inverse t⁵).
    QuintOut,
    /// Quintic easing (combined).
    QuintInOut,
    /// Sine easing.
    SineIn,
    /// Sine easing.
    SineOut,
    /// Sine easing.
    SineInOut,
    /// Exponential easing.
    ExpoIn,
    /// Exponential easing.
    ExpoOut,
    /// Exponential easing.
    ExpoInOut,
    /// Circular easing.
    CircIn,
    /// Circular easing.
    CircOut,
    /// Circular easing.
    CircInOut,
    /// Back easing (overshoots).
    BackIn,
    /// Back easing (overshoots).
    BackOut,
    /// Back easing (overshoots).
    BackInOut,
    /// Elastic easing (bounces).
    ElasticIn,
    /// Elastic easing (bounces).
    ElasticOut,
    /// Elastic easing (bounces).
    ElasticInOut,
    /// Bounce easing.
    BounceIn,
    /// Bounce easing.
    BounceOut,
    /// Bounce easing.
    BounceInOut,
}

impl EasingFunction {
    /// Evaluates the easing function at progress `t`.
    ///
    /// # Arguments
    /// * `t` - Progress value, typically in `[0.0, 1.0]`. Values outside this range are clamped.
    ///
    /// # Returns
    /// The eased progress value in `[0.0, 1.0]`.
    ///
    /// # Examples
    /// ```
    /// use ctui_animate::easing::EasingFunction;
    ///
    /// assert_eq!(EasingFunction::Linear.eval(0.5), 0.5);
    /// assert!(EasingFunction::QuadIn.eval(0.5) < 0.5);
    /// assert!(EasingFunction::QuadOut.eval(0.5) > 0.5);
    /// ```
    #[must_use]
    pub fn eval(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => linear(t),
            Self::QuadIn => quad_in(t),
            Self::QuadOut => quad_out(t),
            Self::QuadInOut => quad_in_out(t),
            Self::CubicIn => cubic_in(t),
            Self::CubicOut => cubic_out(t),
            Self::CubicInOut => cubic_in_out(t),
            Self::QuartIn => quart_in(t),
            Self::QuartOut => quart_out(t),
            Self::QuartInOut => quart_in_out(t),
            Self::QuintIn => quint_in(t),
            Self::QuintOut => quint_out(t),
            Self::QuintInOut => quint_in_out(t),
            Self::SineIn => sine_in(t),
            Self::SineOut => sine_out(t),
            Self::SineInOut => sine_in_out(t),
            Self::ExpoIn => expo_in(t),
            Self::ExpoOut => expo_out(t),
            Self::ExpoInOut => expo_in_out(t),
            Self::CircIn => circ_in(t),
            Self::CircOut => circ_out(t),
            Self::CircInOut => circ_in_out(t),
            Self::BackIn => back_in(t),
            Self::BackOut => back_out(t),
            Self::BackInOut => back_in_out(t),
            Self::ElasticIn => elastic_in(t),
            Self::ElasticOut => elastic_out(t),
            Self::ElasticInOut => elastic_in_out(t),
            Self::BounceIn => bounce_in(t),
            Self::BounceOut => bounce_out(t),
            Self::BounceInOut => bounce_in_out(t),
        }
    }
}

// ============================================================================
// Linear
// ============================================================================

/// Linear easing (no easing, just linear interpolation).
#[inline]
fn linear(t: f64) -> f64 {
    t
}

// ============================================================================
// Quad (Quadratic)
// ============================================================================

/// Quadratic ease-in: f(t) = t²
#[inline]
fn quad_in(t: f64) -> f64 {
    t * t
}

/// Quadratic ease-out: f(t) = 1 - (1-t)²
#[inline]
fn quad_out(t: f64) -> f64 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Quadratic ease-in-out.
#[inline]
fn quad_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

// ============================================================================
// Cubic
// ============================================================================

/// Cubic ease-in: f(t) = t³
#[inline]
fn cubic_in(t: f64) -> f64 {
    t.powi(3)
}

/// Cubic ease-out: f(t) = 1 - (1-t)³
#[inline]
fn cubic_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// Cubic ease-in-out.
#[inline]
fn cubic_in_out(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t.powi(3)
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

// ============================================================================
// Quart (Quartic)
// ============================================================================

/// Quartic ease-in: f(t) = t⁴
#[inline]
fn quart_in(t: f64) -> f64 {
    t.powi(4)
}

/// Quartic ease-out: f(t) = 1 - (1-t)⁴
#[inline]
fn quart_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(4)
}

/// Quartic ease-in-out.
#[inline]
fn quart_in_out(t: f64) -> f64 {
    if t < 0.5 {
        8.0 * t.powi(4)
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

// ============================================================================
// Quint (Quintic)
// ============================================================================

/// Quintic ease-in: f(t) = t⁵
#[inline]
fn quint_in(t: f64) -> f64 {
    t.powi(5)
}

/// Quintic ease-out: f(t) = 1 - (1-t)⁵
#[inline]
fn quint_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(5)
}

/// Quintic ease-in-out.
#[inline]
fn quint_in_out(t: f64) -> f64 {
    if t < 0.5 {
        16.0 * t.powi(5)
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
}

// ============================================================================
// Sine
// ============================================================================

/// Sinusoidal ease-in.
#[inline]
fn sine_in(t: f64) -> f64 {
    1.0 - (t * FRAC_PI_2).cos()
}

/// Sinusoidal ease-out.
#[inline]
fn sine_out(t: f64) -> f64 {
    (t * FRAC_PI_2).sin()
}

/// Sinusoidal ease-in-out.
#[inline]
fn sine_in_out(t: f64) -> f64 {
    -(PI * t).cos() / 2.0 + 0.5
}

// ============================================================================
// Expo (Exponential)
// ============================================================================

/// Exponential ease-in.
#[inline]
fn expo_in(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else {
        2.0_f64.powf(10.0 * (t - 1.0))
    }
}

/// Exponential ease-out.
#[inline]
fn expo_out(t: f64) -> f64 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f64.powf(-10.0 * t)
    }
}

/// Exponential ease-in-out.
#[inline]
fn expo_in_out(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        2.0_f64.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
    }
}

// ============================================================================
// Circ (Circular)
// ============================================================================

/// Circular ease-in.
#[inline]
fn circ_in(t: f64) -> f64 {
    1.0 - (1.0 - t * t).sqrt()
}

/// Circular ease-out.
#[inline]
fn circ_out(t: f64) -> f64 {
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

/// Circular ease-in-out.
#[inline]
fn circ_in_out(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        f64::midpoint((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt(), 1.0)
    }
}

// ============================================================================
// Back
// ============================================================================

/// Back easing constant.
const C1: f64 = 1.70158;
/// Back easing constant for in-out.
const C2: f64 = C1 * 1.525;
/// Back easing constant for in-out.
const C3: f64 = C1 + 1.0;

/// Back ease-in (overshoots at start).
#[inline]
fn back_in(t: f64) -> f64 {
    C3 * t.powi(3) - C1 * t.powi(2)
}

/// Back ease-out (overshoots at end).
#[inline]
fn back_out(t: f64) -> f64 {
    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

/// Back ease-in-out (overshoots at both ends).
#[inline]
fn back_in_out(t: f64) -> f64 {
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
    } else {
        f64::midpoint((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2), 2.0)
    }
}

// ============================================================================
// Elastic
// ============================================================================

/// Elastic ease-in.
#[inline]
fn elastic_in(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -(2.0_f64.powf(10.0 * (t - 1.0))) * ((t - 1.0 - 0.075) * TAU / 0.3).sin()
    }
}

/// Elastic ease-out.
#[inline]
fn elastic_out(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0_f64.powf(-10.0 * t) * ((t - 0.075) * TAU / 0.3).cos() + 1.0
    }
}

/// Elastic ease-in-out.
#[inline]
fn elastic_in_out(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        -(2.0_f64.powf(20.0 * t - 10.0)) * ((20.0 * t - 11.125) * TAU / 4.5).sin() / 2.0
    } else {
        2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * TAU / 4.5).cos() / 2.0 + 1.0
    }
}

// ============================================================================
// Bounce
// ============================================================================

/// Bounce constant.
const N1: f64 = 7.5625;
/// Bounce constant.
const D1: f64 = 2.75;

/// Bounce ease-in.
#[inline]
fn bounce_in(t: f64) -> f64 {
    1.0 - bounce_out(1.0 - t)
}

/// Bounce ease-out.
#[inline]
fn bounce_out(t: f64) -> f64 {
    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984_375
    }
}

/// Bounce ease-in-out.
#[inline]
fn bounce_in_out(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
    } else {
        f64::midpoint(1.0, bounce_out(2.0 * t - 1.0))
    }
}

#[cfg(test)]
#[allow(
    clippy::float_cmp,
    clippy::unreadable_literal,
    clippy::uninlined_format_args
)]
mod tests {
    use super::*;

    /// Test that all easing functions return 0.0 at t=0.0
    #[test]
    fn test_eval_zero() {
        let easings = [
            EasingFunction::Linear,
            EasingFunction::QuadIn,
            EasingFunction::QuadOut,
            EasingFunction::QuadInOut,
            EasingFunction::CubicIn,
            EasingFunction::CubicOut,
            EasingFunction::CubicInOut,
            EasingFunction::QuartIn,
            EasingFunction::QuartOut,
            EasingFunction::QuartInOut,
            EasingFunction::QuintIn,
            EasingFunction::QuintOut,
            EasingFunction::QuintInOut,
            EasingFunction::SineIn,
            EasingFunction::SineOut,
            EasingFunction::SineInOut,
            EasingFunction::ExpoIn,
            EasingFunction::ExpoOut,
            EasingFunction::ExpoInOut,
            EasingFunction::CircIn,
            EasingFunction::CircOut,
            EasingFunction::CircInOut,
            EasingFunction::BackIn,
            EasingFunction::BackOut,
            EasingFunction::BackInOut,
            EasingFunction::ElasticIn,
            EasingFunction::ElasticOut,
            EasingFunction::ElasticInOut,
            EasingFunction::BounceIn,
            EasingFunction::BounceOut,
            EasingFunction::BounceInOut,
        ];

        for easing in easings {
            let result = easing.eval(0.0);
            // Allow small floating-point tolerance for BackOut which has arithmetic at boundary
            assert!(
                result.abs() < 1e-10,
                "Easing {:?} should return 0.0 at t=0.0, got {}",
                easing,
                result
            );
        }
    }

    /// Test that all easing functions return 1.0 at t=1.0
    #[test]
    fn test_eval_one() {
        let easings = [
            EasingFunction::Linear,
            EasingFunction::QuadIn,
            EasingFunction::QuadOut,
            EasingFunction::QuadInOut,
            EasingFunction::CubicIn,
            EasingFunction::CubicOut,
            EasingFunction::CubicInOut,
            EasingFunction::QuartIn,
            EasingFunction::QuartOut,
            EasingFunction::QuartInOut,
            EasingFunction::QuintIn,
            EasingFunction::QuintOut,
            EasingFunction::QuintInOut,
            EasingFunction::SineIn,
            EasingFunction::SineOut,
            EasingFunction::SineInOut,
            EasingFunction::ExpoIn,
            EasingFunction::ExpoOut,
            EasingFunction::ExpoInOut,
            EasingFunction::CircIn,
            EasingFunction::CircOut,
            EasingFunction::CircInOut,
            EasingFunction::BackIn,
            EasingFunction::BackOut,
            EasingFunction::BackInOut,
            EasingFunction::ElasticIn,
            EasingFunction::ElasticOut,
            EasingFunction::ElasticInOut,
            EasingFunction::BounceIn,
            EasingFunction::BounceOut,
            EasingFunction::BounceInOut,
        ];

        for easing in easings {
            let result = easing.eval(1.0);
            assert!(
                (result - 1.0).abs() < 1e-10,
                "Easing {:?} should return 1.0 at t=1.0, got {}",
                easing,
                result
            );
        }
    }

    /// Test that all easing functions return valid values at t=0.5
    #[test]
    fn test_eval_half() {
        let easings = [
            EasingFunction::Linear,
            EasingFunction::QuadIn,
            EasingFunction::QuadOut,
            EasingFunction::QuadInOut,
            EasingFunction::CubicIn,
            EasingFunction::CubicOut,
            EasingFunction::CubicInOut,
            EasingFunction::QuartIn,
            EasingFunction::QuartOut,
            EasingFunction::QuartInOut,
            EasingFunction::QuintIn,
            EasingFunction::QuintOut,
            EasingFunction::QuintInOut,
            EasingFunction::SineIn,
            EasingFunction::SineOut,
            EasingFunction::SineInOut,
            EasingFunction::ExpoIn,
            EasingFunction::ExpoOut,
            EasingFunction::ExpoInOut,
            EasingFunction::CircIn,
            EasingFunction::CircOut,
            EasingFunction::CircInOut,
            EasingFunction::BackIn,
            EasingFunction::BackOut,
            EasingFunction::BackInOut,
            EasingFunction::ElasticIn,
            EasingFunction::ElasticOut,
            EasingFunction::ElasticInOut,
            EasingFunction::BounceIn,
            EasingFunction::BounceOut,
            EasingFunction::BounceInOut,
        ];

        for easing in easings {
            let result = easing.eval(0.5);
            assert!(
                !result.is_nan(),
                "Easing {:?} should not return NaN at t=0.5",
                easing
            );
            assert!(
                !result.is_infinite(),
                "Easing {:?} should not return infinite at t=0.5",
                easing
            );
        }
    }

    /// Test linear easing (pass-through)
    #[test]
    fn test_linear() {
        assert_eq!(EasingFunction::Linear.eval(0.0), 0.0);
        assert_eq!(EasingFunction::Linear.eval(0.25), 0.25);
        assert_eq!(EasingFunction::Linear.eval(0.5), 0.5);
        assert_eq!(EasingFunction::Linear.eval(0.75), 0.75);
        assert_eq!(EasingFunction::Linear.eval(1.0), 1.0);
    }

    /// Test that values are clamped
    #[test]
    fn test_clamping() {
        // Negative values should clamp to 0
        assert_eq!(EasingFunction::Linear.eval(-1.0), 0.0);
        // Values > 1 should clamp to 1
        assert_eq!(EasingFunction::Linear.eval(2.0), 1.0);
    }

    // Reference value tests from easings.net

    #[test]
    fn test_quad_in_reference() {
        // Reference: https://easings.net/#easeInQuad
        approx::assert_relative_eq!(EasingFunction::QuadIn.eval(0.25), 0.0625, epsilon = 1e-10);
        approx::assert_relative_eq!(EasingFunction::QuadIn.eval(0.5), 0.25, epsilon = 1e-10);
        approx::assert_relative_eq!(EasingFunction::QuadIn.eval(0.75), 0.5625, epsilon = 1e-10);
    }

    #[test]
    fn test_quad_out_reference() {
        // Reference: https://easings.net/#easeOutQuad
        approx::assert_relative_eq!(EasingFunction::QuadOut.eval(0.25), 0.4375, epsilon = 1e-10);
        approx::assert_relative_eq!(EasingFunction::QuadOut.eval(0.5), 0.75, epsilon = 1e-10);
        approx::assert_relative_eq!(EasingFunction::QuadOut.eval(0.75), 0.9375, epsilon = 1e-10);
    }

    #[test]
    fn test_cubic_in_reference() {
        // Reference: https://easings.net/#easeInCubic
        approx::assert_relative_eq!(
            EasingFunction::CubicIn.eval(0.25),
            0.015625,
            epsilon = 1e-10
        );
        approx::assert_relative_eq!(EasingFunction::CubicIn.eval(0.5), 0.125, epsilon = 1e-10);
        approx::assert_relative_eq!(
            EasingFunction::CubicIn.eval(0.75),
            0.421875,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_cubic_out_reference() {
        // Reference: https://easings.net/#easeOutCubic
        approx::assert_relative_eq!(
            EasingFunction::CubicOut.eval(0.25),
            0.578125,
            epsilon = 1e-10
        );
        approx::assert_relative_eq!(EasingFunction::CubicOut.eval(0.5), 0.875, epsilon = 1e-10);
        approx::assert_relative_eq!(
            EasingFunction::CubicOut.eval(0.75),
            0.984375,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_sine_in_reference() {
        // sin(t * PI/2) complement
        approx::assert_relative_eq!(
            EasingFunction::SineIn.eval(0.5),
            0.2928932188134524,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_sine_out_reference() {
        approx::assert_relative_eq!(
            EasingFunction::SineOut.eval(0.5),
            0.7071067811865475,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_bounce_out_reference() {
        // Bounce is well-defined at key points
        approx::assert_relative_eq!(EasingFunction::BounceOut.eval(0.0), 0.0, epsilon = 1e-10);
        approx::assert_relative_eq!(EasingFunction::BounceOut.eval(1.0), 1.0, epsilon = 1e-10);
    }
}
