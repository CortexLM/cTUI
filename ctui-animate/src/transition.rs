//! Transition system for animating style and layout changes.
//!
//! This module provides types for defining and executing transitions on
//! style and layout properties. Transitions animate changes between two values
//! over a specified duration with configurable easing.
//!
//! # Example
//!
//! ```ignore
//! use ctui_animate::{Transition, TransitionProperty, TransitionBuilder, EasingFunction};
//! use ctui_core::{Color, Position, Size};
//!
//! // Create a color transition
//! let transition = TransitionBuilder::new()
//!     .color(Color::Red, Color::Blue)
//!     .duration_ms(300)
//!     .easing(EasingFunction::QuadOut)
//!     .build();
//!
//! // Interpolate at 50% progress
//! let value = transition.interpolate(0.5);
//! ```

use crate::easing::EasingFunction;
use ctui_core::geometry::{Position, Size};
use ctui_core::Color;

/// A property that can be transitioned.
///
/// Each variant stores both the start and end values for the transition.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionProperty {
    /// Transition between two colors.
    Color {
        /// Starting color value.
        from: Color,
        /// Target color value.
        to: Color,
    },
    /// Transition between two positions.
    Position {
        /// Starting position value.
        from: Position,
        /// Target position value.
        to: Position,
    },
    /// Transition between two sizes.
    Size {
        /// Starting size value.
        from: Size,
        /// Target size value.
        to: Size,
    },
    /// Transition between two opacity values (0.0 to 1.0).
    Opacity {
        /// Starting opacity value.
        from: f32,
        /// Target opacity value.
        to: f32,
    },
}

/// The result of interpolating a transition property.
///
/// This enum holds the interpolated value at a given progress point.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionValue {
    /// An interpolated color value.
    Color(Color),
    /// An interpolated position value.
    Position(Position),
    /// An interpolated size value.
    Size(Size),
    /// An interpolated opacity value.
    Opacity(f32),
}

impl TransitionProperty {
    /// Interpolates the property at the given progress value.
    ///
    /// # Arguments
    /// * `progress` - A value between 0.0 and 1.0 representing the transition progress.
    ///
    /// # Returns
    /// The interpolated value at the given progress.
    #[must_use]
    pub fn interpolate(&self, progress: f32) -> TransitionValue {
        let progress = progress.clamp(0.0, 1.0);

        match self {
            Self::Color { from, to } => {
                TransitionValue::Color(interpolate_color(from, to, progress))
            }
            Self::Position { from, to } => {
                TransitionValue::Position(interpolate_position(from, to, progress))
            }
            Self::Size { from, to } => TransitionValue::Size(interpolate_size(from, to, progress)),
            Self::Opacity { from, to } => {
                let value = from + (to - from) * progress;
                TransitionValue::Opacity(value.clamp(0.0, 1.0))
            }
        }
    }
}

/// Interpolates between two colors.
///
/// For RGB colors, each channel is interpolated independently.
/// Named colors and indexed colors are snapped to the nearest endpoint based on progress.
#[must_use]
pub fn interpolate_color(from: &Color, to: &Color, progress: f32) -> Color {
    // If colors are the same, no interpolation needed
    if from == to {
        return *from;
    }

    // Handle RGB-to-RGB interpolation
    match (from, to) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = interpolate_u8(*r1, *r2, progress);
            let g = interpolate_u8(*g1, *g2, progress);
            let b = interpolate_u8(*b1, *b2, progress);
            Color::Rgb(r, g, b)
        }
        // For named colors and indexed, snap at midpoint
        _ => {
            if progress < 0.5 {
                *from
            } else {
                *to
            }
        }
    }
}

/// Interpolates between two positions.
///
/// Each coordinate (x, y) is interpolated independently.
#[must_use]
pub fn interpolate_position(from: &Position, to: &Position, progress: f32) -> Position {
    Position {
        x: interpolate_u16(from.x, to.x, progress),
        y: interpolate_u16(from.y, to.y, progress),
    }
}

/// Interpolates between two sizes.
///
/// Each dimension (width, height) is interpolated independently.
#[must_use]
pub fn interpolate_size(from: &Size, to: &Size, progress: f32) -> Size {
    Size {
        width: interpolate_u16(from.width, to.width, progress),
        height: interpolate_u16(from.height, to.height, progress),
    }
}

/// Interpolates between two u8 values.
#[inline]
#[must_use]
fn interpolate_u8(from: u8, to: u8, progress: f32) -> u8 {
    let from_f = f32::from(from);
    let to_f = f32::from(to);
    let result = (to_f - from_f).mul_add(progress, from_f);
    result.round().clamp(0.0, 255.0) as u8
}

/// Interpolates between two u16 values.
#[inline]
#[must_use]
fn interpolate_u16(from: u16, to: u16, progress: f32) -> u16 {
    let from_f = f32::from(from);
    let to_f = f32::from(to);
    let result = (to_f - from_f).mul_add(progress, from_f);
    result.round().clamp(0.0, f32::from(u16::MAX)) as u16
}

/// A transition that animates a property change.
///
/// Transitions define how a property changes from one value to another,
/// including the duration, easing function, and optional delay.
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    /// The property being transitioned.
    pub property: TransitionProperty,
    /// Duration of the transition in milliseconds.
    pub duration_ms: u64,
    /// Easing function applied to the transition.
    pub easing: EasingFunction,
    /// Delay before the transition starts in milliseconds.
    pub delay_ms: u64,
}

impl Transition {
    /// Creates a new transition with the given property and default settings.
    ///
    /// Default settings:
    /// - Duration: 300ms
    /// - Easing: `EaseOutQuad`
    /// - Delay: 0ms
    #[must_use]
    pub const fn new(property: TransitionProperty) -> Self {
        Self {
            property,
            duration_ms: 300,
            easing: EasingFunction::QuadOut,
            delay_ms: 0,
        }
    }

    /// Sets the duration of the transition.
    #[must_use]
    pub const fn duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Sets the easing function for the transition.
    #[must_use]
    pub const fn easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Sets the delay before the transition starts.
    #[must_use]
    pub const fn delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    /// Interpolates the property at the given progress value.
    ///
    /// This applies the easing function to the progress before interpolation.
    ///
    /// # Arguments
    /// * `progress` - A value between 0.0 and 1.0 representing raw progress.
    ///
    /// # Returns
    /// The eased and interpolated value.
    #[must_use]
    pub fn interpolate(&self, progress: f32) -> TransitionValue {
        let eased_progress = self.easing.eval(f64::from(progress)) as f32;
        self.property.interpolate(eased_progress)
    }

    /// Returns true if the transition is still in the delay phase.
    #[must_use]
    pub const fn is_delayed(&self, elapsed_ms: u64) -> bool {
        elapsed_ms < self.delay_ms
    }

    /// Returns the progress of the transition given elapsed time.
    ///
    /// Accounts for delay and duration.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn progress(&self, elapsed_ms: u64) -> f32 {
        // If still in delay, return 0
        if elapsed_ms < self.delay_ms {
            return 0.0;
        }

        let effective_elapsed = elapsed_ms - self.delay_ms;

        // If duration is 0, return 1 immediately
        if self.duration_ms == 0 {
            return 1.0;
        }

        let progress = effective_elapsed as f32 / self.duration_ms as f32;
        progress.min(1.0)
    }

    /// Returns true if the transition has completed.
    #[must_use]
    pub const fn is_complete(&self, elapsed_ms: u64) -> bool {
        let total = self.delay_ms.saturating_add(self.duration_ms);
        elapsed_ms >= total
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::new(TransitionProperty::Opacity { from: 1.0, to: 1.0 })
    }
}

/// Builder for creating transitions with a fluent API.
///
/// # Example
///
/// ```
/// use ctui_animate::{TransitionBuilder, EasingFunction};
/// use ctui_core::Color;
///
/// let transition = TransitionBuilder::new()
///     .color(Color::Red, Color::Blue)
///     .duration_ms(500)
///     .easing(EasingFunction::CubicOut)
///     .delay(100)
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct TransitionBuilder {
    property: Option<TransitionProperty>,
    duration_ms: Option<u64>,
    easing: Option<EasingFunction>,
    delay_ms: Option<u64>,
}

impl TransitionBuilder {
    /// Creates a new transition builder with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            property: None,
            duration_ms: None,
            easing: None,
            delay_ms: None,
        }
    }

    /// Sets the property to a color transition.
    #[must_use]
    pub const fn color(mut self, from: Color, to: Color) -> Self {
        self.property = Some(TransitionProperty::Color { from, to });
        self
    }

    /// Sets the property to a position transition.
    #[must_use]
    pub const fn position(mut self, from: Position, to: Position) -> Self {
        self.property = Some(TransitionProperty::Position { from, to });
        self
    }

    /// Sets the property to a size transition.
    #[must_use]
    pub const fn size(mut self, from: Size, to: Size) -> Self {
        self.property = Some(TransitionProperty::Size { from, to });
        self
    }

    /// Sets the property to an opacity transition.
    #[must_use]
    pub const fn opacity(mut self, from: f32, to: f32) -> Self {
        self.property = Some(TransitionProperty::Opacity { from, to });
        self
    }

    /// Sets the duration of the transition in milliseconds.
    #[must_use]
    pub const fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    /// Sets the easing function for the transition.
    #[must_use]
    pub const fn easing(mut self, easing: EasingFunction) -> Self {
        self.easing = Some(easing);
        self
    }

    /// Sets the delay before the transition starts in milliseconds.
    #[must_use]
    pub const fn delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = Some(delay_ms);
        self
    }

    /// Builds the transition.
    ///
    /// # Panics
    ///
    /// Panics if no property has been set.
    #[must_use]
    pub fn build(self) -> Transition {
        let property = self
            .property
            .expect("TransitionBuilder requires a property to be set");

        Transition {
            property,
            duration_ms: self.duration_ms.unwrap_or(300),
            easing: self.easing.unwrap_or(EasingFunction::QuadOut),
            delay_ms: self.delay_ms.unwrap_or(0),
        }
    }

    /// Tries to build the transition, returning None if no property was set.
    #[must_use]
    pub fn try_build(self) -> Option<Transition> {
        self.property.map(|property| Transition {
            property,
            duration_ms: self.duration_ms.unwrap_or(300),
            easing: self.easing.unwrap_or(EasingFunction::QuadOut),
            delay_ms: self.delay_ms.unwrap_or(0),
        })
    }
}

use std::collections::HashMap;

/// Unique identifier for an active transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransitionId(pub u64);

/// Manager for active transitions on a component.
///
/// This type manages multiple transitions that apply to a single component.
/// It tracks elapsed time, handles delay periods, and provides interpolated
/// values for rendering.
///
/// # Example
///
/// ```
/// use ctui_animate::{TransitionContext, TransitionBuilder, EasingFunction};
/// use ctui_core::Color;
///
/// let mut ctx = TransitionContext::new();
///
/// // Start a color transition
/// let id = ctx.start(
///     TransitionBuilder::new()
///         .color(Color::Red, Color::Blue)
///         .duration_ms(500)
///         .build()
/// );
///
/// // Tick and get interpolated values
/// let updates = ctx.tick(250);
/// for (id, value) in updates {
///     // Use interpolated value
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TransitionContext {
    transitions: HashMap<TransitionId, ActiveTransition>,
    next_id: u64,
}

/// An active transition being executed.
#[derive(Debug, Clone)]
struct ActiveTransition {
    transition: Transition,
    elapsed_ms: u64,
}

impl TransitionContext {
    /// Creates a new empty transition context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            next_id: 0,
        }
    }

    /// Starts a new transition and returns its ID.
    pub fn start(&mut self, transition: Transition) -> TransitionId {
        let id = TransitionId(self.next_id);
        self.next_id += 1;

        self.transitions.insert(
            id,
            ActiveTransition {
                transition,
                elapsed_ms: 0,
            },
        );

        id
    }

    /// Cancels an active transition.
    pub fn cancel(&mut self, id: TransitionId) {
        self.transitions.remove(&id);
    }

    /// Cancels all active transitions.
    pub fn cancel_all(&mut self) {
        self.transitions.clear();
    }

    /// Returns true if a transition is active.
    #[must_use]
    pub fn is_active(&self, id: TransitionId) -> bool {
        self.transitions.contains_key(&id)
    }

    /// Returns the number of active transitions.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.transitions.len()
    }

    /// Advances all transitions by the given delta time.
    ///
    /// Returns a vector of (`TransitionId`, `TransitionValue`) for all
    /// transitions that produced values this tick. Completed transitions
    /// are automatically removed.
    #[must_use]
    pub fn tick(&mut self, delta_ms: u64) -> Vec<(TransitionId, TransitionValue)> {
        let mut results = Vec::with_capacity(self.transitions.len());
        let mut completed = Vec::new();

        for (id, active) in &mut self.transitions {
            active.elapsed_ms = active.elapsed_ms.saturating_add(delta_ms);

            if active.transition.is_complete(active.elapsed_ms) {
                let value = active.transition.interpolate(1.0);
                results.push((*id, value));
                completed.push(*id);
            } else {
                let progress = active.transition.progress(active.elapsed_ms);
                let value = active.transition.interpolate(progress);
                results.push((*id, value));
            }
        }

        for id in completed {
            self.transitions.remove(&id);
        }

        results
    }

    /// Returns the current interpolated value for a transition.
    #[must_use]
    pub fn current_value(&self, id: TransitionId) -> Option<TransitionValue> {
        self.transitions.get(&id).map(|active| {
            let progress = active.transition.progress(active.elapsed_ms);
            active.transition.interpolate(progress)
        })
    }

    /// Returns the progress (0.0 to 1.0) for a transition.
    #[must_use]
    pub fn progress(&self, id: TransitionId) -> Option<f32> {
        self.transitions
            .get(&id)
            .map(|active| active.transition.progress(active.elapsed_ms))
    }
}

impl Default for TransitionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for adding transitions to types.
///
/// This trait provides a fluent API for creating transitions on any type.
/// Types implementing this trait can use the `transition()` method to
/// start a transition chain.
///
/// # Example
///
/// ```
/// use ctui_animate::{TransitionExt, TransitionBuilder, EasingFunction};
/// use ctui_core::Color;
///
/// let value = Color::Red;
/// let builder = value.transition()
///     .to(Color::Blue)
///     .duration_ms(300)
///     .build();
/// ```
pub trait TransitionExt: Sized {
    /// The type of transition property this value can animate.
    type PropertyType;

    /// Creates a transition from this value.
    fn transition(self) -> TransitionFrom<Self::PropertyType>;
}

/// Builder for creating a transition from a starting value.
///
/// This type is returned by `transition()` and provides a fluent API
/// for specifying the target value and transition parameters.
#[derive(Debug)]
pub struct TransitionFrom<T> {
    from: T,
}

impl<T> TransitionFrom<T> {
    /// Creates a new transition builder from the given starting value.
    #[must_use]
    pub const fn new(from: T) -> Self {
        Self { from }
    }
}

impl TransitionFrom<Color> {
    /// Sets the target color for the transition.
    #[must_use]
    pub const fn to(self, to: Color) -> TransitionBuilder {
        TransitionBuilder::new().color(self.from, to)
    }
}

impl TransitionFrom<Position> {
    /// Sets the target position for the transition.
    #[must_use]
    pub const fn to(self, to: Position) -> TransitionBuilder {
        TransitionBuilder::new().position(self.from, to)
    }
}

impl TransitionFrom<Size> {
    /// Sets the target size for the transition.
    #[must_use]
    pub const fn to(self, to: Size) -> TransitionBuilder {
        TransitionBuilder::new().size(self.from, to)
    }
}

impl TransitionFrom<f32> {
    /// Sets the target opacity for the transition.
    #[must_use]
    pub const fn to(self, to: f32) -> TransitionBuilder {
        TransitionBuilder::new().opacity(self.from, to)
    }
}

impl TransitionExt for Color {
    type PropertyType = Self;

    fn transition(self) -> TransitionFrom<Self> {
        TransitionFrom::new(self)
    }
}

impl TransitionExt for Position {
    type PropertyType = Self;

    fn transition(self) -> TransitionFrom<Self> {
        TransitionFrom::new(self)
    }
}

impl TransitionExt for Size {
    type PropertyType = Self;

    fn transition(self) -> TransitionFrom<Self> {
        TransitionFrom::new(self)
    }
}

impl TransitionExt for f32 {
    type PropertyType = Self;

    fn transition(self) -> TransitionFrom<Self> {
        TransitionFrom::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_color_rgb() {
        let from = Color::Rgb(0, 0, 0);
        let to = Color::Rgb(255, 255, 255);

        // Start
        let result = interpolate_color(&from, &to, 0.0);
        assert_eq!(result, Color::Rgb(0, 0, 0));

        // End
        let result = interpolate_color(&from, &to, 1.0);
        assert_eq!(result, Color::Rgb(255, 255, 255));

        // Midpoint
        let result = interpolate_color(&from, &to, 0.5);
        assert_eq!(result, Color::Rgb(128, 128, 128));
    }

    #[test]
    fn test_interpolate_color_same() {
        let color = Color::Rgb(100, 150, 200);
        let result = interpolate_color(&color, &color, 0.5);
        assert_eq!(result, Color::Rgb(100, 150, 200));
    }

    #[test]
    fn test_interpolate_color_named() {
        // Named colors snap at midpoint
        let result = interpolate_color(&Color::Red, &Color::Blue, 0.0);
        assert_eq!(result, Color::Red);

        let result = interpolate_color(&Color::Red, &Color::Blue, 0.49);
        assert_eq!(result, Color::Red);

        let result = interpolate_color(&Color::Red, &Color::Blue, 0.5);
        assert_eq!(result, Color::Blue);

        let result = interpolate_color(&Color::Red, &Color::Blue, 1.0);
        assert_eq!(result, Color::Blue);
    }

    #[test]
    fn test_interpolate_position() {
        let from = Position::new(0, 0);
        let to = Position::new(100, 200);

        // Start
        let result = interpolate_position(&from, &to, 0.0);
        assert_eq!(result, Position::new(0, 0));

        // End
        let result = interpolate_position(&from, &to, 1.0);
        assert_eq!(result, Position::new(100, 200));

        // Midpoint
        let result = interpolate_position(&from, &to, 0.5);
        assert_eq!(result, Position::new(50, 100));
    }

    #[test]
    fn test_interpolate_size() {
        let from = Size::new(10, 20);
        let to = Size::new(110, 120);

        // Start
        let result = interpolate_size(&from, &to, 0.0);
        assert_eq!(result, Size::new(10, 20));

        // End
        let result = interpolate_size(&from, &to, 1.0);
        assert_eq!(result, Size::new(110, 120));

        // Midpoint
        let result = interpolate_size(&from, &to, 0.5);
        assert_eq!(result, Size::new(60, 70));
    }

    #[test]
    fn test_interpolate_opacity() {
        let property = TransitionProperty::Opacity { from: 0.0, to: 1.0 };

        // Start
        let result = property.interpolate(0.0);
        assert_eq!(result, TransitionValue::Opacity(0.0));

        // End
        let result = property.interpolate(1.0);
        assert_eq!(result, TransitionValue::Opacity(1.0));

        // Midpoint
        let result = property.interpolate(0.5);
        assert_eq!(result, TransitionValue::Opacity(0.5));
    }

    #[test]
    fn test_opacity_clamping() {
        let property = TransitionProperty::Opacity {
            from: -0.5,
            to: 1.5,
        };

        // Values should be clamped
        let result = property.interpolate(0.0);
        assert_eq!(result, TransitionValue::Opacity(0.0));

        let result = property.interpolate(1.0);
        assert_eq!(result, TransitionValue::Opacity(1.0));
    }

    #[test]
    fn test_transition_new() {
        let property = TransitionProperty::Opacity { from: 0.0, to: 1.0 };
        let transition = Transition::new(property.clone());

        assert_eq!(transition.property, property);
        assert_eq!(transition.duration_ms, 300);
        assert_eq!(transition.easing, EasingFunction::QuadOut);
        assert_eq!(transition.delay_ms, 0);
    }

    #[test]
    fn test_transition_builder() {
        let transition = TransitionBuilder::new()
            .color(Color::Red, Color::Blue)
            .duration_ms(500)
            .easing(EasingFunction::CubicOut)
            .delay(100)
            .build();

        assert_eq!(
            transition.property,
            TransitionProperty::Color {
                from: Color::Red,
                to: Color::Blue,
            }
        );
        assert_eq!(transition.duration_ms, 500);
        assert_eq!(transition.easing, EasingFunction::CubicOut);
        assert_eq!(transition.delay_ms, 100);
    }

    #[test]
    fn test_transition_builder_defaults() {
        let transition = TransitionBuilder::new().opacity(0.0, 1.0).build();

        assert_eq!(transition.duration_ms, 300);
        assert_eq!(transition.easing, EasingFunction::QuadOut);
        assert_eq!(transition.delay_ms, 0);
    }

    #[test]
    #[should_panic(expected = "TransitionBuilder requires a property")]
    fn test_transition_builder_no_property() {
        let _ = TransitionBuilder::new().build();
    }

    #[test]
    fn test_transition_try_build() {
        let transition = TransitionBuilder::new().try_build();
        assert!(transition.is_none());

        let transition = TransitionBuilder::new().opacity(0.0, 1.0).try_build();
        assert!(transition.is_some());
    }

    #[test]
    fn test_transition_progress() {
        let transition =
            Transition::new(TransitionProperty::Opacity { from: 0.0, to: 1.0 }).duration(1000);

        // At start
        approx::assert_relative_eq!(transition.progress(0), 0.0, epsilon = 1e-6);

        // At half
        approx::assert_relative_eq!(transition.progress(500), 0.5, epsilon = 1e-6);

        // At end
        approx::assert_relative_eq!(transition.progress(1000), 1.0, epsilon = 1e-6);

        // Beyond
        approx::assert_relative_eq!(transition.progress(2000), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_transition_progress_with_delay() {
        let transition = Transition::new(TransitionProperty::Opacity { from: 0.0, to: 1.0 })
            .duration(1000)
            .delay(500);

        // During delay
        approx::assert_relative_eq!(transition.progress(0), 0.0, epsilon = 1e-6);
        approx::assert_relative_eq!(transition.progress(250), 0.0, epsilon = 1e-6);
        assert!(transition.is_delayed(250));

        // Just after delay starts
        approx::assert_relative_eq!(transition.progress(500), 0.0, epsilon = 1e-6);
        assert!(!transition.is_delayed(500));

        // Halfway through animation
        approx::assert_relative_eq!(transition.progress(1000), 0.5, epsilon = 1e-6);

        // At end
        approx::assert_relative_eq!(transition.progress(1500), 1.0, epsilon = 1e-6);
        assert!(transition.is_complete(1500));

        // Beyond
        approx::assert_relative_eq!(transition.progress(2000), 1.0, epsilon = 1e-6);
        assert!(transition.is_complete(2000));
    }

    #[test]
    fn test_transition_interpolate_with_easing() {
        let transition = Transition::new(TransitionProperty::Opacity { from: 0.0, to: 1.0 })
            .duration(1000)
            .easing(EasingFunction::QuadOut);

        // Linear progress at 0.5 would give 0.5
        // QuadOut at 0.5 should give ~0.75
        let result = transition.interpolate(0.5);
        if let TransitionValue::Opacity(value) = result {
            approx::assert_relative_eq!(value, 0.75, epsilon = 1e-6);
        } else {
            panic!("Expected Opacity value");
        }
    }

    #[test]
    fn test_transition_zero_duration() {
        let transition =
            Transition::new(TransitionProperty::Opacity { from: 0.0, to: 1.0 }).duration(0);

        // Zero duration should immediately complete
        approx::assert_relative_eq!(transition.progress(0), 1.0, epsilon = 1e-6);
        assert!(transition.is_complete(0));
    }

    #[test]
    fn test_interpolate_u8() {
        assert_eq!(interpolate_u8(0, 255, 0.0), 0);
        assert_eq!(interpolate_u8(0, 255, 1.0), 255);
        assert_eq!(interpolate_u8(0, 255, 0.5), 128);
        assert_eq!(interpolate_u8(100, 200, 0.5), 150);
    }

    #[test]
    fn test_interpolate_u16() {
        assert_eq!(interpolate_u16(0, 1000, 0.0), 0);
        assert_eq!(interpolate_u16(0, 1000, 1.0), 1000);
        assert_eq!(interpolate_u16(0, 1000, 0.5), 500);
        assert_eq!(interpolate_u16(100, 200, 0.5), 150);
    }

    #[test]
    fn test_property_interpolate() {
        let prop = TransitionProperty::Color {
            from: Color::Rgb(0, 0, 0),
            to: Color::Rgb(255, 255, 255),
        };

        let result = prop.interpolate(0.5);
        assert_eq!(result, TransitionValue::Color(Color::Rgb(128, 128, 128)));
    }

    #[test]
    fn test_transition_default() {
        let transition = Transition::default();
        assert_eq!(transition.duration_ms, 300);
        assert_eq!(transition.easing, EasingFunction::QuadOut);
    }

    #[test]
    fn test_builder_default() {
        let builder = TransitionBuilder::default();
        assert!(builder.property.is_none());
        assert!(builder.duration_ms.is_none());
        assert!(builder.easing.is_none());
        assert!(builder.delay_ms.is_none());
    }

    #[test]
    fn test_transition_context_new() {
        let ctx = TransitionContext::new();
        assert_eq!(ctx.active_count(), 0);
    }

    #[test]
    fn test_transition_context_default() {
        let ctx = TransitionContext::default();
        assert_eq!(ctx.active_count(), 0);
    }

    #[test]
    fn test_transition_context_start() {
        let mut ctx = TransitionContext::new();
        let transition = TransitionBuilder::new()
            .opacity(0.0, 1.0)
            .duration_ms(500)
            .build();

        let id = ctx.start(transition);
        assert!(ctx.is_active(id));
        assert_eq!(ctx.active_count(), 1);
    }

    #[test]
    fn test_transition_context_cancel() {
        let mut ctx = TransitionContext::new();
        let transition = TransitionBuilder::new().opacity(0.0, 1.0).build();

        let id = ctx.start(transition);
        assert!(ctx.is_active(id));

        ctx.cancel(id);
        assert!(!ctx.is_active(id));
        assert_eq!(ctx.active_count(), 0);
    }

    #[test]
    fn test_transition_context_cancel_all() {
        let mut ctx = TransitionContext::new();

        let _ = ctx.start(TransitionBuilder::new().opacity(0.0, 1.0).build());
        let _ = ctx.start(TransitionBuilder::new().opacity(0.5, 1.0).build());
        let _ = ctx.start(TransitionBuilder::new().opacity(0.0, 0.5).build());

        assert_eq!(ctx.active_count(), 3);

        ctx.cancel_all();
        assert_eq!(ctx.active_count(), 0);
    }

    #[test]
    fn test_transition_context_tick() {
        let mut ctx = TransitionContext::new();
        let transition = TransitionBuilder::new()
            .opacity(0.0, 1.0)
            .duration_ms(500)
            .easing(EasingFunction::Linear)
            .build();

        let id = ctx.start(transition);

        // Tick halfway
        let updates = ctx.tick(250);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].0, id);

        if let TransitionValue::Opacity(value) = updates[0].1 {
            approx::assert_relative_eq!(value, 0.5, epsilon = 1e-6);
        } else {
            panic!("Expected Opacity value");
        }

        assert!(ctx.is_active(id));
    }

    #[test]
    fn test_transition_context_tick_complete() {
        let mut ctx = TransitionContext::new();
        let transition = TransitionBuilder::new()
            .opacity(0.0, 1.0)
            .duration_ms(100)
            .build();

        let id = ctx.start(transition);

        // Tick beyond completion
        let updates = ctx.tick(200);
        assert_eq!(updates.len(), 1);

        // Transition should be removed
        assert!(!ctx.is_active(id));
        assert_eq!(ctx.active_count(), 0);
    }

    #[test]
    fn test_transition_context_current_value() {
        let mut ctx = TransitionContext::new();
        let transition = TransitionBuilder::new()
            .opacity(0.0, 1.0)
            .duration_ms(1000)
            .easing(EasingFunction::Linear)
            .build();

        let id = ctx.start(transition);

        // Tick to 50%
        let _ = ctx.tick(500);

        let value = ctx.current_value(id);
        assert!(value.is_some());

        if let Some(TransitionValue::Opacity(v)) = value {
            approx::assert_relative_eq!(v, 0.5, epsilon = 1e-6);
        } else {
            panic!("Expected Opacity value");
        }
    }

    #[test]
    fn test_transition_context_progress() {
        let mut ctx = TransitionContext::new();
        let transition = TransitionBuilder::new()
            .opacity(0.0, 1.0)
            .duration_ms(1000)
            .easing(EasingFunction::Linear)
            .build();

        let id = ctx.start(transition);

        let _ = ctx.tick(250);
        let progress = ctx.progress(id);
        assert!(progress.is_some());
        approx::assert_relative_eq!(progress.unwrap(), 0.25, epsilon = 1e-6);

        let _ = ctx.tick(250);
        let progress = ctx.progress(id);
        approx::assert_relative_eq!(progress.unwrap(), 0.5, epsilon = 1e-6);
    }

    #[test]
    fn test_transition_ext_color() {
        let transition = Color::Red
            .transition()
            .to(Color::Blue)
            .duration_ms(300)
            .build();

        assert_eq!(
            transition.property,
            TransitionProperty::Color {
                from: Color::Red,
                to: Color::Blue,
            }
        );
        assert_eq!(transition.duration_ms, 300);
    }

    #[test]
    fn test_transition_ext_position() {
        let from = Position::new(0, 0);
        let to = Position::new(100, 100);

        let transition = from.transition().to(to).duration_ms(500).build();

        assert_eq!(
            transition.property,
            TransitionProperty::Position { from, to }
        );
        assert_eq!(transition.duration_ms, 500);
    }

    #[test]
    fn test_transition_ext_size() {
        let from = Size::new(10, 10);
        let to = Size::new(50, 50);

        let transition = from.transition().to(to).duration_ms(200).build();

        assert_eq!(transition.property, TransitionProperty::Size { from, to });
        assert_eq!(transition.duration_ms, 200);
    }

    #[test]
    fn test_transition_ext_opacity() {
        let transition = 0.0_f32.transition().to(1.0).duration_ms(400).build();

        assert_eq!(
            transition.property,
            TransitionProperty::Opacity { from: 0.0, to: 1.0 }
        );
        assert_eq!(transition.duration_ms, 400);
    }

    #[test]
    fn test_transition_id_sequential() {
        let mut ctx = TransitionContext::new();

        let id1 = ctx.start(TransitionBuilder::new().opacity(0.0, 1.0).build());
        let id2 = ctx.start(TransitionBuilder::new().opacity(0.5, 1.0).build());
        let id3 = ctx.start(TransitionBuilder::new().opacity(0.0, 0.5).build());

        assert_eq!(id1, TransitionId(0));
        assert_eq!(id2, TransitionId(1));
        assert_eq!(id3, TransitionId(2));
    }

    #[test]
    fn test_multiple_transitions_tick() {
        let mut ctx = TransitionContext::new();

        let id1 = ctx.start(
            TransitionBuilder::new()
                .opacity(0.0, 1.0)
                .duration_ms(1000)
                .easing(EasingFunction::Linear)
                .build(),
        );

        let id2 = ctx.start(
            TransitionBuilder::new()
                .color(Color::Red, Color::Blue)
                .duration_ms(500)
                .easing(EasingFunction::Linear)
                .build(),
        );

        // Tick 500ms - id1 should be 50%, id2 should be complete
        let updates = ctx.tick(500);
        assert_eq!(updates.len(), 2);

        // id2 should be removed after completion
        assert!(ctx.is_active(id1));
        assert!(!ctx.is_active(id2));
        assert_eq!(ctx.active_count(), 1);
    }
}
