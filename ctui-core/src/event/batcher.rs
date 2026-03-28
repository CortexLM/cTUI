//! Event batcher for aggregating multiple events
//!
//! This module provides [`EventBatcher`] for collecting events over a configurable
//! time window and flushing them in batches. This follows the same pattern as
//! `Cmd::Batch` for command aggregation.
//!
//! # Example
//!
//! ```
//! use ctui_core::event::batcher::EventBatcher;
//! use ctui_core::event::{Event, KeyEvent, KeyCode, KeyModifiers};
//! use std::time::Duration;
//!
//! let mut batcher = EventBatcher::with_window(Duration::from_millis(16));
//!
//! // Feed events
//! batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));
//! batcher.feed(Event::key(KeyCode::Char('b'), KeyModifiers::new()));
//!
//! // Flush returns all pending events
//! let events = batcher.flush();
//! assert_eq!(events.len(), 2);
//! ```

use super::Event;
use std::time::{Duration, Instant};

/// Batches events within a configurable time window.
///
/// Similar to `Cmd::Batch` for command aggregation, `EventBatcher` collects
/// events and flushes them in batches when the time window expires or when
/// manually flushed.
///
/// # Time Window
///
/// Events are accumulated until:
/// - The configured time window has elapsed since the last flush
/// - `flush()` is called manually
///
/// This is useful for:
/// - Reducing render frequency during rapid input
/// - Batching mouse movement events
/// - Coalescing multiple key events before processing
#[derive(Debug)]
pub struct EventBatcher {
    /// Time window for batching events
    window: Duration,
    /// Pending events waiting to be flushed
    pending: Vec<Event>,
    /// Time of last flush
    last_flush: Instant,
}

impl EventBatcher {
    /// Creates a new `EventBatcher` with the default window of 16ms (~60fps).
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    ///
    /// let batcher = EventBatcher::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::with_window(Duration::from_millis(16))
    }

    /// Creates a new `EventBatcher` with a custom time window.
    ///
    /// # Arguments
    ///
    /// * `window` - The duration to wait before flushing events.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use std::time::Duration;
    ///
    /// let batcher = EventBatcher::with_window(Duration::from_millis(8));
    /// ```
    #[must_use]
    pub fn with_window(window: Duration) -> Self {
        Self {
            window,
            pending: Vec::new(),
            last_flush: Instant::now(),
        }
    }

    /// Feeds an event into the batcher.
    ///
    /// Events are accumulated until `flush()` is called or the window expires.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to add to the batch.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use ctui_core::event::{Event, KeyCode, KeyModifiers};
    ///
    /// let mut batcher = EventBatcher::new();
    /// batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));
    /// ```
    pub fn feed(&mut self, event: Event) {
        self.pending.push(event);
    }

    /// Returns true if the batch window has elapsed since the last flush.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use std::time::Duration;
    ///
    /// let batcher = EventBatcher::with_window(Duration::from_millis(1));
    /// std::thread::sleep(Duration::from_millis(2));
    /// assert!(batcher.is_window_elapsed());
    /// ```
    #[must_use]
    pub fn is_window_elapsed(&self) -> bool {
        self.last_flush.elapsed() >= self.window
    }

    /// Flushes pending events, returning them as a batch.
    ///
    /// If the window has not elapsed, this still returns all pending events.
    /// The `last_flush` timestamp is updated regardless.
    ///
    /// # Returns
    ///
    /// A vector of all pending events. The vector may be empty if no events
    /// were fed since the last flush.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use ctui_core::event::{Event, KeyCode, KeyModifiers};
    ///
    /// let mut batcher = EventBatcher::new();
    /// batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));
    /// batcher.feed(Event::key(KeyCode::Char('b'), KeyModifiers::new()));
    ///
    /// let events = batcher.flush();
    /// assert_eq!(events.len(), 2);
    /// ```
    #[must_use]
    pub fn flush(&mut self) -> Vec<Event> {
        self.last_flush = Instant::now();
        std::mem::take(&mut self.pending)
    }

    /// Flushes pending events only if the window has elapsed.
    ///
    /// # Returns
    ///
    /// `Some(events)` if the window elapsed and events were flushed,
    /// `None` if the window has not elapsed yet.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use std::time::Duration;
    ///
    /// let mut batcher = EventBatcher::with_window(Duration::from_secs(10));
    /// let events = batcher.try_flush();
    /// assert!(events.is_none());
    /// ```
    #[must_use]
    pub fn try_flush(&mut self) -> Option<Vec<Event>> {
        if self.is_window_elapsed() {
            Some(self.flush())
        } else {
            None
        }
    }

    /// Returns the number of pending events without flushing.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use ctui_core::event::{Event, KeyCode, KeyModifiers};
    ///
    /// let mut batcher = EventBatcher::new();
    /// assert_eq!(batcher.len(), 0);
    /// batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));
    /// assert_eq!(batcher.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    /// Returns true if there are no pending events.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    ///
    /// let batcher = EventBatcher::new();
    /// assert!(batcher.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Clears all pending events without returning them.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use ctui_core::event::{Event, KeyCode, KeyModifiers};
    ///
    /// let mut batcher = EventBatcher::new();
    /// batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));
    /// batcher.clear();
    /// assert!(batcher.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.pending.clear();
    }

    /// Returns the configured window duration.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use std::time::Duration;
    ///
    /// let batcher = EventBatcher::with_window(Duration::from_millis(32));
    /// assert_eq!(batcher.window(), Duration::from_millis(32));
    /// ```
    #[must_use]
    pub fn window(&self) -> Duration {
        self.window
    }

    /// Sets a new window duration.
    ///
    /// # Arguments
    ///
    /// * `window` - The new time window duration.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::event::batcher::EventBatcher;
    /// use std::time::Duration;
    ///
    /// let mut batcher = EventBatcher::new();
    /// batcher.set_window(Duration::from_millis(8));
    /// assert_eq!(batcher.window(), Duration::from_millis(8));
    /// ```
    pub fn set_window(&mut self, window: Duration) {
        self.window = window;
    }
}

impl Default for EventBatcher {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_new() {
        let batcher = EventBatcher::new();
        assert!(batcher.is_empty());
        assert_eq!(batcher.window(), Duration::from_millis(16));
    }

    #[test]
    fn test_with_window() {
        let batcher = EventBatcher::with_window(Duration::from_millis(32));
        assert_eq!(batcher.window(), Duration::from_millis(32));
    }

    #[test]
    fn test_feed_and_flush() {
        let mut batcher = EventBatcher::new();
        batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));
        batcher.feed(Event::key(KeyCode::Char('b'), KeyModifiers::new()));
        batcher.feed(Event::key(KeyCode::Char('c'), KeyModifiers::new()));

        assert_eq!(batcher.len(), 3);

        let events = batcher.flush();
        assert_eq!(events.len(), 3);
        assert!(batcher.is_empty());
    }

    #[test]
    fn test_flush_empty() {
        let mut batcher = EventBatcher::new();
        let events = batcher.flush();
        assert!(events.is_empty());
    }

    #[test]
    fn test_try_flush_window_not_elapsed() {
        let mut batcher = EventBatcher::with_window(Duration::from_secs(10));
        batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));

        let result = batcher.try_flush();
        assert!(result.is_none());
        assert_eq!(batcher.len(), 1);
    }

    #[test]
    fn test_try_flush_window_elapsed() {
        let mut batcher = EventBatcher::with_window(Duration::from_millis(1));
        batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));

        std::thread::sleep(Duration::from_millis(2));

        let result = batcher.try_flush();
        assert!(result.is_some());
        assert!(batcher.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut batcher = EventBatcher::new();
        batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));
        batcher.feed(Event::key(KeyCode::Char('b'), KeyModifiers::new()));

        assert_eq!(batcher.len(), 2);
        batcher.clear();
        assert!(batcher.is_empty());
    }

    #[test]
    fn test_set_window() {
        let mut batcher = EventBatcher::new();
        assert_eq!(batcher.window(), Duration::from_millis(16));

        batcher.set_window(Duration::from_millis(8));
        assert_eq!(batcher.window(), Duration::from_millis(8));
    }

    #[test]
    fn test_default() {
        let batcher = EventBatcher::default();
        assert_eq!(batcher.window(), Duration::from_millis(16));
        assert!(batcher.is_empty());
    }

    #[test]
    fn test_window_elapsed_timing() {
        let batcher = EventBatcher::with_window(Duration::from_millis(1));
        assert!(!batcher.is_window_elapsed());

        std::thread::sleep(Duration::from_millis(2));

        assert!(batcher.is_window_elapsed());
    }

    #[test]
    fn test_batch_preserves_order() {
        let mut batcher = EventBatcher::new();

        // Feed events in specific order
        batcher.feed(Event::key(KeyCode::Char('a'), KeyModifiers::new()));
        batcher.feed(Event::key(KeyCode::Char('b'), KeyModifiers::new()));
        batcher.feed(Event::key(KeyCode::Char('c'), KeyModifiers::new()));
        batcher.feed(Event::key(KeyCode::Char('d'), KeyModifiers::new()));

        let events = batcher.flush();

        // Verify order is preserved: a, b, c, d
        assert_eq!(events.len(), 4);
        if let Event::Key(ke) = &events[0] {
            assert!(matches!(ke.code, KeyCode::Char('a')));
        } else {
            panic!("Expected Key event");
        }
        if let Event::Key(ke) = &events[1] {
            assert!(matches!(ke.code, KeyCode::Char('b')));
        } else {
            panic!("Expected Key event");
        }
        if let Event::Key(ke) = &events[2] {
            assert!(matches!(ke.code, KeyCode::Char('c')));
        } else {
            panic!("Expected Key event");
        }
        if let Event::Key(ke) = &events[3] {
            assert!(matches!(ke.code, KeyCode::Char('d')));
        } else {
            panic!("Expected Key event");
        }
    }

    #[test]
    fn test_empty_batch_returns_empty_vec() {
        let mut batcher = EventBatcher::new();

        // Do not feed any events, just flush
        let events = batcher.flush();

        assert!(events.is_empty());
        assert_eq!(events.len(), 0);

        // Batch is still empty after flush
        assert!(batcher.is_empty());
    }
}
