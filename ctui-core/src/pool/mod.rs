//! Message pooling for reducing allocation overhead.
//!
//! This module provides `MessagePool`, a memory pool for reusing message objects.
//! It uses `typed_arena::Arena` to allocate messages in batches, reducing the
//! overhead of `Box<dyn Msg>` allocations in the hot path of `Component::update()`.
//!
//! # Features
//!
//! This module is only available when the `component-pool` feature is enabled.
//!
//! # Example
//!
//! ```
//! use ctui_core::{MessagePool, Msg};
//!
//! #[derive(Debug)]
//! struct Increment;
//! impl Msg for Increment {}
//!
//! let pool = MessagePool::new();
//! let msg = pool.acquire(Increment);
//! // msg is &mut Increment - use it in your component update
//! ```
//!
//! # Memory Model
//!
//! The arena allocates objects in contiguous chunks. When the pool is dropped,
//! all allocations are freed at once. This provides:
//!
//! - **Cache locality**: Objects are packed tightly
//! - **Fast allocation**: O(1) bump allocation
//! - **No fragmentation**: All objects in arena memory
//!
//! # Trade-offs
//!
//! - Objects cannot be individually deallocated (pool resets on drop)
//! - Object lifetime is tied to the pool lifetime
//! - Best for message types that are short-lived within a frame

use crate::component::Msg;
use typed_arena::Arena;

/// A memory pool for message object reuse.
///
/// `MessagePool` uses `typed_arena::Arena` to efficiently allocate messages
/// without the overhead of individual heap allocations. This is particularly
/// useful for the hot path in `Component::update()` where `Box<dyn Msg>` 
/// allocations can be frequent.
///
/// # Type Parameters
///
/// * `T` - The message type, must implement `Msg`
///
/// # Example
///
/// ```ignore
/// use ctui_core::MessagePool;
///
/// // Create a pool for Increment messages
/// let pool: MessagePool<Increment> = MessagePool::new();
///
/// // Acquire a message slot
/// let msg = pool.acquire(Increment);
/// // Use msg in your component
/// ```
pub struct MessagePool<T: Msg> {
    arena: Arena<T>,
}

impl<T: Msg> MessagePool<T> {
    /// Creates a new message pool with default capacity.
    ///
    /// The arena starts with a small capacity and grows as needed.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::{MessagePool, Msg};
    ///
    /// struct MyMsg;
    /// impl Msg for MyMsg {}
    ///
    /// let pool = MessagePool::<MyMsg>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
        }
    }

    /// Creates a new message pool with the specified initial capacity.
    ///
    /// Pre-allocating capacity can improve performance when you know
    /// approximately how many messages will be allocated.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Initial number of slots to pre-allocate
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::{MessagePool, Msg};
    ///
    /// struct MyMsg;
    /// impl Msg for MyMsg {}
    ///
    /// // Pre-allocate for 100 messages
    /// let pool = MessagePool::<MyMsg>::with_capacity(100);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: Arena::with_capacity(capacity),
        }
    }

    /// Acquires a message slot from the pool.
    ///
    /// Allocates a new message in the arena and returns a mutable reference
    /// to it. The message is initialized with the provided value.
    ///
    /// # Arguments
    ///
    /// * `value` - The message value to initialize the slot with
    ///
    /// # Returns
    ///
    /// A mutable reference to the allocated message
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::{MessagePool, Msg};
    ///
    /// #[derive(Debug)]
    /// struct SetValue(i32);
    /// impl Msg for SetValue {}
    ///
    /// let pool = MessagePool::new();
    /// let msg = pool.acquire(SetValue(42));
    /// assert_eq!(msg.0, 42);
    /// ```
    pub fn acquire(&self, value: T) -> &mut T {
        self.arena.alloc(value)
    }

    /// Returns the number of messages currently allocated in the pool.
    ///
    /// This can be useful for monitoring pool usage and determining
    /// appropriate initial capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::{MessagePool, Msg};
    ///
    /// struct MyMsg;
    /// impl Msg for MyMsg {}
    ///
    /// let pool = MessagePool::<MyMsg>::new();
    /// assert_eq!(pool.len(), 0);
    /// 
    /// pool.acquire(MyMsg);
    /// pool.acquire(MyMsg);
    /// assert_eq!(pool.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    /// Returns `true` if the pool contains no allocated messages.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::{MessagePool, Msg};
    ///
    /// struct MyMsg;
    /// impl Msg for MyMsg {}
    ///
    /// let pool = MessagePool::<MyMsg>::new();
    /// assert!(pool.is_empty());
    /// 
    /// pool.acquire(MyMsg);
    /// assert!(!pool.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.arena.len() == 0
    }
}

impl<T: Msg> Default for MessagePool<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test message for pool testing
    #[derive(Debug, Clone, PartialEq)]
    struct TestMsg(i32);
    impl Msg for TestMsg {}

    #[test]
    fn test_pool_new() {
        let pool = MessagePool::<TestMsg>::new();
        assert!(pool.is_empty());
        assert_eq!(pool.len(), 0);
    }

    #[test]
    fn test_pool_with_capacity() {
        let pool = MessagePool::<TestMsg>::with_capacity(50);
        assert!(pool.is_empty());
    }

    #[test]
    fn test_pool_acquire() {
        let pool = MessagePool::new();
        let msg = pool.acquire(TestMsg(42));
        
        assert_eq!(msg.0, 42);
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn test_pool_acquire_multiple() {
        let pool = MessagePool::new();
        
        let msg1 = pool.acquire(TestMsg(1));
        let msg2 = pool.acquire(TestMsg(2));
        let msg3 = pool.acquire(TestMsg(3));
        
        assert_eq!(msg1.0, 1);
        assert_eq!(msg2.0, 2);
        assert_eq!(msg3.0, 3);
        assert_eq!(pool.len(), 3);
    }

    #[test]
    fn test_pool_acquire_mutable() {
        let pool = MessagePool::new();
        let msg = pool.acquire(TestMsg(10));
        
        // Can mutate the acquired message
        msg.0 = 20;
        assert_eq!(msg.0, 20);
    }

    #[test]
    fn test_pool_default() {
        let pool = MessagePool::<TestMsg>::default();
        assert!(pool.is_empty());
    }

    #[test]
    fn test_pool_is_empty() {
        let pool = MessagePool::new();
        assert!(pool.is_empty());
        
        pool.acquire(TestMsg(1));
        assert!(!pool.is_empty());
    }

    #[test]
    fn test_pool_len() {
        let pool = MessagePool::new();
        assert_eq!(pool.len(), 0);
        
        for i in 0..10 {
            pool.acquire(TestMsg(i));
        }
        assert_eq!(pool.len(), 10);
    }
}
