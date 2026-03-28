//! Grapheme width validation tests
//!
//! Tests for proper Unicode grapheme cluster width handling.
//!
//! # Key Finding
//! The `display_width` function handles grapheme clusters correctly:
//! - VS15 variation selector (text): width 1
//! - VS16 variation selector (emoji): width 2
//! - ZWJ sequences (family emoji): width 2 ✓
//! - Flag emoji (regional indicators): width 2 ✓
//! - Skin tone modified emoji: width 2 ✓
//! - CJK characters: width 2 ✓

use ctui_core::cell::Cell;

// ============================================================================
// ASCII (Should pass - already correct)
// ============================================================================

#[test]
fn test_width_ascii_single_char() {
    let cell = Cell::new("a");
    assert_eq!(cell.width(), 1, "ASCII 'a' should have width 1");
}

#[test]
fn test_width_ascii_string() {
    let cell = Cell::new("hello");
    assert_eq!(cell.width(), 5, "ASCII 'hello' should have width 5");
}

// ============================================================================
// CJK Characters (Should pass - already correct)
// ============================================================================

#[test]
fn test_width_cjk_hiragana() {
    let cell = Cell::new("あ");
    assert_eq!(cell.width(), 2, "CJK hiragana 'あ' should have width 2");
}

#[test]
fn test_width_cjk_kanji() {
    let cell = Cell::new("漢");
    assert_eq!(cell.width(), 2, "CJK kanji '漢' should have width 2");
}

#[test]
fn test_width_cjk_hangul() {
    let cell = Cell::new("한");
    assert_eq!(cell.width(), 2, "CJK hangul '한' should have width 2");
}

// ============================================================================
// Single Emoji (Should pass - already correct)
// ============================================================================

#[test]
fn test_width_single_emoji() {
    let cell = Cell::new("😀");
    assert_eq!(cell.width(), 2, "Single emoji '😀' should have width 2");
}

#[test]
fn test_width_single_emoji_heart() {
    let cell = Cell::new("❤");
    let width = cell.width();
    assert!(
        width == 1 || width == 2,
        "Heart '❤' has ambiguous width, got {}",
        width
    );
}

// ============================================================================
// ZWJ Sequences - CORRECTLY HANDLED
// ============================================================================

#[test]
fn test_width_zwj_family() {
    let cell = Cell::new("\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}");
    assert_eq!(cell.width(), 2, "ZWJ family emoji should have width 2");
}

#[test]
fn test_width_zwj_handshake() {
    let cell = Cell::new("\u{1F9D1}\u{200D}\u{1F91D}\u{200D}\u{1F9D1}");
    assert_eq!(cell.width(), 2, "ZWJ handshake emoji should have width 2");
}

#[test]
fn test_width_zwj_kiss() {
    let cell = Cell::new("\u{1F469}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}");
    assert_eq!(cell.width(), 2, "ZWJ kiss emoji should have width 2");
}

// ============================================================================
// Flag Emoji (Regional Indicator Pairs) - CORRECTLY HANDLED
// ============================================================================

#[test]
fn test_width_flag_russia() {
    let cell = Cell::new("\u{1F1F7}\u{1F1FA}");
    assert_eq!(cell.width(), 2, "Russian flag should have width 2");
}

#[test]
fn test_width_flag_usa() {
    let cell = Cell::new("\u{1F1FA}\u{1F1F8}");
    assert_eq!(cell.width(), 2, "USA flag should have width 2");
}

#[test]
fn test_width_flag_japan() {
    let cell = Cell::new("\u{1F1EF}\u{1F1F5}");
    assert_eq!(cell.width(), 2, "Japan flag should have width 2");
}

// ============================================================================
// VARIATION SELECTOR TESTS (VS15/VS16) - AT LEAST 10 TESTS
// VS15 (U+FE0E): Force text presentation → width 1
// VS16 (U+FE0F): Force emoji presentation → width 2
// ============================================================================

/// Test 1: Paperclip with VS15 (text presentation)
#[test]
fn test_vs15_paperclip_text() {
    // 📎 U+1F4CE + VS15 U+FE0E → text presentation, width 1
    let cell = Cell::new("\u{1F4CE}\u{FE0E}");
    assert_eq!(cell.width(), 1, "Paperclip + VS15 should have width 1 (text)");
}

/// Test 2: Paperclip with VS16 (emoji presentation)
#[test]
fn test_vs16_paperclip_emoji() {
    // 📎 U+1F4CE + VS16 U+FE0F → emoji presentation, width 2
    let cell = Cell::new("\u{1F4CE}\u{FE0F}");
    assert_eq!(cell.width(), 2, "Paperclip + VS16 should have width 2 (emoji)");
}

/// Test 3: Clipboard with VS15 (text presentation)
#[test]
fn test_vs15_clipboard_text() {
    // 📋 U+1F4CB + VS15 U+FE0E → text presentation, width 1
    let cell = Cell::new("\u{1F4CB}\u{FE0E}");
    assert_eq!(cell.width(), 1, "Clipboard + VS15 should have width 1 (text)");
}

/// Test 4: Clipboard with VS16 (emoji presentation)
#[test]
fn test_vs16_clipboard_emoji() {
    // 📋 U+1F4CB + VS16 U+FE0F → emoji presentation, width 2
    let cell = Cell::new("\u{1F4CB}\u{FE0F}");
    assert_eq!(cell.width(), 2, "Clipboard + VS16 should have width 2 (emoji)");
}

/// Test 5: Pushpin with VS15 (text presentation)
#[test]
fn test_vs15_pushpin_text() {
    // 📌 U+1F4CC + VS15 U+FE0E → text presentation, width 1
    let cell = Cell::new("\u{1F4CC}\u{FE0E}");
    assert_eq!(cell.width(), 1, "Pushpin + VS15 should have width 1 (text)");
}

/// Test 6: Pushpin with VS16 (emoji presentation)
#[test]
fn test_vs16_pushpin_emoji() {
    // 📌 U+1F4CC + VS16 U+FE0F → emoji presentation, width 2
    let cell = Cell::new("\u{1F4CC}\u{FE0F}");
    assert_eq!(cell.width(), 2, "Pushpin + VS16 should have width 2 (emoji)");
}

/// Test 7: Memo with VS15 (text presentation)
#[test]
fn test_vs15_memo_text() {
    // 📝 U+1F4DD + VS15 U+FE0E → text presentation, width 1
    let cell = Cell::new("\u{1F4DD}\u{FE0E}");
    assert_eq!(cell.width(), 1, "Memo + VS15 should have width 1 (text)");
}

/// Test 8: Memo with VS16 (emoji presentation)
#[test]
fn test_vs16_memo_emoji() {
    // 📝 U+1F4DD + VS16 U+FE0F → emoji presentation, width 2
    let cell = Cell::new("\u{1F4DD}\u{FE0F}");
    assert_eq!(cell.width(), 2, "Memo + VS16 should have width 2 (emoji)");
}

/// Test 9: Globe with VS15 (text presentation)
#[test]
fn test_vs15_globe_text() {
    // 🌐 U+1F310 + VS15 U+FE0E → text presentation, width 1
    let cell = Cell::new("\u{1F310}\u{FE0E}");
    assert_eq!(cell.width(), 1, "Globe + VS15 should have width 1 (text)");
}

/// Test 10: Globe with VS16 (emoji presentation)
#[test]
fn test_vs16_globe_emoji() {
    // 🌐 U+1F310 + VS16 U+FE0F → emoji presentation, width 2
    let cell = Cell::new("\u{1F310}\u{FE0F}");
    assert_eq!(cell.width(), 2, "Globe + VS16 should have width 2 (emoji)");
}

/// Test 11: Compare VS15 vs VS16 widths for same base character
#[test]
fn test_vs15_vs16_width_comparison() {
    let _base = Cell::new("\u{1F4CE}");
    let with_vs15 = Cell::new("\u{1F4CE}\u{FE0E}");
    let with_vs16 = Cell::new("\u{1F4CE}\u{FE0F}");
    
    // VS15 should make it narrower (text presentation)
    // VS16 should make it wider (emoji presentation)
    assert!(with_vs15.width() <= with_vs16.width(),
        "VS15 ({}) should be <= VS16 ({})", with_vs15.width(), with_vs16.width());
}

/// Test 12: Multiple variation selectors in sequence (edge case)
#[test]
fn test_multiple_vs16() {
    // Two VS16 should still result in width 2 (only first one affects base)
    let cell = Cell::new("\u{1F4CE}\u{FE0F}\u{FE0F}");
    // Note: This is an edge case - behavior may vary
    assert!(cell.width() >= 2, "Multiple VS16 should at least have width 2");
}

// ============================================================================
// Skin Tone Modifiers - CORRECTLY HANDLED
// ============================================================================

#[test]
fn test_width_skin_tone_default() {
    // 👋 waving hand (no skin tone)
    let cell = Cell::new("\u{1F44B}");
    assert_eq!(cell.width(), 2, "Waving hand should have width 2");
}

#[test]
fn test_width_skin_tone_light() {
    // 👋🏻 waving hand with light skin tone: U+1F44B + U+1F3FB
    let cell = Cell::new("\u{1F44B}\u{1F3FB}");
    assert_eq!(cell.width(), 2, "Light skin tone emoji should have width 2");
}

#[test]
fn test_width_skin_tone_medium() {
    // 👋🏽 waving hand with medium skin tone: U+1F44B + U+1F3FD
    let cell = Cell::new("\u{1F44B}\u{1F3FD}");
    assert_eq!(cell.width(), 2, "Medium skin tone emoji should have width 2");
}

#[test]
fn test_width_skin_tone_dark() {
    // 👋🏿 waving hand with dark skin tone: U+1F44B + U+1F3FF
    let cell = Cell::new("\u{1F44B}\u{1F3FF}");
    assert_eq!(cell.width(), 2, "Dark skin tone emoji should have width 2");
}

/// Test: Emoji with skin tone AND variation selector
#[test]
fn test_skin_tone_with_vs16() {
    // Hand with skin tone modifier + VS16
    let cell = Cell::new("\u{1F44B}\u{1F3FB}\u{FE0F}");
    assert_eq!(cell.width(), 2, "Skin tone + VS16 should have width 2");
}

// ============================================================================
// Combining Characters
// ============================================================================

#[test]
fn test_width_combining_accent() {
    // e + combining acute accent: "é" = e\u{0301}
    let cell = Cell::new("e\u{0301}");
    assert_eq!(cell.width(), 1, "e + combining acute should have width 1");
}

#[test]
fn test_width_precomposed_accent() {
    // Precomposed é: U+00E9
    let cell = Cell::new("\u{00E9}");
    assert_eq!(cell.width(), 1, "Precomposed 'é' should have width 1");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_width_empty_string() {
    let cell = Cell::new("");
    assert_eq!(cell.width(), 0, "Empty string should have width 0");
}

#[test]
fn test_width_space() {
    let cell = Cell::new(" ");
    assert_eq!(cell.width(), 1, "Space should have width 1");
}

#[test]
fn test_width_control_characters() {
    // Control characters: unicode_width returns 0 for C0 controls
    let cell = Cell::new("\x00");
    let width = cell.width();
    assert!(
        width == 0 || width == 1,
        "Null character behavior varies, got width {}",
        width
    );
}

// ============================================================================
// Mixed Content Tests
// ============================================================================

#[test]
fn test_width_mixed_ascii_and_emoji() {
    // "Hello 😀" = 5 + 1 + 2 = 8
    let cell = Cell::new("Hello \u{1F600}");
    assert_eq!(cell.width(), 8, "Mixed ASCII + emoji should sum correctly");
}

#[test]
fn test_width_mixed_cjk_and_ascii() {
    // "あA" = 2 + 1 = 3
    let cell = Cell::new("あA");
    assert_eq!(cell.width(), 3, "Mixed CJK + ASCII should sum correctly");
}

// ============================================================================
// Unicode9 vs Unicode14 Compatibility Mode Tests (in display_width)
// ============================================================================

/// Test Unicode9 mode with VS15 (different behavior)
#[test]
fn test_unicode9_vs15() {
    use ctui_core::unicode::{display_width, UnicodeCompat};
    // In Unicode9 mode, VS15 still gives width 2 (emoji was default)
    let width = display_width("\u{1F4CE}\u{FE0E}", UnicodeCompat::Unicode9);
    // Note: Our implementation gives width 2 for Unicode9, 1 for Unicode14
    assert!(width == 2, "Unicode9 VS15 should give width 2");
}

/// Test Unicode14 mode with VS15 (narrow behavior)
#[test]
fn test_unicode14_vs15() {
    use ctui_core::unicode::{display_width, UnicodeCompat};
    // In Unicode14 mode, VS15 gives width 1 (text presentation)
    let width = display_width("\u{1F4CE}\u{FE0E}", UnicodeCompat::Unicode14);
    assert_eq!(width, 1, "Unicode14 VS15 should give width 1");
}

// ============================================================================
// Summary of Findings
// ============================================================================
//
// PASSING with display_width implementation:
// - VS15 (text presentation): width 1 in Unicode14 mode ✓
// - VS16 (emoji presentation): width 2 ✓
// - ZWJ Sequences (👨‍👩‍👧‍👦): width 2 ✓
// - Flag Emoji (🇷🇺, 🇺🇸): width 2 ✓
// - Skin Tone Modified (👋🏻): width 2 ✓
// - Single Emoji: width 2 ✓
// - CJK Characters: width 2 ✓
// - ASCII: width 1 ✓
// - Combining characters: width of base character ✓
//
// CONCLUSION: display_width properly handles all variation selector cases.
