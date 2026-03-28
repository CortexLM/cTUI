//! Event conversion utilities for WebAssembly.
//!
//! This module provides functions to convert web-sys events (`KeyboardEvent`, `MouseEvent`)
//! into cTUI's native Event types for seamless integration with the TUI framework.

use ctui_core::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
};
use web_sys::{KeyboardEvent, MouseEvent as WebMouseEvent};

/// Converts a web-sys `KeyboardEvent` to a cTUI `Event`.
///
/// This function maps browser keyboard events to cTUI's `KeyEvent` structure,
/// handling special keys, function keys, modifiers, and character keys.
///
/// # Arguments
///
/// * `js_event` - The web-sys `KeyboardEvent` from the browser
///
/// # Returns
///
/// A cTUI `Event::Key` containing the mapped key code and modifiers.
///
/// # Example
///
/// ```ignore
/// use wasm_bindgen::prelude::*;
/// use web_sys::KeyboardEvent;
/// use ctui_wasm::events::keyboard_event_to_key;
///
/// #[wasm_bindgen]
/// pub fn on_keydown(event: KeyboardEvent) {
///     let ctui_event = keyboard_event_to_key(&event);
///     // Handle the event in your TUI app
/// }
/// ```
#[must_use]
pub fn keyboard_event_to_key(js_event: &KeyboardEvent) -> Event {
    let code = map_key_code(js_event);
    let modifiers = extract_modifiers(js_event);
    let kind = KeyEventKind::Press; // web-sys doesn't distinguish Press/Repeat/Release directly

    Event::Key(KeyEvent::with_kind(code, modifiers, kind))
}

/// Converts a web-sys `MouseEvent` to a cTUI `Event`.
///
/// This function maps browser mouse events to cTUI's `MouseEvent` structure,
/// converting pixel coordinates to cell coordinates and mapping button states.
///
/// # Arguments
///
/// * `js_event` - The web-sys `MouseEvent` from the browser
/// * `char_width` - Width of a single character cell in pixels
/// * `char_height` - Height of a single character cell in pixels
///
/// # Returns
///
/// A cTUI `Event::Mouse` containing the column, row, button, and modifiers.
///
/// # Example
///
/// ```ignore
/// use wasm_bindgen::prelude::*;
/// use web_sys::MouseEvent;
/// use ctui_wasm::events::mouse_event_to_mouse;
///
/// #[wasm_bindgen]
/// pub fn on_click(event: MouseEvent) {
///     let char_width = 10.0; // pixels per character
///     let char_height = 16.0; // pixels per line
///     let ctui_event = mouse_event_to_mouse(&event, char_width, char_height);
///     // Handle the event in your TUI app
/// }
/// ```
#[must_use]
pub fn mouse_event_to_mouse(
    js_event: &WebMouseEvent,
    char_width: f64,
    char_height: f64,
) -> Event {
    // Intentional truncation: screen coordinates are always positive and fit in u16
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let column = (f64::from(js_event.client_x()) / char_width).floor() as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let row = (f64::from(js_event.client_y()) / char_height).floor() as u16;
    let button = map_mouse_button(js_event.button());
    let modifiers = extract_mouse_modifiers(js_event);

    Event::Mouse(MouseEvent::new(column, row, button, modifiers))
}

/// Maps a web-sys `KeyboardEvent` to a `KeyCode`.
///
/// Uses the `code` property for physical key identification (layout-independent)
/// and falls back to `key` for character keys.
fn map_key_code(event: &KeyboardEvent) -> KeyCode {
    let code = event.code();

    // Map physical key codes first (layout-independent)
    match code.as_str() {
        // Arrow keys
        "ArrowUp" => KeyCode::Up,
        "ArrowDown" => KeyCode::Down,
        "ArrowLeft" => KeyCode::Left,
        "ArrowRight" => KeyCode::Right,

        // Navigation keys
        "Home" => KeyCode::Home,
        "End" => KeyCode::End,
        "PageUp" => KeyCode::PageUp,
        "PageDown" => KeyCode::PageDown,

        // Editing keys
        "Insert" => KeyCode::Insert,
        "Delete" => KeyCode::Delete,
        "Backspace" => KeyCode::Backspace,

        // Action keys
        "Enter" | "NumpadEnter" => KeyCode::Enter,
        "Tab" => KeyCode::Tab,
        "Escape" => KeyCode::Esc,

        // Function keys
        "F1" => KeyCode::F(1),
        "F2" => KeyCode::F(2),
        "F3" => KeyCode::F(3),
        "F4" => KeyCode::F(4),
        "F5" => KeyCode::F(5),
        "F6" => KeyCode::F(6),
        "F7" => KeyCode::F(7),
        "F8" => KeyCode::F(8),
        "F9" => KeyCode::F(9),
        "F10" => KeyCode::F(10),
        "F11" => KeyCode::F(11),
        "F12" => KeyCode::F(12),

        // Numpad keys (treat as their primary equivalents)
        "NumpadAdd" => KeyCode::Char('+'),
        "NumpadSubtract" => KeyCode::Char('-'),
        "NumpadMultiply" => KeyCode::Char('*'),
        "NumpadDivide" => KeyCode::Char('/'),
        "NumpadDecimal" => KeyCode::Char('.'),
        "Numpad0" => KeyCode::Char('0'),
        "Numpad1" => KeyCode::Char('1'),
        "Numpad2" => KeyCode::Char('2'),
        "Numpad3" => KeyCode::Char('3'),
        "Numpad4" => KeyCode::Char('4'),
        "Numpad5" => KeyCode::Char('5'),
        "Numpad6" => KeyCode::Char('6'),
        "Numpad7" => KeyCode::Char('7'),
        "Numpad8" => KeyCode::Char('8'),
        "Numpad9" => KeyCode::Char('9'),

        // For letter keys, use the key property for the actual character
        // (respects shift and layout)
        _ => {
            let key = event.key();

            // Handle special key values
            match key.as_str() {
                // Single character keys (letters, numbers, symbols)
                k if k.len() == 1 => {
                    let ch = k.chars().next().unwrap_or('\0');
                    KeyCode::Char(ch)
                }

                // Unrecognized key
                _ => KeyCode::Null,
            }
        }
    }
}

/// Extracts `KeyModifiers` from a web-sys `KeyboardEvent`.
fn extract_modifiers(event: &KeyboardEvent) -> KeyModifiers {
    KeyModifiers {
        shift: event.shift_key(),
        ctrl: event.ctrl_key(),
        alt: event.alt_key(),
        super_key: event.meta_key(), // Meta key is Command on macOS, Windows key on Windows
        hyper: false, // Web doesn't have hyper key
        meta: false, // Web doesn't distinguish meta from alt
        caps_lock: event.get_modifier_state("CapsLock"),
        num_lock: event.get_modifier_state("NumLock"),
    }
}

/// Maps a web-sys mouse button code to a `MouseButton`.
///
/// Web button codes:
/// - 0: Left button (primary)
/// - 1: Middle button (auxiliary)
/// - 2: Right button (secondary)
/// - 3: Back button (browser back)
/// - 4: Forward button (browser forward)
const fn map_mouse_button(button: i16) -> MouseButton {
    match button {
        // Explicitly handle known button codes for documentation
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        // Default to Left for button code 0 and any unknown/future codes
        _ => MouseButton::Left,
    }
}

/// Extracts `KeyModifiers` from a web-sys `MouseEvent`.
fn extract_mouse_modifiers(event: &WebMouseEvent) -> KeyModifiers {
    KeyModifiers {
        shift: event.shift_key(),
        ctrl: event.ctrl_key(),
        alt: event.alt_key(),
        super_key: event.meta_key(),
        hyper: false,
        meta: false,
        caps_lock: false, // Mouse events don't provide caps_lock state
        num_lock: false,  // Mouse events don't provide num_lock state
    }
}

/// Creates a mouse wheel scroll event.
///
/// Browser wheel events are typically separate from click events.
/// Use this function to convert wheel delta to scroll events.
///
/// # Arguments
///
/// * `delta_y` - The vertical scroll amount (negative = up, positive = down)
///
/// # Returns
///
/// A cTUI `Event::Mouse` with `MouseButton::WheelUp` or `WheelDown`.
#[must_use]
pub fn wheel_event_to_scroll(delta_y: f64, char_width: f64, char_height: f64, x: i32, y: i32) -> Event {
    // Intentional truncation: screen coordinates are always positive and fit in u16
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let column = (f64::from(x) / char_width).floor() as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let row = (f64::from(y) / char_height).floor() as u16;
    let button = if delta_y < 0.0 {
        MouseButton::WheelUp
    } else {
        MouseButton::WheelDown
    };

    Event::Mouse(MouseEvent::new(column, row, button, KeyModifiers::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Testing web-sys types requires a headless browser environment.
    // These tests document the expected behavior.

    #[test]
    fn test_map_mouse_button() {
        assert_eq!(map_mouse_button(0), MouseButton::Left);
        assert_eq!(map_mouse_button(1), MouseButton::Middle);
        assert_eq!(map_mouse_button(2), MouseButton::Right);
        assert_eq!(map_mouse_button(3), MouseButton::Left); // Unknown defaults to Left
        assert_eq!(map_mouse_button(-1), MouseButton::Left); // Negative defaults to Left
    }

    #[test]
    fn test_wheel_event_to_scroll() {
        let scroll_up = wheel_event_to_scroll(-10.0, 10.0, 16.0, 100, 50);
        let scroll_down = wheel_event_to_scroll(10.0, 10.0, 16.0, 100, 50);

        match scroll_up {
            Event::Mouse(me) => {
                assert_eq!(me.button, MouseButton::WheelUp);
                assert_eq!(me.column, 10);
                assert_eq!(me.row, 3);
            }
            _ => panic!("Expected Mouse event"),
        }

        match scroll_down {
            Event::Mouse(me) => {
                assert_eq!(me.button, MouseButton::WheelDown);
            }
            _ => panic!("Expected Mouse event"),
        }
    }

    // ===========================================
    // KeyCode Tests
    // ===========================================

    #[test]
    fn test_keycode_letter_keys() {
        // Test that KeyCode::Char can represent all letter keys
        assert!(matches!(KeyCode::Char('a'), KeyCode::Char('a')));
        assert!(matches!(KeyCode::Char('b'), KeyCode::Char('b')));
        assert!(matches!(KeyCode::Char('z'), KeyCode::Char('z')));
        assert!(matches!(KeyCode::Char('A'), KeyCode::Char('A')));
        assert!(matches!(KeyCode::Char('Z'), KeyCode::Char('Z')));
    }

    #[test]
    fn test_keycode_number_keys() {
        // Test that KeyCode::Char can represent all number keys
        assert!(matches!(KeyCode::Char('0'), KeyCode::Char('0')));
        assert!(matches!(KeyCode::Char('5'), KeyCode::Char('5')));
        assert!(matches!(KeyCode::Char('9'), KeyCode::Char('9')));
    }

    #[test]
    fn test_keycode_symbol_keys() {
        // Test that KeyCode::Char can represent symbol keys
        assert!(matches!(KeyCode::Char(' '), KeyCode::Char(' ')));
        assert!(matches!(KeyCode::Char('!'), KeyCode::Char('!')));
        assert!(matches!(KeyCode::Char('@'), KeyCode::Char('@')));
        assert!(matches!(KeyCode::Char('#'), KeyCode::Char('#')));
        assert!(matches!(KeyCode::Char('$'), KeyCode::Char('$')));
        assert!(matches!(KeyCode::Char('%'), KeyCode::Char('%')));
        assert!(matches!(KeyCode::Char('^'), KeyCode::Char('^')));
        assert!(matches!(KeyCode::Char('&'), KeyCode::Char('&')));
        assert!(matches!(KeyCode::Char('*'), KeyCode::Char('*')));
        assert!(matches!(KeyCode::Char('('), KeyCode::Char('(')));
    }

    #[test]
    fn test_keycode_function_keys_f1_to_f12() {
        // Test all function keys F1-F12
        assert!(matches!(KeyCode::F(1), KeyCode::F(1)));
        assert!(matches!(KeyCode::F(2), KeyCode::F(2)));
        assert!(matches!(KeyCode::F(3), KeyCode::F(3)));
        assert!(matches!(KeyCode::F(4), KeyCode::F(4)));
        assert!(matches!(KeyCode::F(5), KeyCode::F(5)));
        assert!(matches!(KeyCode::F(6), KeyCode::F(6)));
        assert!(matches!(KeyCode::F(7), KeyCode::F(7)));
        assert!(matches!(KeyCode::F(8), KeyCode::F(8)));
        assert!(matches!(KeyCode::F(9), KeyCode::F(9)));
        assert!(matches!(KeyCode::F(10), KeyCode::F(10)));
        assert!(matches!(KeyCode::F(11), KeyCode::F(11)));
        assert!(matches!(KeyCode::F(12), KeyCode::F(12)));
    }

    #[test]
    fn test_keycode_arrow_keys() {
        // Test arrow key codes
        assert!(matches!(KeyCode::Up, KeyCode::Up));
        assert!(matches!(KeyCode::Down, KeyCode::Down));
        assert!(matches!(KeyCode::Left, KeyCode::Left));
        assert!(matches!(KeyCode::Right, KeyCode::Right));
    }

    #[test]
    fn test_keycode_navigation_keys() {
        // Test navigation keys: Home, End, PageUp, PageDown
        assert!(matches!(KeyCode::Home, KeyCode::Home));
        assert!(matches!(KeyCode::End, KeyCode::End));
        assert!(matches!(KeyCode::PageUp, KeyCode::PageUp));
        assert!(matches!(KeyCode::PageDown, KeyCode::PageDown));
    }

    #[test]
    fn test_keycode_editing_keys() {
        // Test editing keys: Insert, Delete, Backspace
        assert!(matches!(KeyCode::Insert, KeyCode::Insert));
        assert!(matches!(KeyCode::Delete, KeyCode::Delete));
        assert!(matches!(KeyCode::Backspace, KeyCode::Backspace));
    }

    #[test]
    fn test_keycode_action_keys() {
        // Test action keys: Enter, Tab, Esc
        assert!(matches!(KeyCode::Enter, KeyCode::Enter));
        assert!(matches!(KeyCode::Tab, KeyCode::Tab));
        assert!(matches!(KeyCode::Esc, KeyCode::Esc));
    }

    #[test]
    fn test_keycode_null() {
        // Null represents unrecognized/missing key
        assert!(matches!(KeyCode::Null, KeyCode::Null));
    }

    // ===========================================
    // KeyModifiers Tests
    // ===========================================

    #[test]
    fn test_key_modifiers_default_all_false() {
        let mods = KeyModifiers::default();
        assert!(!mods.shift);
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.super_key);
        assert!(!mods.hyper);
        assert!(!mods.meta);
        assert!(!mods.caps_lock);
        assert!(!mods.num_lock);
    }

    #[test]
    fn test_key_modifiers_new_empty() {
        let mods = KeyModifiers::new();
        assert!(!mods.shift);
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.super_key);
    }

    #[test]
    fn test_key_modifiers_shift_only() {
        let mods = KeyModifiers { shift: true, ..KeyModifiers::default() };
        assert!(mods.shift);
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.super_key);
    }

    #[test]
    fn test_key_modifiers_ctrl_only() {
        let mods = KeyModifiers { ctrl: true, ..KeyModifiers::default() };
        assert!(!mods.shift);
        assert!(mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.super_key);
    }

    #[test]
    fn test_key_modifiers_alt_only() {
        let mods = KeyModifiers { alt: true, ..KeyModifiers::default() };
        assert!(!mods.shift);
        assert!(!mods.ctrl);
        assert!(mods.alt);
        assert!(!mods.super_key);
    }

    #[test]
    fn test_key_modifiers_super_key() {
        // Meta key (super_key) is Command on macOS, Windows key on Windows
        let mods = KeyModifiers { super_key: true, ..KeyModifiers::default() };
        assert!(!mods.shift);
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(mods.super_key);
    }

    #[test]
    fn test_key_modifiers_caps_lock() {
        let mods = KeyModifiers { caps_lock: true, ..KeyModifiers::default() };
        assert!(mods.caps_lock);
        assert!(!mods.shift);
        assert!(!mods.ctrl);
    }

    #[test]
    fn test_key_modifiers_num_lock() {
        let mods = KeyModifiers { num_lock: true, ..KeyModifiers::default() };
        assert!(mods.num_lock);
        assert!(!mods.shift);
        assert!(!mods.ctrl);
    }

    #[test]
    fn test_key_modifiers_shift_ctrl() {
        // Common combination: Ctrl+Shift
        let mods = KeyModifiers {
            shift: true,
            ctrl: true,
            ..KeyModifiers::default()
        };
        assert!(mods.shift);
        assert!(mods.ctrl);
        assert!(!mods.alt);
    }

    #[test]
    fn test_key_modifiers_ctrl_alt() {
        // Ctrl+Alt combination
        let mods = KeyModifiers {
            ctrl: true,
            alt: true,
            ..KeyModifiers::default()
        };
        assert!(!mods.shift);
        assert!(mods.ctrl);
        assert!(mods.alt);
    }

    #[test]
    fn test_key_modifiers_ctrl_shift_alt() {
        // Three modifier combination
        let mods = KeyModifiers {
            shift: true,
            ctrl: true,
            alt: true,
            ..KeyModifiers::default()
        };
        assert!(mods.shift);
        assert!(mods.ctrl);
        assert!(mods.alt);
    }

    #[test]
    fn test_key_modifiers_all_true() {
        let mods = KeyModifiers {
            shift: true,
            ctrl: true,
            alt: true,
            super_key: true,
            hyper: true,
            meta: true,
            caps_lock: true,
            num_lock: true,
        };
        assert!(mods.shift);
        assert!(mods.ctrl);
        assert!(mods.alt);
        assert!(mods.super_key);
        assert!(mods.hyper);
        assert!(mods.meta);
        assert!(mods.caps_lock);
        assert!(mods.num_lock);
    }

    // ===========================================
    // KeyEvent Tests
    // ===========================================

    #[test]
    fn test_key_event_new_basic() {
        let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::default());
        assert!(matches!(event.code, KeyCode::Char('a')));
        assert_eq!(event.kind, KeyEventKind::Press);
        assert!(!event.modifiers.shift);
        assert!(!event.modifiers.ctrl);
    }

    #[test]
    fn test_key_event_with_kind() {
        let mods = KeyModifiers { shift: true, ..KeyModifiers::default() };
        let event = KeyEvent::with_kind(KeyCode::Up, mods, KeyEventKind::Press);
        assert!(matches!(event.code, KeyCode::Up));
        assert!(event.modifiers.shift);
        assert_eq!(event.kind, KeyEventKind::Press);
    }

    #[test]
    fn test_key_event_from_keycode() {
        let event = KeyEvent::from(KeyCode::Enter);
        assert!(matches!(event.code, KeyCode::Enter));
        assert!(!event.modifiers.shift);
        assert!(!event.modifiers.ctrl);
        assert!(!event.modifiers.alt);
    }

    #[test]
    fn test_key_event_kind_is_press() {
        // Browser events always have KeyEventKind::Press
        let event = KeyEvent::new(KeyCode::Down, KeyModifiers::default());
        assert_eq!(event.kind, KeyEventKind::Press);
    }

    #[test]
    fn test_key_event_function_key() {
        let event = KeyEvent::new(KeyCode::F(1), KeyModifiers::default());
        assert!(matches!(event.code, KeyCode::F(1)));
        assert_eq!(event.kind, KeyEventKind::Press);
    }

    #[test]
    fn test_key_event_with_ctrl_modifier() {
        let mods = KeyModifiers { ctrl: true, ..KeyModifiers::default() };
        let event = KeyEvent::new(KeyCode::Char('c'), mods);
        assert!(event.modifiers.ctrl);
        assert!(!event.modifiers.shift);
        assert!(!event.modifiers.alt);
    }

    #[test]
    fn test_key_event_with_shift_modifier() {
        let mods = KeyModifiers { shift: true, ..KeyModifiers::default() };
        let event = KeyEvent::new(KeyCode::Char('A'), mods);
        assert!(event.modifiers.shift);
        assert!(matches!(event.code, KeyCode::Char('A')));
    }

    #[test]
    fn test_key_event_with_alt_modifier() {
        let mods = KeyModifiers { alt: true, ..KeyModifiers::default() };
        let event = KeyEvent::new(KeyCode::Tab, mods);
        assert!(event.modifiers.alt);
        assert!(matches!(event.code, KeyCode::Tab));
    }

    #[test]
    fn test_key_event_arrow_with_shift() {
        let mods = KeyModifiers { shift: true, ..KeyModifiers::default() };
        let event = KeyEvent::new(KeyCode::Right, mods);
        assert!(matches!(event.code, KeyCode::Right));
        assert!(event.modifiers.shift);
    }

    // ===========================================
    // Event Type Tests
    // ===========================================

    #[test]
    fn test_event_key_variant() {
        let key_event = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::default());
        let event = Event::Key(key_event);
        match event {
            Event::Key(ke) => {
                assert!(matches!(ke.code, KeyCode::Char('x')));
                assert_eq!(ke.kind, KeyEventKind::Press);
            }
            _ => panic!("Expected Event::Key"),
        }
    }

    #[test]
    fn test_event_mouse_variant() {
        let mouse_event = MouseEvent::new(0, 0, MouseButton::Left, KeyModifiers::default());
        let event = Event::Mouse(mouse_event);
        match event {
            Event::Mouse(me) => {
                assert_eq!(me.column, 0);
                assert_eq!(me.row, 0);
                assert!(matches!(me.button, MouseButton::Left));
            }
            _ => panic!("Expected Event::Mouse"),
        }
    }

    #[test]
    fn test_event_key_with_esc() {
        let event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::Esc)),
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_event_key_with_enter() {
        let event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::Enter)),
            _ => panic!("Expected Key event"),
        }
    }

    #[test]
    fn test_event_key_with_backspace() {
        let event = Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::Backspace)),
            _ => panic!("Expected Key event"),
        }
    }

    #[test]
    fn test_event_key_with_delete() {
        let event = Event::Key(KeyEvent::new(KeyCode::Delete, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::Delete)),
            _ => panic!("Expected Key event"),
        }
    }

    #[test]
    fn test_event_key_with_tab() {
        let event = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::Tab)),
            _ => panic!("Expected Key event"),
        }
    }

    #[test]
    fn test_event_key_with_navigation_keys() {
        // Home
        let event = Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::Home)),
            _ => panic!("Expected Key event"),
        }

        // End
        let event = Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::End)),
            _ => panic!("Expected Key event"),
        }

        // PageUp
        let event = Event::Key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::PageUp)),
            _ => panic!("Expected Key event"),
        }

        // PageDown
        let event = Event::Key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::default()));
        match event {
            Event::Key(ke) => assert!(matches!(ke.code, KeyCode::PageDown)),
            _ => panic!("Expected Key event"),
        }
    }

    #[test]
    fn test_event_key_with_all_function_keys() {
        for f in 1..=12u8 {
            let event = Event::Key(KeyEvent::new(KeyCode::F(f), KeyModifiers::default()));
            match event {
                Event::Key(ke) => assert!(matches!(ke.code, KeyCode::F(n) if n == f)),
                _ => panic!("Expected Key event for F{}", f),
            }
        }
    }

    #[test]
    fn test_event_key_with_all_modifiers() {
        // Test all modifier combinations
        let shift_event = Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers { shift: true, ..KeyModifiers::default() }
        ));
        let ctrl_event = Event::Key(KeyEvent::new(
            KeyCode::Char('b'),
            KeyModifiers { ctrl: true, ..KeyModifiers::default() }
        ));
        let alt_event = Event::Key(KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers { alt: true, ..KeyModifiers::default() }
        ));
        
        match shift_event {
            Event::Key(ke) => assert!(ke.modifiers.shift),
            _ => panic!("Expected Key event"),
        }
        match ctrl_event {
            Event::Key(ke) => assert!(ke.modifiers.ctrl),
            _ => panic!("Expected Key event"),
        }
        match alt_event {
            Event::Key(ke) => assert!(ke.modifiers.alt),
            _ => panic!("Expected Key event"),
        }
    }

    #[test]
    fn test_key_event_modifiers_equality() {
        let mods1 = KeyModifiers { shift: true, ctrl: true, ..KeyModifiers::default() };
        let mods2 = KeyModifiers { shift: true, ctrl: true, ..KeyModifiers::default() };
        let mods3 = KeyModifiers { shift: true, ctrl: false, ..KeyModifiers::default() };
        
        assert_eq!(mods1, mods2);
        assert_ne!(mods1, mods3);
    }

    #[test]
    fn test_key_event_keycode_equality() {
        let code1 = KeyCode::Char('a');
        let code2 = KeyCode::Char('a');
        let code3 = KeyCode::Char('b');
        
        assert_eq!(code1, code2);
        assert_ne!(code1, code3);
    }

    #[test]
    fn test_key_event_clone() {
        let event = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::default());
        let cloned = event.clone();
        assert_eq!(event.code, cloned.code);
        assert_eq!(event.modifiers, cloned.modifiers);
        assert_eq!(event.kind, cloned.kind);
    }

    #[test]
    fn test_key_event_copy_behavior() {
        // KeyEvent derives Clone but not Copy
        let event = KeyEvent::new(KeyCode::Up, KeyModifiers::default());
        let moved = event.clone();
        assert!(matches!(event.code, KeyCode::Up));
        assert!(matches!(moved.code, KeyCode::Up));
    }
}
