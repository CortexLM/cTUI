//! Keyframe-based animation system.
//!
//! This module provides keyframe animations that allow defining arbitrary
//! animation curves through a series of keyframe points. The system supports:
//! - Custom value types via the `Interpolator` trait
//! - Multiple playback modes: Once, Loop, Reverse, `PingPong`
//! - Automatic interpolation between keyframes
//!
//! # Example
//!
//! ```
//! use ctui_animate::{Keyframe, KeyframeAnimation, PlaybackMode};
//! use ctui_animate::easing::EasingFunction;
//!
//! // Create a bouncing opacity animation
//! let mut animation = KeyframeAnimation::new()
//!     .keyframe(Keyframe::new(0.0, 0.0))
//!     .keyframe(Keyframe::new(0.5, 1.0))
//!     .keyframe(Keyframe::new(1.0, 0.0))
//!     .duration_ms(1000)
//!     .playback_mode(PlaybackMode::Loop);
//!
//! // Tick the animation
//! animation.tick(500);
//! let value = animation.current_value().unwrap_or(0.0);
//! ```

use crate::interpolate::Interpolator;
use std::time::Duration;

/// A single keyframe in an animation sequence.
///
/// Each keyframe has a normalized time (0.0 to 1.0) and a value.
/// The time represents when in the animation this keyframe occurs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keyframe<T> {
    /// Normalized time position (0.0 = start, 1.0 = end)
    pub time: f32,
    /// The value at this keyframe
    pub value: T,
}

impl<T> Keyframe<T> {
    /// Creates a new keyframe at the given time with the given value.
    ///
    /// # Arguments
    /// * `time` - Normalized time position (0.0 to 1.0)
    /// * `value` - The value at this keyframe
    #[must_use]
    pub const fn new(time: f32, value: T) -> Self {
        Self { time, value }
    }
}

/// Playback mode for keyframe animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub enum PlaybackMode {
    /// Play once and stop at the end
    #[default]
    Once,
    /// Loop from beginning after reaching end
    Loop,
    /// Play in reverse (from end to start)
    Reverse,
    /// Alternate between forward and reverse (ping-pong)
    PingPong,
}


/// A keyframe-based animation that interpolates between keyframes.
///
/// This type allows defining complex animation curves by specifying
/// keyframe values at specific normalized times.
///
/// # Type Parameters
/// * `T` - The value type being animated, must implement `Interpolator + Copy`
#[derive(Debug, Clone, PartialEq)]
pub struct KeyframeAnimation<T>
where
    T: Interpolator + Copy,
{
    /// Keyframes sorted by time
    keyframes: Vec<Keyframe<T>>,
    /// Total duration in milliseconds
    duration_ms: u64,
    /// Elapsed time in milliseconds
    elapsed_ms: u64,
    /// Playback mode
    playback_mode: PlaybackMode,
    /// Number of completed iterations
    iterations: u64,
    /// Maximum iterations (0 = infinite)
    max_iterations: u64,
    /// Whether the animation is paused
    paused: bool,
    /// Time scale multiplier (1.0 = normal speed)
    time_scale: f32,
}

impl<T: Interpolator + Copy> KeyframeAnimation<T> {
    /// Creates a new empty keyframe animation.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            keyframes: Vec::new(),
            duration_ms: 1000,
            elapsed_ms: 0,
            playback_mode: PlaybackMode::Once,
            iterations: 0,
            max_iterations: 0,
            paused: false,
            time_scale: 1.0,
        }
    }

    /// Adds a keyframe to the animation.
    ///
    /// Keyframes are automatically sorted by time.
    #[must_use]
    pub fn keyframe(mut self, keyframe: Keyframe<T>) -> Self {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by(|a, b| {
            a.time
                .partial_cmp(&b.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        self
    }

    /// Sets the total duration of the animation.
    #[must_use]
    pub const fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Sets the duration from a `Duration`.
    #[must_use]
    pub const fn duration(mut self, duration: Duration) -> Self {
        self.duration_ms = duration.as_millis() as u64;
        self
    }

    /// Sets the total duration of the animation (alias for `duration_ms`).
    #[must_use]
    pub const fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Sets the playback mode.
    #[must_use]
    pub const fn playback_mode(mut self, mode: PlaybackMode) -> Self {
        self.playback_mode = mode;
        self
    }

    /// Sets the maximum number of iterations.
    ///
    /// A value of 0 means infinite looping.
    #[must_use]
    pub const fn max_iterations(mut self, iterations: u64) -> Self {
        self.max_iterations = iterations;
        self
    }

    /// Sets the time scale (1.0 = normal, 2.0 = double speed, etc.).
    #[must_use]
    pub const fn time_scale(mut self, scale: f32) -> Self {
        self.time_scale = scale;
        self
    }

    /// Pauses the animation.
    pub const fn pause(&mut self) {
        self.paused = true;
    }

    /// Resumes the animation.
    pub const fn resume(&mut self) {
        self.paused = false;
    }

    /// Returns whether the animation is paused.
    #[must_use]
    pub const fn is_paused(&self) -> bool {
        self.paused
    }

    /// Resets the animation to the beginning.
    pub const fn reset(&mut self) {
        self.elapsed_ms = 0;
        self.iterations = 0;
    }

    /// Returns the normalized progress (0.0 to 1.0).
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn progress(&self) -> f32 {
        if self.duration_ms == 0 {
            return 1.0;
        }
        self.elapsed_ms as f32 / self.duration_ms as f32
    }

    /// Returns the current value of the animation.
    ///
    /// Returns `None` if there are no keyframes.
    #[must_use]
    pub fn current_value(&self) -> Option<T> {
        if self.keyframes.is_empty() {
            return None;
        }

        let t = self.normalized_time();
        Some(self.interpolate_at(t))
    }

    /// Gets the normalized time accounting for playback mode.
    #[must_use]
    fn normalized_time(&self) -> f32 {
        let progress = self.progress().min(1.0);

        match self.playback_mode {
            PlaybackMode::Once | PlaybackMode::Loop => progress,
            PlaybackMode::Reverse => 1.0 - progress,
            PlaybackMode::PingPong => {
                // Calculate where we are in the ping-pong cycle
                // Forward (0->1): elapsed in 0..duration
                // Backward (1->0): elapsed in duration..2*duration
                let cycle_duration = self.duration_ms * 2;
                let cycle_position = self.elapsed_ms % cycle_duration;

                if cycle_position <= self.duration_ms {
                    // Forward phase
                    let progress = if self.duration_ms > 0 {
                        cycle_position as f32 / self.duration_ms as f32
                    } else {
                        1.0
                    };
                    progress.min(1.0)
                } else {
                    // Backward phase
                    let backward_elapsed = cycle_position - self.duration_ms;
                    let progress = if self.duration_ms > 0 {
                        backward_elapsed as f32 / self.duration_ms as f32
                    } else {
                        1.0
                    };
                    (1.0 - progress).max(0.0)
                }
            }
        }
    }

    /// Interpolates a value at the given normalized time.
    #[must_use]
    fn interpolate_at(&self, t: f32) -> T {
        // Handle edge cases
        assert!(!self.keyframes.is_empty(), "Cannot interpolate empty keyframes");

        if self.keyframes.len() == 1 {
            return self.keyframes[0].value;
        }

        // Before first keyframe
        if t <= self.keyframes[0].time {
            return self.keyframes[0].value;
        }

        // After last keyframe
        if t >= self.keyframes.last().unwrap().time {
            return self.keyframes.last().unwrap().value;
        }

        // Find the two keyframes to interpolate between
        for i in 0..self.keyframes.len() - 1 {
            let k1 = &self.keyframes[i];
            let k2 = &self.keyframes[i + 1];

            if t >= k1.time && t <= k2.time {
                let segment_duration = k2.time - k1.time;
                let segment_progress = if segment_duration > 0.0 {
                    (t - k1.time) / segment_duration
                } else {
                    0.5
                };
                return k1.value.interpolate(&k2.value, segment_progress);
            }
        }

        // Fallback to last keyframe
        self.keyframes.last().unwrap().value
    }

    /// Advances the animation by the given delta time in milliseconds.
    ///
    /// Returns `true` if the animation completed (or a cycle completed for looping animations).
    #[allow(clippy::cast_possible_truncation, clippy::cast_lossless)]
    pub fn tick(&mut self, delta_ms: u64) -> bool {
        if self.paused || self.keyframes.is_empty() {
            return false;
        }

        // Don't advance if already complete
        if self.is_complete() {
            return true;
        }

        // Apply time scale
        let scaled_delta = (delta_ms as f64 * f64::from(self.time_scale)) as u64;
        self.elapsed_ms = self.elapsed_ms.saturating_add(scaled_delta);

        // Check for completion
        let completed = self.elapsed_ms >= self.duration_ms;

        if completed {
            self.iterations += 1;

            // Handle looping
            match self.playback_mode {
                PlaybackMode::Once => {
                    self.elapsed_ms = self.duration_ms;
                }
                PlaybackMode::Loop | PlaybackMode::Reverse => {
                    if self.max_iterations == 0 || self.iterations < self.max_iterations {
                        self.elapsed_ms %= self.duration_ms;
                    } else {
                        self.elapsed_ms = self.duration_ms;
                    }
                }
                PlaybackMode::PingPong => {
                    let cycle_duration = self.duration_ms * 2;
                    if self.max_iterations == 0 || self.iterations < self.max_iterations {
                        self.elapsed_ms %= cycle_duration;
                    } else {
                        self.elapsed_ms = self.duration_ms;
                    }
                }
            }
        }

        completed
    }

    /// Returns whether the animation has completed.
    ///
    /// For looping animations, this returns `true` only when max iterations
    /// have been reached (or never if `max_iterations` is 0).
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        match self.playback_mode {
            PlaybackMode::Once => self.elapsed_ms >= self.duration_ms,
            PlaybackMode::Loop | PlaybackMode::Reverse | PlaybackMode::PingPong => {
                if self.max_iterations == 0 {
                    false
                } else {
                    self.iterations >= self.max_iterations
                }
            }
        }
    }

    /// Returns the number of keyframes.
    #[must_use]
    pub const fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }

    /// Returns a reference to the keyframes.
    #[must_use]
    pub fn keyframes(&self) -> &[Keyframe<T>] {
        &self.keyframes
    }

    /// Returns the elapsed time in milliseconds.
    #[must_use]
    pub const fn elapsed_ms(&self) -> u64 {
        self.elapsed_ms
    }

    /// Returns the total duration in milliseconds.
    #[must_use]
    pub const fn get_duration_ms(&self) -> u64 {
        self.duration_ms
    }

    /// Returns the current iteration count.
    #[must_use]
    pub const fn iterations(&self) -> u64 {
        self.iterations
    }

    /// Sets the elapsed time directly (for seeking).
    pub fn set_elapsed_ms(&mut self, elapsed_ms: u64) {
        self.elapsed_ms = elapsed_ms.min(self.duration_ms);
    }

    /// Seeks to a specific normalized time (0.0 to 1.0).
    #[allow(clippy::cast_precision_loss)]
    pub fn seek(&mut self, progress: f32) {
        let progress = progress.clamp(0.0, 1.0);
        self.elapsed_ms = (progress * self.duration_ms as f32) as u64;
    }
}

impl<T: Interpolator + Copy> Default for KeyframeAnimation<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Concrete type aliases for common use cases
/// A keyframe animation for f32 values.
pub type FloatAnimation = KeyframeAnimation<f32>;
/// A keyframe animation for f64 values.
pub type Float64Animation = KeyframeAnimation<f64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframe_new() {
        let kf: Keyframe<f32> = Keyframe::new(0.5, 42.0);
        assert!((kf.time - 0.5).abs() < f32::EPSILON);
        assert!((kf.value - 42.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_animation_new() {
        let anim: FloatAnimation = KeyframeAnimation::new();
        assert_eq!(anim.keyframe_count(), 0);
        assert_eq!(anim.get_duration_ms(), 1000);
        assert!(anim.current_value().is_none());
    }

    #[test]
    fn test_animation_default() {
        let anim: FloatAnimation = KeyframeAnimation::default();
        assert_eq!(anim.keyframe_count(), 0);
    }

    #[test]
    fn test_animation_single_keyframe() {
        let anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 10.0))
            .duration_ms(1000);

        assert_eq!(anim.current_value(), Some(10.0));
    }

    #[test]
    fn test_animation_two_keyframes() {
        let anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(1000);

        // At start
        assert_eq!(anim.current_value(), Some(0.0));

        // At 50%
        let mut anim = anim;
        anim.tick(500);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);

        // At end
        anim.tick(500);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 100.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_multiple_keyframes() {
        let anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(0.5, 100.0))
            .keyframe(Keyframe::new(1.0, 0.0))
            .duration_ms(1000);

        // At start
        assert_eq!(anim.current_value(), Some(0.0));

        // At 25% (halfway to first keyframe)
        let mut anim = anim;
        anim.tick(250);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);

        // At 50% (at second keyframe)
        anim.tick(250);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 100.0, epsilon = 1e-6);

        // At 75% (halfway between second and third)
        anim.tick(250);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_reverse_mode() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(1000)
            .playback_mode(PlaybackMode::Reverse)
            .max_iterations(1);

        // At start in reverse mode = 100
        approx::assert_relative_eq!(anim.current_value().unwrap(), 100.0, epsilon = 1e-6);

        // At 50% in reverse mode = 50
        anim.tick(500);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);

        // At end in reverse mode = 0
        anim.tick(500);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_loop_mode() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100)
            .playback_mode(PlaybackMode::Loop);

        // Play through once
        anim.tick(100);
        assert_eq!(anim.iterations(), 1);

        // Should loop back
        anim.tick(50);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_ping_pong_mode() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100)
            .playback_mode(PlaybackMode::PingPong);

        // Forward pass
        anim.tick(50);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);

        anim.tick(50);
        assert_eq!(anim.iterations(), 1);

        // Backward pass (elapsed now in backward phase due to modulo)
        anim.tick(50);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_max_iterations() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100)
            .playback_mode(PlaybackMode::Loop)
            .max_iterations(2);

        // First iteration
        let completed = anim.tick(100);
        assert!(completed);
        assert!(!anim.is_complete());

        // Second iteration
        anim.tick(100);
        assert!(anim.is_complete());

        // Should not loop after max iterations
        anim.tick(100);
        assert_eq!(anim.iterations(), 2);
    }

    #[test]
    fn test_animation_pause() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100);

        anim.pause();
        assert!(anim.is_paused());

        anim.tick(50);
        // Paused animation should not advance
        assert_eq!(anim.elapsed_ms(), 0);

        anim.resume();
        anim.tick(50);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_time_scale() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100)
            .time_scale(2.0);

        // Double speed = reaches end in half time
        anim.tick(50);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 100.0, epsilon = 1e-6);

        // Reset and try half speed
        anim.reset();
        anim = anim.time_scale(0.5);
        anim.tick(50);
        // Half speed = only 25% progress after 50ms
        approx::assert_relative_eq!(anim.current_value().unwrap(), 25.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_seek() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100);

        anim.seek(0.5);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 50.0, epsilon = 1e-6);

        anim.seek(1.0);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 100.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_reset() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100);

        anim.tick(75);
        assert!(anim.iterations() > 0 || anim.elapsed_ms() > 0);

        anim.reset();
        assert_eq!(anim.elapsed_ms(), 0);
        assert_eq!(anim.iterations(), 0);
        assert_eq!(anim.current_value(), Some(0.0));
    }

    #[test]
    fn test_animation_zero_duration() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(0);

        // Zero duration should immediately complete
        assert_eq!(anim.progress(), 1.0);
    }

    #[test]
    fn test_animation_unsorted_keyframes() {
        // Keyframes should be automatically sorted
        let anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.5, 50.0))
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100);

        assert_eq!(anim.keyframes()[0].time, 0.0);
        assert_eq!(anim.keyframes()[1].time, 0.5);
        assert_eq!(anim.keyframes()[2].time, 1.0);
    }

    #[test]
    fn test_animation_before_first_keyframe() {
        // If animation starts after 0, value before first keyframe is first keyframe
        let anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.25, 25.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100);

        assert_eq!(anim.current_value(), Some(25.0));
    }

    #[test]
    fn test_animation_progress_clamping() {
        let mut anim: FloatAnimation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, 0.0))
            .keyframe(Keyframe::new(1.0, 100.0))
            .duration_ms(100);

        // Tick beyond duration
        anim.tick(200);
        approx::assert_relative_eq!(anim.current_value().unwrap(), 100.0, epsilon = 1e-6);
    }

    #[test]
    fn test_playback_mode_default() {
        assert_eq!(PlaybackMode::default(), PlaybackMode::Once);
    }
}
