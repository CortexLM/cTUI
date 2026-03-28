//! Kitty keyboard protocol tests.
//!
//! These tests cover the Kitty keyboard protocol extension which provides:
//! - Key event kinds (Press, Repeat, Release)
//! - Extended modifiers (Hyper, Meta, CapsLock, NumLock)
//! - Distinguishing key-up vs key-down events
//!
//! Reference: https://sw.kovidgoyal.net/kitty/keyboard-protocol/

use super::*;

/// Test parsing of key press event from CSI sequence.
/// Kitty protocol format: CSI <keycode> ; <modifiers> u
#[test]
fn test_kitty_press_event_parsing() {
    // Parse: ESC[97;1u  (press 'a', no modifiers)
    // Modifier value 1 = no modifiers (base value)
    let event = super::kitty::parse_kitty_sequence("\x1b[97;1u");
    assert!(event.is_some());
    
    let event = event.unwrap();
    assert_eq!(event.code, KeyCode::Char('a'));
    assert_eq!(event.kind, KeyEventKind::Press);
    assert!(event.modifiers.is_empty());
}

/// Test parsing of key repeat event from CSI sequence.
/// Kitty protocol uses event kind 2 for repeat events.
#[test]
fn test_kitty_repeat_event_parsing() {
    // Parse: ESC[97;1:2u  (repeat 'a')
    // 1 = no modifiers, :2 = repeat event
    let event = super::kitty::parse_kitty_sequence("\x1b[97;1:2u");
    assert!(event.is_some());
    
    let event = event.unwrap();
    assert_eq!(event.code, KeyCode::Char('a'));
    assert_eq!(event.kind, KeyEventKind::Repeat);
    assert!(event.is_repeat());
    assert!(!event.is_press());
    assert!(!event.is_release());
}

/// Test parsing of key release event from CSI sequence.
/// Kitty protocol uses event kind 3 for release events.
#[test]
fn test_kitty_release_event_parsing() {
    // Parse: ESC[97;1:3u  (release 'a')
    let event = super::kitty::parse_kitty_sequence("\x1b[97;1:3u");
    assert!(event.is_some());
    
    let event = event.unwrap();
    assert_eq!(event.code, KeyCode::Char('a'));
    assert_eq!(event.kind, KeyEventKind::Release);
    assert!(event.is_release());
    assert!(!event.is_press());
    assert!(!event.is_repeat());
}

/// Test Ctrl+Alt modifier combination parsing.
/// Kitty protocol modifier encoding:
/// - Base value: 1
/// - Shift: +1 → value 2
/// - Alt: +2 → value 3
/// - Ctrl: +4 → value 5
/// Combined Ctrl+Alt = 1 + 2 + 4 = 7
#[test]
fn test_kitty_ctrl_alt_modifiers() {
    // Test the KeyModifiers struct with Ctrl+Alt combination
    let modifiers = KeyModifiers::new().with_ctrl().with_alt();

    assert!(modifiers.ctrl, "Ctrl should be set");
    assert!(modifiers.alt, "Alt should be set");
    assert!(!modifiers.shift, "Shift should not be set");
    assert!(!modifiers.super_key, "Super should not be set");

    // Test encoding: 1 + 2(alt) + 4(ctrl) = 7
    let encoded = super::kitty::encode_kitty_modifiers(&modifiers);
    assert_eq!(encoded, 7);
    
    // Test decoding roundtrip
    let decoded = super::kitty::decode_kitty_modifiers(encoded);
    assert!(decoded.ctrl);
    assert!(decoded.alt);
}

/// Test Hyper modifier parsing.
/// Kitty protocol supports extended modifiers including:
/// - Hyper: +16 → value 17
/// - Meta: +32 → value 33
#[test]
fn test_kitty_hyper_modifier() {
    // Test Hyper modifier: 1 + 16 = 17
    let modifiers = KeyModifiers::new().with_hyper();
    assert!(modifiers.hyper);
    assert_eq!(super::kitty::encode_kitty_modifiers(&modifiers), 17);

    // Test parsing with Shift+Hyper: 1 + 1 + 16 = 18
    let event = super::kitty::parse_kitty_sequence("\x1b[97;18u");
    assert!(event.is_some());
    let event = event.unwrap();
    assert!(event.modifiers.hyper);
    assert!(event.modifiers.shift);
}

/// Test Meta modifier parsing.
/// Meta is distinct from Alt in Kitty protocol.
#[test]
fn test_kitty_meta_modifier() {
    // Test Meta modifier: 1 + 32 = 33
    let modifiers = KeyModifiers::new().with_meta();
    assert!(modifiers.meta);
    assert_eq!(super::kitty::encode_kitty_modifiers(&modifiers), 33);

    // Alt and Meta can both be set: 1 + 2(alt) + 32(meta) = 35
    let modifiers = KeyModifiers::new().with_alt().with_meta();
    assert!(modifiers.alt);
    assert!(modifiers.meta);
    assert_eq!(super::kitty::encode_kitty_modifiers(&modifiers), 35);
}

/// Test full modifier combination (Ctrl+Alt+Shift).
/// This tests the bit manipulation for combining multiple modifiers.
#[test]
fn test_kitty_all_standard_modifiers() {
    // All four standard modifiers: 1 + 1 + 2 + 4 + 8 = 16
    let modifiers = KeyModifiers::new().with_shift().with_ctrl().with_alt().with_super();

    assert!(modifiers.shift);
    assert!(modifiers.ctrl);
    assert!(modifiers.alt);
    assert!(modifiers.super_key);
    assert!(modifiers.is_standard_only());
    assert_eq!(super::kitty::encode_kitty_modifiers(&modifiers), 16);
}

/// Test CSI sequence for function keys.
/// Kitty encodes function keys with special keycodes:
/// - F1 = 11, F2 = 12, ..., F12 = 22 (in CSI u format)
#[test]
fn test_kitty_function_key_encoding() {
    // Test F1
    let event = super::kitty::parse_kitty_sequence("\x1b[11;1u");
    assert!(event.is_some());
    assert_eq!(event.unwrap().code, KeyCode::F(1));

    // Test F12
    let event = super::kitty::parse_kitty_sequence("\x1b[22;1u");
    assert!(event.is_some());
    assert_eq!(event.unwrap().code, KeyCode::F(12));

    // Test function key with Ctrl: 1 + 4 = 5
    let event = super::kitty::parse_kitty_sequence("\x1b[11;5u");
    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.code, KeyCode::F(1));
    assert!(event.modifiers.ctrl);
}

/// Test that CapsLock modifier is distinguishable.
/// Kitty protocol can report caps-lock state separately.
#[test]
fn test_kitty_capslock_modifier() {
    // CapsLock: 1 + 64 = 65
    let modifiers = KeyModifiers::new().with_caps_lock();
    assert!(modifiers.caps_lock);
    assert!(!modifiers.is_standard_only());
    assert_eq!(super::kitty::encode_kitty_modifiers(&modifiers), 65);

    // CapsLock can combine with other modifiers: 1 + 1 + 64 = 66
    let modifiers = KeyModifiers::new().with_shift().with_caps_lock();
    assert_eq!(super::kitty::encode_kitty_modifiers(&modifiers), 66);

    // Test decoding
    let decoded = super::kitty::decode_kitty_modifiers(66);
    assert!(decoded.shift);
    assert!(decoded.caps_lock);
}

/// Test that NumLock modifier is distinguishable.
#[test]
fn test_kitty_numlock_modifier() {
    // NumLock: 1 + 128 = 129
    let modifiers = KeyModifiers::new().with_num_lock();
    assert!(modifiers.num_lock);
    assert!(!modifiers.is_standard_only());
    assert_eq!(super::kitty::encode_kitty_modifiers(&modifiers), 129);

    // All extended modifiers: 1 + 16 + 32 + 64 + 128 = 241
    let modifiers = KeyModifiers::new()
        .with_hyper()
        .with_meta()
        .with_caps_lock()
        .with_num_lock();
    assert_eq!(super::kitty::encode_kitty_modifiers(&modifiers), 241);
}

/// Test Kitty protocol versioning.
/// Different terminals may support different protocol versions.
#[test]
fn test_kitty_protocol_version() {
    // The query sequence
    let query = super::kitty::query_kitty_keyboard_version_sequence();
    assert_eq!(query, "\x1b[?u");

    // The enable sequence
    let enable = super::kitty::enable_kitty_keyboard_sequence();
    assert_eq!(enable, "\x1b[>1u");

    // The disable sequence
    let disable = super::kitty::disable_kitty_keyboard_sequence();
    assert_eq!(disable, "\x1b[<1u");
}

/// Test parser state machine
#[test]
fn test_kitty_parser_state_machine() {
    use super::kitty::KittyKeyboardParser;

    let mut parser = KittyKeyboardParser::new();
    
    // Test parsing Ctrl+a: 1 + 4 = 5
    let event = parser.parse("\x1b[97;5u");
    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.code, KeyCode::Char('a'));
    assert!(event.modifiers.ctrl);
    assert_eq!(event.kind, KeyEventKind::Press);

    // Reset and parse again
    parser.reset();
    assert!(!parser.is_complete());
}

/// Test terminal detection
#[test]
fn test_kitty_terminal_detection() {
    // Just ensure the function runs without panic
    let is_kitty = super::kitty::is_kitty_terminal();
    // Result depends on environment
    let _ = is_kitty;
}

/// Test key event with all extended modifiers
#[test]
fn test_key_event_with_extended_modifiers() {
    let modifiers = KeyModifiers::new()
        .with_shift()
        .with_ctrl()
        .with_hyper()
        .with_meta();

    let event = KeyEvent::with_kind(
        KeyCode::Char('z'),
        modifiers,
        KeyEventKind::Press,
    );

    assert_eq!(event.code, KeyCode::Char('z'));
    assert!(event.modifiers.shift);
    assert!(event.modifiers.ctrl);
    assert!(event.modifiers.hyper);
    assert!(event.modifiers.meta);
    assert!(event.is_press());

    // Verify encoding/decoding roundtrip
    let encoded = super::kitty::encode_kitty_modifiers(&event.modifiers);
    let decoded = super::kitty::decode_kitty_modifiers(encoded);
    assert_eq!(event.modifiers, decoded);
}

/// Test edge cases
#[test]
fn test_kitty_parser_edge_cases() {
    // Empty sequence
    let event = super::kitty::parse_kitty_sequence("");
    assert!(event.is_none());

    // Incomplete sequence
    let event = super::kitty::parse_kitty_sequence("\x1b[");
    assert!(event.is_none());

    // Invalid keycode (non-numeric)
    let event = super::kitty::parse_kitty_sequence("\x1b[abc;1u");
    assert!(event.is_none());

    // Space character (keycode 32)
    let event = super::kitty::parse_kitty_sequence("\x1b[32;1u");
    assert!(event.is_some());
    assert_eq!(event.unwrap().code, KeyCode::Char(' '));
}

/// Test modifier encoding table
#[test]
fn test_kitty_modifier_encoding_table() {
    use super::kitty::{encode_kitty_modifiers, decode_kitty_modifiers};

    // Value 1: no modifiers
    let mods = decode_kitty_modifiers(1);
    assert!(mods.is_empty());
    assert_eq!(encode_kitty_modifiers(&mods), 1);

    // Value 2: Shift only
    let mods = decode_kitty_modifiers(2);
    assert!(mods.shift);
    assert!(!mods.alt);

    // Value 3: Alt only
    let mods = decode_kitty_modifiers(3);
    assert!(mods.alt);
    assert!(!mods.shift);

    // Value 5: Ctrl only
    let mods = decode_kitty_modifiers(5);
    assert!(mods.ctrl);

    // Value 9: Super only
    let mods = decode_kitty_modifiers(9);
    assert!(mods.super_key);

    // Value 17: Hyper only
    let mods = decode_kitty_modifiers(17);
    assert!(mods.hyper);

    // Value 33: Meta only
    let mods = decode_kitty_modifiers(33);
    assert!(mods.meta);

    // Value 65: CapsLock only
    let mods = decode_kitty_modifiers(65);
    assert!(mods.caps_lock);

    // Value 129: NumLock only
    let mods = decode_kitty_modifiers(129);
    assert!(mods.num_lock);
}
