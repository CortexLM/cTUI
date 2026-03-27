//! Global animation coordinator for managing all animations.
//!
//! The `AnimationManager` provides a central point for controlling
//! all animations in an application:
//! - Pause/resume all animations
//! - Time scale (slow motion, fast forward)
//! - Track animation statistics
//!
//! # Example
//!
//! ```
//! use ctui_animate::manager::AnimationManager;
//!
//! let mut manager = AnimationManager::new();
//!
//! // Pause all animations
//! manager.pause_all();
//!
//! // Slow motion effect
//! manager.set_time_scale(0.5);
//!
//! // Resume
//! manager.resume_all();
//! ```

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

/// Unique identifier for a managed animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManagedId(pub u64);

/// State of the animation manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagerState {
    /// Animations are running normally.
    Running,
    /// All animations are paused.
    Paused,
}

/// Statistics about active animations.
#[derive(Debug, Clone, Copy, Default)]
pub struct AnimationStats {
    /// Total number of animations ever created.
    pub total_created: u64,
    /// Number of currently active animations.
    pub active_count: usize,
    /// Number of completed animations.
    pub completed_count: u64,
    /// Number of cancelled animations.
    pub cancelled_count: u64,
}

/// Callback triggered when an animation completes.
pub type OnCompleteCallback = Box<dyn FnMut() + Send + Sync>;

/// Entry for tracking a managed animation.
struct ManagedAnimation {
    group: Option<String>,
    on_complete: Option<OnCompleteCallback>,
    paused: bool,
    time_scale: f32,
    accumulated_ms: f64,
}

impl std::fmt::Debug for ManagedAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagedAnimation")
            .field("group", &self.group)
            .field("paused", &self.paused)
            .field("time_scale", &self.time_scale)
            .field("accumulated_ms", &self.accumulated_ms)
            .finish_non_exhaustive()
    }
}

/// Global animation coordinator.
///
/// The manager tracks all active animations, provides pause/resume
/// functionality, and allows time scaling effects.
pub struct AnimationManager {
    animations: HashMap<ManagedId, ManagedAnimation>,
    next_id: u64,
    state: ManagerState,
    time_scale: f32,
    stats: AnimationStats,
    auto_remove: bool,
}

impl std::fmt::Debug for AnimationManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnimationManager")
            .field("active_count", &self.animations.len())
            .field("state", &self.state)
            .field("time_scale", &self.time_scale)
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationManager {
    /// Creates a new animation manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            next_id: 0,
            state: ManagerState::Running,
            time_scale: 1.0,
            stats: AnimationStats::default(),
            auto_remove: true,
        }
    }

    /// Registers a new animation and returns its ID.
    ///
    /// The animation is assigned to no group and has no completion callback.
    #[must_use]
    pub fn register(&mut self) -> ManagedId {
        self.register_with_group(None)
    }

    /// Registers a new animation with a group.
    #[must_use]
    pub fn register_with_group(&mut self, group: Option<String>) -> ManagedId {
        let id = ManagedId(self.next_id);
        self.next_id += 1;
        self.stats.total_created += 1;

        self.animations.insert(
            id,
            ManagedAnimation {
                group,
                on_complete: None,
                paused: false,
                time_scale: 1.0,
                accumulated_ms: 0.0,
            },
        );

        self.stats.active_count = self.animations.len();
        id
    }

    /// Registers with a completion callback.
    pub fn register_with_callback<F>(&mut self, callback: F) -> ManagedId
    where
        F: FnMut() + Send + Sync + 'static,
    {
        self.register_with_group_and_callback(None, callback)
    }

    /// Registers with a group and completion callback.
    pub fn register_with_group_and_callback<F>(
        &mut self,
        group: Option<String>,
        callback: F,
    ) -> ManagedId
    where
        F: FnMut() + Send + Sync + 'static,
    {
        let id = ManagedId(self.next_id);
        self.next_id += 1;
        self.stats.total_created += 1;

        self.animations.insert(
            id,
            ManagedAnimation {
                group,
                on_complete: Some(Box::new(callback)),
                paused: false,
                time_scale: 1.0,
                accumulated_ms: 0.0,
            },
        );

        self.stats.active_count = self.animations.len();
        id
    }

    /// Sets the completion callback for an animation.
    pub fn set_callback<F>(&mut self, id: ManagedId, callback: F)
    where
        F: FnMut() + Send + Sync + 'static,
    {
        if let Some(entry) = self.animations.get_mut(&id) {
            entry.on_complete = Some(Box::new(callback));
        }
    }

    /// Marks an animation as completed.
    ///
    /// If `auto_remove` is enabled, the animation is removed.
    /// Otherwise, its completion callback is invoked (if present).
    pub fn complete(&mut self, id: ManagedId) {
        let entry = self.animations.remove(&id);
        if let Some(mut entry) = entry {
            self.stats.completed_count += 1;
            if let Some(ref mut callback) = entry.on_complete {
                callback();
            }
        }
        self.stats.active_count = self.animations.len();
    }

    /// Cancels an animation (removes without triggering callback).
    pub fn cancel(&mut self, id: ManagedId) {
        if self.animations.remove(&id).is_some() {
            self.stats.cancelled_count += 1;
        }
        self.stats.active_count = self.animations.len();
    }

    /// Returns the effective time scale for an animation.
    ///
    /// This multiplies the global time scale by the animation's local scale.
    #[must_use]
    fn effective_time_scale(&self, entry: &ManagedAnimation) -> f32 {
        self.time_scale * entry.time_scale
    }

    /// Appliess time scaling to a tick delta.
    ///
    /// Returns the scaled delta that should be passed to the actual animation.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn scale_delta(&self, id: ManagedId, delta_ms: u64) -> u64 {
        let Some(entry) = self.animations.get(&id) else {
            return delta_ms;
        };

        if entry.paused || self.state == ManagerState::Paused {
            return 0;
        }

        let scale = self.effective_time_scale(entry);
        (delta_ms as f64 * f64::from(scale)) as u64
    }

    /// Ticks the manager with the given delta.
    ///
    /// This method should be called once per frame to update time tracking.
    /// Returns a list of IDs that should be ticked (not paused).
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn tick(&mut self, delta_ms: u64) -> Vec<(ManagedId, u64)> {
        if self.state == ManagerState::Paused {
            return Vec::new();
        }

        self.animations
            .iter()
            .filter(|(_, entry)| !entry.paused)
            .map(|(id, entry)| {
                let scale = self.effective_time_scale(entry);
                (*id, (delta_ms as f64 * f64::from(scale)) as u64)
            })
            .collect()
    }

    /// Pauses a specific animation.
    pub fn pause(&mut self, id: ManagedId) {
        if let Some(entry) = self.animations.get_mut(&id) {
            entry.paused = true;
        }
    }

    /// Resumes a specific animation.
    pub fn resume(&mut self, id: ManagedId) {
        if let Some(entry) = self.animations.get_mut(&id) {
            entry.paused = false;
        }
    }

    /// Checks if a specific animation is paused.
    #[must_use]
    pub fn is_paused(&self, id: ManagedId) -> bool {
        self.animations
            .get(&id)
            .is_none_or(|e| e.paused || self.state == ManagerState::Paused)
    }

    /// Pauses all animations.
    pub const fn pause_all(&mut self) {
        self.state = ManagerState::Paused;
    }

    /// Resumes all animations.
    pub const fn resume_all(&mut self) {
        self.state = ManagerState::Running;
    }

    /// Returns the global manager state.
    #[must_use]
    pub const fn state(&self) -> ManagerState {
        self.state
    }

    /// Sets the global time scale.
    ///
    /// - `1.0` = normal speed
    /// - `2.0` = double speed (fast forward)
    /// - `0.5` = half speed (slow motion)
    pub const fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }

    /// Returns the global time scale.
    #[must_use]
    pub const fn time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Sets the time scale for a specific animation.
    pub fn set_animation_time_scale(&mut self, id: ManagedId, scale: f32) {
        if let Some(entry) = self.animations.get_mut(&id) {
            entry.time_scale = scale.max(0.0);
        }
    }

    /// Returns the time scale for a specific animation.
    #[must_use]
    pub fn animation_time_scale(&self, id: ManagedId) -> Option<f32> {
        self.animations
            .get(&id)
            .map(|e| self.effective_time_scale(e))
    }

    /// Sets whether to auto-remove completed animations.
    pub const fn set_auto_remove(&mut self, auto_remove: bool) {
        self.auto_remove = auto_remove;
    }

    /// Returns animation statistics.
    #[must_use]
    pub const fn stats(&self) -> AnimationStats {
        self.stats
    }

    /// Returns the number of active animations.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.animations.len()
    }

    /// Checks if an animation is registered.
    #[must_use]
    pub fn is_registered(&self, id: ManagedId) -> bool {
        self.animations.contains_key(&id)
    }

    /// Pauses all animations in a group.
    pub fn pause_group(&mut self, group: &str) {
        for entry in self.animations.values_mut() {
            if entry.group.as_deref() == Some(group) {
                entry.paused = true;
            }
        }
    }

    /// Resumes all animations in a group.
    pub fn resume_group(&mut self, group: &str) {
        for entry in self.animations.values_mut() {
            if entry.group.as_deref() == Some(group) {
                entry.paused = false;
            }
        }
    }

    /// Cancels all animations in a group.
    pub fn cancel_group(&mut self, group: &str) {
        self.animations.retain(|_, entry| {
            if entry.group.as_deref() == Some(group) {
                self.stats.cancelled_count += 1;
                false
            } else {
                true
            }
        });
        self.stats.active_count = self.animations.len();
    }

    /// Clears all animations.
    pub fn clear(&mut self) {
        let count = self.animations.len() as u64;
        self.stats.cancelled_count += count;
        self.animations.clear();
        self.stats.active_count = 0;
    }

    /// Returns IDs of all animations in a group.
    #[must_use]
    pub fn group_ids(&self, group: &str) -> Vec<ManagedId> {
        self.animations
            .iter()
            .filter(|(_, entry)| entry.group.as_deref() == Some(group))
            .map(|(id, _)| *id)
            .collect()
    }

    /// Returns all registered animation IDs.
    #[must_use]
    pub fn all_ids(&self) -> Vec<ManagedId> {
        self.animations.keys().copied().collect()
    }
}

/// Thread-safe handle for animation manager state.
///
/// This type allows sharing animation state across threads using
/// atomic operations for the global pause/state.
#[derive(Debug)]
pub struct AtomicAnimationState {
    /// Whether animations are paused.
    paused: AtomicBool,
    /// Global time scale (stored as fixed-point: 1000 = 1.0).
    time_scale: AtomicU64,
}

impl Default for AtomicAnimationState {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomicAnimationState {
    /// Creates a new atomic animation state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            paused: AtomicBool::new(false),
            time_scale: AtomicU64::new(1000),
        }
    }

    /// Pauses all animations.
    pub fn pause(&self) {
        self.paused.store(true, Ordering::SeqCst);
    }

    /// Resumes all animations.
    pub fn resume(&self) {
        self.paused.store(false, Ordering::SeqCst);
    }

    /// Returns whether animations are paused.
    #[must_use]
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }

    /// Sets the time scale.
    pub fn set_time_scale(&self, scale: f32) {
        let scale = (scale.max(0.0) * 1000.0) as u64;
        self.time_scale.store(scale, Ordering::SeqCst);
    }

    /// Returns the time scale.
    #[must_use]
    pub fn time_scale(&self) -> f32 {
        self.time_scale.load(Ordering::SeqCst) as f32 / 1000.0
    }

    /// Scales a delta value by the current time scale.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn scale_delta(&self, delta_ms: u64) -> u64 {
        if self.is_paused() {
            return 0;
        }
        let scale = self.time_scale();
        (delta_ms as f64 * f64::from(scale)) as u64
    }

    /// Creates an Arc wrapper for sharing.
    #[must_use]
    pub fn shared() -> Arc<Self> {
        Arc::new(Self::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;

    #[test]
    fn test_manager_new() {
        let manager = AnimationManager::new();
        assert_eq!(manager.active_count(), 0);
        assert_eq!(manager.state(), ManagerState::Running);
        approx::assert_relative_eq!(manager.time_scale(), 1.0);
    }

    #[test]
    fn test_manager_default() {
        let manager = AnimationManager::default();
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_manager_register() {
        let mut manager = AnimationManager::new();
        let id = manager.register();

        assert!(manager.is_registered(id));
        assert_eq!(manager.active_count(), 1);
        assert_eq!(id, ManagedId(0));

        let id2 = manager.register();
        assert_eq!(id2, ManagedId(1));
        assert_eq!(manager.active_count(), 2);
    }

    #[test]
    fn test_manager_register_with_group() {
        let mut manager = AnimationManager::new();
        let id = manager.register_with_group(Some("test".to_string()));

        assert!(manager.is_registered(id));
    }

    #[test]
    fn test_manager_complete() {
        use std::sync::atomic::{AtomicBool, Ordering};
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let mut manager = AnimationManager::new();
        let id = manager.register_with_callback(move || {
            called_clone.store(true, Ordering::SeqCst);
        });

        manager.complete(id);
        assert!(called.load(Ordering::SeqCst));
        assert!(!manager.is_registered(id));
        assert_eq!(manager.stats().completed_count, 1);
    }

    #[test]
    fn test_manager_cancel() {
        let mut manager = AnimationManager::new();
        let id = manager.register();

        manager.cancel(id);
        assert!(!manager.is_registered(id));
        assert_eq!(manager.stats().cancelled_count, 1);
    }

    #[test]
    fn test_manager_pause_resume() {
        let mut manager = AnimationManager::new();
        let id = manager.register();

        manager.pause(id);
        assert!(manager.is_paused(id));

        manager.resume(id);
        assert!(!manager.is_paused(id));
    }

    #[test]
    fn test_manager_pause_all() {
        let mut manager = AnimationManager::new();
        let id1 = manager.register();
        let id2 = manager.register();

        manager.pause_all();
        assert_eq!(manager.state(), ManagerState::Paused);
        assert!(manager.is_paused(id1));
        assert!(manager.is_paused(id2));

        manager.resume_all();
        assert_eq!(manager.state(), ManagerState::Running);
        assert!(!manager.is_paused(id1));
        assert!(!manager.is_paused(id2));
    }

    #[test]
    fn test_manager_time_scale() {
        let mut manager = AnimationManager::new();

        manager.set_time_scale(2.0);
        approx::assert_relative_eq!(manager.time_scale(), 2.0);

        manager.set_time_scale(0.5);
        approx::assert_relative_eq!(manager.time_scale(), 0.5);

        manager.set_time_scale(-1.0);
        approx::assert_relative_eq!(manager.time_scale(), 0.0);
    }

    #[test]
    fn test_manager_animation_time_scale() {
        let mut manager = AnimationManager::new();
        let id = manager.register();

        manager.set_animation_time_scale(id, 2.0);
        let scale = manager.animation_time_scale(id);
        approx::assert_relative_eq!(scale.unwrap(), 2.0);
    }

    #[test]
    fn test_manager_scale_delta() {
        let mut manager = AnimationManager::new();
        manager.set_time_scale(2.0);
        let id = manager.register();

        let scaled = manager.scale_delta(id, 100);
        assert_eq!(scaled, 200);
    }

    #[test]
    fn test_manager_scale_delta_when_paused() {
        let mut manager = AnimationManager::new();
        let id = manager.register();

        manager.pause_all();
        let scaled = manager.scale_delta(id, 100);
        assert_eq!(scaled, 0);
    }

    #[test]
    fn test_manager_tick() {
        let mut manager = AnimationManager::new();
        let id1 = manager.register();
        let id2 = manager.register();

        let ticks = manager.tick(16);
        assert_eq!(ticks.len(), 2);

        manager.pause(id1);
        let ticks = manager.tick(16);
        assert_eq!(ticks.len(), 1);
        assert_eq!(ticks[0].0, id2);
    }

    #[test]
    fn test_manager_group_operations() {
        let mut manager = AnimationManager::new();
        let id1 = manager.register_with_group(Some("group_a".to_string()));
        let id2 = manager.register_with_group(Some("group_a".to_string()));
        let id3 = manager.register_with_group(Some("group_b".to_string()));

        manager.pause_group("group_a");
        assert!(manager.is_paused(id1));
        assert!(manager.is_paused(id2));
        assert!(!manager.is_paused(id3));

        manager.resume_group("group_a");
        assert!(!manager.is_paused(id1));
        assert!(!manager.is_paused(id2));

        manager.cancel_group("group_a");
        assert!(!manager.is_registered(id1));
        assert!(!manager.is_registered(id2));
        assert!(manager.is_registered(id3));
    }

    #[test]
    fn test_manager_group_ids() {
        let mut manager = AnimationManager::new();
        let _id1 = manager.register_with_group(Some("group_a".to_string()));
        let _id2 = manager.register_with_group(Some("group_a".to_string()));
        let _id3 = manager.register_with_group(Some("group_b".to_string()));

        let ids = manager.group_ids("group_a");
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_manager_clear() {
        let mut manager = AnimationManager::new();
        let _ = manager.register();
        let _ = manager.register();

        manager.clear();
        assert_eq!(manager.active_count(), 0);
        assert_eq!(manager.stats().cancelled_count, 2);
    }

    #[test]
    fn test_manager_stats() {
        let mut manager = AnimationManager::new();

        let id1 = manager.register();
        let id2 = manager.register();
        manager.cancel(id1);
        manager.complete(id2);

        let stats = manager.stats();
        assert_eq!(stats.total_created, 2);
        assert_eq!(stats.completed_count, 1);
        assert_eq!(stats.cancelled_count, 1);
        assert_eq!(stats.active_count, 0);
    }

    #[test]
    fn test_atomic_state_new() {
        let state = AtomicAnimationState::new();
        assert!(!state.is_paused());
        approx::assert_relative_eq!(state.time_scale(), 1.0);
    }

    #[test]
    fn test_atomic_state_default() {
        let state = AtomicAnimationState::default();
        assert!(!state.is_paused());
    }

    #[test]
    fn test_atomic_state_pause_resume() {
        let state = AtomicAnimationState::new();

        state.pause();
        assert!(state.is_paused());

        state.resume();
        assert!(!state.is_paused());
    }

    #[test]
    fn test_atomic_state_time_scale() {
        let state = AtomicAnimationState::new();

        state.set_time_scale(2.0);
        approx::assert_relative_eq!(state.time_scale(), 2.0, epsilon = 0.001);

        state.set_time_scale(0.5);
        approx::assert_relative_eq!(state.time_scale(), 0.5, epsilon = 0.001);
    }

    #[test]
    fn test_atomic_state_scale_delta() {
        let state = AtomicAnimationState::new();

        state.set_time_scale(2.0);
        let scaled = state.scale_delta(100);
        assert_eq!(scaled, 200);

        state.pause();
        assert_eq!(state.scale_delta(100), 0);
    }

    #[test]
    fn test_atomic_state_shared() {
        let state = AtomicAnimationState::shared();
        state.pause();
        assert!(state.is_paused());
    }

    #[test]
    fn test_manager_auto_remove() {
        let mut manager = AnimationManager::new();
        manager.set_auto_remove(true);

        let id = manager.register();
        manager.complete(id);
        assert!(!manager.is_registered(id));
    }

    #[test]
    fn test_manager_all_ids() {
        let mut manager = AnimationManager::new();
        let id1 = manager.register();
        let id2 = manager.register();

        let ids = manager.all_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
    }

    #[test]
    fn test_manager_set_callback() {
        use std::sync::atomic::AtomicBool;
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let mut manager = AnimationManager::new();
        let id = manager.register();

        manager.set_callback(id, move || {
            called_clone.store(true, Ordering::SeqCst);
        });

        manager.complete(id);
        assert!(called.load(Ordering::SeqCst));
    }
}
