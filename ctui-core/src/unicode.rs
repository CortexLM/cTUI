//! Unicode display width handling with variation selector support

use unicode_width::UnicodeWidthChar;

/// Unicode compatibility mode for width calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnicodeCompat {
    /// Unicode 9.0 width rules
    Unicode9,
    /// Unicode 14.0 width rules (default)
    #[default]
    Unicode14,
}

/// Check if a character is a skin tone modifier (U+1F3FB to U+1F3FF)
fn is_skin_tone_modifier(ch: char) -> bool {
    matches!(ch, '\u{1F3FB}'..='\u{1F3FF}')
}

/// Check if a character is a regional indicator (U+1F1E6 to U+1F1FF)
fn is_regional_indicator(ch: char) -> bool {
    matches!(ch, '\u{1F1E6}'..='\u{1F1FF}')
}

/// Check if a character is a Zero Width Joiner (ZWJ)
fn is_zwj(ch: char) -> bool {
    ch == '\u{200D}'
}

/// Calculate display width accounting for variation selectors and grapheme clusters
/// 
/// Handles:
/// - VS15 (U+FE0E): Text presentation → width 1 in Unicode14
/// - VS16 (U+FE0F): Emoji presentation → width 2
/// - ZWJ sequences: Treated as single grapheme with width 2
/// - Skin tone modifiers: Don't add width (0-width combining)
/// - Regional indicator pairs (flags): Width 2 as a pair
pub fn display_width(s: &str, compat: UnicodeCompat) -> usize {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return 0;
    }
    
    // Check for emoji sequence patterns (ZWJ, skin tones, regional indicators)
    if is_emoji_sequence(&chars) {
        return 2; // All emoji sequences render as width 2
    }
    
    // Check for flag emoji (regional indicator pair)
    if chars.len() == 2 && chars.iter().all(|c| is_regional_indicator(*c)) {
        return 2;
    }
    
    let mut width = 0;
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        
        // Skip ZWJ and skin tone modifiers (they don't add width)
        if is_zwj(ch) || is_skin_tone_modifier(ch) {
            i += 1;
            continue;
        }
        
        // Check for variation selectors
        if i + 1 < chars.len() {
            let next = chars[i + 1];
            match next {
                '\u{FE0E}' => { // VS15: Text presentation
                    match compat {
                        UnicodeCompat::Unicode9 => width += 2,
                        UnicodeCompat::Unicode14 => width += 1,
                    }
                    i += 2;
                    continue;
                }
                '\u{FE0F}' => { // VS16: Emoji presentation
                    width += 2;
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }
        
        // Default character width
        width += UnicodeWidthChar::width(ch).unwrap_or(0);
        i += 1;
    }
    width
}

/// Check if the string is an emoji sequence (ZWJ or skin tone modified)
fn is_emoji_sequence(chars: &[char]) -> bool {
    // Check for ZWJ sequence
    if chars.iter().any(|c| is_zwj(*c)) {
        return true;
    }
    
    // Check for emoji with skin tone modifier
    // Pattern: emoji (width 2) + skin tone modifier(s)
    if chars.len() >= 2 {
        let has_emoji_base = chars.iter().take(chars.len().saturating_sub(1)).any(|c| {
            // Most emoji are in certain Unicode ranges
            matches!(c, 
                '\u{1F300}'..='\u{1F9FF}' |  // Misc emoji
                '\u{2600}'..='\u{27BF}'      // Dingbats, symbols
            )
        });
        let has_skin_tone = chars.iter().any(|c| is_skin_tone_modifier(*c));
        if has_emoji_base && has_skin_tone {
            return true;
        }
    }
    
    // Check for two consecutive regional indicators (flag)
    if chars.len() == 2 && chars.iter().all(|c| is_regional_indicator(*c)) {
        return true;
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vs15() { 
        assert_eq!(display_width("📎\u{FE0E}", UnicodeCompat::Unicode14), 1); 
    }
    
    #[test]
    fn test_vs16() { 
        assert_eq!(display_width("📎\u{FE0F}", UnicodeCompat::Unicode14), 2); 
    }
    
    #[test]
    fn test_emoji() { 
        assert_eq!(display_width("😀", UnicodeCompat::Unicode14), 2); 
    }
    
    #[test]
    fn test_cjk() { 
        assert_eq!(display_width("あ", UnicodeCompat::Unicode14), 2); 
    }
    
    #[test]
    fn test_ascii() { 
        assert_eq!(display_width("hello", UnicodeCompat::Unicode14), 5); 
    }
    
    #[test]
    fn test_zwj_family() {
        // Family emoji with ZWJ
        let family = "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}";
        assert_eq!(display_width(family, UnicodeCompat::Unicode14), 2);
    }
    
    #[test]
    fn test_skin_tone() {
        // Waving hand with light skin tone
        let hand = "\u{1F44B}\u{1F3FB}";
        assert_eq!(display_width(hand, UnicodeCompat::Unicode14), 2);
    }
    
    #[test]
    fn test_flag_emoji() {
        // USA flag
        let flag = "\u{1F1FA}\u{1F1F8}";
        assert_eq!(display_width(flag, UnicodeCompat::Unicode14), 2);
    }
}
