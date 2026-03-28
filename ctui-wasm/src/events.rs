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
}
