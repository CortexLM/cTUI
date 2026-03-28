//! Kitty keyboard protocol parser
//!
//! This module implements parsing of the Kitty keyboard protocol CSI sequences.
//! The protocol provides enhanced keyboard input with:
//! - Key event kinds (Press, Repeat, Release)
//! - Extended modifiers (Hyper, Meta, CapsLock, NumLock)
//! - Distinguishing key-up vs key-down events
//!
//! ## Protocol Format
//!
//! The Kitty protocol uses CSI sequences in the format:
//! - `CSI <keycode> ; <modifiers> u` for basic keys
//! - `CSI <keycode> ; <modifiers> : <event_kind> u` with event kind
//!
//! ## Modifier Encoding
//!
//! Kitty modifier encoding (CSI u format):
//! - Base value: 1
//! - Shift: +1
//! - Alt: +2
//! - Ctrl: +4
//! - Super: +8
//! - Hyper: +16
//! - Meta: +32
//! - CapsLock: +64
//! - NumLock: +128
//!
//! Reference: <https://sw.kovidgoyal.net/kitty/keyboard-protocol/>

use super::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Parser state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ParserState {
    #[default]
    Start,
    Keycode,
    ModifierSeparator,
    Modifiers,
    EventKindSeparator,
    EventKind,
    Complete,
    Error,
}

/// Kitty keyboard protocol CSI sequence parser
///
/// This parser handles CSI u sequences used by the Kitty keyboard protocol.
/// It uses a state machine to incrementally parse sequences.
#[derive(Debug, Default)]
pub struct KittyKeyboardParser {
    state: ParserState,
    keycode: u32,
    modifiers: u32,
    event_kind: u32,
    keycode_str: String,
    modifiers_str: String,
    event_kind_str: String,
}

impl KittyKeyboardParser {
    /// Creates a new parser
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the parser to initial state
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Feeds a byte to the parser
    ///
    /// Returns `Some(KeyEvent)` when a complete sequence is parsed,
    /// `None` if more input is needed.
    pub fn feed(&mut self, byte: u8) -> Option<KeyEvent> {
        match self.state {
            ParserState::Start => {
                // Expect ESC (0x1B)
                if byte == 0x1B {
                    // Reset for new sequence
                    self.keycode = 0;
                    self.modifiers = 0;
                    self.event_kind = 0;
                    self.keycode_str.clear();
                    self.modifiers_str.clear();
                    self.event_kind_str.clear();
                    // Stay in Start, waiting for '[' after ESC
                }
                None
            }
            ParserState::Keycode => {
                if byte == b';' {
                    self.keycode = self.keycode_str.parse().unwrap_or(0);
                    self.state = ParserState::ModifierSeparator;
                } else if byte == b'u' {
                    // Simple format: CSI <keycode> u
                    self.keycode = self.keycode_str.parse().unwrap_or(0);
                    self.state = ParserState::Complete;
                    return self.finalize();
                } else if byte.is_ascii_digit() {
                    self.keycode_str.push(byte as char);
                } else {
                    self.state = ParserState::Error;
                }
                None
            }
            ParserState::ModifierSeparator => {
                if byte == b':' {
                    self.state = ParserState::EventKindSeparator;
                } else if byte == b'u' {
                    self.modifiers = self.modifiers_str.parse().unwrap_or(1);
                    self.state = ParserState::Complete;
                    return self.finalize();
                } else if byte.is_ascii_digit() {
                    self.modifiers_str.push(byte as char);
                    self.state = ParserState::Modifiers;
                } else {
                    self.state = ParserState::Error;
                }
                None
            }
            ParserState::Modifiers => {
                if byte == b':' {
                    self.modifiers = self.modifiers_str.parse().unwrap_or(1);
                    self.state = ParserState::EventKindSeparator;
                } else if byte == b'u' {
                    self.modifiers = self.modifiers_str.parse().unwrap_or(1);
                    self.state = ParserState::Complete;
                    return self.finalize();
                } else if byte.is_ascii_digit() {
                    self.modifiers_str.push(byte as char);
                } else {
                    self.state = ParserState::Error;
                }
                None
            }
            ParserState::EventKindSeparator => {
                if byte.is_ascii_digit() {
                    self.event_kind_str.push(byte as char);
                    self.state = ParserState::EventKind;
                } else {
                    self.state = ParserState::Error;
                }
                None
            }
            ParserState::EventKind => {
                if byte == b'u' {
                    self.event_kind = self.event_kind_str.parse().unwrap_or(1);
                    self.state = ParserState::Complete;
                    return self.finalize();
                } else if byte.is_ascii_digit() {
                    self.event_kind_str.push(byte as char);
                } else {
                    self.state = ParserState::Error;
                }
                None
            }
            ParserState::Complete | ParserState::Error => {
                self.reset();
                None
            }
        }
    }

    /// Handle CSI sequence after ESC[ has been detected
    ///
    /// Call this when you've already consumed ESC[
    pub fn feed_csi_sequence(&mut self, seq: &str) -> Option<KeyEvent> {
        // Parse format: <keycode>; <modifiers> [: <event_kind>] u
        let seq = seq.trim_end_matches('u');
        
        let (keycode_mods, event_kind) = if let Some(idx) = seq.find(':') {
            (&seq[..idx], seq[idx + 1..].parse().unwrap_or(1u32))
        } else {
            (seq, 1u32)
        };

        let parts: Vec<&str> = keycode_mods.split(';').collect();
        if parts.len() < 1 {
            return None;
        }

        let keycode: u32 = parts[0].parse().ok()?;
        let modifiers: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);

        self.keycode = keycode;
        self.modifiers = modifiers;
        self.event_kind = event_kind;
        self.state = ParserState::Complete;

        self.finalize()
    }

    /// Parse a complete CSI u sequence string
    ///
    /// Expects format: `ESC[<keycode>;<modifiers>[:<event_kind>]u`
    pub fn parse(&mut self, sequence: &str) -> Option<KeyEvent> {
        self.reset();
        
        // Check for ESC[
        if !sequence.starts_with("\x1b[") {
            return None;
        }

        // Extract the part after ESC[
        let inner = &sequence[2..];
        self.feed_csi_sequence(inner)
    }

    /// Convert keycode number to KeyCode enum
    fn keycode_to_key_code(keycode: u32) -> KeyCode {
        match keycode {
            // Special keys (Kitty-specific codes)
            1 => KeyCode::Home,
            2 => KeyCode::Insert,
            3 => KeyCode::Delete,
            4 => KeyCode::End,
            5 => KeyCode::PageUp,
            6 => KeyCode::PageDown,
            // Kitty arrow keys use key codes 57360-57363
            // But CSI u format uses regular key codes for arrow keys
            // Arrow key codes in standard format
            57360 => KeyCode::Up,
            57361 => KeyCode::Down,
            57362 => KeyCode::Left,
            57363 => KeyCode::Right,
            // Function keys (CSI u uses 11-21 for F1-F12, or 57350-57361 for extended)
            11 => KeyCode::F(1),
            12 => KeyCode::F(2),
            13 => KeyCode::F(3),
            14 => KeyCode::F(4),
            15 => KeyCode::F(5),
            16 => KeyCode::F(6),
            17 => KeyCode::F(7),
            18 => KeyCode::F(8),
            19 => KeyCode::F(9),
            20 => KeyCode::F(10),
            21 => KeyCode::F(11),
            22 => KeyCode::F(12),
            // Special keys in Kitty extended encoding
            57348 => KeyCode::Backspace,
            57346 => KeyCode::Tab,
            57345 => KeyCode::Enter,
            57344 => KeyCode::Esc,
            // Character codes (ASCII printable range)
            32..=126 => KeyCode::Char((keycode as u8) as char),
            _ => KeyCode::Null,
        }
    }

    /// Convert event kind number to KeyEventKind
    fn event_kind_to_key_event_kind(kind: u32) -> KeyEventKind {
        match kind {
            1 => KeyEventKind::Press,
            2 => KeyEventKind::Repeat,
            3 => KeyEventKind::Release,
            _ => KeyEventKind::Press,
        }
    }

    /// Finalize parsing and produce a KeyEvent
    fn finalize(&self) -> Option<KeyEvent> {
        let code = Self::keycode_to_key_code(self.keycode);
        let modifiers = decode_kitty_modifiers(self.modifiers);
        let kind = Self::event_kind_to_key_event_kind(self.event_kind);

        Some(KeyEvent::with_kind(code, modifiers, kind))
    }

    /// Check if the parser is in an error state
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self.state, ParserState::Error)
    }

    /// Check if the parser has completed successfully
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.state, ParserState::Complete)
    }
}

/// Decode Kitty protocol modifier value to KeyModifiers
///
/// Kitty CSI u modifier encoding:
/// - Value 1 = no modifiers
/// - Shift: +1 → value 2
/// - Alt: +2 → value 3 (or value 4 if combined)
/// - Ctrl: +4 → value 5 (or value 6+)
/// - Super: +8
/// - Hyper: +16
/// - Meta: +32
/// - CapsLock: +64
/// - NumLock: +128
///
/// Formula: 1 + shift*1 + alt*2 + ctrl*4 + super*8 + hyper*16 + meta*32 + capslock*64 + numlock*128
#[must_use]
pub fn decode_kitty_modifiers(value: u32) -> KeyModifiers {
    // Subtract base value of 1
    let bits = value.saturating_sub(1);
    
    KeyModifiers {
        shift: (bits & 1) != 0,
        alt: (bits & 2) != 0,
        ctrl: (bits & 4) != 0,
        super_key: (bits & 8) != 0,
        hyper: (bits & 16) != 0,
        meta: (bits & 32) != 0,
        caps_lock: (bits & 64) != 0,
        num_lock: (bits & 128) != 0,
    }
}

/// Encode KeyModifiers to Kitty protocol modifier value
#[must_use]
pub fn encode_kitty_modifiers(modifiers: &KeyModifiers) -> u32 {
    // Base value is 1
    let mut value = 1u32;
    
    if modifiers.shift { value += 1; }
    if modifiers.alt { value += 2; }
    if modifiers.ctrl { value += 4; }
    if modifiers.super_key { value += 8; }
    if modifiers.hyper { value += 16; }
    if modifiers.meta { value += 32; }
    if modifiers.caps_lock { value += 64; }
    if modifiers.num_lock { value += 128; }
    
    value
}

/// Check if Kitty keyboard protocol is available
///
/// Detects if the current terminal supports the Kitty keyboard protocol
/// by checking environment variables.
#[must_use]
pub fn is_kitty_terminal() -> bool {
    std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("KITTY_PID").is_ok()
        || std::env::var("TERM")
            .map(|t| t == "xterm-kitty")
            .unwrap_or(false)
}

/// Generate the CSI sequence to enable Kitty keyboard protocol
///
/// Format: `CSI > 1 u` enables full protocol
/// Format: `CSI > 2 u` enables protocol without SPARE keys
/// Format: `CSI > 3 u` enables all keys including modifiers
#[must_use]
pub const fn enable_kitty_keyboard_sequence() -> &'static str {
    "\x1b[>1u"
}

/// Generate the CSI sequence to disable Kitty keyboard protocol
///
/// Format: `CSI < 1 u` disables the protocol
#[must_use]
pub const fn disable_kitty_keyboard_sequence() -> &'static str {
    "\x1b[<1u"
}

/// Generate the CSI sequence to query Kitty keyboard protocol version
#[must_use]
pub const fn query_kitty_keyboard_version_sequence() -> &'static str {
    "\x1b[?u"
}

/// Helper function to parse a CSI u sequence string
pub fn parse_kitty_sequence(sequence: &str) -> Option<KeyEvent> {
    let mut parser = KittyKeyboardParser::new();
    parser.parse(sequence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_basic_key() {
        // ESC[97;1u = 'a' pressed
        let mut parser = KittyKeyboardParser::new();
        let event = parser.parse("\x1b[97;1u");
        
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.code, KeyCode::Char('a'));
        assert_eq!(event.kind, KeyEventKind::Press);
        assert!(event.modifiers.is_empty());
    }

    #[test]
    fn test_parser_with_modifiers() {
        // ESC[97;5u = 'a' + Ctrl pressed (1 + 4 = 5)
        let mut parser = KittyKeyboardParser::new();
        let event = parser.parse("\x1b[97;5u");
        
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.code, KeyCode::Char('a'));
        assert!(event.modifiers.ctrl);
        assert!(!event.modifiers.shift);
        assert!(!event.modifiers.alt);
    }

    #[test]
    fn test_parser_shift_alt() {
        // Shift + Alt = 1 + 1 + 2 = 4
        let event = parse_kitty_sequence("\x1b[97;4u");
        assert!(event.is_some());
        let event = event.unwrap();
        assert!(event.modifiers.shift);
        assert!(event.modifiers.alt);
        assert!(!event.modifiers.ctrl);
    }

    #[test]
    fn test_parser_with_event_kind() {
        // ESC[97;1:3u = 'a' released
        let mut parser = KittyKeyboardParser::new();
        let event = parser.parse("\x1b[97;1:3u");
        
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.code, KeyCode::Char('a'));
        assert_eq!(event.kind, KeyEventKind::Release);
    }

    #[test]
    fn test_parser_function_key() {
        // ESC[11;1u = F1 pressed
        let mut parser = KittyKeyboardParser::new();
        let event = parser.parse("\x1b[11;1u");
        
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.code, KeyCode::F(1));
    }

    #[test]
    fn test_parser_sequence_backslash() {
        // ESC[92;5u = '\\' + Ctrl (1 + 4 = 5)
        let mut parser = KittyKeyboardParser::new();
        let event = parser.parse("\x1b[92;5u");
        
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.code, KeyCode::Char('\\'));
        assert!(event.modifiers.ctrl);
    }

    #[test]
    fn test_keycode_conversion() {
        assert_eq!(KittyKeyboardParser::keycode_to_key_code(97), KeyCode::Char('a'));
        assert_eq!(KittyKeyboardParser::keycode_to_key_code(65), KeyCode::Char('A'));
        assert_eq!(KittyKeyboardParser::keycode_to_key_code(32), KeyCode::Char(' '));
        assert_eq!(KittyKeyboardParser::keycode_to_key_code(11), KeyCode::F(1));
        assert_eq!(KittyKeyboardParser::keycode_to_key_code(57360), KeyCode::Up);
    }

    #[test]
    fn test_event_kind_conversion() {
        assert_eq!(
            KittyKeyboardParser::event_kind_to_key_event_kind(1),
            KeyEventKind::Press
        );
        assert_eq!(
            KittyKeyboardParser::event_kind_to_key_event_kind(2),
            KeyEventKind::Repeat
        );
        assert_eq!(
            KittyKeyboardParser::event_kind_to_key_event_kind(3),
            KeyEventKind::Release
        );
    }

    #[test]
    fn test_modifiers_encoding_decoding() {
        // Test roundtrip for various modifier combinations
        
        // No modifiers: value = 1
        let mods = KeyModifiers::new();
        assert_eq!(encode_kitty_modifiers(&mods), 1);
        assert_eq!(decode_kitty_modifiers(1), mods);

        // Shift only: value = 1 + 1 = 2
        let mods = KeyModifiers::new().with_shift();
        assert_eq!(encode_kitty_modifiers(&mods), 2);
        let decoded = decode_kitty_modifiers(2);
        assert!(decoded.shift);

        // Alt only: value = 1 + 2 = 3
        let mods = KeyModifiers::new().with_alt();
        assert_eq!(encode_kitty_modifiers(&mods), 3);
        let decoded = decode_kitty_modifiers(3);
        assert!(decoded.alt);

        // Ctrl only: value = 1 + 4 = 5
        let mods = KeyModifiers::new().with_ctrl();
        assert_eq!(encode_kitty_modifiers(&mods), 5);
        let decoded = decode_kitty_modifiers(5);
        assert!(decoded.ctrl);

        // Ctrl+Alt: value = 1 + 2 + 4 = 7
        let mods = KeyModifiers::new().with_ctrl().with_alt();
        assert_eq!(encode_kitty_modifiers(&mods), 7);

        // Hyper: value = 1 + 16 = 17
        let mods = KeyModifiers::new().with_hyper();
        assert_eq!(encode_kitty_modifiers(&mods), 17);
        let decoded = decode_kitty_modifiers(17);
        assert!(decoded.hyper);

        // Meta: value = 1 + 32 = 33
        let mods = KeyModifiers::new().with_meta();
        assert_eq!(encode_kitty_modifiers(&mods), 33);
        let decoded = decode_kitty_modifiers(33);
        assert!(decoded.meta);
    }

    #[test]
    fn test_extended_modifiers() {
        // CapsLock: value = 1 + 64 = 65
        let decoded = decode_kitty_modifiers(65);
        assert!(decoded.caps_lock);

        // NumLock: value = 1 + 128 = 129
        let decoded = decode_kitty_modifiers(129);
        assert!(decoded.num_lock);

        // All modifiers: 1 + 1 + 2 + 4 + 8 + 16 + 32 + 64 + 128 = 256
        let all = KeyModifiers::new()
            .with_shift()
            .with_alt()
            .with_ctrl()
            .with_super()
            .with_hyper()
            .with_meta()
            .with_caps_lock()
            .with_num_lock();
        assert_eq!(encode_kitty_modifiers(&all), 256);
        
        let decoded = decode_kitty_modifiers(256);
        assert!(decoded.shift);
        assert!(decoded.alt);
        assert!(decoded.ctrl);
        assert!(decoded.super_key);
        assert!(decoded.hyper);
        assert!(decoded.meta);
        assert!(decoded.caps_lock);
        assert!(decoded.num_lock);
    }

    #[test]
    fn test_helper_function() {
        let event = parse_kitty_sequence("\x1b[97;1:2u");
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.code, KeyCode::Char('a'));
        assert_eq!(event.kind, KeyEventKind::Repeat);
    }

    #[test]
    fn test_enable_disable_sequences() {
        assert_eq!(enable_kitty_keyboard_sequence(), "\x1b[>1u");
        assert_eq!(disable_kitty_keyboard_sequence(), "\x1b[<1u");
        assert_eq!(query_kitty_keyboard_version_sequence(), "\x1b[?u");
    }

    #[test]
    fn test_kitty_terminal_detection() {
        // This test will pass or fail depending on environment
        // Just ensure it doesn't panic
        let _ = is_kitty_terminal();
    }

    #[test]
    fn test_parser_reset() {
        let mut parser = KittyKeyboardParser::new();
        parser.parse("\x1b[97;1u");
        parser.reset();
        
        // Should be able to parse another sequence after reset
        let event = parser.parse("\x1b[98;1u");
        assert!(event.is_some());
        assert_eq!(event.unwrap().code, KeyCode::Char('b'));
    }

    #[test]
    fn test_parser_feed_csi_sequence() {
        let mut parser = KittyKeyboardParser::new();
        let event = parser.feed_csi_sequence("97;1:3u");
        
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.code, KeyCode::Char('a'));
        assert_eq!(event.kind, KeyEventKind::Release);
    }

    #[test]
    fn test_parser_invalid_sequence() {
        let mut parser = KittyKeyboardParser::new();
        let event = parser.parse("invalid");
        assert!(event.is_none());

        let event = parser.parse("\x1b[abcd;efgu");
        assert!(event.is_none());
    }
}
