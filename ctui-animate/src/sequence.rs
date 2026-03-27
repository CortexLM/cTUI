//! Animation sequencing for running animations in series or parallel.
//!
//! This module provides:
//! - `AnimationSequence`: Run animations in series (one after another)
//! - `AnimationGroup`: Run animations in parallel (all at once)
//!
//! # Example
//!
//! ```
//! use ctui_animate::{AnimationSequence, AnimationGroup, AnimationController};
//!
//! # struct MyAnimation;
//! # impl MyAnimation { fn new() -> Self { Self } }
//! # impl AnimationController for MyAnimation {
//! #     fn tick(&mut self, _delta_ms: u64) -> bool { false }
//! #     fn is_complete(&self) -> bool { false }
//! #     fn reset(&mut self) {}
//! #     fn pause(&mut self) {}
//! #     fn resume(&mut self) {}
//! #     fn is_paused(&self) -> bool { false }
//! # }
//! // Create a sequence
//! let mut sequence = AnimationSequence::new()
//!     .delay(100)
//!     .animation(MyAnimation::new())
//!     .delay(200);
//!
//! // Create a group (both animations must be the same type)
//! let mut group = AnimationGroup::new()
//!     .add(MyAnimation::new())
//!     .add(MyAnimation::new());
//! ```

use std::collections::VecDeque;

/// Unique identifier for an animation in a sequence or group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub struct SequenceId(pub u64);


/// Trait for controllable animations.
///
/// Types implementing this trait can be used in sequences and groups.
pub trait AnimationController {
    /// Advances the animation by the given delta time in milliseconds.
    /// Returns `true` if the animation completed this tick.
    fn tick(&mut self, delta_ms: u64) -> bool;

    /// Returns whether the animation has completed.
    fn is_complete(&self) -> bool;

    /// Resets the animation to its initial state.
    fn reset(&mut self);

    /// Pauses the animation.
    fn pause(&mut self);

    /// Resumes the animation.
    fn resume(&mut self);

    /// Returns whether the animation is paused.
    fn is_paused(&self) -> bool;
}

/// An item in an animation sequence.
#[derive(Debug)]
enum SequenceItem<T: AnimationController> {
    /// A delay before the next animation.
    Delay { remaining_ms: u64 },
    /// An animation to run.
    Animation { animation: T },
}

/// A sequence of animations that run one after another.
///
/// Animations in a sequence run serially - each animation must complete
/// before the next one starts. Delays can be inserted between animations.
pub struct AnimationSequence<T: AnimationController> {
    items: VecDeque<SequenceItem<T>>,
    current: Option<T>,
    delay_remaining_ms: u64,
    paused: bool,
    completed: bool,
    on_complete: Option<Box<dyn FnMut() + Send + Sync>>,
    next_id: u64,
}

impl<T: AnimationController> AnimationSequence<T> {
    /// Creates a new empty animation sequence.
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
            current: None,
            delay_remaining_ms: 0,
            paused: false,
            completed: false,
            on_complete: None,
            next_id: 0,
        }
    }

    /// Adds a delay before the next animation (in milliseconds).
    #[must_use]
    pub fn delay(mut self, ms: u64) -> Self {
        self.items
            .push_back(SequenceItem::Delay { remaining_ms: ms });
        self
    }

    /// Adds an animation to the sequence.
    #[must_use]
    pub fn animation(mut self, animation: T) -> Self {
        self.items.push_back(SequenceItem::Animation { animation });
        self
    }

    /// Sets a callback to run when the sequence completes.
    pub fn on_complete<F: FnMut() + Send + Sync + 'static>(mut self, callback: F) -> Self {
        self.on_complete = Some(Box::new(callback));
        self
    }

    /// Advances the sequence by the given delta time.
    ///
    /// Returns `true` if the entire sequence completed.
    pub fn tick(&mut self, delta_ms: u64) -> bool {
        if self.paused || self.completed {
            return self.completed;
        }

        let mut remaining = delta_ms;

        loop {
            // Handle any active delay
            if self.delay_remaining_ms > 0 {
                if remaining >= self.delay_remaining_ms {
                    remaining -= self.delay_remaining_ms;
                    self.delay_remaining_ms = 0;
                } else {
                    self.delay_remaining_ms -= remaining;
                    return false;
                }
            }

            // Handle current animation
            if let Some(ref mut animation) = self.current {
                if animation.is_complete() {
                    self.current = None;
                } else {
                    let completed = animation.tick(remaining);
                    if completed && animation.is_complete() {
                        self.current = None;
                        remaining = 0;
                    } else {
                        return false;
                    }
                }
            }

            // Get next item
            match self.items.pop_front() {
                Some(SequenceItem::Delay { remaining_ms }) => {
                    self.delay_remaining_ms = remaining_ms;
                }
                Some(SequenceItem::Animation { animation }) => {
                    self.current = Some(animation);
                }
                None => {
                    self.completed = true;
                    if let Some(ref mut callback) = self.on_complete {
                        callback();
                    }
                    return true;
                }
            }

            if remaining == 0 {
                return false;
            }
        }
    }

    /// Returns whether the sequence has completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.completed
    }

    /// Pauses the sequence.
    pub fn pause(&mut self) {
        self.paused = true;
        if let Some(ref mut animation) = self.current {
            animation.pause();
        }
    }

    /// Resumes the sequence.
    pub fn resume(&mut self) {
        self.paused = false;
        if let Some(ref mut animation) = self.current {
            animation.resume();
        }
    }

    /// Returns whether the sequence is paused.
    #[must_use]
    pub const fn is_paused(&self) -> bool {
        self.paused
    }

    /// Resets the sequence to the beginning.
    pub fn reset(&mut self) {
        self.current = None;
        self.delay_remaining_ms = 0;
        self.completed = false;
        self.paused = false;
    }

    /// Returns a reference to the current animation.
    #[must_use]
    pub const fn current(&self) -> Option<&T> {
        self.current.as_ref()
    }

    /// Returns a mutable reference to the current animation.
    #[must_use]
    pub const fn current_mut(&mut self) -> Option<&mut T> {
        self.current.as_mut()
    }

    /// Returns the number of remaining items in the queue.
    #[must_use]
    pub fn remaining_count(&self) -> usize {
        self.items.len() + usize::from(self.current.is_some())
    }
}

impl<T: AnimationController> Default for AnimationSequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A group of animations that run in parallel.
///
/// All animations in a group start simultaneously and run until
/// all have completed (or until `wait_for_all` is disabled).
pub struct AnimationGroup<T: AnimationController> {
    animations: Vec<(SequenceId, T, bool)>,
    wait_for_all: bool,
    paused: bool,
    next_id: u64,
    on_complete: Option<Box<dyn FnMut() + Send + Sync>>,
}

impl<T: AnimationController> AnimationGroup<T> {
    /// Creates a new empty animation group.
    #[must_use]
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            wait_for_all: true,
            paused: false,
            next_id: 0,
            on_complete: None,
        }
    }

    /// Adds an animation to the group.
    #[must_use]
    pub fn add(mut self, animation: T) -> Self {
        let id = SequenceId(self.next_id);
        self.next_id += 1;
        self.animations.push((id, animation, false));
        self
    }

    /// Sets whether to wait for all animations to complete.
    ///
    /// If `false`, the group completes when any animation completes.
    #[must_use]
    pub const fn wait_for_all(mut self, wait: bool) -> Self {
        self.wait_for_all = wait;
        self
    }

    /// Sets a callback to run when the group completes.
    pub fn on_complete<F: FnMut() + Send + Sync + 'static>(mut self, callback: F) -> Self {
        self.on_complete = Some(Box::new(callback));
        self
    }

    /// Advances all animations by the given delta time.
    ///
    /// Returns `true` if the group completed (based on `wait_for_all` setting).
    pub fn tick(&mut self, delta_ms: u64) -> bool {
        if self.paused {
            return self.is_complete();
        }

        let mut any_completed = false;
        let mut all_completed = true;

        for (_, animation, completed) in &mut self.animations {
            if !*completed {
                if animation.tick(delta_ms) || animation.is_complete() {
                    *completed = true;
                    any_completed = true;
                } else {
                    all_completed = false;
                }
            }
        }

        let group_complete = if self.wait_for_all {
            all_completed && !self.animations.is_empty()
        } else {
            any_completed
        };

        if group_complete {
            if let Some(ref mut callback) = self.on_complete {
                callback();
            }
        }

        group_complete
    }

    /// Returns whether the group has completed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        if self.animations.is_empty() {
            return true;
        }

        if self.wait_for_all {
            self.animations.iter().all(|(_, _, completed)| *completed)
        } else {
            self.animations.iter().any(|(_, _, completed)| *completed)
        }
    }

    /// Pauses all animations in the group.
    pub fn pause(&mut self) {
        self.paused = true;
        for (_, animation, _) in &mut self.animations {
            animation.pause();
        }
    }

    /// Resumes all animations in the group.
    pub fn resume(&mut self) {
        self.paused = false;
        for (_, animation, _) in &mut self.animations {
            animation.resume();
        }
    }

    /// Returns whether the group is paused.
    #[must_use]
    pub const fn is_paused(&self) -> bool {
        self.paused
    }

    /// Resets all animations in the group.
    pub fn reset(&mut self) {
        self.paused = false;
        for (_, animation, completed) in &mut self.animations {
            animation.reset();
            *completed = false;
        }
    }

    /// Returns the number of animations in the group.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.animations.len()
    }

    /// Returns whether the group is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.animations.is_empty()
    }

    /// Returns an iterator over the animations.
    pub fn animations(&self) -> impl Iterator<Item = &T> {
        self.animations.iter().map(|(_, a, _)| a)
    }

    /// Returns a mutable iterator over the animations.
    pub fn animations_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.animations.iter_mut().map(|(_, a, _)| a)
    }

    /// Returns the number of completed animations.
    #[must_use]
    pub fn completed_count(&self) -> usize {
        self.animations.iter().filter(|(_, _, c)| *c).count()
    }

    /// Returns the number of active (incomplete) animations.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.animations.iter().filter(|(_, _, c)| !*c).count()
    }
}

impl<T: AnimationController> Default for AnimationGroup<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAnimation {
        duration_ms: u64,
        elapsed_ms: u64,
        paused: bool,
        complete: bool,
    }

    impl MockAnimation {
        fn new(duration_ms: u64) -> Self {
            Self {
                duration_ms,
                elapsed_ms: 0,
                paused: false,
                complete: false,
            }
        }
    }

    impl AnimationController for MockAnimation {
        fn tick(&mut self, delta_ms: u64) -> bool {
            if self.paused || self.complete {
                return self.complete;
            }
            self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);
            if self.elapsed_ms >= self.duration_ms {
                self.complete = true;
            }
            self.complete
        }

        fn is_complete(&self) -> bool {
            self.complete
        }

        fn reset(&mut self) {
            self.elapsed_ms = 0;
            self.paused = false;
            self.complete = false;
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

    #[test]
    fn test_sequence_new() {
        let sequence: AnimationSequence<MockAnimation> = AnimationSequence::new();
        assert!(!sequence.is_complete());
        assert_eq!(sequence.remaining_count(), 0);
    }

    #[test]
    fn test_sequence_default() {
        let sequence: AnimationSequence<MockAnimation> = AnimationSequence::default();
        assert!(!sequence.is_complete());
    }

    #[test]
    fn test_sequence_single_animation() {
        let mut sequence = AnimationSequence::new().animation(MockAnimation::new(100));

        assert_eq!(sequence.remaining_count(), 1);

        let completed = sequence.tick(50);
        assert!(!completed);
        assert!(!sequence.is_complete());

        let completed = sequence.tick(50);
        assert!(completed);
        assert!(sequence.is_complete());
    }

    #[test]
    fn test_sequence_multiple_animations() {
        let mut sequence = AnimationSequence::new()
            .animation(MockAnimation::new(100))
            .animation(MockAnimation::new(100));

        assert_eq!(sequence.remaining_count(), 2);

        sequence.tick(100);
        assert!(!sequence.is_complete());
        assert_eq!(sequence.remaining_count(), 1);

        sequence.tick(100);
        assert!(sequence.is_complete());
        assert_eq!(sequence.remaining_count(), 0);
    }

    #[test]
    fn test_sequence_with_delay() {
        let mut sequence = AnimationSequence::new()
            .delay(50)
            .animation(MockAnimation::new(100));

        sequence.tick(25);
        assert!(sequence.current().is_none());

        sequence.tick(25);
        assert!(sequence.current().is_some());

        sequence.tick(100);
        assert!(sequence.is_complete());
    }

    #[test]
    fn test_sequence_pause_resume() {
        let mut sequence = AnimationSequence::new().animation(MockAnimation::new(100));

        sequence.tick(25);

        sequence.pause();
        assert!(sequence.is_paused());

        sequence.tick(25);
        assert_eq!(sequence.current().unwrap().elapsed_ms, 25);

        sequence.resume();
        assert!(!sequence.is_paused());

        sequence.tick(25);
        assert_eq!(sequence.current().unwrap().elapsed_ms, 50);
    }

    #[test]
    fn test_sequence_reset() {
        let mut sequence = AnimationSequence::new().animation(MockAnimation::new(100));

        sequence.tick(100);
        assert!(sequence.is_complete());

        sequence.reset();
        assert!(!sequence.is_complete());
    }

    #[test]
    fn test_group_new() {
        let group: AnimationGroup<MockAnimation> = AnimationGroup::new();
        assert!(group.is_empty());
        assert!(group.is_complete());
    }

    #[test]
    fn test_group_default() {
        let group: AnimationGroup<MockAnimation> = AnimationGroup::default();
        assert!(group.is_empty());
    }

    #[test]
    fn test_group_single_animation() {
        let mut group = AnimationGroup::new().add(MockAnimation::new(100));

        assert_eq!(group.len(), 1);
        assert!(!group.is_complete());

        group.tick(100);
        assert!(group.is_complete());
    }

    #[test]
    fn test_group_multiple_animations() {
        let mut group = AnimationGroup::new()
            .add(MockAnimation::new(100))
            .add(MockAnimation::new(200));

        assert_eq!(group.len(), 2);

        group.tick(100);
        assert!(!group.is_complete());
        assert_eq!(group.completed_count(), 1);
        assert_eq!(group.active_count(), 1);

        group.tick(100);
        assert!(group.is_complete());
        assert_eq!(group.completed_count(), 2);
    }

    #[test]
    fn test_group_wait_for_all_false() {
        let mut group = AnimationGroup::new()
            .wait_for_all(false)
            .add(MockAnimation::new(100))
            .add(MockAnimation::new(200));

        group.tick(100);
        assert!(group.is_complete());
    }

    #[test]
    fn test_group_pause_resume() {
        let mut group = AnimationGroup::new().add(MockAnimation::new(100));

        group.pause();
        assert!(group.is_paused());

        group.tick(50);
        assert_eq!(group.completed_count(), 0);

        group.resume();
        group.tick(100);
        assert!(group.is_complete());
    }

    #[test]
    fn test_group_reset() {
        let mut group = AnimationGroup::new().add(MockAnimation::new(100));

        group.tick(100);
        assert!(group.is_complete());

        group.reset();
        assert!(!group.is_complete());
        assert_eq!(group.completed_count(), 0);
    }

    #[test]
    fn test_sequence_on_complete() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let mut sequence = AnimationSequence::new()
            .animation(MockAnimation::new(100))
            .on_complete(move || {
                called_clone.store(true, Ordering::SeqCst);
            });

        sequence.tick(100);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_group_on_complete() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let mut group = AnimationGroup::new()
            .add(MockAnimation::new(100))
            .on_complete(move || {
                called_clone.store(true, Ordering::SeqCst);
            });

        group.tick(100);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_sequence_current_mut() {
        let mut sequence = AnimationSequence::new().animation(MockAnimation::new(100));

        // Need to tick to start the animation
        sequence.tick(0);

        if let Some(anim) = sequence.current_mut() {
            anim.elapsed_ms = 50;
        }

        assert_eq!(sequence.current().unwrap().elapsed_ms, 50);
    }

    #[test]
    fn test_group_animations_iterator() {
        let group = AnimationGroup::new()
            .add(MockAnimation::new(100))
            .add(MockAnimation::new(200));

        let durations: Vec<_> = group.animations().map(|a| a.duration_ms).collect();
        assert_eq!(durations, vec![100, 200]);
    }

    #[test]
    fn test_sequence_id_default() {
        let id = SequenceId::default();
        assert_eq!(id, SequenceId(0));
    }
}
