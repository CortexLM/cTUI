//! Event system for terminal input handling
//!
//! This module provides types and traits for handling terminal events including
//! keyboard input, mouse events, resize events, and focus events.
//!
//! ## Kitty Keyboard Protocol Support
//!
//! This module supports the Kitty keyboard protocol for enhanced keyboard input:
//! - Key event kinds (Press, Repeat, Release)
//! - Extended modifiers (Hyper, Meta, CapsLock, NumLock)
//! - Distinguishing key-up vs key-down events
//!
//! Reference: <https://sw.kovidgoyal.net/kitty/keyboard-protocol/>

use std::fmt;

/// Represents the kind of key event (Kitty keyboard protocol)
///
/// The Kitty keyboard protocol can distinguish between different key event
/// states, allowing applications to respond differently to press, repeat,
/// and release events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum KeyEventKind {
    /// Key was pressed down
    #[default]
    Press,
    /// Key is being held down (auto-repeat)
    Repeat,
    /// Key was released
    Release,
}

impl KeyEventKind {
    /// Returns true if this is a press event
    #[must_use]
    pub const fn is_press(&self) -> bool {
        matches!(self, Self::Press)
    }

    /// Returns true if this is a repeat event
    #[must_use]
    pub const fn is_repeat(&self) -> bool {
        matches!(self, Self::Repeat)
    }

    /// Returns true if this is a release event
    #[must_use]
    pub const fn is_release(&self) -> bool {
        matches!(self, Self::Release)
    }
}

/// Represents a key code for keyboard events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    /// A character key (letters, numbers, symbols)
    Char(char),
    /// Function key (F1-F12)
    F(u8),
    /// Up arrow key
    Up,
    /// Down arrow key
    Down,
    /// Left arrow key
    Left,
    /// Right arrow key
    Right,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,
    /// Insert key
    Insert,
    /// Delete key
    Delete,
    /// Backspace key
    Backspace,
    /// Enter/Return key
    Enter,
    /// Tab key
    Tab,
    /// Escape key
    Esc,
    /// Null key (no key pressed)
    Null,
}

/// Represents key modifiers (Shift, Ctrl, Alt, etc.)
///
/// This struct supports both standard modifiers (Shift, Ctrl, Alt, Super)
/// and extended modifiers from the Kitty keyboard protocol (Hyper, Meta,
/// CapsLock state, NumLock state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[must_use]
pub struct KeyModifiers {
    /// Shift key is pressed
    pub shift: bool,
    /// Control key is pressed
    pub ctrl: bool,
    /// Alt/Option key is pressed
    pub alt: bool,
    /// Super/Windows/Command key is pressed
    pub super_key: bool,
    /// Hyper modifier (Kitty protocol extension)
    pub hyper: bool,
    /// Meta modifier (Kitty protocol extension, distinct from Alt)
    pub meta: bool,
    /// Caps Lock state (Kitty protocol extension)
    pub caps_lock: bool,
    /// Num Lock state (Kitty protocol extension)
    pub num_lock: bool,
}

impl KeyModifiers {
    /// Creates a new KeyModifiers with no modifiers.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            super_key: false,
            hyper: false,
            meta: false,
            caps_lock: false,
            num_lock: false,
        }
    }

    /// Returns true if no modifiers are pressed.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        !self.shift && !self.ctrl && !self.alt && !self.super_key
            && !self.hyper && !self.meta && !self.caps_lock && !self.num_lock
    }

    /// Returns true if only standard modifiers are set (no Kitty extensions)
    #[must_use]
    pub const fn is_standard_only(&self) -> bool {
        !self.hyper && !self.meta && !self.caps_lock && !self.num_lock
    }

    /// Adds the shift modifier.
    #[must_use]
    pub const fn with_shift(mut self) -> Self {
        self.shift = true;
        self
    }

    /// Adds the ctrl modifier.
    #[must_use]
    pub const fn with_ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }

    /// Adds the alt modifier.
    #[must_use]
    pub const fn with_alt(mut self) -> Self {
        self.alt = true;
        self
    }

    /// Adds the super modifier.
    #[must_use]
    pub const fn with_super(mut self) -> Self {
        self.super_key = true;
        self
    }

    /// Adds the hyper modifier (Kitty protocol).
    #[must_use]
    pub const fn with_hyper(mut self) -> Self {
        self.hyper = true;
        self
    }

    /// Adds the meta modifier (Kitty protocol).
    #[must_use]
    pub const fn with_meta(mut self) -> Self {
        self.meta = true;
        self
    }

    /// Sets caps lock state (Kitty protocol).
    #[must_use]
    pub const fn with_caps_lock(mut self) -> Self {
        self.caps_lock = true;
        self
    }

    /// Sets num lock state (Kitty protocol).
    #[must_use]
    pub const fn with_num_lock(mut self) -> Self {
        self.num_lock = true;
        self
    }
}

/// A keyboard event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    /// The key that was pressed
    pub code: KeyCode,
    /// Any modifiers that were active
    pub modifiers: KeyModifiers,
    /// The kind of key event (Press, Repeat, Release)
    pub kind: KeyEventKind,
}

impl KeyEvent {
    #[must_use]
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { 
            code, 
            modifiers, 
            kind: KeyEventKind::Press,
        }
    }

    /// Creates a KeyEvent with a specific kind
    #[must_use]
    pub const fn with_kind(code: KeyCode, modifiers: KeyModifiers, kind: KeyEventKind) -> Self {
        Self { code, modifiers, kind }
    }

    #[must_use]
    pub const fn from(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::new())
    }

    /// Returns true if this is a press event
    #[must_use]
    pub const fn is_press(&self) -> bool {
        self.kind.is_press()
    }

    /// Returns true if this is a repeat event
    #[must_use]
    pub const fn is_repeat(&self) -> bool {
        self.kind.is_repeat()
    }

    /// Returns true if this is a release event
    #[must_use]
    pub const fn is_release(&self) -> bool {
        self.kind.is_release()
    }
}

/// Represents mouse button state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button (scroll wheel click)
    Middle,
    /// Scroll wheel up
    WheelUp,
    /// Scroll wheel down
    WheelDown,
}

/// A mouse event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseEvent {
    /// The column position (0-indexed)
    pub column: u16,
    /// The row position (0-indexed)
    pub row: u16,
    /// The button that was pressed/released
    pub button: MouseButton,
    /// Any modifiers that were active
    pub modifiers: KeyModifiers,
}

impl MouseEvent {
    #[must_use]
    pub const fn new(column: u16, row: u16, button: MouseButton, modifiers: KeyModifiers) -> Self {
        Self {
            column,
            row,
            button,
            modifiers,
        }
    }
}

/// Represents the type of mouse event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseEventKind {
    /// Mouse button was pressed down
    Down(MouseButton),
    /// Mouse button was released
    Up(MouseButton),
    /// Mouse was moved while a button was held (drag)
    Drag(MouseButton),
    /// Mouse was moved without any button
    Moved,
    /// Mouse scroll wheel
    Scroll,
}

/// A terminal resize event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResizeEvent {
    /// New width in columns
    pub width: u16,
    /// New height in rows
    pub height: u16,
}

impl ResizeEvent {
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

/// A focus event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusEvent {
    /// Terminal gained focus
    Gained,
    /// Terminal lost focus
    Lost,
}

/// Represents an event from the terminal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    /// Keyboard event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize event
    Resize(ResizeEvent),
    /// Focus event
    Focus(FocusEvent),
    /// Paste event (for bracketed paste)
    Paste(String),
}

impl Event {
    #[must_use]
    pub const fn key(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self::Key(KeyEvent::new(code, modifiers))
    }

    #[must_use]
    pub const fn mouse(
        column: u16,
        row: u16,
        button: MouseButton,
        modifiers: KeyModifiers,
    ) -> Self {
        Self::Mouse(MouseEvent::new(column, row, button, modifiers))
    }

    #[must_use]
    pub const fn resize(width: u16, height: u16) -> Self {
        Self::Resize(ResizeEvent::new(width, height))
    }

    #[must_use]
    pub const fn is_key(&self) -> bool {
        matches!(self, Self::Key(_))
    }

    #[must_use]
    pub const fn is_mouse(&self) -> bool {
        matches!(self, Self::Mouse(_))
    }

    #[must_use]
    pub const fn is_resize(&self) -> bool {
        matches!(self, Self::Resize(_))
    }

    #[must_use]
    pub const fn is_focus(&self) -> bool {
        matches!(self, Self::Focus(_))
    }
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(c) => write!(f, "{}", c),
            Self::F(n) => write!(f, "F{}", n),
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
            Self::Home => write!(f, "Home"),
            Self::End => write!(f, "End"),
            Self::PageUp => write!(f, "PageUp"),
            Self::PageDown => write!(f, "PageDown"),
            Self::Insert => write!(f, "Insert"),
            Self::Delete => write!(f, "Delete"),
            Self::Backspace => write!(f, "Backspace"),
            Self::Enter => write!(f, "Enter"),
            Self::Tab => write!(f, "Tab"),
            Self::Esc => write!(f, "Esc"),
            Self::Null => write!(f, "Null"),
        }
    }
}

impl fmt::Display for KeyEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Press => write!(f, "Press"),
            Self::Repeat => write!(f, "Repeat"),
            Self::Release => write!(f, "Release"),
        }
    }
}

/// Trait for handling events.
///
/// Implement this trait to receive events from the terminal.
pub trait EventHandler {
    /// Called when an event is received.
    ///
    /// Returns `true` if the event was handled and should not be
    /// propagated further.
    fn handle(&mut self, event: &Event) -> bool;
}

/// A simple event handler that uses a closure.
pub struct FnEventHandler<F>(pub F);

impl<F: FnMut(&Event) -> bool> EventHandler for FnEventHandler<F> {
    fn handle(&mut self, event: &Event) -> bool {
        (self.0)(event)
    }
}

/// Async event stream for reading events with tokio.
///
/// This wraps crossterm's event-stream feature and provides a convenient
/// async iterator for events.
#[cfg(feature = "event-stream")]
pub use crossterm::event::EventStream;

/// Kitty keyboard protocol parser module.
pub mod kitty;
pub mod batcher;

#[cfg(feature = "event-stream")]
impl From<crossterm::event::Event> for Event {
    fn from(event: crossterm::event::Event) -> Self {
        match event {
            crossterm::event::Event::Key(key) => {
                let code = match key.code {
                    crossterm::event::KeyCode::Char(c) => KeyCode::Char(c),
                    crossterm::event::KeyCode::F(n) => KeyCode::F(n),
                    crossterm::event::KeyCode::Up => KeyCode::Up,
                    crossterm::event::KeyCode::Down => KeyCode::Down,
                    crossterm::event::KeyCode::Left => KeyCode::Left,
                    crossterm::event::KeyCode::Right => KeyCode::Right,
                    crossterm::event::KeyCode::Home => KeyCode::Home,
                    crossterm::event::KeyCode::End => KeyCode::End,
                    crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
                    crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
                    crossterm::event::KeyCode::Insert => KeyCode::Insert,
                    crossterm::event::KeyCode::Delete => KeyCode::Delete,
                    crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
                    crossterm::event::KeyCode::Enter => KeyCode::Enter,
                    crossterm::event::KeyCode::Tab => KeyCode::Tab,
                    crossterm::event::KeyCode::Esc => KeyCode::Esc,
                    crossterm::event::KeyCode::Null => KeyCode::Null,
                    _ => KeyCode::Null,
                };
                let modifiers = KeyModifiers {
                    shift: key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::SHIFT),
                    ctrl: key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL),
                    alt: key.modifiers.contains(crossterm::event::KeyModifiers::ALT),
                    super_key: key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::SUPER),
                    hyper: false,
                    meta: false,
                    caps_lock: false,
                    num_lock: false,
                };
                let kind = match key.kind {
                    crossterm::event::KeyEventKind::Press => KeyEventKind::Press,
                    crossterm::event::KeyEventKind::Repeat => KeyEventKind::Repeat,
                    crossterm::event::KeyEventKind::Release => KeyEventKind::Release,
                };
                Event::Key(KeyEvent::with_kind(code, modifiers, kind))
            }
            crossterm::event::Event::Mouse(mouse) => {
                let button = match mouse.kind {
                    crossterm::event::MouseEventKind::Down(b) => match b {
                        crossterm::event::MouseButton::Left => MouseButton::Left,
                        crossterm::event::MouseButton::Right => MouseButton::Right,
                        crossterm::event::MouseButton::Middle => MouseButton::Middle,
                    },
                    crossterm::event::MouseEventKind::Up(b) => match b {
                        crossterm::event::MouseButton::Left => MouseButton::Left,
                        crossterm::event::MouseButton::Right => MouseButton::Right,
                        crossterm::event::MouseButton::Middle => MouseButton::Middle,
                    },
                    crossterm::event::MouseEventKind::Drag(b) => match b {
                        crossterm::event::MouseButton::Left => MouseButton::Left,
                        crossterm::event::MouseButton::Right => MouseButton::Right,
                        crossterm::event::MouseButton::Middle => MouseButton::Middle,
                    },
                    crossterm::event::MouseEventKind::ScrollUp => MouseButton::WheelUp,
                    crossterm::event::MouseEventKind::ScrollDown => MouseButton::WheelDown,
                    _ => MouseButton::Left,
                };
                let modifiers = KeyModifiers {
                    shift: mouse
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::SHIFT),
                    ctrl: mouse
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL),
                    alt: mouse
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::ALT),
                    super_key: mouse
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::SUPER),
                    hyper: false,
                    meta: false,
                    caps_lock: false,
                    num_lock: false,
                };
                Event::Mouse(MouseEvent::new(mouse.column, mouse.row, button, modifiers))
            }
            crossterm::event::Event::Resize(width, height) => {
                Event::Resize(ResizeEvent::new(width, height))
            }
            crossterm::event::Event::FocusGained => Event::Focus(FocusEvent::Gained),
            crossterm::event::Event::FocusLost => Event::Focus(FocusEvent::Lost),
            crossterm::event::Event::Paste(s) => Event::Paste(s),
        }
    }
}

/// Kitty keyboard protocol tests.
///
/// These tests cover the Kitty keyboard protocol extension which provides:
/// - Key event kinds (Press, Repeat, Release)
/// - Extended modifiers (Hyper, Meta, CapsLock, NumLock)
/// - Distinguishing key-up vs key-down events
///
/// Reference: https://sw.kovidgoyal.net/kitty/keyboard-protocol/
#[cfg(test)]
mod kitty_test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_code_display() {
        assert_eq!(KeyCode::Char('a').to_string(), "a");
        assert_eq!(KeyCode::F(1).to_string(), "F1");
        assert_eq!(KeyCode::Up.to_string(), "Up");
        assert_eq!(KeyCode::Enter.to_string(), "Enter");
    }

    #[test]
    fn test_key_event_kind() {
        assert!(KeyEventKind::Press.is_press());
        assert!(!KeyEventKind::Press.is_repeat());
        assert!(!KeyEventKind::Press.is_release());

        assert!(!KeyEventKind::Repeat.is_press());
        assert!(KeyEventKind::Repeat.is_repeat());
        assert!(!KeyEventKind::Repeat.is_release());

        assert!(!KeyEventKind::Release.is_press());
        assert!(!KeyEventKind::Release.is_repeat());
        assert!(KeyEventKind::Release.is_release());

        assert_eq!(KeyEventKind::default(), KeyEventKind::Press);
    }

    #[test]
    fn test_key_event_kind_display() {
        assert_eq!(KeyEventKind::Press.to_string(), "Press");
        assert_eq!(KeyEventKind::Repeat.to_string(), "Repeat");
        assert_eq!(KeyEventKind::Release.to_string(), "Release");
    }

    #[test]
    fn test_key_modifiers_empty() {
        let mods = KeyModifiers::new();
        assert!(mods.is_empty());
        assert!(!mods.with_shift().is_empty());
    }

    #[test]
    fn test_key_modifiers_extended() {
        let mods = KeyModifiers::new()
            .with_hyper()
            .with_meta()
            .with_caps_lock()
            .with_num_lock();
        
        assert!(mods.hyper);
        assert!(mods.meta);
        assert!(mods.caps_lock);
        assert!(mods.num_lock);
        assert!(!mods.is_standard_only());
    }

    #[test]
    fn test_key_modifiers_chain() {
        let mods = KeyModifiers::new().with_shift().with_ctrl().with_alt();
        assert!(mods.shift);
        assert!(mods.ctrl);
        assert!(mods.alt);
        assert!(!mods.super_key);
    }

    #[test]
    fn test_key_event_new() {
        let event = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::new().with_ctrl());
        assert_eq!(event.code, KeyCode::Char('x'));
        assert!(event.modifiers.ctrl);
        assert_eq!(event.kind, KeyEventKind::Press);
    }

    #[test]
    fn test_key_event_with_kind() {
        let event = KeyEvent::with_kind(
            KeyCode::Char('a'),
            KeyModifiers::new(),
            KeyEventKind::Release,
        );
        assert!(event.is_release());
        assert!(!event.is_press());
    }

    #[test]
    fn test_key_event_from() {
        let event = KeyEvent::from(KeyCode::Enter);
        assert_eq!(event.code, KeyCode::Enter);
        assert!(event.modifiers.is_empty());
    }

    #[test]
    fn test_mouse_event_new() {
        let event = MouseEvent::new(10, 20, MouseButton::Left, KeyModifiers::new());
        assert_eq!(event.column, 10);
        assert_eq!(event.row, 20);
        assert_eq!(event.button, MouseButton::Left);
    }

    #[test]
    fn test_resize_event_new() {
        let event = ResizeEvent::new(80, 24);
        assert_eq!(event.width, 80);
        assert_eq!(event.height, 24);
    }

    #[test]
    fn test_event_constructors() {
        let key_event = Event::key(KeyCode::Char('a'), KeyModifiers::new());
        assert!(key_event.is_key());

        let mouse_event = Event::mouse(5, 10, MouseButton::Right, KeyModifiers::new());
        assert!(mouse_event.is_mouse());

        let resize_event = Event::resize(100, 50);
        assert!(resize_event.is_resize());

        let focus_event = Event::Focus(FocusEvent::Gained);
        assert!(focus_event.is_focus());
    }

    #[test]
    fn test_event_predicate_methods() {
        assert!(!Event::key(KeyCode::Up, KeyModifiers::new()).is_mouse());
        assert!(!Event::mouse(0, 0, MouseButton::Left, KeyModifiers::new()).is_key());
        assert!(!Event::resize(80, 24).is_focus());
        assert!(!Event::Focus(FocusEvent::Lost).is_resize());
    }

    #[test]
    fn test_fn_event_handler() {
        let mut handler = FnEventHandler(|event: &Event| {
            matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    ..
                })
            )
        });

        assert!(handler.handle(&Event::key(KeyCode::Esc, KeyModifiers::new())));
        assert!(!handler.handle(&Event::key(KeyCode::Char('a'), KeyModifiers::new())));
    }
}
