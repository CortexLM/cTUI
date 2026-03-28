//! Packed cell representation for memory-efficient buffer storage
//!
//! This module provides a compact `PackedCell` type that reduces memory usage
//! by storing cell data in a packed format instead of the ~40-byte `Cell` struct.
//!
//! # Memory Layout (default, without `float-colors` feature)
//! - `symbol_id: u16` - references `SymbolTable` for string interning
//! - `fg_packed: u16` - packed foreground color (named/indexed/RGB565)
//! - `bg_packed: u16` - packed background color (named/indexed/RGB565)
//! - `modifier_skip: u16` - modifier flags (bits 0-8) + skip flag (bit 15)
//!
//! Total: 8 bytes (vs ~40 bytes for `Cell`)
//!
//! # Memory Layout (with `float-colors` feature)
//! - `symbol_id: u16` - references `SymbolTable` for string interning
//! - `modifier_skip: u16` - modifier flags + skip flag
//! - `fg: Color32` - foreground color as f32 RGBA (16 bytes)
//! - `bg: Color32` - background color as f32 RGBA (16 bytes)
//!
//! Total: 36 bytes (Color32 provides full precision f32 color channels)

use crate::cell::Cell;
use crate::style::{Color, Modifier};
use crate::symbol_table::{SymbolId, SymbolTable};

#[cfg(feature = "float-colors")]
use crate::style::Color32;

// Color encoding constants (16-bit) - only used without float-colors feature
#[cfg(not(feature = "float-colors"))]
const COLOR_BIT_15: u16 = 0x8000;
#[cfg(not(feature = "float-colors"))]
const COLOR_BIT_14: u16 = 0x4000;

#[cfg(not(feature = "float-colors"))]
#[inline]
fn is_named_color(packed: u16) -> bool {
    packed & COLOR_BIT_15 == 0
}

#[cfg(not(feature = "float-colors"))]
#[inline]
fn is_indexed_color(packed: u16) -> bool {
    (packed & COLOR_BIT_15) != 0 && (packed & COLOR_BIT_14) == 0
}

#[cfg(not(feature = "float-colors"))]
#[inline]
fn is_rgb_color(packed: u16) -> bool {
    (packed & COLOR_BIT_15) != 0 && (packed & COLOR_BIT_14) != 0
}

// Modifier bit positions (bits 0-8 of modifier_skip field)
const MODIFIER_BOLD: u16 = 1 << 0;
const MODIFIER_DIM: u16 = 1 << 1;
const MODIFIER_ITALIC: u16 = 1 << 2;
const MODIFIER_UNDERLINED: u16 = 1 << 3;
const MODIFIER_SLOW_BLINK: u16 = 1 << 4;
const MODIFIER_RAPID_BLINK: u16 = 1 << 5;
const MODIFIER_REVERSED: u16 = 1 << 6;
const MODIFIER_HIDDEN: u16 = 1 << 7;
const MODIFIER_CROSSED_OUT: u16 = 1 << 8;

// Skip flag (bit 15)
const SKIP_FLAG: u16 = 1 << 15;

/// Encodes a `Color` into a 16-bit packed representation.
#[cfg(not(feature = "float-colors"))]
fn pack_color(color: Color) -> u16 {
    match color {
        Color::Reset => 0,
        Color::Black => 1,
        Color::Red => 2,
        Color::Green => 3,
        Color::Yellow => 4,
        Color::Blue => 5,
        Color::Magenta => 6,
        Color::Cyan => 7,
        Color::White => 8,
        Color::DarkGray => 9,
        Color::LightRed => 10,
        Color::LightGreen => 11,
        Color::LightYellow => 12,
        Color::LightBlue => 13,
        Color::LightMagenta => 14,
        Color::LightCyan => 15,
        Color::Gray => 16,
        Color::Indexed(idx) => COLOR_BIT_15 | (idx as u16),
        Color::Rgb(r, g, b) => {
            let r5 = (r as u16 >> 3) & 0x1F;
            let g4 = (g as u16 >> 4) & 0x0F;
            let b5 = (b as u16 >> 3) & 0x1F;
            COLOR_BIT_15 | COLOR_BIT_14 | (r5 << 9) | (g4 << 5) | b5
        }
    }
}

/// Decodes a 16-bit packed color back to `Color`.
#[cfg(not(feature = "float-colors"))]
fn unpack_color(packed: u16) -> Color {
    if is_named_color(packed) {
        match packed & 0x001F {
            0 => Color::Reset,
            1 => Color::Black,
            2 => Color::Red,
            3 => Color::Green,
            4 => Color::Yellow,
            5 => Color::Blue,
            6 => Color::Magenta,
            7 => Color::Cyan,
            8 => Color::White,
            9 => Color::DarkGray,
            10 => Color::LightRed,
            11 => Color::LightGreen,
            12 => Color::LightYellow,
            13 => Color::LightBlue,
            14 => Color::LightMagenta,
            15 => Color::LightCyan,
            16 => Color::Gray,
            _ => Color::Reset,
        }
    } else if is_indexed_color(packed) {
        Color::Indexed((packed & 0x00FF) as u8)
    } else if is_rgb_color(packed) {
        let r5 = ((packed >> 9) & 0x1F) as u8;
        let g4 = ((packed >> 5) & 0x0F) as u8;
        let b5 = (packed & 0x1F) as u8;
        let r = (r5 << 3) | (r5 >> 2);
        let g = (g4 << 4) | g4;
        let b = (b5 << 3) | (b5 >> 2);
        Color::Rgb(r, g, b)
    } else {
        Color::Reset
    }
}

/// Encodes `Modifier` flags into a 16-bit value (uses bits 0-8).
fn pack_modifier(modifier: Modifier) -> u16 {
    let mut packed: u16 = 0;
    if modifier.contains(Modifier::BOLD) {
        packed |= MODIFIER_BOLD;
    }
    if modifier.contains(Modifier::DIM) {
        packed |= MODIFIER_DIM;
    }
    if modifier.contains(Modifier::ITALIC) {
        packed |= MODIFIER_ITALIC;
    }
    if modifier.contains(Modifier::UNDERLINED) {
        packed |= MODIFIER_UNDERLINED;
    }
    if modifier.contains(Modifier::SLOW_BLINK) {
        packed |= MODIFIER_SLOW_BLINK;
    }
    if modifier.contains(Modifier::RAPID_BLINK) {
        packed |= MODIFIER_RAPID_BLINK;
    }
    if modifier.contains(Modifier::REVERSED) {
        packed |= MODIFIER_REVERSED;
    }
    if modifier.contains(Modifier::HIDDEN) {
        packed |= MODIFIER_HIDDEN;
    }
    if modifier.contains(Modifier::CROSSED_OUT) {
        packed |= MODIFIER_CROSSED_OUT;
    }
    packed
}

/// Decodes a 16-bit value back to `Modifier`.
fn unpack_modifier(packed: u16) -> Modifier {
    let mut modifier = Modifier::empty();
    if packed & MODIFIER_BOLD != 0 {
        modifier |= Modifier::BOLD;
    }
    if packed & MODIFIER_DIM != 0 {
        modifier |= Modifier::DIM;
    }
    if packed & MODIFIER_ITALIC != 0 {
        modifier |= Modifier::ITALIC;
    }
    if packed & MODIFIER_UNDERLINED != 0 {
        modifier |= Modifier::UNDERLINED;
    }
    if packed & MODIFIER_SLOW_BLINK != 0 {
        modifier |= Modifier::SLOW_BLINK;
    }
    if packed & MODIFIER_RAPID_BLINK != 0 {
        modifier |= Modifier::RAPID_BLINK;
    }
    if packed & MODIFIER_REVERSED != 0 {
        modifier |= Modifier::REVERSED;
    }
    if packed & MODIFIER_HIDDEN != 0 {
        modifier |= Modifier::HIDDEN;
    }
    if packed & MODIFIER_CROSSED_OUT != 0 {
        modifier |= Modifier::CROSSED_OUT;
    }
    modifier
}

// ============================================================================
// PackedCell without float-colors feature (8 bytes)
// ============================================================================

/// A compact cell representation using packed bitfields (8 bytes).
///
/// This is the default representation when `float-colors` feature is not enabled.
/// Uses u16 packed color encoding for memory efficiency.
#[cfg(not(feature = "float-colors"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct PackedCell {
    /// Symbol identifier referencing `SymbolTable`
    symbol_id: u16,
    /// Packed foreground color (named/indexed/RGB565)
    fg_packed: u16,
    /// Packed background color (named/indexed/RGB565)
    bg_packed: u16,
    /// Modifier flags (bits 0-8) + skip flag (bit 15)
    modifier_skip: u16,
}

#[cfg(not(feature = "float-colors"))]
impl PackedCell {
    /// Creates a new `PackedCell` from a `Cell` using the provided `SymbolTable`.
    pub fn from_cell(cell: &Cell, symbol_table: &mut SymbolTable) -> Self {
        let symbol_id = symbol_table.insert(&cell.symbol);
        let fg_packed = pack_color(cell.fg);
        let bg_packed = pack_color(cell.bg);
        let mut modifier_skip = pack_modifier(cell.modifier);
        if cell.skip {
            modifier_skip |= SKIP_FLAG;
        }

        Self {
            symbol_id: symbol_id.as_u16(),
            fg_packed,
            bg_packed,
            modifier_skip,
        }
    }

    /// Converts this `PackedCell` back to a `Cell` using the provided `SymbolTable`.
    pub fn to_cell(&self, symbol_table: &SymbolTable) -> Cell {
        let symbol_id = SymbolId::from_u16_raw(self.symbol_id);
        let symbol = symbol_table.get(symbol_id).unwrap_or(" ").to_string();

        Cell {
            symbol,
            fg: unpack_color(self.fg_packed),
            bg: unpack_color(self.bg_packed),
            modifier: unpack_modifier(self.modifier_skip),
            skip: self.modifier_skip & SKIP_FLAG != 0,
        }
    }

    /// Returns the `SymbolId` for this cell.
    pub fn symbol_id(&self) -> SymbolId {
        SymbolId::from_u16_raw(self.symbol_id)
    }

    /// Returns the packed foreground color.
    pub fn fg_packed(&self) -> u16 {
        self.fg_packed
    }

    /// Returns the packed background color.
    pub fn bg_packed(&self) -> u16 {
        self.bg_packed
    }

    /// Returns the foreground color as `Color`.
    pub fn fg(&self) -> Color {
        unpack_color(self.fg_packed)
    }

    /// Returns the background color as `Color`.
    pub fn bg(&self) -> Color {
        unpack_color(self.bg_packed)
    }

    /// Returns the modifier flags.
    pub fn modifier(&self) -> Modifier {
        unpack_modifier(self.modifier_skip)
    }

    /// Returns whether this cell should be skipped.
    pub fn skip(&self) -> bool {
        self.modifier_skip & SKIP_FLAG != 0
    }
}

#[cfg(not(feature = "float-colors"))]
impl Default for PackedCell {
    fn default() -> Self {
        Self {
            symbol_id: 0,
            fg_packed: pack_color(Color::Reset),
            bg_packed: pack_color(Color::Reset),
            modifier_skip: 0,
        }
    }
}

// ============================================================================
// PackedCell with float-colors feature (36 bytes, full Color32 precision)
// ============================================================================

/// A cell representation with full Color32 precision (f32 RGBA channels).
///
/// This implementation is used when `float-colors` feature is enabled.
/// Provides high-precision color storage at the cost of increased memory usage.
///
/// # Memory Trade-off
/// - Without `float-colors`: 8 bytes per cell (u16 packed colors)
/// - With `float-colors`: 36 bytes per cell (full f32 precision)
///
/// The Color32 mode is useful for applications requiring:
/// - Smooth color gradients
/// - Alpha blending support
/// - Precise color manipulation
#[cfg(feature = "float-colors")]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct PackedCell {
    /// Symbol identifier referencing `SymbolTable`
    symbol_id: u16,
    /// Modifier flags (bits 0-8) + skip flag (bit 15)
    modifier_skip: u16,
    /// Foreground color as Color32 (f32 RGBA)
    fg: Color32,
    /// Background color as Color32 (f32 RGBA)
    bg: Color32,
}

#[cfg(feature = "float-colors")]
impl PackedCell {
    /// Creates a new `PackedCell` from a `Cell` using the provided `SymbolTable`.
    pub fn from_cell(cell: &Cell, symbol_table: &mut SymbolTable) -> Self {
        let symbol_id = symbol_table.insert(&cell.symbol);
        let mut modifier_skip = pack_modifier(cell.modifier);
        if cell.skip {
            modifier_skip |= SKIP_FLAG;
        }

        Self {
            symbol_id: symbol_id.as_u16(),
            modifier_skip,
            fg: Color32::from(cell.fg),
            bg: Color32::from(cell.bg),
        }
    }

    /// Converts this `PackedCell` back to a `Cell` using the provided `SymbolTable`.
    pub fn to_cell(&self, symbol_table: &SymbolTable) -> Cell {
        let symbol_id = SymbolId::from_u16_raw(self.symbol_id);
        let symbol = symbol_table.get(symbol_id).unwrap_or(" ").to_string();

        Cell {
            symbol,
            fg: Color::from(self.fg),
            bg: Color::from(self.bg),
            modifier: unpack_modifier(self.modifier_skip),
            skip: self.modifier_skip & SKIP_FLAG != 0,
        }
    }

    /// Returns the `SymbolId` for this cell.
    pub fn symbol_id(&self) -> SymbolId {
        SymbolId::from_u16_raw(self.symbol_id)
    }

    /// Returns the foreground color as Color32.
    pub fn fg_color32(&self) -> Color32 {
        self.fg
    }

    /// Returns the background color as Color32.
    pub fn bg_color32(&self) -> Color32 {
        self.bg
    }

    /// Returns the foreground color as `Color`.
    pub fn fg(&self) -> Color {
        Color::from(self.fg)
    }

    /// Returns the background color as `Color`.
    pub fn bg(&self) -> Color {
        Color::from(self.bg)
    }

    /// Returns the modifier flags.
    pub fn modifier(&self) -> Modifier {
        unpack_modifier(self.modifier_skip)
    }

    /// Returns whether this cell should be skipped.
    pub fn skip(&self) -> bool {
        self.modifier_skip & SKIP_FLAG != 0
    }
}

#[cfg(feature = "float-colors")]
impl Default for PackedCell {
    fn default() -> Self {
        Self {
            symbol_id: 0,
            modifier_skip: 0,
            fg: Color32::transparent(),
            bg: Color32::transparent(),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_cell_size() {
        // Without float-colors: 8 bytes
        // With float-colors: 36 bytes (2x u16 + 2x Color32)
        #[cfg(not(feature = "float-colors"))]
        assert!(
            std::mem::size_of::<PackedCell>() <= 8,
            "PackedCell is {} bytes, expected <= 8",
            std::mem::size_of::<PackedCell>()
        );

        #[cfg(feature = "float-colors")]
        assert!(
            std::mem::size_of::<PackedCell>() <= 40,
            "PackedCell is {} bytes, expected <= 40",
            std::mem::size_of::<PackedCell>()
        );
    }

    #[test]
    #[cfg(not(feature = "float-colors"))]
    fn test_packed_cell_size_exact() {
        // With repr(packed), should be exactly 8 bytes
        assert_eq!(std::mem::size_of::<PackedCell>(), 8);
    }

    #[test]
    #[cfg(feature = "float-colors")]
    fn test_packed_cell_size_float_colors() {
        // With float-colors: 2 (u16) + 2 (u16) + 16 (Color32) + 16 (Color32) = 36 bytes
        // Plus alignment padding may bring it to 40
        let size = std::mem::size_of::<PackedCell>();
        assert!(size >= 36 && size <= 40, "PackedCell size: {}", size);
    }

    #[test]
    fn test_packed_cell_default() {
        let cell = PackedCell::default();
        assert_eq!(cell.symbol_id().as_u16(), 0);
        assert_eq!(cell.modifier(), Modifier::empty());
        assert!(!cell.skip());
    }

    #[test]
    #[cfg(not(feature = "float-colors"))]
    fn test_packed_cell_default_colors_packed() {
        let cell = PackedCell::default();
        assert_eq!(cell.fg_packed(), pack_color(Color::Reset));
        assert_eq!(cell.bg_packed(), pack_color(Color::Reset));
    }

    #[test]
    fn test_pack_unpack_modifier() {
        let modifiers = [
            Modifier::empty(),
            Modifier::BOLD,
            Modifier::ITALIC,
            Modifier::BOLD | Modifier::ITALIC,
            Modifier::UNDERLINED | Modifier::DIM,
            Modifier::BOLD
                | Modifier::DIM
                | Modifier::ITALIC
                | Modifier::UNDERLINED
                | Modifier::SLOW_BLINK
                | Modifier::RAPID_BLINK
                | Modifier::REVERSED
                | Modifier::HIDDEN
                | Modifier::CROSSED_OUT,
        ];

        for modifier in modifiers {
            let packed = pack_modifier(modifier);
            let unpacked = unpack_modifier(packed);
            assert_eq!(modifier, unpacked, "Round-trip failed for {:?}", modifier);
        }
    }

    #[test]
    fn test_from_cell_to_cell_roundtrip() {
        let mut table = SymbolTable::new();

        let original = Cell {
            symbol: "A".to_string(),
            fg: Color::Red,
            bg: Color::Blue,
            modifier: Modifier::BOLD | Modifier::ITALIC,
            skip: false,
        };

        let packed = PackedCell::from_cell(&original, &mut table);
        let unpacked = packed.to_cell(&table);

        assert_eq!(original.symbol, unpacked.symbol);
        assert_eq!(original.modifier, unpacked.modifier);
        assert_eq!(original.skip, unpacked.skip);
    }

    #[test]
    fn test_from_cell_to_cell_with_skip() {
        let mut table = SymbolTable::new();

        let original = Cell {
            symbol: " ".to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
            skip: true,
        };

        let packed = PackedCell::from_cell(&original, &mut table);
        assert!(packed.skip());

        let unpacked = packed.to_cell(&table);
        assert!(unpacked.skip);
    }

    #[test]
    fn test_from_cell_to_cell_with_unicode() {
        let mut table = SymbolTable::new();

        let original = Cell {
            symbol: "漢字".to_string(),
            fg: Color::Green,
            bg: Color::Indexed(42),
            modifier: Modifier::UNDERLINED,
            skip: false,
        };

        let packed = PackedCell::from_cell(&original, &mut table);
        let unpacked = packed.to_cell(&table);

        assert_eq!(original.symbol, unpacked.symbol);
        assert_eq!(original.modifier, unpacked.modifier);
    }

    #[test]
    fn test_multiple_cells_share_symbol() {
        let mut table = SymbolTable::new();

        let cell1 = Cell::new("X");
        let cell2 = Cell::new("X");
        let cell3 = Cell::new("Y");

        let packed1 = PackedCell::from_cell(&cell1, &mut table);
        let packed2 = PackedCell::from_cell(&cell2, &mut table);
        let packed3 = PackedCell::from_cell(&cell3, &mut table);

        // Same symbol should have same ID
        assert_eq!(packed1.symbol_id(), packed2.symbol_id());
        // Different symbol should have different ID
        assert_ne!(packed1.symbol_id(), packed3.symbol_id());
    }

    #[test]
    fn test_all_modifier_flags() {
        let mut table = SymbolTable::new();

        let all_modifiers = Modifier::BOLD
            | Modifier::DIM
            | Modifier::ITALIC
            | Modifier::UNDERLINED
            | Modifier::SLOW_BLINK
            | Modifier::RAPID_BLINK
            | Modifier::REVERSED
            | Modifier::HIDDEN
            | Modifier::CROSSED_OUT;

        let original = Cell {
            symbol: "T".to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: all_modifiers,
            skip: true,
        };

        let packed = PackedCell::from_cell(&original, &mut table);
        let unpacked = packed.to_cell(&table);

        assert_eq!(original.modifier, unpacked.modifier);
        assert_eq!(original.skip, unpacked.skip);
    }

    // Tests specific to packed color encoding (without float-colors)
    #[cfg(not(feature = "float-colors"))]
    mod packed_color_tests {
        use super::*;

        #[test]
        fn test_pack_unpack_named_colors() {
            let colors = [
                Color::Reset,
                Color::Black,
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta,
                Color::Cyan,
                Color::White,
                Color::DarkGray,
                Color::LightRed,
                Color::LightGreen,
                Color::LightYellow,
                Color::LightBlue,
                Color::LightMagenta,
                Color::LightCyan,
                Color::Gray,
            ];

            for color in colors {
                let packed = pack_color(color);
                let unpacked = unpack_color(packed);
                assert_eq!(color, unpacked, "Round-trip failed for {:?}", color);
            }
        }

        #[test]
        fn test_pack_unpack_indexed_colors() {
            for i in 0u8..=255 {
                let color = Color::Indexed(i);
                let packed = pack_color(color);
                let unpacked = unpack_color(packed);
                assert_eq!(color, unpacked, "Round-trip failed for Indexed({})", i);
            }
        }

        #[test]
        fn test_pack_unpack_rgb_colors() {
            let test_colors = [
                Color::Rgb(0, 0, 0),
                Color::Rgb(255, 255, 255),
                Color::Rgb(255, 0, 0),
                Color::Rgb(0, 255, 0),
                Color::Rgb(0, 0, 255),
                Color::Rgb(128, 128, 128),
                Color::Rgb(255, 128, 64),
            ];

            for color in test_colors {
                let packed = pack_color(color);
                let unpacked = unpack_color(packed);
                if let Color::Rgb(r1, g1, b1) = color {
                    if let Color::Rgb(r2, g2, b2) = unpacked {
                        assert!(
                            (r1 as i16 - r2 as i16).abs() <= 8,
                            "R error for {:?}",
                            color
                        );
                        assert!(
                            (g1 as i16 - g2 as i16).abs() <= 17,
                            "G error for {:?}",
                            color
                        );
                        assert!(
                            (b1 as i16 - b2 as i16).abs() <= 8,
                            "B error for {:?}",
                            color
                        );
                    }
                }
            }
        }

        #[test]
        fn test_from_cell_to_cell_with_emoji() {
            let mut table = SymbolTable::new();

            let original = Cell {
                symbol: "👨‍👩‍👧‍👦".to_string(),
                fg: Color::Rgb(255, 100, 50),
                bg: Color::Reset,
                modifier: Modifier::empty(),
                skip: false,
            };

            let packed = PackedCell::from_cell(&original, &mut table);
            let unpacked = packed.to_cell(&table);

            assert_eq!(original.symbol, unpacked.symbol);
            if let Color::Rgb(r, g, b) = unpacked.fg {
                assert!((255i16 - r as i16).abs() <= 8);
                assert!((100i16 - g as i16).abs() <= 17);
                assert!((50i16 - b as i16).abs() <= 8);
            } else {
                panic!("Expected RGB color");
            }
        }
    }

    // Tests specific to Color32 mode
    #[cfg(feature = "float-colors")]
    mod float_color_tests {
        use super::*;

        #[test]
        fn test_color32_rgb_conversion() {
            let original = Color::Rgb(128, 64, 192);
            let color32 = Color32::from(original);
            
            // Verify conversion to f32 range
            assert!((color32.r - (128.0 / 255.0)).abs() < 0.01);
            assert!((color32.g - (64.0 / 255.0)).abs() < 0.01);
            assert!((color32.b - (192.0 / 255.0)).abs() < 0.01);
            assert!((color32.a - 1.0).abs() < 0.01);
        }

        #[test]
        fn test_from_cell_preserves_colors() {
            let mut table = SymbolTable::new();

            let original = Cell {
                symbol: "A".to_string(),
                fg: Color::Rgb(200, 100, 50),
                bg: Color::Rgb(25, 75, 125),
                modifier: Modifier::empty(),
                skip: false,
            };

            let packed = PackedCell::from_cell(&original, &mut table);
            let unpacked = packed.to_cell(&table);

            // Colors should be preserved exactly (lossless in float mode)
            if let Color::Rgb(r, g, b) = unpacked.fg {
                assert!((200i16 - r as i16).abs() <= 1, "FG R mismatch");
                assert!((100i16 - g as i16).abs() <= 1, "FG G mismatch");
                assert!((50i16 - b as i16).abs() <= 1, "FG B mismatch");
            } else {
                panic!("Expected RGB color for fg");
            }

            if let Color::Rgb(r, g, b) = unpacked.bg {
                assert!((25i16 - r as i16).abs() <= 1, "BG R mismatch");
                assert!((75i16 - g as i16).abs() <= 1, "BG G mismatch");
                assert!((125i16 - b as i16).abs() <= 1, "BG B mismatch");
            } else {
                panic!("Expected RGB color for bg");
            }
        }
    }
}
