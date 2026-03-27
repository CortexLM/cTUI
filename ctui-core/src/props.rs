//! Props system for component configuration
//!
//! This module provides the `Props` trait and builder utilities for configuring components.
//! Props are the configuration passed when creating a component, following the builder pattern
//! for ergonomic construction.
//!
//! # Architecture
//!
//! The Props trait establishes the relationship between props and their components:
//! - `Props::Component` - The component type created from these props
//! - `Props::into_component()` - Creates a component instance
//!
//! # Example
//!
//! ```
//! use ctui_core::{Props, Component, Rect, Buffer, Cmd, Msg};
//!
//! // Define props for a component
//! #[derive(Clone)]
//! struct ButtonProps {
//!     label: String,
//!     disabled: bool,
//! }
//!
//! impl ButtonProps {
//!     pub fn new(label: impl Into<String>) -> Self {
//!         Self {
//!             label: label.into(),
//!             disabled: false,
//!         }
//!     }
//!
//!     pub fn disabled(mut self, disabled: bool) -> Self {
//!         self.disabled = disabled;
//!         self
//!     }
//! }
//!
//! // Props trait implementation
//! impl Props for ButtonProps {
//!     type Component = Button;
//!
//!     fn into_component(self) -> Self::Component {
//!         Button::create(self)
//!     }
//! }
//!
//! // The component
//! struct Button {
//!     label: String,
//!     disabled: bool,
//! }
//!
//! impl Component for Button {
//!     type Props = ButtonProps;
//!     type State = ();
//!
//!     fn create(props: Self::Props) -> Self {
//!         Self {
//!             label: props.label,
//!             disabled: props.disabled,
//!         }
//!     }
//!
//!     fn render(&self, area: Rect, buf: &mut Buffer) {}
//!     fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd { Cmd::Noop }
//! }
//!
//! // Usage
//! let button = ButtonProps::new("Click me")
//!     .disabled(true)
//!     .into_component();
//! ```

use crate::Component;

/// Trait for component configuration/props.
///
/// Props define the configuration passed when creating a component.
/// They must be `Clone` to support reconciliation and comparison during updates.
///
/// # Associated Types
///
/// - `Component`: The component type that these props configure
///
/// # Required Methods
///
/// - `into_component()`: Creates a component instance from the props
///
/// # Example
///
/// ```
/// use ctui_core::{Props, Component, Rect, Buffer, Cmd, Msg};
///
/// #[derive(Clone)]
/// struct MyProps {
///     value: i32,
/// }
///
/// impl Props for MyProps {
///     type Component = MyComponent;
///
///     fn into_component(self) -> Self::Component {
///         MyComponent::create(self)
///     }
/// }
///
/// struct MyComponent {
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
///     fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd { Cmd::Noop }
/// }
/// ```
pub trait Props: Clone {
    /// The component type created from these props.
    type Component: Component<Props = Self>;

    /// Creates a component instance from these props.
    ///
    /// This method bridges the props type and the component type,
    /// enabling ergonomic construction patterns.
    ///
    /// # Example
    ///
    /// ```
    /// use ctui_core::{Props, Component, Rect, Buffer, Cmd, Msg};
    ///
    /// #[derive(Clone)]
    /// struct TextProps {
    ///     content: String,
    /// }
    ///
    /// impl Props for TextProps {
    ///     type Component = Text;
    ///
    ///     fn into_component(self) -> Self::Component {
    ///         Text::create(self)
    ///     }
    /// }
    ///
    /// struct Text { content: String }
    /// impl Component for Text {
    ///     type Props = TextProps;
    ///     type State = ();
    ///     fn create(props: Self::Props) -> Self { Self { content: props.content } }
    ///     fn render(&self, area: Rect, buf: &mut Buffer) {}
    ///     fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd { Cmd::Noop }
    /// }
    ///
    /// // Usage - builder pattern is ergonomic and type-safe
    /// let text = TextProps { content: "Hello".to_string() }.into_component();
    /// ```
    fn into_component(self) -> Self::Component;
}

/// Default empty props for components without configuration.
///
/// Use `()` as `Props` type for stateless or static components that
/// don't require any configuration.
///
/// # Example
///
/// ```
/// use ctui_core::{DefaultProps, Component, Rect, Buffer, Cmd, Msg};
///
/// // Component that doesn't need props
/// struct Spacer;
///
/// impl Component for Spacer {
///     type Props = ();
///     type State = ();
///
///     fn create(_props: Self::Props) -> Self {
///         Self
///     }
///
///     fn render(&self, area: Rect, buf: &mut Buffer) {}
///     fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd { Cmd::Noop }
/// }
///
/// // Use unit type directly
/// let spacer = Spacer::create(());
/// ```
pub trait DefaultProps: Default + Clone + 'static {}

/// Blanket implementation of DefaultProps for unit type.
impl DefaultProps for () {}

/// A builder helper for constructing props with optional fields.
///
/// Use this when props have many optional fields and you want
/// ergonomic construction with defaults.
///
/// # Example
///
/// ```
/// use ctui_core::props::PropsBuilder;
///
/// #[derive(Clone, Default)]
/// struct MyProps {
///     name: String,
///     count: i32,
///     enabled: bool,
/// }
///
/// // Build with defaults, override as needed
/// let props = PropsBuilder::new(|| MyProps::default())
///     .with(|p| p.name = "test".to_string())
///     .with(|p| p.count = 42)
///     .build();
///
/// assert_eq!(props.name, "test");
/// assert_eq!(props.count, 42);
/// assert!(!props.enabled);
/// ```
pub struct PropsBuilder<P, F>
where
    P: Clone,
    F: FnOnce() -> P,
{
    factory: Option<F>,
    modifiers: Vec<Box<dyn FnOnce(&mut P)>>,
}

impl<P, F> PropsBuilder<P, F>
where
    P: Clone,
    F: FnOnce() -> P,
{
    /// Creates a new builder with a factory function.
    ///
    /// # Arguments
    ///
    /// * `factory` - A function that creates the initial props instance
    pub fn new(factory: F) -> Self {
        Self {
            factory: Some(factory),
            modifiers: Vec::new(),
        }
    }

    /// Adds a modifier function to customize the props.
    ///
    /// Modifiers are applied in order when `build()` is called.
    ///
    /// # Arguments
    ///
    /// * `modifier` - A function that modifies the props
    pub fn with<M>(mut self, modifier: M) -> Self
    where
        M: FnOnce(&mut P) + 'static,
    {
        self.modifiers.push(Box::new(modifier));
        self
    }

    /// Builds the final props instance.
    ///
    /// Applies all modifiers in sequence to the factory-created instance.
    pub fn build(mut self) -> P {
        let mut props = (self.factory.take().expect("factory already consumed"))();
        for modifier in self.modifiers {
            modifier(&mut props);
        }
        props
    }
}

/// Extension trait for props that provides builder-style chaining.
///
/// This trait is automatically implemented for all `Clone` types,
/// enabling fluent construction patterns.
///
/// # Example
///
/// ```
/// use ctui_core::props::PropsExt;
///
/// #[derive(Clone)]
/// struct Config {
///     name: String,
///     value: i32,
/// }
///
/// let config = Config { name: "default".to_string(), value: 0 }
///     .with(|c| c.name = "custom".to_string())
///     .with(|c| c.value = 42);
///
/// assert_eq!(config.name, "custom");
/// assert_eq!(config.value, 42);
/// ```
pub trait PropsExt: Clone + Sized {
    /// Apply a modification function and return self for chaining.
    fn with<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        let mut this = self;
        f(&mut this);
        this
    }
}

/// Blanket implementation for all Clone types.
impl<T: Clone + Sized> PropsExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Buffer, Cmd, Msg, Rect};

    /// Test component for Props testing
    #[derive(Clone)]
    struct TestProps {
        value: i32,
        name: String,
    }

    impl TestProps {
        fn new(value: i32) -> Self {
            Self {
                value,
                name: String::new(),
            }
        }

        fn name(mut self, name: impl Into<String>) -> Self {
            self.name = name.into();
            self
        }
    }

    impl Props for TestProps {
        type Component = TestComponent;

        fn into_component(self) -> Self::Component {
            TestComponent::create(self)
        }
    }

    struct TestComponent {
        value: i32,
        name: String,
    }

    impl Component for TestComponent {
        type Props = TestProps;
        type State = ();

        fn create(props: Self::Props) -> Self {
            Self {
                value: props.value,
                name: props.name,
            }
        }

        fn render(&self, _area: Rect, _buf: &mut Buffer) {}
        fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
            Cmd::Noop
        }
    }

    #[test]
    fn test_props_into_component() {
        let props = TestProps::new(42).name("test");
        let component = props.into_component();

        assert_eq!(component.value, 42);
        assert_eq!(component.name, "test");
    }

    #[test]
    fn test_props_clone() {
        let props = TestProps::new(10).name("original");
        let cloned = props.clone();

        assert_eq!(cloned.value, 10);
        assert_eq!(cloned.name, "original");
    }

    #[test]
    fn test_props_builder_basic() {
        #[derive(Clone, Default)]
        struct DerivedProps {
            a: i32,
            b: String,
            c: bool,
        }

        let props = PropsBuilder::new(DerivedProps::default)
            .with(|p| p.a = 100)
            .with(|p| p.b = "hello".to_string())
            .build();

        assert_eq!(props.a, 100);
        assert_eq!(props.b, "hello");
        assert!(!props.c);
    }

    #[test]
    fn test_props_builder_chaining() {
        #[derive(Clone)]
        struct ChainProps {
            x: i32,
            y: i32,
        }

        let props = PropsBuilder::new(|| ChainProps { x: 0, y: 0 })
            .with(|p| p.x = 1)
            .with(|p| p.y = 2)
            .with(|p| p.x += 10)
            .build();

        assert_eq!(props.x, 11);
        assert_eq!(props.y, 2);
    }

    #[test]
    fn test_props_ext_with() {
        #[derive(Clone)]
        struct ExtProps {
            value: i32,
            label: String,
        }

        let props = ExtProps {
            value: 0,
            label: String::new(),
        }
        .with(|p| {
            p.value = 99;
            p.label = "modified".to_string();
        });

        assert_eq!(props.value, 99);
        assert_eq!(props.label, "modified");
    }

    #[test]
    fn test_default_props_unit() {
        // Verify () satisfies DefaultProps
        fn accepts_default_props<P: DefaultProps>(_: P) {}
        accepts_default_props(());
    }

    #[test]
    fn test_unit_props_in_component() {
        struct SimpleComponent;

        impl Component for SimpleComponent {
            type Props = ();
            type State = ();

            fn create(_props: Self::Props) -> Self {
                Self
            }

            fn render(&self, _area: Rect, _buf: &mut Buffer) {}
            fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
                Cmd::Noop
            }
        }

        let _component = SimpleComponent::create(());
    }

    #[test]
    fn test_props_builder_empty_modifiers() {
        #[derive(Clone)]
        struct EmptyProps {
            value: i32,
        }

        let props = PropsBuilder::new(|| EmptyProps { value: 42 }).build();

        assert_eq!(props.value, 42);
    }

    /// Test that Props trait requires Clone
    #[test]
    fn test_props_clone_required() {
        // This test verifies compile-time that Props: Clone bound works
        fn requires_clone<P: Clone>(_: P) {}
        let props = TestProps::new(5);
        requires_clone(props.clone());
    }

    /// Test multiple modifications in PropsBuilder
    #[test]
    fn test_props_builder_multiple_modifications() {
        #[derive(Clone, Default)]
        struct MultiProps {
            a: i32,
            b: i32,
            c: i32,
        }

        let props = PropsBuilder::new(MultiProps::default)
            .with(|p| p.a = 1)
            .with(|p| p.b = 2)
            .with(|p| p.c = 3)
            .with(|p| {
                // Later modifications can depend on earlier ones
                p.a += p.b + p.c;
            })
            .build();

        assert_eq!(props.a, 6); // 1 + 2 + 3
        assert_eq!(props.b, 2);
        assert_eq!(props.c, 3);
    }
}
