//! Animation scheduler for managing concurrent animations
//!
//! This module provides the core animation infrastructure for cTUI:
//! - `AnimationId`: Unique identifier for tracking animations
//! - `Animation`: Represents a single active animation
//! - `AnimationScheduler`: Manages multiple concurrent animations
//!
//! # Example
//!
//! ```
//! use ctui_animate::{AnimationScheduler, EasingFunction};
//!
//! let mut scheduler = AnimationScheduler::new();
//!
//! let id = scheduler.spawn(1000, EasingFunction::Linear);
//! let progress = scheduler.tick(16);
//! scheduler.cancel(id);
//! ```

use crate::easing::EasingFunction;
use std::collections::HashMap;

/// Unique identifier for an active animation.
///
/// Uses a monotonically increasing counter rather than UUIDs for efficiency.
/// IDs are unique within an `AnimationScheduler` instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationId(pub u64);

/// A single active animation.
///
/// Tracks the timing and easing configuration for one animation instance.
/// The scheduler updates `elapsed_ms` on each tick.
#[derive(Debug, Clone)]
pub struct Animation {
    /// Unique identifier for this animation
    pub id: AnimationId,
    /// Total duration of the animation in milliseconds
    pub duration_ms: u64,
    /// Time elapsed since the animation started in milliseconds
    pub elapsed_ms: u64,
    /// Easing function to apply to the animation
    pub easing: EasingFunction,
}

impl Animation {
    /// Creates a new animation with the given parameters.
    #[must_use]
    pub const fn new(id: AnimationId, duration_ms: u64, easing: EasingFunction) -> Self {
        Self {
            id,
            duration_ms,
            elapsed_ms: 0,
            easing,
        }
    }

    /// Returns the raw progress of the animation (0.0 to 1.0).
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn progress(&self) -> f32 {
        if self.duration_ms == 0 {
            return 1.0;
        }
        let progress = self.elapsed_ms as f32 / self.duration_ms as f32;
        progress.min(1.0)
    }

    /// Returns the eased progress of the animation.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_lossless)]
    pub fn eased_progress(&self) -> f32 {
        self.easing.eval(f64::from(self.progress())) as f32
    }

    /// Returns true if the animation has completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.elapsed_ms >= self.duration_ms
    }

    /// Advances the animation by the given delta time in milliseconds.
    pub const fn advance(&mut self, delta_ms: u64) {
        self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);
    }
}

/// Scheduler for managing multiple concurrent animations.
///
/// The scheduler handles spawning, ticking, and canceling animations.
/// On each tick, it advances all active animations and returns their progress.
#[derive(Debug, Clone)]
pub struct AnimationScheduler {
    animations: HashMap<AnimationId, Animation>,
    next_id: u64,
}

impl AnimationScheduler {
    /// Creates a new empty animation scheduler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            next_id: 0,
        }
    }

    /// Spawns a new animation and returns its ID.
    #[must_use]
    pub fn spawn(&mut self, duration_ms: u64, easing: EasingFunction) -> AnimationId {
        let id = AnimationId(self.next_id);
        self.next_id += 1;

        let animation = Animation::new(id, duration_ms, easing);
        self.animations.insert(id, animation);

        id
    }

    /// Advances all active animations by the given delta time.
    ///
    /// Returns a vector of (`AnimationId`, progress) tuples for all active
    /// animations. Completed animations are automatically removed.
    #[must_use]
    #[allow(clippy::explicit_iter_loop)]
    pub fn tick(&mut self, delta_ms: u64) -> Vec<(AnimationId, f32)> {
        let mut results: Vec<(AnimationId, f32)> = Vec::with_capacity(self.animations.len());
        let mut completed: Vec<AnimationId> = Vec::new();

        for (id, animation) in self.animations.iter_mut() {
            animation.advance(delta_ms);

            if animation.is_complete() {
                results.push((*id, 1.0));
                completed.push(*id);
            } else {
                results.push((*id, animation.eased_progress()));
            }
        }

        for id in completed {
            self.animations.remove(&id);
        }

        results
    }

    /// Cancels an animation by its ID.
    pub fn cancel(&mut self, id: AnimationId) {
        self.animations.remove(&id);
    }

    /// Returns true if an animation is currently active.
    #[must_use]
    pub fn is_active(&self, id: AnimationId) -> bool {
        self.animations.contains_key(&id)
    }

    /// Returns the number of active animations.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.animations.len()
    }

    /// Returns an iterator over all active animation IDs.
    pub fn active_ids(&self) -> impl Iterator<Item = AnimationId> + '_ {
        self.animations.keys().copied()
    }

    /// Clears all active animations.
    pub fn clear(&mut self) {
        self.animations.clear();
    }
}

impl Default for AnimationScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_id_equality() {
        let id1 = AnimationId(1);
        let id2 = AnimationId(1);
        let id3 = AnimationId(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_animation_new() {
        let id = AnimationId(1);
        let animation = Animation::new(id, 1000, EasingFunction::Linear);

        assert_eq!(animation.id, id);
        assert_eq!(animation.duration_ms, 1000);
        assert_eq!(animation.elapsed_ms, 0);
        assert_eq!(animation.easing, EasingFunction::Linear);
    }

    #[test]
    fn test_animation_progress_zero() {
        let animation = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
        approx::assert_relative_eq!(animation.progress(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_progress_half() {
        let mut animation = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
        animation.elapsed_ms = 500;
        approx::assert_relative_eq!(animation.progress(), 0.5, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_progress_complete() {
        let mut animation = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
        animation.elapsed_ms = 1000;
        approx::assert_relative_eq!(animation.progress(), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_progress_exceeds_duration() {
        let mut animation = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
        animation.elapsed_ms = 2000;
        approx::assert_relative_eq!(animation.progress(), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_progress_zero_duration() {
        let animation = Animation::new(AnimationId(1), 0, EasingFunction::Linear);
        approx::assert_relative_eq!(animation.progress(), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_is_complete() {
        let mut animation = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
        assert!(!animation.is_complete());

        animation.elapsed_ms = 500;
        assert!(!animation.is_complete());

        animation.elapsed_ms = 1000;
        assert!(animation.is_complete());

        animation.elapsed_ms = 1500;
        assert!(animation.is_complete());
    }

    #[test]
    fn test_animation_advance() {
        let mut animation = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);

        animation.advance(250);
        assert_eq!(animation.elapsed_ms, 250);

        animation.advance(250);
        assert_eq!(animation.elapsed_ms, 500);
    }

    #[test]
    fn test_animation_advance_overflow() {
        let mut animation = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
        animation.elapsed_ms = u64::MAX - 100;
        animation.advance(200);
        assert_eq!(animation.elapsed_ms, u64::MAX);
    }

    #[test]
    fn test_scheduler_new() {
        let scheduler = AnimationScheduler::new();
        assert_eq!(scheduler.active_count(), 0);
    }

    #[test]
    fn test_scheduler_default() {
        let scheduler = AnimationScheduler::default();
        assert_eq!(scheduler.active_count(), 0);
    }

    #[test]
    fn test_scheduler_spawn() {
        let mut scheduler = AnimationScheduler::new();

        let id1 = scheduler.spawn(1000, EasingFunction::Linear);
        assert_eq!(scheduler.active_count(), 1);
        assert!(scheduler.is_active(id1));

        let id2 = scheduler.spawn(500, EasingFunction::QuadOut);
        assert_eq!(scheduler.active_count(), 2);
        assert!(scheduler.is_active(id2));

        assert_ne!(id1, id2);
    }

    #[test]
    fn test_scheduler_spawn_sequential_ids() {
        let mut scheduler = AnimationScheduler::new();

        let id1 = scheduler.spawn(1000, EasingFunction::Linear);
        let id2 = scheduler.spawn(1000, EasingFunction::Linear);
        let id3 = scheduler.spawn(1000, EasingFunction::Linear);

        assert_eq!(id1, AnimationId(0));
        assert_eq!(id2, AnimationId(1));
        assert_eq!(id3, AnimationId(2));
    }

    #[test]
    fn test_scheduler_tick_partial() {
        let mut scheduler = AnimationScheduler::new();
        let id = scheduler.spawn(1000, EasingFunction::Linear);

        let updates = scheduler.tick(250);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].0, id);
        approx::assert_relative_eq!(updates[0].1, 0.25, epsilon = 1e-6);

        assert!(scheduler.is_active(id));
        assert_eq!(scheduler.active_count(), 1);
    }

    #[test]
    fn test_scheduler_tick_complete() {
        let mut scheduler = AnimationScheduler::new();
        let id = scheduler.spawn(500, EasingFunction::Linear);

        let updates = scheduler.tick(500);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].0, id);
        approx::assert_relative_eq!(updates[0].1, 1.0, epsilon = 1e-6);

        assert!(!scheduler.is_active(id));
        assert_eq!(scheduler.active_count(), 0);
    }

    #[test]
    fn test_scheduler_tick_exceeds_duration() {
        let mut scheduler = AnimationScheduler::new();
        let id = scheduler.spawn(500, EasingFunction::Linear);

        let updates = scheduler.tick(1000);
        assert_eq!(updates.len(), 1);
        approx::assert_relative_eq!(updates[0].1, 1.0, epsilon = 1e-6);

        assert!(!scheduler.is_active(id));
    }

    #[test]
    fn test_scheduler_tick_multiple_animations() {
        let mut scheduler = AnimationScheduler::new();

        let id1 = scheduler.spawn(1000, EasingFunction::Linear);
        let id2 = scheduler.spawn(500, EasingFunction::Linear);
        let id3 = scheduler.spawn(250, EasingFunction::Linear);

        let mut updates = scheduler.tick(500);

        assert_eq!(updates.len(), 3);

        updates.sort_by_key(|(id, _)| id.0);

        let id1_update = updates.iter().find(|(id, _)| *id == id1).unwrap();
        approx::assert_relative_eq!(id1_update.1, 0.5, epsilon = 1e-6);
        assert!(scheduler.is_active(id1));

        assert!(!scheduler.is_active(id2));
        assert!(!scheduler.is_active(id3));
        assert_eq!(scheduler.active_count(), 1);
    }

    #[test]
    fn test_scheduler_cancel() {
        let mut scheduler = AnimationScheduler::new();

        let id1 = scheduler.spawn(1000, EasingFunction::Linear);
        let id2 = scheduler.spawn(1000, EasingFunction::Linear);

        assert_eq!(scheduler.active_count(), 2);

        scheduler.cancel(id1);

        assert!(!scheduler.is_active(id1));
        assert!(scheduler.is_active(id2));
        assert_eq!(scheduler.active_count(), 1);
    }

    #[test]
    fn test_scheduler_cancel_non_existent() {
        let mut scheduler = AnimationScheduler::new();

        scheduler.cancel(AnimationId(999));

        assert_eq!(scheduler.active_count(), 0);
    }

    #[test]
    fn test_scheduler_cancel_twice() {
        let mut scheduler = AnimationScheduler::new();
        let id = scheduler.spawn(1000, EasingFunction::Linear);

        scheduler.cancel(id);
        assert!(!scheduler.is_active(id));

        scheduler.cancel(id);
        assert!(!scheduler.is_active(id));
    }

    #[test]
    fn test_scheduler_is_active() {
        let mut scheduler = AnimationScheduler::new();

        let id = scheduler.spawn(1000, EasingFunction::Linear);
        assert!(scheduler.is_active(id));

        assert!(!scheduler.is_active(AnimationId(999)));

        let _ = scheduler.tick(1000);
        assert!(!scheduler.is_active(id));
    }

    #[test]
    fn test_scheduler_active_ids() {
        let mut scheduler = AnimationScheduler::new();

        let id1 = scheduler.spawn(1000, EasingFunction::Linear);
        let id2 = scheduler.spawn(1000, EasingFunction::Linear);
        let id3 = scheduler.spawn(1000, EasingFunction::Linear);

        let ids: Vec<_> = scheduler.active_ids().collect();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
        assert!(ids.contains(&id3));
    }

    #[test]
    fn test_scheduler_clear() {
        let mut scheduler = AnimationScheduler::new();

        let _ = scheduler.spawn(1000, EasingFunction::Linear);
        let _ = scheduler.spawn(1000, EasingFunction::Linear);
        let _ = scheduler.spawn(1000, EasingFunction::Linear);

        assert_eq!(scheduler.active_count(), 3);

        scheduler.clear();

        assert_eq!(scheduler.active_count(), 0);
    }

    #[test]
    fn test_scheduler_sequential_ticks() {
        let mut scheduler = AnimationScheduler::new();
        let id = scheduler.spawn(1000, EasingFunction::Linear);

        let updates1 = scheduler.tick(200);
        approx::assert_relative_eq!(updates1[0].1, 0.2, epsilon = 1e-6);

        let updates2 = scheduler.tick(200);
        approx::assert_relative_eq!(updates2[0].1, 0.4, epsilon = 1e-6);

        let updates3 = scheduler.tick(200);
        approx::assert_relative_eq!(updates3[0].1, 0.6, epsilon = 1e-6);

        let updates4 = scheduler.tick(200);
        approx::assert_relative_eq!(updates4[0].1, 0.8, epsilon = 1e-6);

        let updates5 = scheduler.tick(200);
        approx::assert_relative_eq!(updates5[0].1, 1.0, epsilon = 1e-6);

        assert!(!scheduler.is_active(id));
    }

    #[test]
    fn test_scheduler_tick_empty() {
        let mut scheduler = AnimationScheduler::new();

        let updates = scheduler.tick(100);
        assert!(updates.is_empty());
    }

    #[test]
    fn test_animation_eased_progress_with_easing() {
        let animation = Animation::new(AnimationId(1), 1000, EasingFunction::QuadOut);

        // At 0% progress, QuadOut should return 0.0
        let mut anim = animation.clone();
        anim.elapsed_ms = 0;
        approx::assert_relative_eq!(anim.eased_progress(), 0.0, epsilon = 1e-6);

        // At 50% progress, QuadOut should return ~0.75
        anim.elapsed_ms = 500;
        approx::assert_relative_eq!(anim.eased_progress(), 0.75, epsilon = 1e-6);

        // At 100% progress, should return 1.0
        anim.elapsed_ms = 1000;
        approx::assert_relative_eq!(anim.eased_progress(), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_animation_clone() {
        let anim1 = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
        let anim2 = anim1.clone();

        assert_eq!(anim1.id, anim2.id);
        assert_eq!(anim1.duration_ms, anim2.duration_ms);
    }

    #[test]
    fn test_scheduler_clone() {
        let mut scheduler = AnimationScheduler::new();
        let _ = scheduler.spawn(1000, EasingFunction::Linear);

        let scheduler2 = scheduler.clone();
        assert_eq!(scheduler2.active_count(), 1);
    }
}
