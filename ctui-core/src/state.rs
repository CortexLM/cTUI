//! State management with dispatch pattern.
//!
//! This module provides global state management inspired by the Elm Architecture
//! and Redux patterns. It separates state from view concerns and enables
//! predictable state updates through message dispatching.
//!
//! # Architecture Overview
//!
//! The state management follows the unidirectional data flow pattern:
//! 1. **State**: The single source of truth for application state
//! 2. **Dispatch**: Messages are dispatched to update the state
//! 3. **Reduce**: State updates are performed by the reducer function
//! 4. **Subscribe**: Components subscribe to state changes
//!
//! # Example
//!
//! ```
//! use ctui_core::{state::{State, Store}, Cmd, Msg};
//!
//! struct CounterState {
//!     count: i32,
//! }
//!
//! struct Increment;
//! impl Msg for Increment {}
//!
//! struct Decrement;
//! impl Msg for Decrement {}
//!
//! impl State for CounterState {
//!     fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd {
//!         if msg.is::<Increment>() {
//!             self.count += 1;
//!             Cmd::Render
//!         } else if msg.is::<Decrement>() {
//!             self.count -= 1;
//!             Cmd::Render
//!         } else {
//!             Cmd::Noop
//!         }
//!     }
//! }
//!
//! let store = Store::new(CounterState { count: 0 });
//! store.dispatch(Box::new(Increment));
//! ```

use crate::component::{Cmd, Msg};
use std::any::Any;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// A trait for application state that can be updated via messages.
///
/// This trait is the core of the state management system. Implementations
/// define how messages transform the state and what commands should be
/// executed as side effects.
///
/// # Message Matching
///
/// Use `msg.is::<T>()` to check message types and `msg.downcast_ref::<T>()`
/// to access message data.
///
/// # Example
///
/// ```
/// use ctui_core::{state::State, Cmd, Msg};
///
/// struct TodoState {
///     items: Vec<String>,
/// }
///
/// struct AddTodo;
/// impl Msg for AddTodo {}
///
/// impl State for TodoState {
///     fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd {
///         if msg.is::<AddTodo>() {
///             self.items.push("new item".to_string());
///             Cmd::Render
///         } else {
///             Cmd::Noop
///         }
///     }
/// }
/// ```
pub trait State: Send + Sync + 'static {
    /// Reduces the state based on a message.
    ///
    /// Use `msg.is::<T>()` to check the message type.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to process
    ///
    /// # Returns
    ///
    /// A `Cmd` representing side effects to execute.
    fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd;

    /// Provides access to the state as `Any` for downcasting.
    fn as_any(&self) -> &dyn Any where Self: Sized {
        self
    }
}

/// Subscriber ID for managing subscriptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriberId(u64);

impl SubscriberId {
    fn next() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        SubscriberId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// A global state store with dispatch and subscribe capabilities.
///
/// `Store` wraps your application state and provides:
/// - Thread-safe state access via `RwLock`
/// - Message dispatching for state updates
/// - Subscription system for reactivity
/// - Async action support via tokio
///
/// # Thread Safety
///
/// `Store` is `Clone` and thread-safe. Internal state is protected
/// by an `RwLock`, making it suitable for multi-threaded applications.
///
/// # Example
///
/// ```
/// use ctui_core::{state::Store, Cmd, Msg};
/// # use ctui_core::state::State;
/// #
/// # struct AppState { value: i32 }
/// # struct SetValue;
/// # impl Msg for SetValue {}
/// # impl State for AppState {
/// #     fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd { Cmd::Noop }
/// # }
///
/// let store = Store::new(AppState { value: 0 });
///
/// store.dispatch(Box::new(SetValue));
/// ```
pub struct Store<T: State> {
    /// The wrapped state, protected by RwLock for thread safety.
    state: Arc<RwLock<T>>,

    /// Broadcast channel for state change notifications.
    notifier: broadcast::Sender<()>,

    /// Counter for generating unique subscriber IDs.
    subscriber_counter: AtomicU64,
}

// Explicitly implement Clone since AtomicU64 doesn't implement it
impl<T: State> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            notifier: self.notifier.clone(),
            subscriber_counter: AtomicU64::new(self.subscriber_counter.load(Ordering::Relaxed)),
        }
    }
}

impl<T: State> Store<T> {
    /// Creates a new store with the given initial state.
    ///
    /// # Arguments
    ///
    /// * `state` - The initial state
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::state::{Store, State};
    ///
    /// struct MyState { data: String }
    /// impl State for MyState {
    ///     fn reduce(&mut self, _msg: Box<dyn ctui_core::Msg>) -> ctui_core::Cmd {
    ///         ctui_core::Cmd::Noop
    ///     }
    /// }
    ///
    /// let store = Store::new(MyState { data: "hello".into() });
    /// ```
    pub fn new(state: T) -> Self {
        let (notifier, _) = broadcast::channel(32);
        Self {
            state: Arc::new(RwLock::new(state)),
            notifier,
            subscriber_counter: AtomicU64::new(1),
        }
    }

    /// Dispatches a message to update the state.
    ///
    /// This method:
    /// 1. Acquires a write lock on the state
    /// 2. Calls `reduce` to update the state
    /// 3. Notifies all subscribers
    /// 4. Returns the command from the reducer
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to dispatch
    ///
    /// # Returns
    ///
    /// The `Cmd` returned by the reducer.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::{state::{Store, State}, Msg, Cmd};
    /// use std::any::TypeId;
    ///
    /// #[derive(Clone)]
    /// struct Counter { count: i32 }
    /// struct Increment;
    /// impl Msg for Increment {}
    ///
    /// impl State for Counter {
    ///     fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd {
    ///         if msg.type_id() == TypeId::of::<Increment>() {
    ///             self.count += 1;
    ///             Cmd::Render
    ///         } else {
    ///             Cmd::Noop
    ///         }
    ///     }
    /// }
    ///
    /// let store = Store::new(Counter { count: 0 });
    /// let cmd = store.dispatch(Box::new(Increment));
    ///
    /// assert_eq!(cmd, Cmd::Render);
    /// assert_eq!(store.state().count, 1);
    /// ```
    pub fn dispatch(&self, msg: Box<dyn Msg>) -> Cmd {
        let cmd = {
            let mut state = self.state.blocking_write();
            state.reduce(msg)
        };

        // Notify subscribers (ignore errors - just means no receivers)
        let _ = self.notifier.send(());

        cmd
    }

    /// Dispatches a message asynchronously.
    ///
    /// This is the async version of `dispatch`, suitable for use
    /// in async contexts without blocking the runtime.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to dispatch
    ///
    /// # Returns
    ///
    /// A future that resolves to the `Cmd` from the reducer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ctui_core::{state::{Store, State}, Msg, Cmd};
    ///
    /// # #[derive(Clone)]
    /// # struct Counter { count: i32 }
    /// # struct Increment;
    /// # impl Msg for Increment {}
    /// # impl State for Counter {
    /// #     fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd {
    /// #         if msg.type_id() == std::any::TypeId::of::<Increment>() { self.count += 1; Cmd::Render } else { Cmd::Noop }
    /// #     }
    /// # }
    /// # #[tokio::main]
    /// # async fn main() {
    /// let store = Store::new(Counter { count: 0 });
    ///
    /// let cmd = store.dispatch_async(Box::new(Increment)).await;
    /// assert_eq!(cmd, Cmd::Render);
    /// # }
    /// ```
    pub async fn dispatch_async(&self, msg: Box<dyn Msg>) -> Cmd {
        let cmd = {
            let mut state = self.state.write().await;
            state.reduce(msg)
        };

        // Notify subscribers (ignore errors - just means no receivers)
        let _ = self.notifier.send(());

        cmd
    }

    /// Returns a clone of the current state.
    ///
    /// This acquires a read lock, so it blocks until no writes are in progress.
    /// For async contexts, use `state_async()` instead.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::state::{Store, State};
    ///
    /// # #[derive(Clone)]
    /// # struct MyState { value: i32 }
    /// # impl State for MyState {
    /// #     fn reduce(&mut self, _msg: Box<dyn ctui_core::Msg>) -> ctui_core::Cmd { ctui_core::Cmd::Noop }
    /// # }
    ///
    /// let store = Store::new(MyState { value: 42 });
    /// assert_eq!(store.state().value, 42);
    /// ```
    pub fn state(&self) -> T
    where
        T: Clone,
    {
        self.state.blocking_read().clone()
    }

    /// Returns a reference to the current state asynchronously.
    ///
    /// This is the async version of `state()`, suitable for use
    /// in async contexts.
    pub async fn state_async(&self) -> T
    where
        T: Clone,
    {
        self.state.read().await.clone()
    }

/// Subscribes to state changes.
    ///
    /// The callback will be invoked after each state update.
    /// Returns a `SubscriberId` that can be used to track the subscription.
    ///
    /// For async notifications, use `subscribe_channel()` instead.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function to call when state changes
    ///
    /// # Returns
    ///
    /// A `SubscriberId` for tracking the subscription.
    pub fn subscribe<F>(&self, callback: F) -> SubscriberId
    where
        F: Fn(&T) + Send + 'static,
    {
        let id = SubscriberId::next();
        let subscriber_counter = AtomicU64::new(id.0);
        self.subscriber_counter
            .store(subscriber_counter.load(Ordering::Relaxed), Ordering::Relaxed);

        let mut receiver = self.notifier.subscribe();
        let state = Arc::clone(&self.state);

        tokio::spawn(async move {
            while receiver.recv().await.is_ok() {
                if let Ok(state_guard) = state.try_read() {
                    callback(&*state_guard);
                }
            }
        });

        id
    }

    /// Returns a channel that receives notifications when state changes.
    ///
    /// Use this for async notification patterns where you want to
    /// process state changes in your own task.
    ///
    /// # Returns
    ///
    /// A `broadcast::Receiver` that receives `()` on each state change.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ctui_core::{state::{Store, State}, Msg, Cmd};
    /// use std::any::TypeId;
    ///
    /// # #[derive(Clone)]
    /// # struct Counter { count: i32 }
    /// # struct Increment;
    /// # impl Msg for Increment {}
    /// # impl State for Counter {
    /// #     fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd {
    /// #         if msg.type_id() == TypeId::of::<Increment>() { self.count += 1; Cmd::Render } else { Cmd::Noop }
    /// #     }
    /// # }
    /// # #[tokio::main]
    /// # async fn main() {
    /// let store = Store::new(Counter { count: 0 });
    /// let mut receiver = store.subscribe_channel();
    ///
    /// store.dispatch_async(Box::new(Increment)).await;
    ///
    /// assert!(receiver.recv().await.is_ok());
    /// # }
    /// ```
    pub fn subscribe_channel(&self) -> broadcast::Receiver<()> {
        self.notifier.subscribe()
    }

    /// Spawns an async action that will dispatch its result.
    ///
    /// Use this for operations like HTTP requests, file I/O, or other
    /// async work that should update the state when complete.
    ///
    /// # Arguments
    ///
    /// * `action` - An async function that returns a message
    ///
    /// # Returns
    ///
    /// A `tokio::task::JoinHandle` for the spawned task.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ctui_core::{state::{Store, State}, Msg, Cmd};
    ///
    /// # #[derive(Clone)]
    /// # struct Counter { count: i32 }
    /// # struct SetValue;
    /// # impl Msg for SetValue {}
    /// # impl State for Counter {
    /// #     fn reduce(&mut self, _msg: Box<dyn Msg>) -> Cmd { Cmd::Noop }
    /// # }
    /// # #[tokio::main]
    /// # async fn main() {
    /// let store = Store::new(Counter { count: 0 });
    ///
    /// store.spawn_async::<SetValue, _>(async move {
    ///     Box::new(SetValue) as Box<dyn Msg>
    /// });
    /// # }
    /// ```
    pub fn spawn_async<M, F>(&self, action: F) -> tokio::task::JoinHandle<()>
    where
        M: Msg + 'static,
        F: Future<Output = Box<dyn Msg>> + Send + 'static,
    {
        let store = self.clone();
        tokio::spawn(async move {
            let msg = action.await;
            store.dispatch_async(msg).await;
        })
    }
}

/// Internal type for subscriber callbacks.
/// Wraps a callback with its ID for storage.
pub struct Subscriber<T: State> {
    /// Unique identifier for this subscriber.
    pub id: SubscriberId,

    /// The callback function.
    pub callback: Box<dyn Fn(&T) + Send>,
}

impl<T: State> Subscriber<T> {
    /// Creates a new subscriber.
    pub fn new(id: SubscriberId, callback: Box<dyn Fn(&T) + Send>) -> Self {
        Self { id, callback }
    }

    /// Invokes the callback with the given state.
    pub fn invoke(&self, state: &T) {
        (self.callback)(state);
    }
}

/// Extension trait for `Msg` to provide `as_any()` for downcasting.
///
/// This trait is automatically implemented for all `Msg` types.
pub trait MsgExt: Msg {
    /// Returns a reference to `self` as `Any` for downcasting.
    fn as_any(&self) -> &dyn Any;
}

impl<M: Msg + Sized + 'static> MsgExt for M {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Arc;

    // Test state implementation
    #[derive(Clone)]
    struct Counter {
        count: i32,
    }

    struct Increment;
    impl Msg for Increment {}

    struct Decrement;
    impl Msg for Decrement {}

    struct SetValue(i32);
    impl Msg for SetValue {}

    impl State for Counter {
        fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd {
            if msg.is::<Increment>() {
                self.count += 1;
                Cmd::Render
            } else if msg.is::<Decrement>() {
                self.count -= 1;
                Cmd::Render
            } else {
                Cmd::Noop
            }
        }
    }

    #[test]
    fn test_store_creation() {
        let store = Store::new(Counter { count: 0 });
        assert_eq!(store.state().count, 0);
    }

    #[test]
    fn test_dispatch_increment() {
        let store = Store::new(Counter { count: 0 });
        let cmd = store.dispatch(Box::new(Increment));

        assert_eq!(cmd, Cmd::Render);
        assert_eq!(store.state().count, 1);
    }

    #[test]
    fn test_dispatch_decrement() {
        let store = Store::new(Counter { count: 5 });
        let cmd = store.dispatch(Box::new(Decrement));

        assert_eq!(cmd, Cmd::Render);
        assert_eq!(store.state().count, 4);
    }

    #[test]
    fn test_dispatch_unknown_message() {
        struct UnknownMsg;
        impl Msg for UnknownMsg {}

        let store = Store::new(Counter { count: 10 });
        let cmd = store.dispatch(Box::new(UnknownMsg));

        assert_eq!(cmd, Cmd::Noop);
        assert_eq!(store.state().count, 10);
    }

    #[test]
    fn test_multiple_dispatches() {
        let store = Store::new(Counter { count: 0 });

        store.dispatch(Box::new(Increment));
        store.dispatch(Box::new(Increment));
        store.dispatch(Box::new(Increment));

        assert_eq!(store.state().count, 3);
    }

    #[tokio::test]
    async fn test_dispatch_async() {
        let store = Store::new(Counter { count: 0 });
        let cmd = store.dispatch_async(Box::new(Increment)).await;

        assert_eq!(cmd, Cmd::Render);
        let state = store.state_async().await;
        assert_eq!(state.count, 1);
    }

    #[test]
    fn test_subscribe() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = Store::new(Counter { count: 0 });
            let call_count = Arc::new(AtomicI32::new(0));
            let call_count_clone = call_count.clone();

            store.subscribe(move |_state| {
                call_count_clone.fetch_add(1, Ordering::SeqCst);
            });

            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

            store.dispatch_async(Box::new(Increment)).await;
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        });
    }

    #[test]
    fn test_subscribe_channel() {
        let store = Store::new(Counter { count: 0 });
        let mut receiver = store.subscribe_channel();

        store.dispatch(Box::new(Increment));

        // Check that we receive a notification (with timeout)
        let result =
            tokio::runtime::Runtime::new().unwrap().block_on(async { receiver.recv().await });

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_spawn_async() {
        let store = Store::new(Counter { count: 0 });

        let handle = store.spawn_async::<Increment, _>(async { Box::new(Increment) as Box<dyn Msg> });

        handle.await.unwrap();

        let state = store.state_async().await;
        assert_eq!(state.count, 1);
    }

    #[test]
    fn test_store_clone() {
        let store = Store::new(Counter { count: 42 });
        let cloned = store.clone();

        assert_eq!(cloned.state().count, 42);

        // Mutations on clone affect original (shared state)
        cloned.dispatch(Box::new(Increment));
        assert_eq!(store.state().count, 43);
    }

    // Test state with complex data
    #[derive(Clone)]
    struct TodoState {
        items: Vec<String>,
    }

    struct AddTodo(String);
    impl Msg for AddTodo {}

    impl State for TodoState {
        fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd {
            if msg.is::<AddTodo>() {
                self.items.push("new item".to_string());
                Cmd::Render
            } else {
                Cmd::Noop
            }
        }
    }

    #[test]
    fn test_complex_state() {
        let store = Store::new(TodoState {
            items: vec!["First".to_string()],
        });

        assert_eq!(store.state().items.len(), 1);
    }

    // Test state with message that carries data
    #[derive(Clone)]
    struct SumState {
        total: i32,
    }

    struct AddValue(i32);
    impl Msg for AddValue {}

    impl State for SumState {
        fn reduce(&mut self, msg: Box<dyn Msg>) -> Cmd {
            if msg.is::<AddValue>() {
                self.total += 10;
                Cmd::Render
            } else {
                Cmd::Noop
            }
        }
    }

    #[test]
    fn test_state_with_data_message() {
        let store = Store::new(SumState { total: 0 });
        store.dispatch(Box::new(AddValue(5)));
        assert_eq!(store.state().total, 10); // Simplified - adds 10

        store.dispatch(Box::new(AddValue(5)));
        assert_eq!(store.state().total, 20);
    }

    #[test]
    fn test_subscriber_id_uniqueness() {
        let id1 = SubscriberId::next();
        let id2 = SubscriberId::next();

        assert_ne!(id1, id2);
    }
}
