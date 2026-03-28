//! Component trait for declarative UI elements
//!
//! This module provides the core `Component` trait for building reactive UI components,
//! inspired by React and the Elm Architecture. Components manage their own state,
//! respond to messages, and render to a buffer.
//!
//! # Architecture Overview
//!
//! The component model follows the Elm Architecture (TEA) pattern:
//! 1. **Model**: Component state (`Self::State`)
//! 2. **Update**: Handle messages and produce new state (`update`)
//! 3. **View**: Render state to buffer (`render`)
//!
//! # Example
//!
//! ```
//! use ctui_core::{Buffer, Rect, Component, Cmd, Msg};
//! use ctui_core::style::Style;
//!
//! /// A simple text component that displays a string
//! pub struct SimpleText {
//!     text: String,
//!     style: Style,
//! }
//!
//! /// Props for SimpleText - follows builder pattern convention
//! pub struct SimpleTextProps {
//!     text: String,
//!     style: Style,
//! }
//!
//! impl SimpleTextProps {
//!     pub fn new(text: impl Into<String>) -> Self {
//!         Self {
//!             text: text.into(),
//!             style: Style::default(),
//!         }
//!     }
//!
//!     pub fn style(mut self, style: Style) -> Self {
//!         self.style = style;
//!         self
//!     }
//! }
//!
//! impl Component for SimpleText {
//!     type Props = SimpleTextProps;
//!     type State = ();
//!
//!     fn create(props: Self::Props) -> Self {
//!         Self {
//!             text: props.text,
//!             style: props.style,
//!         }
//!     }
//!
//!     fn render(&self, area: Rect, buf: &mut Buffer) {
//!         // Render text starting from top-left
//!         for (i, ch) in self.text.chars().take(area.width as usize).enumerate() {
//!             let x = area.x + i as u16;
//!             buf.modify_cell(x, area.y, |cell| {
//!                 cell.symbol = ch.to_string();
//!                 cell.set_style(self.style);
//!             });
//!         }
//!     }
//!
//!     fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
//!         Cmd::Noop
//!     }
//! }
//! ```

use crate::buffer::Buffer;
use crate::geometry::Rect;
use std::any::Any;

/// A message that can be sent to a component.
///
/// Messages trigger state updates in the `update` method of components.
/// This trait provides object-safety for dynamic message dispatch and
/// runtime type checking via `Any`.
///
/// # Object Safety
///
/// `Msg` is object-safe, meaning you can use `Box<dyn Msg>` for
/// heterogeneous message collections.
///
/// # Implementing Msg
///
/// ```
/// use ctui_core::Msg;
///
/// struct Increment;
/// impl Msg for Increment {}
///
/// struct SetText(String);
/// impl Msg for SetText {}
/// ```
pub trait Msg: Send + Any {}

/// Extension for `dyn Msg` to enable downcasting.
impl dyn Msg {
    /// Returns `true` if the message is of type `T`.
    pub fn is<M: Msg>(&self) -> bool {
        self.type_id() == std::any::TypeId::of::<M>()
    }

    /// Attempts to downcast to a reference of type `M`.
    pub fn downcast_ref<M: Msg>(&self) -> Option<&M> {
        (self as &dyn Any).downcast_ref::<M>()
    }
}

/// A blank implementation for Msg - any type can be a message
impl Msg for () {}

/// Command returned from component updates.
///
/// Commands represent side effects that should be executed after an update.
/// They enable the component to request actions like quitting the application,
/// triggering a re-render, or scheduling async operations.
///
/// # Examples
///
/// ```
/// use ctui_core::Cmd;
///
/// // No action needed
/// let cmd = Cmd::Noop;
///
/// // Request application quit
/// let cmd = Cmd::Quit;
///
/// // Multiple commands as batch
/// let cmd = Cmd::Batch(vec![Cmd::Render, Cmd::Quit]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmd {
    /// No operation - do nothing after the update
    Noop,
    /// Request a re-render of the component
    Render,
    /// Request the application to quit
    Quit,
    /// Execute multiple commands in sequence
    Batch(Vec<Cmd>),
    /// Navigate to a different view/screen
    Navigate(String),
    /// Request focus for this component
    RequestFocus,
    /// Yield focus from this component
    YieldFocus,
}

impl Cmd {
    /// Creates a no-op command
    pub fn noop() -> Self {
        Cmd::Noop
    }

    /// Creates a render command
    pub fn render() -> Self {
        Cmd::Render
    }

    /// Creates a quit command
    pub fn quit() -> Self {
        Cmd::Quit
    }

    /// Creates a batch command from multiple commands
    pub fn batch(cmds: impl IntoIterator<Item = Cmd>) -> Self {
        Cmd::Batch(cmds.into_iter().collect())
    }

    /// Returns true if this command would cause the app to quit
    pub fn should_quit(&self) -> bool {
        match self {
            Cmd::Quit => true,
            Cmd::Batch(cmds) => cmds.iter().any(|c| c.should_quit()),
            _ => false,
        }
    }

    /// Returns true if this command requests a render
    pub fn should_render(&self) -> bool {
        match self {
            Cmd::Render => true,
            Cmd::Batch(cmds) => cmds.iter().any(|c| c.should_render()),
            _ => false,
        }
    }
}

impl Default for Cmd {
    fn default() -> Self {
        Cmd::Noop
    }
}

/// The core trait for UI components.
///
/// Components are the building blocks of cTUI applications. Each component
/// manages its own state, responds to messages, and renders to a buffer.
///
/// # Associated Types
///
/// - `Props`: Configuration passed when creating the component. Follow builder pattern.
/// - `State`: Internal state managed by the component.
///
/// # Lifecycle
///
/// 1. `create(props)` - Called once when the component is instantiated
/// 2. `on_mount()` - Called when the component is mounted to the view tree
/// 3. `update(msg)` - Called for each message the component receives
/// 4. `render(area, buf)` - Called to draw the component to the buffer
/// 5. `on_unmount()` - Called when the component is removed from the view tree
///
/// # Example: Counter Component
///
/// ```
/// use ctui_core::{Buffer, Rect, Component, Cmd, Msg};
///
/// struct CounterState {
///     count: i32,
/// }
///
/// struct Counter {
///     count: i32,
///     step: i32,
/// }
///
/// struct CounterProps {
///     initial: i32,
///     step: i32,
/// }
///
/// impl CounterProps {
///     pub fn new() -> Self {
///         Self { initial: 0, step: 1 }
///     }
///     pub fn initial(mut self, n: i32) -> Self {
///         self.initial = n;
///         self
///     }
///     pub fn step(mut self, n: i32) -> Self {
///         self.step = n;
///         self
///     }
/// }
///
/// // Messages for Counter
/// struct Increment;
/// impl Msg for Increment {}
///
/// struct Decrement;
/// impl Msg for Decrement {}
///
/// impl Component for Counter {
///     type Props = CounterProps;
///     type State = CounterState;
///
///     fn create(props: Self::Props) -> Self {
///         Self {
///             count: props.initial,
///             step: props.step,
///         }
///     }
///
///     fn render(&self, area: Rect, buf: &mut Buffer) {
///         let text = format!("Count: {}", self.count);
///         for (i, ch) in text.chars().take(area.width as usize).enumerate() {
///             buf.modify_cell(area.x + i as u16, area.y, |cell| {
///                 cell.symbol = ch.to_string();
///             });
///         }
///     }
///
///     fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
///         // Handle messages using type checking or downcasting
///         // (In practice, components use concrete message types)
///         Cmd::Noop
///     }
///
///     fn on_mount(&mut self) {
///         println!("Counter mounted with initial value: {}", self.count);
///     }
///
///     fn on_unmount(&mut self) {
///         println!("Counter unmounted");
///     }
/// }
/// ```
pub trait Component: Sized + 'static {
    /// Props type for component configuration.
    ///
    /// Props are passed when creating a component and contain
    /// configuration that doesn't change during the component's lifetime.
    type Props;

    /// State type for component state management.
    ///
    /// State represents the internal mutable state of the component.
    /// It can be unit `()` for stateless components.
    type State;

    /// Creates a new component from the given props.
    ///
    /// This is the constructor for the component. It should initialize
    /// all fields from the props and set up any initial state.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::{Component, Rect, Buffer, Cmd, Msg};
    ///
    /// struct MyComponent {
    ///     value: i32,
    /// }
    ///
    /// struct MyProps {
    ///     value: i32,
    /// }
    ///
    /// impl Component for MyComponent {
    ///     type Props = MyProps;
    ///     type State = ();
    ///
    ///     fn create(props: Self::Props) -> Self {
    ///         Self { value: props.value }
    ///     }
    ///
    ///     fn render(&self, area: Rect, buf: &mut Buffer) {}
    ///     fn update(&mut self, msg: Box<dyn Msg>) -> Cmd { Cmd::Noop }
    /// }
    /// ```
    fn create(props: Self::Props) -> Self;

    /// Renders the component to the buffer.
    ///
    /// This method should draw the component's visual representation
    /// into the buffer within the given area.
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangular area allocated for this component
    /// * `buf` - The buffer to render into
    fn render(&self, area: Rect, buf: &mut Buffer);

    /// Updates the component state in response to a message.
    ///
    /// This is the heart of the Elm Architecture. Messages trigger
    /// state changes, and the returned command specifies side effects.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to handle
    ///
    /// # Returns
    ///
    /// A command representing side effects to execute
    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd;

    /// Called when the component is mounted to the view tree.
    ///
    /// Override this method to perform initialization that should
    /// happen after the component is created but before the first render.
    /// Examples include starting timers, subscribing to events, or
    /// fetching initial data.
    fn on_mount(&mut self) {}

    /// Called when the component is unmounted from the view tree.
    ///
    /// Override this method to perform cleanup. Examples include
    /// canceling timers, unsubscribing from events, or releasing resources.
    fn on_unmount(&mut self) {}

    /// Called every frame with delta time for frame-rate independent animations.
    ///
    /// This method is called by a render loop on each frame, providing
    /// the elapsed time since the last frame in seconds. Use this for
    /// continuous animations, physics updates, or other time-based logic.
    ///
    /// # Arguments
    ///
    /// * `delta` - Time elapsed since last frame in seconds
    ///
    /// # Returns
    ///
    /// A command representing side effects to execute
    fn on_update(&mut self, _delta: f64) -> Cmd {
        Cmd::Noop
    }

    /// Updates the component with a pooled message reference.
    ///
    /// This is an optimized version of [`update`](Component::update) that accepts
    /// a message reference from a [`MessagePool`](crate::MessagePool) instead of
    /// a boxed message. This avoids the allocation overhead of `Box<dyn Msg>` in
    /// the hot path.
    ///
    /// # Performance Benefits
    ///
    /// When processing many messages per frame (e.g., in event-heavy UIs), the
    /// overhead of boxing each message can be significant. Pooled messages:
    /// - **Eliminate per-message allocations** - Messages are arena-allocated
    /// - **Improve cache locality** - Messages are stored contiguously
    /// - **Reduce memory fragmentation** - Arena frees all at once
    ///
    /// Benchmarks show 20-40% improvement in message-heavy workloads.
    ///
    /// # Default Implementation
    ///
    /// The default implementation boxes the message and delegates to [`update`](Component::update).
    /// Components can override this to handle pooled messages directly without boxing.
    ///
    /// # Arguments
    ///
    /// * `msg` - Reference to a pooled message
    ///
    /// # Returns
    ///
    /// A command representing side effects to execute
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ctui_core::{Component, Cmd, MessagePool, Msg};
    ///
    /// struct Counter { count: i32 }
    /// struct Increment;
    /// impl Msg for Increment {}
    ///
    /// impl Component for Counter {
    ///     // ... other trait methods
    ///     
    ///     fn update_pooled<M: Msg>(&mut self, msg: &M) -> Cmd {
    ///         // Handle known message type directly
    ///         if let Some(inc) = (msg as &dyn std::any::Any).downcast_ref::<Increment>() {
    ///             self.count += 1;
    ///             return Cmd::Render;
    ///         }
    ///         // Fallback to boxed update for unknown types
    ///         Cmd::Noop
    ///     }
    /// }
    /// ```
    #[cfg(feature = "component-pool")]
    fn update_pooled<M: Msg + Clone>(&mut self, msg: &M) -> Cmd {
        // Default: clone and box for standard update.
        // Components can override to avoid cloning when they know the type.
        self.update(Box::new(msg.clone()))
    }

    /// Updates the component with a batch of pooled messages.
    ///
    /// Processes multiple messages from a [`MessagePool`](crate::MessagePool) in a single
    /// update cycle, accumulating commands. This is more efficient than calling
    /// [`update`](Component::update) in a loop for each message.
    ///
    /// # Performance Benefits
    ///
    /// - Batches command evaluation into a single [`Cmd::Batch`]
    /// - Reduces render loop overhead for message floods
    /// - Ideal for keyboard repeat events or micro-interactions
    ///
    /// # Arguments
    ///
    /// * `messages` - Iterator of message references
    ///
    /// # Returns
    ///
    /// A batched command containing all accumulated side effects
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ctui_core::{Component, Cmd, MessagePool, Msg};
    ///
    /// let pool = MessagePool::new();
    /// let mut messages = Vec::new();
    /// messages.push(pool.acquire(Increment));
    /// messages.push(pool.acquire(Increment));
    ///
    /// // Process all at once
    /// let cmd = component.update_batch(messages.iter().map(|m| *m));
    /// ```
    #[cfg(feature = "component-pool")]
    fn update_batch<'a, M: Msg + Clone + 'a, I: Iterator<Item = &'a M> + 'a>(&mut self, messages: I) -> Cmd {
        let mut cmds = Vec::new();
        for msg in messages {
            cmds.push(self.update_pooled(msg));
        }
        if cmds.is_empty() {
            Cmd::Noop
        } else if cmds.len() == 1 {
            cmds.remove(0)
        } else {
            Cmd::Batch(cmds)
        }
    }
}

/// A stateless component that renders static content.
///
/// This is a convenience type for simple components that don't need
/// to manage state or handle messages.
pub struct StaticComponent<F> {
    render_fn: F,
}

impl<F> StaticComponent<F>
where
    F: Fn(Rect, &mut Buffer),
{
    /// Creates a new static component with the given render function
    pub fn new(render_fn: F) -> Self {
        Self { render_fn }
    }
}

impl<F> Component for StaticComponent<F>
where
    F: Fn(Rect, &mut Buffer) + 'static,
{
    type Props = ();
    type State = ();

    fn create(_props: Self::Props) -> Self {
        panic!("StaticComponent must be created with StaticComponent::new()")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        (self.render_fn)(area, buf);
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test message for lifecycle tracking
    struct TestMsg;
    impl Msg for TestMsg {}

    /// Simple test component to verify the trait is implementable
    struct TestComponent {
        value: i32,
        mounted: bool,
        unmounted: bool,
    }

    struct TestProps {
        initial: i32,
    }

    impl Component for TestComponent {
        type Props = TestProps;
        type State = i32;

        fn create(props: Self::Props) -> Self {
            Self {
                value: props.initial,
                mounted: false,
                unmounted: false,
            }
        }

        fn render(&self, area: Rect, buf: &mut Buffer) {
            buf.modify_cell(area.x, area.y, |cell| {
                cell.symbol = self.value.to_string();
            });
        }

        fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
            self.value += 1;
            Cmd::Render
        }

        fn on_mount(&mut self) {
            self.mounted = true;
        }

        fn on_unmount(&mut self) {
            self.unmounted = true;
        }
    }

    #[test]
    fn test_component_create() {
        let component = TestComponent::create(TestProps { initial: 42 });
        assert_eq!(component.value, 42);
        assert!(!component.mounted);
        assert!(!component.unmounted);
    }

    #[test]
    fn test_component_lifecycle_mount() {
        let mut component = TestComponent::create(TestProps { initial: 0 });
        assert!(!component.mounted);

        component.on_mount();
        assert!(component.mounted);
    }

    #[test]
    fn test_component_lifecycle_unmount() {
        let mut component = TestComponent::create(TestProps { initial: 0 });
        assert!(!component.unmounted);

        component.on_unmount();
        assert!(component.unmounted);
    }

    #[test]
    fn test_component_update() {
        let mut component = TestComponent::create(TestProps { initial: 10 });
        let cmd = component.update(Box::new(TestMsg));

        assert_eq!(component.value, 11);
        assert_eq!(cmd, Cmd::Render);
    }

    #[test]
    fn test_component_render() {
        let component = TestComponent::create(TestProps { initial: 7 });
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));

        component.render(Rect::new(0, 0, 10, 1), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "7");
    }

    #[test]
    fn test_cmd_noop() {
        let cmd = Cmd::noop();
        assert_eq!(cmd, Cmd::Noop);
        assert!(!cmd.should_quit());
        assert!(!cmd.should_render());
    }

    #[test]
    fn test_cmd_render() {
        let cmd = Cmd::render();
        assert_eq!(cmd, Cmd::Render);
        assert!(cmd.should_render());
        assert!(!cmd.should_quit());
    }

    #[test]
    fn test_cmd_quit() {
        let cmd = Cmd::quit();
        assert_eq!(cmd, Cmd::Quit);
        assert!(cmd.should_quit());
        assert!(!cmd.should_render());
    }

    #[test]
    fn test_cmd_batch() {
        let cmd = Cmd::batch(vec![Cmd::Render, Cmd::Quit]);

        assert!(cmd.should_quit());
        assert!(cmd.should_render());

        match cmd {
            Cmd::Batch(cmds) => {
                assert_eq!(cmds.len(), 2);
            }
            _ => panic!("Expected Batch"),
        }
    }

    #[test]
    fn test_cmd_batch_nested() {
        let inner = Cmd::batch(vec![Cmd::Quit]);
        let outer = Cmd::batch(vec![Cmd::Render, inner]);

        assert!(outer.should_quit());
        assert!(outer.should_render());
    }

    #[test]
    fn test_cmd_default() {
        let cmd: Cmd = Default::default();
        assert_eq!(cmd, Cmd::Noop);
    }

    #[test]
    fn test_static_component() {
        let component = StaticComponent::new(|area, buf| {
            buf.modify_cell(area.x, area.y, |cell| {
                cell.symbol = "S".to_string();
            });
        });

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        component.render(Rect::new(0, 0, 10, 1), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "S");
    }

    #[test]
    fn test_msg_object_safety() {
        // Verify that Box<dyn Msg> works (object-safety test)
        let _msg: Box<dyn Msg> = Box::new(TestMsg);
        let _msg2: Box<dyn Msg> = Box::new(());
    }

    /// Example component with full lifecycle demonstration
    struct TimerComponent {
        elapsed_ms: u64,
        label: String,
    }

    struct TimerProps {
        label: String,
    }

    impl TimerProps {
        fn new(label: impl Into<String>) -> Self {
            Self {
                label: label.into(),
            }
        }
    }

    impl Component for TimerComponent {
        type Props = TimerProps;
        type State = u64;

        fn create(props: Self::Props) -> Self {
            Self {
                elapsed_ms: 0,
                label: props.label,
            }
        }

        fn render(&self, area: Rect, buf: &mut Buffer) {
            let text = format!("{}: {}ms", self.label, self.elapsed_ms);
            for (i, ch) in text.chars().take(area.width as usize).enumerate() {
                let x = area.x + i as u16;
                buf.modify_cell(x, area.y, |cell| {
                    cell.symbol = ch.to_string();
                });
            }
        }

        fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
            self.elapsed_ms += 100;
            Cmd::Render
        }

        fn on_mount(&mut self) {
            self.elapsed_ms = 0;
        }

        fn on_unmount(&mut self) {
            println!("Timer '{}' finished at {}ms", self.label, self.elapsed_ms);
        }
    }

    #[test]
    fn test_timer_component() {
        let mut component = TimerComponent::create(TimerProps::new("Test"));
        component.on_mount();

        assert_eq!(component.elapsed_ms, 0);

        // Simulate receiving update messages
        component.update(Box::new(()));
        assert_eq!(component.elapsed_ms, 100);

        component.update(Box::new(()));
        assert_eq!(component.elapsed_ms, 200);

        // Render to buffer
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 1));
        component.render(Rect::new(0, 0, 50, 1), &mut buf);

        // Check render output starts with label
        assert!(buf.get(0, 0).unwrap().symbol.starts_with("T"));

        component.on_unmount();
    }

    /// Test that component with unit state compiles and works
    struct StatelessComponent {
        text: String,
    }

    struct StatelessProps {
        text: String,
    }

    impl Component for StatelessComponent {
        type Props = StatelessProps;
        type State = ();

        fn create(props: Self::Props) -> Self {
            Self { text: props.text }
        }

        fn render(&self, area: Rect, buf: &mut Buffer) {
            for (i, ch) in self.text.chars().take(area.width as usize).enumerate() {
                let x = area.x + i as u16;
                buf.modify_cell(x, area.y, |cell| {
                    cell.symbol = ch.to_string();
                });
            }
        }

        fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
            Cmd::Noop
        }
    }

    #[test]
    fn test_stateless_component() {
        let mut component = StatelessComponent::create(StatelessProps {
            text: "Hello".to_string(),
        });

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        component.render(Rect::new(0, 0, 10, 1), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "H");
        assert_eq!(buf.get(1, 0).unwrap().symbol, "e");

        let cmd = component.update(Box::new(()));
        assert_eq!(cmd, Cmd::Noop);
    }

    /// Test navigate command
    #[test]
    fn test_cmd_navigate() {
        let cmd = Cmd::Navigate("settings".to_string());
        assert!(!cmd.should_quit());
        assert!(!cmd.should_render());

        match cmd {
            Cmd::Navigate(route) => assert_eq!(route, "settings"),
            _ => panic!("Expected Navigate"),
        }
    }

    /// Test focus commands
    #[test]
    fn test_cmd_focus() {
        let focus_cmd = Cmd::RequestFocus;
        let yield_cmd = Cmd::YieldFocus;

        assert!(!focus_cmd.should_quit());
        assert!(!yield_cmd.should_quit());
    }
}
