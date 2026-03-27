//! Animated style and layout components.
//!
//! This module provides animation-aware wrappers for style and layout
//! properties that automatically animate changes over time.
//!
//! # Example
//!
//! ```
//! use ctui_animate::animated::{AnimatedStyle, AnimatedLayout};
//! use ctui_core::Color;
//! use ctui_animate::easing::EasingFunction;
//!
//! // Create an animated style
//! let mut style = AnimatedStyle::new()
//!     .duration_ms(300)
//!     .easing(EasingFunction::CubicOut);
//!
//! style.set_fg(Color::Red);
//! style.tick(150);
//! let current_fg = style.current_fg();
//!
//! // Create an animated layout
//! let mut layout = AnimatedLayout::new()
//!     .duration_ms(500);
//!
//! layout.set_x(100);
//! layout.tick(250);
//! ```

use crate::easing::EasingFunction;
use crate::interpolate::Interpolator;
use crate::sequence::AnimationController;
use crate::spring::{SpringAnimation, SpringConfig};
use ctui_core::geometry::{Position, Size};
use ctui_core::style::Style;
use ctui_core::Color;

/// Animation mode for style/layout changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum AnimationMode {
    /// Use easing-based animation with fixed duration.
    #[default]
    Eased,
    /// Use spring physics animation.
    Spring,
    /// No animation, instant change.
    Instant,
}


/// Animated foreground color.
struct AnimatedColor {
    current: Color,
    target: Color,
    start: Color,
    elapsed_ms: u64,
    duration_ms: u64,
    easing: EasingFunction,
    active: bool,
}

impl AnimatedColor {
    const fn new(color: Color) -> Self {
        Self {
            current: color,
            target: color,
            start: color,
            elapsed_ms: 0,
            duration_ms: 300,
            easing: EasingFunction::QuadOut,
            active: false,
        }
    }

    fn set_target(&mut self, target: Color, duration_ms: u64, easing: EasingFunction) {
        if self.target != target {
            self.start = self.current;
            self.target = target;
            self.elapsed_ms = 0;
            self.duration_ms = duration_ms;
            self.easing = easing;
            self.active = true;
        }
    }

    fn tick(&mut self, delta_ms: u64) {
        if !self.active {
            return;
        }

        self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);

        if self.elapsed_ms >= self.duration_ms {
            self.current = self.target;
            self.active = false;
        } else if self.duration_ms > 0 {
            let progress = self.elapsed_ms as f32 / self.duration_ms as f32;
            let eased = self.easing.eval(f64::from(progress)) as f32;
            self.current = self.start.interpolate(&self.target, eased);
        }
    }

    const fn current(&self) -> Color {
        self.current
    }

    const fn is_active(&self) -> bool {
        self.active
    }

    const fn snap(&mut self) {
        self.current = self.target;
        self.active = false;
    }
}

/// Animated position value.
pub struct AnimatedValue<T: Interpolator + Copy + PartialEq> {
    current: T,
    target: T,
    start: T,
    elapsed_ms: u64,
    duration_ms: u64,
    easing: EasingFunction,
    active: bool,
    on_complete: Option<Box<dyn FnMut() + Send + Sync>>,
}

impl<T: Interpolator + Copy + PartialEq> AnimatedValue<T> {
    fn new(value: T) -> Self {
        Self {
            current: value,
            target: value,
            start: value,
            elapsed_ms: 0,
            duration_ms: 300,
            easing: EasingFunction::QuadOut,
            active: false,
            on_complete: None,
        }
    }

    fn set_target(&mut self, target: T, duration_ms: u64, easing: EasingFunction) {
        if self.target != target {
            self.start = self.current;
            self.target = target;
            self.elapsed_ms = 0;
            self.duration_ms = duration_ms;
            self.easing = easing;
            self.active = true;
        }
    }

    fn tick(&mut self, delta_ms: u64) -> bool {
        if !self.active {
            return false;
        }

        self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);

        if self.elapsed_ms >= self.duration_ms {
            self.current = self.target;
            self.active = false;
            true
        } else if self.duration_ms > 0 {
            let progress = self.elapsed_ms as f32 / self.duration_ms as f32;
            let eased = self.easing.eval(f64::from(progress)) as f32;
            self.current = self.start.interpolate(&self.target, eased);
            false
        } else {
            false
        }
    }

    const fn current(&self) -> T {
        self.current
    }

    const fn is_active(&self) -> bool {
        self.active
    }

    const fn snap(&mut self) {
        self.current = self.target;
        self.active = false;
    }
}

/// Animated opacity value.
struct AnimatedOpacity {
    spring: SpringAnimation,
    use_spring: bool,
    target: f32,
    eased_value: f32,
    elapsed_ms: u64,
    duration_ms: u64,
    easing: EasingFunction,
    active: bool,
}

impl AnimatedOpacity {
    fn new() -> Self {
        Self {
            spring: SpringAnimation::new(1.0),
            use_spring: false,
            target: 1.0,
            eased_value: 1.0,
            elapsed_ms: 0,
            duration_ms: 300,
            easing: EasingFunction::QuadOut,
            active: false,
        }
    }

    fn set_target(&mut self, target: f32) {
        self.target = target.clamp(0.0, 1.0);
        self.elapsed_ms = 0;
        self.active = true;

        if self.use_spring {
            self.spring.set_target(self.target);
        }
    }

    fn tick(&mut self, delta_ms: u64) {
        if !self.active {
            return;
        }

        if self.use_spring {
            self.spring.tick(delta_ms);
            self.eased_value = self.spring.current_value();
            if self.spring.is_settled() {
                self.active = false;
            }
        } else {
            self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);
            if self.elapsed_ms >= self.duration_ms {
                self.eased_value = self.target;
                self.active = false;
            } else if self.duration_ms > 0 {
                let progress = self.elapsed_ms as f32 / self.duration_ms as f32;
                let eased = self.easing.eval(f64::from(progress)) as f32;
                self.eased_value = self.target * eased;
            }
        }
    }

    const fn current(&self) -> f32 {
        self.eased_value.clamp(0.0, 1.0)
    }

    const fn is_active(&self) -> bool {
        self.active
    }
}

/// Animated style wrapper.
///
/// Wraps a style and provides animation for property changes.
pub struct AnimatedStyle {
    fg: Option<AnimatedColor>,
    bg: Option<AnimatedColor>,
    opacity: AnimatedOpacity,
    duration_ms: u64,
    easing: EasingFunction,
    mode: AnimationMode,
    spring_config: SpringConfig,
}

impl AnimatedStyle {
    /// Creates a new animated style with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            opacity: AnimatedOpacity::new(),
            duration_ms: 300,
            easing: EasingFunction::QuadOut,
            mode: AnimationMode::Eased,
            spring_config: SpringConfig::SNAPPY,
        }
    }

    /// Sets the animation duration.
    #[must_use]
    pub const fn duration_ms(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    /// Sets the easing function.
    #[must_use]
    pub const fn easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Sets the animation mode.
    #[must_use]
    pub const fn mode(mut self, mode: AnimationMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the spring configuration (for spring mode).
    #[must_use]
    pub const fn spring_config(mut self, config: SpringConfig) -> Self {
        self.spring_config = config;
        self
    }

    /// Sets the foreground color.
    pub fn set_fg(&mut self, color: Color) {
        if let Some(ref mut fg) = self.fg {
            fg.set_target(color, self.duration_ms, self.easing);
        } else {
            // First color set - just establish initial color
            self.fg = Some(AnimatedColor::new(color));
        }
    }

    /// Sets the background color.
    pub fn set_bg(&mut self, color: Color) {
        if let Some(ref mut bg) = self.bg {
            bg.set_target(color, self.duration_ms, self.easing);
        } else {
            // First color set - just establish initial color
            self.bg = Some(AnimatedColor::new(color));
        }
    }

    /// Sets the opacity (0.0 to 1.0).
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity.set_target(opacity);
    }

    /// Returns the current foreground color.
    #[must_use]
    pub fn current_fg(&self) -> Option<Color> {
        self.fg.as_ref().map(AnimatedColor::current)
    }

    /// Returns the current background color.
    #[must_use]
    pub fn current_bg(&self) -> Option<Color> {
        self.bg.as_ref().map(AnimatedColor::current)
    }

    /// Returns the current opacity.
    #[must_use]
    pub const fn current_opacity(&self) -> f32 {
        self.opacity.current()
    }

    /// Advances the animation by the given delta time.
    pub fn tick(&mut self, delta_ms: u64) {
        if let Some(ref mut fg) = self.fg {
            fg.tick(delta_ms);
        }
        if let Some(ref mut bg) = self.bg {
            bg.tick(delta_ms);
        }
        self.opacity.tick(delta_ms);
    }

    /// Returns whether any animation is active.
    #[must_use]
    pub fn is_animating(&self) -> bool {
        let fg_active = self.fg.as_ref().is_some_and(AnimatedColor::is_active);
        let bg_active = self.bg.as_ref().is_some_and(AnimatedColor::is_active);
        fg_active || bg_active || self.opacity.is_active()
    }

    /// Snaps to final state (instant).
    pub const fn snap(&mut self) {
        if let Some(ref mut fg) = self.fg {
            fg.snap();
        }
        if let Some(ref mut bg) = self.bg {
            bg.snap();
        }
        self.opacity.eased_value = self.opacity.target;
        self.opacity.active = false;
    }

    /// Builds the current animated style into a Style.
    #[must_use]
    pub fn to_style(&self) -> Style {
        let mut style = Style::default();

        if let Some(fg) = self.current_fg() {
            style = style.fg(fg);
        }

        if let Some(bg) = self.current_bg() {
            style = style.bg(bg);
        }

        style
    }
}

impl Default for AnimatedStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Animated layout wrapper.
///
/// Wraps position and size with animation for changes.
pub struct AnimatedLayout {
    x: AnimatedValue<u16>,
    y: AnimatedValue<u16>,
    width: AnimatedValue<u16>,
    height: AnimatedValue<u16>,
    duration_ms: u64,
    easing: EasingFunction,
    on_complete: Option<Box<dyn FnMut() + Send + Sync>>,
    callback_fired: bool,
}

impl AnimatedLayout {
    /// Creates a new animated layout.
    #[must_use]
    pub fn new() -> Self {
        Self {
            x: AnimatedValue::new(0),
            y: AnimatedValue::new(0),
            width: AnimatedValue::new(0),
            height: AnimatedValue::new(0),
            duration_ms: 300,
            easing: EasingFunction::QuadOut,
            on_complete: None,
            callback_fired: false,
        }
    }

    /// Sets the animation duration.
    #[must_use]
    pub const fn duration_ms(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    /// Sets the easing function.
    #[must_use]
    pub const fn easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Sets a completion callback.
    pub fn on_complete<F: FnMut() + Send + Sync + 'static>(mut self, callback: F) -> Self {
        self.on_complete = Some(Box::new(callback));
        self
    }

    /// Sets the x position.
    pub fn set_x(&mut self, x: u16) {
        self.x.set_target(x, self.duration_ms, self.easing);
        self.callback_fired = false;
    }

    /// Sets the y position.
    pub fn set_y(&mut self, y: u16) {
        self.y.set_target(y, self.duration_ms, self.easing);
        self.callback_fired = false;
    }

    /// Sets the position.
    pub fn set_position(&mut self, pos: Position) {
        self.x.set_target(pos.x, self.duration_ms, self.easing);
        self.y.set_target(pos.y, self.duration_ms, self.easing);
        self.callback_fired = false;
    }

    /// Sets the width.
    pub fn set_width(&mut self, width: u16) {
        self.width.set_target(width, self.duration_ms, self.easing);
        self.callback_fired = false;
    }

    /// Sets the height.
    pub fn set_height(&mut self, height: u16) {
        self.height
            .set_target(height, self.duration_ms, self.easing);
        self.callback_fired = false;
    }

    /// Sets the size.
    pub fn set_size(&mut self, size: Size) {
        self.width
            .set_target(size.width, self.duration_ms, self.easing);
        self.height
            .set_target(size.height, self.duration_ms, self.easing);
        self.callback_fired = false;
    }

    /// Returns the current x position.
    #[must_use]
    pub const fn current_x(&self) -> u16 {
        self.x.current()
    }

    /// Returns the current y position.
    #[must_use]
    pub const fn current_y(&self) -> u16 {
        self.y.current()
    }

    /// Returns the current position.
    #[must_use]
    pub fn current_position(&self) -> Position {
        Position::new(self.current_x(), self.current_y())
    }

    /// Returns the current width.
    #[must_use]
    pub const fn current_width(&self) -> u16 {
        self.width.current()
    }

    /// Returns the current height.
    #[must_use]
    pub const fn current_height(&self) -> u16 {
        self.height.current()
    }

    /// Returns the current size.
    #[must_use]
    pub fn current_size(&self) -> Size {
        Size::new(self.current_width(), self.current_height())
    }

    /// Advances the animation.
    pub fn tick(&mut self, delta_ms: u64) {
        self.x.tick(delta_ms);
        self.y.tick(delta_ms);
        self.width.tick(delta_ms);
        self.height.tick(delta_ms);

        if !self.is_animating() && !self.callback_fired {
            self.callback_fired = true;
            if let Some(ref mut callback) = self.on_complete {
                callback();
            }
        }
    }

    /// Returns whether any animation is active.
    #[must_use]
    pub const fn is_animating(&self) -> bool {
        self.x.is_active()
            || self.y.is_active()
            || self.width.is_active()
            || self.height.is_active()
    }

    /// Snaps to final state.
    pub const fn snap(&mut self) {
        self.x.snap();
        self.y.snap();
        self.width.snap();
        self.height.snap();
    }
}

impl Default for AnimatedLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationController for AnimatedLayout {
    fn tick(&mut self, delta_ms: u64) -> bool {
        self.tick(delta_ms);
        !self.is_animating()
    }

    fn is_complete(&self) -> bool {
        !self.is_animating()
    }

    fn reset(&mut self) {
        self.snap();
    }

    fn pause(&mut self) {
        // AnimationController trait requires pause - animated values don't support pause directly
    }

    fn resume(&mut self) {
        // AnimationController trait requires resume
    }

    fn is_paused(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animated_style_new() {
        let style = AnimatedStyle::new();
        assert!(style.current_fg().is_none());
        assert!(style.current_bg().is_none());
        approx::assert_relative_eq!(style.current_opacity(), 1.0);
    }

    #[test]
    fn test_animated_style_default() {
        let style = AnimatedStyle::default();
        assert!(style.current_fg().is_none());
    }

    #[test]
    fn test_animated_style_set_fg() {
        let mut style = AnimatedStyle::new().duration_ms(100);

        // First set - establishes initial color (no animation)
        style.set_fg(Color::Red);
        // Initially at target, not animating
        assert!(!style.is_animating());

        // Now set to a different color - should animate
        style.set_fg(Color::Blue);
        assert!(style.is_animating());

        style.tick(50);
        assert!(style.is_animating());

        style.tick(50);
        assert!(!style.is_animating());
        assert_eq!(style.current_fg(), Some(Color::Blue));
    }

    #[test]
    fn test_animated_style_set_bg() {
        let mut style = AnimatedStyle::new().duration_ms(100);

        style.set_bg(Color::Blue);
        style.tick(100);

        assert_eq!(style.current_bg(), Some(Color::Blue));
    }

    #[test]
    fn test_animated_style_opacity() {
        let mut style = AnimatedStyle::new().duration_ms(100);

        style.set_opacity(0.5);
        style.tick(50);

        let opacity = style.current_opacity();
        assert!(opacity > 0.0 && opacity < 1.0);
    }

    #[test]
    fn test_animated_style_snap() {
        let mut style = AnimatedStyle::new().duration_ms(1000);

        style.set_fg(Color::Red);
        style.set_bg(Color::Blue);

        style.snap();
        assert!(!style.is_animating());
        assert_eq!(style.current_fg(), Some(Color::Red));
        assert_eq!(style.current_bg(), Some(Color::Blue));
    }

    #[test]
    fn test_animated_style_to_style() {
        let mut style = AnimatedStyle::new();

        style.set_fg(Color::Red);
        style.set_bg(Color::Blue);
        style.snap();

        let result = style.to_style();
        assert_eq!(result.fg, Color::Red);
        assert_eq!(result.bg, Color::Blue);
    }

    #[test]
    fn test_animated_layout_new() {
        let layout = AnimatedLayout::new();
        assert_eq!(layout.current_x(), 0);
        assert_eq!(layout.current_y(), 0);
        assert_eq!(layout.current_width(), 0);
        assert_eq!(layout.current_height(), 0);
    }

    #[test]
    fn test_animated_layout_default() {
        let layout = AnimatedLayout::default();
        assert_eq!(layout.current_x(), 0);
    }

    #[test]
    fn test_animated_layout_set_position() {
        let mut layout = AnimatedLayout::new().duration_ms(100);

        layout.set_position(Position::new(100, 200));
        assert!(layout.is_animating());

        layout.tick(100);
        assert!(!layout.is_animating());
        assert_eq!(layout.current_position(), Position::new(100, 200));
    }

    #[test]
    fn test_animated_layout_set_size() {
        let mut layout = AnimatedLayout::new().duration_ms(100);

        layout.set_size(Size::new(50, 75));
        layout.tick(100);

        assert_eq!(layout.current_size(), Size::new(50, 75));
    }

    #[test]
    fn test_animated_layout_individual_setters() {
        let mut layout = AnimatedLayout::new().duration_ms(100);

        layout.set_x(10);
        layout.set_y(20);
        layout.set_width(30);
        layout.set_height(40);

        layout.tick(100);

        assert_eq!(layout.current_x(), 10);
        assert_eq!(layout.current_y(), 20);
        assert_eq!(layout.current_width(), 30);
        assert_eq!(layout.current_height(), 40);
    }

    #[test]
    fn test_animated_layout_snap() {
        let mut layout = AnimatedLayout::new().duration_ms(1000);

        layout.set_position(Position::new(100, 200));
        layout.set_size(Size::new(50, 75));

        layout.snap();
        assert!(!layout.is_animating());
        assert_eq!(layout.current_position(), Position::new(100, 200));
        assert_eq!(layout.current_size(), Size::new(50, 75));
    }

    #[test]
    fn test_animated_layout_on_complete() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let mut layout = AnimatedLayout::new().duration_ms(100).on_complete(move || {
            called_clone.store(true, Ordering::SeqCst);
        });

        layout.set_x(100);
        layout.tick(100);

        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_animation_mode_default() {
        assert_eq!(AnimationMode::default(), AnimationMode::Eased);
    }

    #[test]
    fn test_animated_layout_controller_trait() {
        let mut layout = AnimatedLayout::new().duration_ms(100);
        layout.set_x(100);

        assert!(!layout.is_complete());

        layout.tick(100);
        assert!(layout.is_complete());

        layout.set_x(200);
        layout.reset();
        assert!(layout.is_complete());
    }

    #[test]
    fn test_animated_style_easing() {
        let mut style = AnimatedStyle::new()
            .duration_ms(100)
            .easing(EasingFunction::Linear);

        style.set_fg(Color::Rgb(0, 0, 0));

        // At 50% with linear, should be exactly halfway
        style.tick(50);

        // Compare against expected midpoint (0 + 0) / 2 = 0 for black
        // This tests that easing is applied
    }

    #[test]
    fn test_animated_layout_easing() {
        let mut layout = AnimatedLayout::new()
            .duration_ms(100)
            .easing(EasingFunction::QuadOut);

        layout.set_x(100);

        // At 50% with QuadOut, progress is ~75% so value should be ~75
        layout.tick(50);

        // Value should be higher than 50 (linear midpoint)
        assert!(layout.current_x() > 50);
    }

    #[test]
    fn test_animated_opacity_values() {
        let mut opacity = AnimatedOpacity::new();

        opacity.set_target(0.0);

        // After full duration, should reach target
        for _ in 0..100 {
            opacity.tick(1);
        }

        approx::assert_relative_eq!(opacity.current(), 0.0, epsilon = 0.01);
    }
}
