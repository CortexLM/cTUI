//! Spring physics animation for natural motion.
//!
//! This module provides spring-based animations that simulate
//! physical springs with damping and stiffness parameters.
//! Springs produce natural, organic-feeling motion that feels
//! more realistic than easing-based animations.
//!
//! # Physics Model
//!
//! The spring uses a critically-damped harmonic oscillator model:
//! ```text
//! F = -k * x - c * v
//! ```
//! where:
//! - `k` is the stiffness (spring constant)
//! - `c` is the damping coefficient
//! - `x` is the displacement from target
//! - `v` is the velocity
//!
//! # Example
//!
//! ```
//! use ctui_animate::spring::{SpringAnimation, SpringConfig};
//!
//! let mut spring = SpringAnimation::new(0.0)
//!     .target(100.0)
//!     .config(SpringConfig::BOUNCY);
//!
//! spring.tick(16);
//! let value = spring.current_value();
//! ```

use crate::sequence::AnimationController;

/// Configuration for spring physics.
///
/// Preset configurations provide commonly-used spring behaviors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpringConfig {
    /// Stiffness coefficient (spring constant k).
    /// Higher values = faster, snappier motion.
    pub stiffness: f32,
    /// Damping coefficient (c).
    /// Higher values = less oscillation, more settling.
    pub damping: f32,
    /// Mass of the simulated object.
    /// Higher values = slower response.
    pub mass: f32,
    /// Velocity threshold for considering the spring "settled".
    pub velocity_threshold: f32,
    /// Displacement threshold for considering the spring "settled".
    pub displacement_threshold: f32,
}

impl SpringConfig {
    /// Very bouncy spring with lots of oscillation.
    pub const BOUNCY: Self = Self {
        stiffness: 180.0,
        damping: 12.0,
        mass: 1.0,
        velocity_threshold: 0.01,
        displacement_threshold: 0.01,
    };

    /// Quick, snappy spring with minimal overshoot.
    pub const SNAPPY: Self = Self {
        stiffness: 300.0,
        damping: 30.0,
        mass: 1.0,
        velocity_threshold: 0.01,
        displacement_threshold: 0.01,
    };

    /// Slow, gentle spring.
    pub const GENTLE: Self = Self {
        stiffness: 100.0,
        damping: 15.0,
        mass: 1.0,
        velocity_threshold: 0.01,
        displacement_threshold: 0.01,
    };

    /// Stiff spring that quickly settles.
    pub const STIFF: Self = Self {
        stiffness: 400.0,
        damping: 40.0,
        mass: 1.0,
        velocity_threshold: 0.01,
        displacement_threshold: 0.01,
    };

    /// Creates a custom spring configuration.
    #[must_use]
    pub const fn new(stiffness: f32, damping: f32, mass: f32) -> Self {
        Self {
            stiffness,
            damping,
            mass,
            velocity_threshold: 0.01,
            displacement_threshold: 0.01,
        }
    }

    /// Sets the velocity threshold for settling.
    #[must_use]
    pub const fn velocity_threshold(mut self, threshold: f32) -> Self {
        self.velocity_threshold = threshold;
        self
    }

    /// Sets the displacement threshold for settling.
    #[must_use]
    pub const fn displacement_threshold(mut self, threshold: f32) -> Self {
        self.displacement_threshold = threshold;
        self
    }

    /// Creates a critically damped configuration.
    #[must_use]
    pub fn critically_damped(stiffness: f32) -> Self {
        let damping = 2.0 * (stiffness * 1.0).sqrt();
        Self {
            stiffness,
            damping,
            mass: 1.0,
            velocity_threshold: 0.01,
            displacement_threshold: 0.01,
        }
    }

    /// Creates an overdamped configuration.
    #[must_use]
    pub fn overdamped(stiffness: f32) -> Self {
        let damping = 2.5 * (stiffness * 1.0).sqrt();
        Self {
            stiffness,
            damping,
            mass: 1.0,
            velocity_threshold: 0.01,
            displacement_threshold: 0.01,
        }
    }

    /// Creates an underdamped configuration (will oscillate).
    #[must_use]
    pub fn underdamped(stiffness: f32) -> Self {
        let damping = 0.5 * (stiffness * 1.0).sqrt();
        Self {
            stiffness,
            damping,
            mass: 1.0,
            velocity_threshold: 0.01,
            displacement_threshold: 0.01,
        }
    }
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self::SNAPPY
    }
}

/// A spring animation that simulates physical spring physics.
///
/// Springs produce natural-feeling motion with optional
/// overshoot and oscillation depending on the configuration.
#[derive(Debug, Clone)]
pub struct SpringAnimation {
    /// Current value.
    value: f32,
    /// Target value.
    target: f32,
    /// Current velocity.
    velocity: f32,
    /// Spring configuration.
    config: SpringConfig,
    /// Whether the spring has settled.
    settled: bool,
    /// Whether the spring is paused.
    paused: bool,
    /// Initial value (for reset).
    initial_value: f32,
}

impl SpringAnimation {
    /// Creates a new spring animation starting at the given value.
    #[must_use]
    pub fn new(start: f32) -> Self {
        Self {
            value: start,
            target: start,
            velocity: 0.0,
            config: SpringConfig::default(),
            settled: true,
            paused: false,
            initial_value: start,
        }
    }

    /// Sets the target value.
    #[must_use]
    pub fn target(mut self, target: f32) -> Self {
        self.set_target(target);
        self
    }

    /// Sets the spring configuration.
    #[must_use]
    pub const fn config(mut self, config: SpringConfig) -> Self {
        self.config = config;
        self
    }

    /// Sets the initial velocity.
    #[must_use]
    pub fn velocity(mut self, velocity: f32) -> Self {
        self.velocity = velocity;
        if velocity.abs() > 0.0 {
            self.settled = false;
        }
        self
    }

    /// Updates the target value (animates to new target).
    pub fn set_target(&mut self, target: f32) {
        if (self.target - target).abs() > f32::EPSILON {
            self.target = target;
            self.settled = false;
        }
    }

    /// Returns the current value.
    #[must_use]
    pub const fn current_value(&self) -> f32 {
        self.value
    }

    /// Returns the target value.
    #[must_use]
    pub const fn target_value(&self) -> f32 {
        self.target
    }

    /// Returns the current velocity.
    #[must_use]
    pub const fn current_velocity(&self) -> f32 {
        self.velocity
    }

    /// Returns whether the spring has settled.
    #[must_use]
    pub const fn is_settled(&self) -> bool {
        self.settled
    }

    /// Advances the spring by the given delta time in milliseconds.
    ///
    /// Uses semi-implicit Euler integration for stability.
    /// Returns `true` if the spring just settled this tick.
    #[allow(clippy::cast_precision_loss)]
    pub fn tick(&mut self, delta_ms: u64) -> bool {
        if self.paused || self.settled {
            return false;
        }

        let dt = delta_ms as f32 / 1000.0;

        let displacement = self.value - self.target;

        let spring_force = -self.config.stiffness * displacement;
        let damping_force = -self.config.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / self.config.mass;

        self.velocity = acceleration.mul_add(dt, self.velocity);
        self.value = self.velocity.mul_add(dt, self.value);

        let abs_velocity = self.velocity.abs();
        let abs_displacement = displacement.abs();

        if abs_velocity < self.config.velocity_threshold
            && abs_displacement < self.config.displacement_threshold
        {
            self.value = self.target;
            self.velocity = 0.0;
            self.settled = true;
            return true;
        }

        false
    }

    /// Jumps directly to the target (no animation).
    pub const fn snap_to_target(&mut self) {
        self.value = self.target;
        self.velocity = 0.0;
        self.settled = true;
    }

    /// Resets to the initial value.
    pub const fn reset_to_initial(&mut self) {
        self.value = self.initial_value;
        self.target = self.initial_value;
        self.velocity = 0.0;
        self.settled = true;
    }

    /// Returns the spring configuration.
    #[must_use]
    pub const fn get_config(&self) -> &SpringConfig {
        &self.config
    }
}

impl AnimationController for SpringAnimation {
    fn tick(&mut self, delta_ms: u64) -> bool {
        self.tick(delta_ms)
    }

    fn is_complete(&self) -> bool {
        self.settled
    }

    fn reset(&mut self) {
        self.reset_to_initial();
    }

    fn pause(&mut self) {
        self.paused = true;
    }

    fn resume(&mut self) {
        self.paused = false;
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}

/// Builder for creating spring animations.
#[derive(Debug, Clone)]
pub struct SpringBuilder {
    start: f32,
    target: f32,
    velocity: f32,
    config: SpringConfig,
}

impl SpringBuilder {
    /// Creates a new spring builder.
    #[must_use]
    pub const fn new(start: f32) -> Self {
        Self {
            start,
            target: start,
            velocity: 0.0,
            config: SpringConfig::SNAPPY,
        }
    }

    /// Sets the target value.
    #[must_use]
    pub const fn target(mut self, target: f32) -> Self {
        self.target = target;
        self
    }

    /// Sets the initial velocity.
    #[must_use]
    pub const fn velocity(mut self, velocity: f32) -> Self {
        self.velocity = velocity;
        self
    }

    /// Sets the spring configuration.
    #[must_use]
    pub const fn config(mut self, config: SpringConfig) -> Self {
        self.config = config;
        self
    }

    /// Builds the spring animation.
    #[must_use]
    pub fn build(self) -> SpringAnimation {
        let settled =
            (self.target - self.start).abs() < f32::EPSILON && self.velocity.abs() < f32::EPSILON;
        SpringAnimation {
            value: self.start,
            target: self.target,
            velocity: self.velocity,
            config: self.config,
            settled,
            paused: false,
            initial_value: self.start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_config_new() {
        let config = SpringConfig::new(100.0, 10.0, 1.0);
        approx::assert_relative_eq!(config.stiffness, 100.0);
        approx::assert_relative_eq!(config.damping, 10.0);
        approx::assert_relative_eq!(config.mass, 1.0);
    }

    #[test]
    fn test_spring_config_critically_damped() {
        let config = SpringConfig::critically_damped(100.0);
        // Critical damping: c = 2 * sqrt(k * m)
        approx::assert_relative_eq!(config.damping, 2.0 * 100.0_f32.sqrt(), epsilon = 0.01);
    }

    #[test]
    fn test_spring_config_overdamped() {
        let config = SpringConfig::overdamped(100.0);
        assert!(config.damping > 2.0 * 100.0_f32.sqrt());
    }

    #[test]
    fn test_spring_config_underdamped() {
        let config = SpringConfig::underdamped(100.0);
        assert!(config.damping < 2.0 * 100.0_f32.sqrt());
    }

    #[test]
    fn test_spring_config_default() {
        let config = SpringConfig::default();
        assert_eq!(config, SpringConfig::SNAPPY);
    }

    #[test]
    fn test_spring_new() {
        let spring = SpringAnimation::new(0.0);
        assert_eq!(spring.current_value(), 0.0);
        assert_eq!(spring.target_value(), 0.0);
        assert!(spring.is_settled());
    }

    #[test]
    fn test_spring_target() {
        let spring = SpringAnimation::new(0.0).target(100.0);
        assert_eq!(spring.target_value(), 100.0);
        assert!(!spring.is_settled());
    }

    #[test]
    fn test_spring_tick_moves_toward_target() {
        let mut spring = SpringAnimation::new(0.0)
            .target(100.0)
            .config(SpringConfig::STIFF);

        let initial = spring.current_value();
        spring.tick(16);
        assert!(spring.current_value() > initial);
    }

    #[test]
    fn test_spring_settles() {
        let mut spring = SpringAnimation::new(0.0)
            .target(100.0)
            .config(SpringConfig::STIFF);

        for _ in 0..500 {
            spring.tick(16);
        }

        approx::assert_relative_eq!(spring.current_value(), 100.0, epsilon = 1.0);
        assert!(spring.is_settled());
    }

    #[test]
    fn test_spring_oscillates_when_underdamped() {
        let mut spring = SpringAnimation::new(0.0)
            .target(100.0)
            .config(SpringConfig::underdamped(100.0));

        let mut max_value = 0.0_f32;

        for _ in 0..200 {
            spring.tick(16);
            max_value = max_value.max(spring.current_value());
        }

        assert!(max_value > 100.0);
    }

    #[test]
    fn test_spring_no_overshoot_when_overdamped() {
        let mut spring = SpringAnimation::new(0.0)
            .target(100.0)
            .config(SpringConfig::overdamped(200.0));

        let mut max_value = 0.0_f32;

        for _ in 0..500 {
            spring.tick(16);
            max_value = max_value.max(spring.current_value());
            assert!(max_value <= 100.5);
        }
    }

    #[test]
    fn test_spring_pause_resume() {
        let mut spring = SpringAnimation::new(0.0).target(100.0);

        spring.pause();
        assert!(spring.is_paused());

        spring.tick(16);
        assert_eq!(spring.current_value(), 0.0);

        spring.resume();
        spring.tick(16);
        assert!(spring.current_value() > 0.0);
    }

    #[test]
    fn test_spring_snap_to_target() {
        let mut spring = SpringAnimation::new(0.0).target(100.0);

        spring.snap_to_target();
        assert_eq!(spring.current_value(), 100.0);
        assert!(spring.is_settled());
    }

    #[test]
    fn test_spring_reset_to_initial() {
        let mut spring = SpringAnimation::new(50.0).target(100.0);

        spring.tick(16);
        assert!(spring.current_value() > 50.0);

        spring.reset_to_initial();
        assert_eq!(spring.current_value(), 50.0);
        assert_eq!(spring.target_value(), 50.0);
        assert!(spring.is_settled());
    }

    #[test]
    fn test_spring_set_target_mid_animation() {
        let mut spring = SpringAnimation::new(0.0)
            .target(100.0)
            .config(SpringConfig::STIFF);

        spring.tick(16);

        spring.set_target(200.0);
        assert_eq!(spring.target_value(), 200.0);

        spring.tick(16);
        assert!(spring.current_value() < 200.0);
    }

    #[test]
    fn test_spring_velocity() {
        let spring = SpringAnimation::new(0.0).target(100.0).velocity(50.0);

        approx::assert_relative_eq!(spring.current_velocity(), 50.0);
    }

    #[test]
    fn test_spring_builder() {
        let spring = SpringBuilder::new(0.0)
            .target(100.0)
            .velocity(10.0)
            .config(SpringConfig::BOUNCY)
            .build();

        assert_eq!(spring.current_value(), 0.0);
        assert_eq!(spring.target_value(), 100.0);
        approx::assert_relative_eq!(spring.current_velocity(), 10.0);
    }

    #[test]
    fn test_spring_presets() {
        let bouncy = SpringConfig::BOUNCY;
        assert!(bouncy.stiffness > 0.0);
        assert!(bouncy.damping > 0.0);

        let snappy = SpringConfig::SNAPPY;
        assert!(snappy.stiffness > 0.0);
        assert!(snappy.damping > 0.0);

        let gentle = SpringConfig::GENTLE;
        assert!(gentle.stiffness > 0.0);

        let stiff = SpringConfig::STIFF;
        assert!(stiff.stiffness > 0.0);
    }

    #[test]
    fn test_spring_animation_controller_trait() {
        let mut spring = SpringAnimation::new(0.0).target(100.0);

        assert!(!spring.is_complete());

        spring.pause();
        assert!(spring.is_paused());

        spring.resume();
        assert!(!spring.is_paused());

        spring.reset();
        assert!(spring.is_complete());
        assert_eq!(spring.current_value(), 0.0);
    }

    #[test]
    fn test_spring_backwards() {
        let mut spring = SpringAnimation::new(100.0)
            .target(0.0)
            .config(SpringConfig::STIFF);

        let initial = spring.current_value();
        spring.tick(16);
        assert!(spring.current_value() < initial);

        for _ in 0..500 {
            spring.tick(16);
        }

        approx::assert_relative_eq!(spring.current_value(), 0.0, epsilon = 1.0);
    }

    #[test]
    fn test_spring_negative_values() {
        let mut spring = SpringAnimation::new(-100.0)
            .target(100.0)
            .config(SpringConfig::STIFF);

        for _ in 0..500 {
            spring.tick(16);
        }

        approx::assert_relative_eq!(spring.current_value(), 100.0, epsilon = 1.0);
    }

    #[test]
    fn test_spring_config_thresholds() {
        let config = SpringConfig::new(100.0, 10.0, 1.0)
            .velocity_threshold(0.001)
            .displacement_threshold(0.001);

        approx::assert_relative_eq!(config.velocity_threshold, 0.001);
        approx::assert_relative_eq!(config.displacement_threshold, 0.001);
    }
}
