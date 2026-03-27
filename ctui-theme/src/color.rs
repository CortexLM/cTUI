//! Color types for terminal theming
//!
//! This module provides a rich color system supporting:
//! - Named colors (standard ANSI colors)
//! - Indexed colors (256-color palette)
//! - RGB colors (24-bit true color)
//!
//! All colors are serializable to/from TOML for theme files.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Named ANSI colors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum NamedColor {
    /// Default terminal color (reset)
    #[default]
    Default,
    /// Black (#000000)
    Black,
    /// Red (#aa0000)
    Red,
    /// Green (#00aa00)
    Green,
    /// Yellow (#aa5500)
    Yellow,
    /// Blue (#0000aa)
    Blue,
    /// Magenta (#aa00aa)
    Magenta,
    /// Cyan (#00aaaa)
    Cyan,
    /// White (#aaaaaa)
    White,
    /// Bright black / dark gray (#555555)
    BrightBlack,
    /// Bright red (#ff5555)
    BrightRed,
    /// Bright green (#55ff55)
    BrightGreen,
    /// Bright yellow (#ffff55)
    BrightYellow,
    /// Bright blue (#5555ff)
    BrightBlue,
    /// Bright magenta (#ff55ff)
    BrightMagenta,
    /// Bright cyan (#55ffff)
    BrightCyan,
    /// Bright white (#ffffff)
    BrightWhite,
}

impl fmt::Display for NamedColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Black => write!(f, "black"),
            Self::Red => write!(f, "red"),
            Self::Green => write!(f, "green"),
            Self::Yellow => write!(f, "yellow"),
            Self::Blue => write!(f, "blue"),
            Self::Magenta => write!(f, "magenta"),
            Self::Cyan => write!(f, "cyan"),
            Self::White => write!(f, "white"),
            Self::BrightBlack => write!(f, "bright_black"),
            Self::BrightRed => write!(f, "bright_red"),
            Self::BrightGreen => write!(f, "bright_green"),
            Self::BrightYellow => write!(f, "bright_yellow"),
            Self::BrightBlue => write!(f, "bright_blue"),
            Self::BrightMagenta => write!(f, "bright_magenta"),
            Self::BrightCyan => write!(f, "bright_cyan"),
            Self::BrightWhite => write!(f, "bright_white"),
        }
    }
}

impl std::str::FromStr for NamedColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" | "reset" => Ok(Self::Default),
            "black" => Ok(Self::Black),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "yellow" => Ok(Self::Yellow),
            "blue" => Ok(Self::Blue),
            "magenta" => Ok(Self::Magenta),
            "cyan" => Ok(Self::Cyan),
            "white" => Ok(Self::White),
            "bright_black" | "dark_gray" | "dark_grey" => Ok(Self::BrightBlack),
            "bright_red" | "light_red" => Ok(Self::BrightRed),
            "bright_green" | "light_green" => Ok(Self::BrightGreen),
            "bright_yellow" | "light_yellow" => Ok(Self::BrightYellow),
            "bright_blue" | "light_blue" => Ok(Self::BrightBlue),
            "bright_magenta" | "light_magenta" => Ok(Self::BrightMagenta),
            "bright_cyan" | "light_cyan" => Ok(Self::BrightCyan),
            "bright_white" | "gray" | "grey" | "light_gray" | "light_grey" => Ok(Self::BrightWhite),
            _ => Err(format!("Unknown named color: {s}")),
        }
    }
}

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rgb {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
}

impl Rgb {
    /// Creates a new RGB color
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates an RGB color from a hex string (#RRGGBB or RRGGBB)
    ///
    /// # Errors
    /// Returns an error if the string is not a valid hex color
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        if hex.len() != 6 {
            return Err(format!("Invalid hex color length: {hex}"));
        }
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|e| format!("Invalid red component: {e}"))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|e| format!("Invalid green component: {e}"))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|e| format!("Invalid blue component: {e}"))?;
        Ok(Self::new(r, g, b))
    }

    /// Converts to hex string format
    #[must_use]
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Creates a color from HSL values
    ///
    /// # Arguments
    /// * `h` - Hue (0-360)
    /// * `s` - Saturation (0.0-1.0)
    /// * `l` - Lightness (0.0-1.0)
    #[must_use]
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        let c = (1.0 - 2.0f64.mul_add(l, -1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self::new(
            ((r + m) * 255.0).round() as u8,
            ((g + m) * 255.0).round() as u8,
            ((b + m) * 255.0).round() as u8,
        )
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self::new(255, 255, 255)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// Converts HSV values to RGB values.
///
/// # Arguments
/// * `h` - Hue angle in degrees (0-360)
/// * `s` - Saturation (0.0-1.0)
/// * `v` - Value/Brightness (0.0-1.0)
///
/// # Returns
/// A tuple of (r, g, b) where each component is 0-255
#[must_use]
pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let h = h % 360.0;
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

/// Converts RGB values to HSV.
///
/// # Arguments
/// * `r` - Red component (0-255)
/// * `g` - Green component (0-255)
/// * `b` - Blue component (0-255)
///
/// # Returns
/// A tuple of (h, s, v) where:
/// - h is hue angle in degrees (0-360)
/// - s is saturation (0.0-1.0)
/// - v is value/brightness (0.0-1.0)
#[must_use]
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = f64::from(r) / 255.0;
    let g = f64::from(g) / 255.0;
    let b = f64::from(b) / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h, s, v)
}

/// Converts RGB values to a hex string.
///
/// # Arguments
/// * `r` - Red component (0-255)
/// * `g` - Green component (0-255)
/// * `b` - Blue component (0-255)
///
/// # Returns
/// A hex string in the format "#RRGGBB"
#[must_use]
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// Linearly interpolates between two RGB colors.
///
/// # Arguments
/// * `a` - Starting color
/// * `b` - Ending color
/// * `t` - Interpolation factor (0.0-1.0)
///
/// # Returns
/// The interpolated color
#[must_use]
pub fn lerp_rgb(a: Rgb, b: Rgb, t: f64) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    Rgb::new(
        (f64::from(b.r) - f64::from(a.r)).mul_add(t, f64::from(a.r)).round() as u8,
        (f64::from(b.g) - f64::from(a.g)).mul_add(t, f64::from(a.g)).round() as u8,
        (f64::from(b.b) - f64::from(a.b)).mul_add(t, f64::from(a.b)).round() as u8,
    )
}

pub struct ColorGradient {
    colors: Vec<Rgb>,
    positions: Vec<f64>,
}

impl ColorGradient {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            colors: Vec::new(),
            positions: Vec::new(),
        }
    }

    pub fn with_colors(colors: impl IntoIterator<Item = Rgb>) -> Self {
        let colors: Vec<Rgb> = colors.into_iter().collect();
        let len = colors.len();
        let positions: Vec<f64> = if len <= 1 {
            colors.iter().map(|_| 0.0).collect()
        } else {
            (0..len).map(|i| i as f64 / (len - 1) as f64).collect()
        };
        Self { colors, positions }
    }

    #[must_use]
    pub fn add_stop(mut self, position: f64, color: Rgb) -> Self {
        let position = position.clamp(0.0, 1.0);
        let insert_idx = self
            .positions
            .iter()
            .position(|&p| p > position)
            .unwrap_or(self.positions.len());
        self.colors.insert(insert_idx, color);
        self.positions.insert(insert_idx, position);
        self
    }

    #[must_use]
    pub fn sample(&self, t: f64) -> Option<Rgb> {
        if self.colors.is_empty() {
            return None;
        }

        let t = t.clamp(0.0, 1.0);

        if self.colors.len() == 1 {
            return Some(self.colors[0]);
        }

        for i in 0..self.positions.len() - 1 {
            if t >= self.positions[i] && t <= self.positions[i + 1] {
                let local_t = if self.positions[i + 1] == self.positions[i] {
                    0.0
                } else {
                    (t - self.positions[i]) / (self.positions[i + 1] - self.positions[i])
                };
                return Some(lerp_rgb(self.colors[i], self.colors[i + 1], local_t));
            }
        }

        if t <= self.positions[0] {
            Some(self.colors[0])
        } else {
            Some(self.colors[self.colors.len() - 1])
        }
    }

    #[must_use]
    pub fn sample_even(&self, count: usize) -> Vec<Rgb> {
        if count == 0 || self.is_empty() {
            return Vec::new();
        }

        (0..count)
            .map(|i| {
                self.sample(i as f64 / (count - 1) as f64)
                    .unwrap_or(Rgb::new(0, 0, 0))
            })
            .collect()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.colors.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    #[must_use]
    pub fn colors(&self) -> &[Rgb] {
        &self.colors
    }
}

impl Default for ColorGradient {
    fn default() -> Self {
        Self::new()
    }
}

/// A color that can be used in terminal output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// Named ANSI color
    Named(NamedColor),
    /// Indexed color (0-255)
    Indexed {
        /// Index into 256-color palette
        index: u8,
    },
    /// RGB true color
    Rgb(Rgb),
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Named(n) => serializer.serialize_str(&n.to_string()),
            Self::Indexed { index } => {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct("Color", 1)?;
                s.serialize_field("indexed", index)?;
                s.end()
            }
            Self::Rgb(rgb) => {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct("Color", 3)?;
                s.serialize_field("r", &rgb.r)?;
                s.serialize_field("g", &rgb.g)?;
                s.serialize_field("b", &rgb.b)?;
                s.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a color string or RGB/indexed object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                v.parse::<NamedColor>()
                    .map(Color::Named)
                    .or_else(|_| Color::from_hex(v))
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut r = None;
                let mut g = None;
                let mut b = None;
                let mut index = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "r" => r = Some(map.next_value()?),
                        "g" => g = Some(map.next_value()?),
                        "b" => b = Some(map.next_value()?),
                        "indexed" | "index" => index = Some(map.next_value()?),
                        other => {
                            return Err(de::Error::unknown_field(
                                other,
                                &["r", "g", "b", "indexed"],
                            ));
                        }
                    }
                }

                if let Some(index) = index {
                    return Ok(Color::Indexed { index });
                }

                match (r, g, b) {
                    (Some(r), Some(g), Some(b)) => Ok(Color::Rgb(Rgb::new(r, g, b))),
                    _ => Err(de::Error::custom("missing RGB components")),
                }
            }
        }

        deserializer.deserialize_any(ColorVisitor)
    }
}

impl Color {
    /// Creates a named color
    #[must_use]
    pub const fn named(name: NamedColor) -> Self {
        Self::Named(name)
    }

    /// Creates an indexed color
    #[must_use]
    pub const fn indexed(index: u8) -> Self {
        Self::Indexed { index }
    }

    /// Creates an RGB color
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(Rgb::new(r, g, b))
    }

    /// Creates a color from a hex string
    ///
    /// # Errors
    /// Returns an error if the string is not a valid hex color
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        Ok(Self::Rgb(Rgb::from_hex(hex)?))
    }

    /// Default terminal color
    #[must_use]
    pub const fn default_color() -> Self {
        Self::named(NamedColor::Default)
    }

    /// Black color
    #[must_use]
    pub const fn black() -> Self {
        Self::named(NamedColor::Black)
    }

    /// Red color
    #[must_use]
    pub const fn red() -> Self {
        Self::named(NamedColor::Red)
    }

    /// Green color
    #[must_use]
    pub const fn green() -> Self {
        Self::named(NamedColor::Green)
    }

    /// Yellow color
    #[must_use]
    pub const fn yellow() -> Self {
        Self::named(NamedColor::Yellow)
    }

    /// Blue color
    #[must_use]
    pub const fn blue() -> Self {
        Self::named(NamedColor::Blue)
    }

    /// Magenta color
    #[must_use]
    pub const fn magenta() -> Self {
        Self::named(NamedColor::Magenta)
    }

    /// Cyan color
    #[must_use]
    pub const fn cyan() -> Self {
        Self::named(NamedColor::Cyan)
    }

    /// White color
    #[must_use]
    pub const fn white() -> Self {
        Self::named(NamedColor::White)
    }

    /// Returns true if this is the default color
    #[must_use]
    pub const fn is_default(&self) -> bool {
        matches!(self, Self::Named(NamedColor::Default))
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::default_color()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(n) => write!(f, "{n}"),
            Self::Indexed { index } => write!(f, "indexed({index})"),
            Self::Rgb(rgb) => write!(f, "{rgb}"),
        }
    }
}

impl From<NamedColor> for Color {
    fn from(name: NamedColor) -> Self {
        Self::named(name)
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self::Rgb(rgb)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_color_default() {
        let color = NamedColor::default();
        assert_eq!(color, NamedColor::Default);
    }

    #[test]
    fn test_named_color_from_str() {
        assert_eq!("red".parse::<NamedColor>(), Ok(NamedColor::Red));
        assert_eq!(
            "bright_red".parse::<NamedColor>(),
            Ok(NamedColor::BrightRed)
        );
        assert_eq!("light_red".parse::<NamedColor>(), Ok(NamedColor::BrightRed));
        assert!("invalid".parse::<NamedColor>().is_err());
    }

    #[test]
    fn test_rgb_new() {
        let rgb = Rgb::new(255, 128, 64);
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 64);
    }

    #[test]
    fn test_rgb_from_hex() {
        let rgb = Rgb::from_hex("#ff8040").unwrap();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 64);

        let rgb = Rgb::from_hex("ff8040").unwrap();
        assert_eq!(rgb.r, 255);

        assert!(Rgb::from_hex("#gg8040").is_err());
        assert!(Rgb::from_hex("#fff").is_err());
    }

    #[test]
    fn test_rgb_to_hex() {
        let rgb = Rgb::new(255, 128, 64);
        assert_eq!(rgb.to_hex(), "#ff8040");
    }

    #[test]
    fn test_rgb_from_hsl() {
        let red = Rgb::from_hsl(0.0, 1.0, 0.5);
        assert_eq!(red, Rgb::new(255, 0, 0));

        let green = Rgb::from_hsl(120.0, 1.0, 0.5);
        assert_eq!(green, Rgb::new(0, 255, 0));
    }

    #[test]
    fn test_color_named() {
        let color = Color::red();
        assert_eq!(color, Color::named(NamedColor::Red));
    }

    #[test]
    fn test_color_indexed() {
        let color = Color::indexed(42);
        assert_eq!(color, Color::Indexed { index: 42 });
    }

    #[test]
    fn test_color_rgb() {
        let color = Color::rgb(255, 128, 64);
        assert!(matches!(color, Color::Rgb(_)));
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#ff0000").unwrap();
        assert!(matches!(color, Color::Rgb(_)));
    }

    #[test]
    fn test_color_is_default() {
        assert!(Color::default_color().is_default());
        assert!(!Color::red().is_default());
    }

    #[test]
    fn test_color_from_tuple() {
        let color: Color = (255, 0, 0).into();
        assert!(matches!(color, Color::Rgb(_)));
    }

    #[test]
    fn test_color_serde() {
        let color = Color::rgb(255, 128, 64);
        let json = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&json).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_named_color_serde() {
        let color = NamedColor::BrightRed;
        let json = serde_json::to_string(&color).unwrap();
        assert!(json.contains("bright_red"));
        let deserialized: NamedColor = serde_json::from_str(&json).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_hsv_to_rgb() {
        let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0);
        assert_eq!((r, g, b), (255, 0, 0));

        let (r, g, b) = hsv_to_rgb(120.0, 1.0, 1.0);
        assert_eq!((r, g, b), (0, 255, 0));

        let (r, g, b) = hsv_to_rgb(240.0, 1.0, 1.0);
        assert_eq!((r, g, b), (0, 0, 255));

        let (r, g, b) = hsv_to_rgb(60.0, 1.0, 1.0);
        assert_eq!((r, g, b), (255, 255, 0));

        let (r, g, b) = hsv_to_rgb(180.0, 1.0, 1.0);
        assert_eq!((r, g, b), (0, 255, 255));

        let (r, g, b) = hsv_to_rgb(300.0, 1.0, 1.0);
        assert_eq!((r, g, b), (255, 0, 255));

        let (r, g, b) = hsv_to_rgb(0.0, 0.0, 1.0);
        assert_eq!((r, g, b), (255, 255, 255));

        let (r, g, b) = hsv_to_rgb(0.0, 1.0, 0.5);
        assert!((r as i32 - 127).abs() <= 1, "Expected ~127, got {}", r);
        assert_eq!((g, b), (0, 0));
    }

    #[test]
    fn test_rgb_to_hsv() {
        let (h, s, v) = rgb_to_hsv(255, 0, 0);
        assert!((h - 0.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);

        let (h, s, v) = rgb_to_hsv(0, 255, 0);
        assert!((h - 120.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);

        let (h, s, v) = rgb_to_hsv(0, 0, 255);
        assert!((h - 240.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);

        let (h, s, v) = rgb_to_hsv(255, 255, 255);
        assert!((s - 0.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);

        let (h, s, v) = rgb_to_hsv(0, 0, 0);
        assert!((s - 0.0).abs() < 0.01);
        assert!((v - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_rgb_to_hex_function() {
        assert_eq!(rgb_to_hex(255, 128, 64), "#ff8040");
        assert_eq!(rgb_to_hex(0, 0, 0), "#000000");
        assert_eq!(rgb_to_hex(255, 255, 255), "#ffffff");
        assert_eq!(rgb_to_hex(16, 32, 64), "#102040");
    }

    #[test]
    fn test_lerp_rgb() {
        let a = Rgb::new(0, 0, 0);
        let b = Rgb::new(255, 255, 255);

        assert_eq!(lerp_rgb(a, b, 0.0), a);
        assert_eq!(lerp_rgb(a, b, 1.0), b);
        assert_eq!(lerp_rgb(a, b, 0.5), Rgb::new(128, 128, 128));

        let a = Rgb::new(255, 0, 0);
        let b = Rgb::new(0, 0, 255);
        assert_eq!(lerp_rgb(a, b, 0.5), Rgb::new(128, 0, 128));
    }

    #[test]
    fn test_color_gradient_new() {
        let grad = ColorGradient::new();
        assert!(grad.is_empty());

        let grad = ColorGradient::with_colors(vec![Rgb::new(255, 0, 0), Rgb::new(0, 0, 255)]);
        assert_eq!(grad.len(), 2);
    }

    #[test]
    fn test_color_gradient_sample() {
        let grad = ColorGradient::with_colors(vec![Rgb::new(255, 0, 0), Rgb::new(0, 0, 255)]);

        assert_eq!(grad.sample(0.0), Some(Rgb::new(255, 0, 0)));
        assert_eq!(grad.sample(1.0), Some(Rgb::new(0, 0, 255)));
        assert_eq!(grad.sample(0.5), Some(Rgb::new(128, 0, 128)));

        let grad = ColorGradient::with_colors(vec![Rgb::new(0, 0, 0)]);
        assert_eq!(grad.sample(0.5), Some(Rgb::new(0, 0, 0)));

        let grad = ColorGradient::new();
        assert!(grad.sample(0.5).is_none());
    }

    #[test]
    fn test_color_gradient_add_stop() {
        let grad = ColorGradient::new()
            .add_stop(0.0, Rgb::new(255, 0, 0))
            .add_stop(1.0, Rgb::new(0, 0, 255))
            .add_stop(0.5, Rgb::new(0, 255, 0));

        assert_eq!(grad.len(), 3);
        assert_eq!(grad.sample(0.5), Some(Rgb::new(0, 255, 0)));
    }

    #[test]
    fn test_color_gradient_sample_even() {
        let grad = ColorGradient::with_colors(vec![Rgb::new(0, 0, 0), Rgb::new(255, 255, 255)]);
        let samples = grad.sample_even(5);

        assert_eq!(samples.len(), 5);
        assert_eq!(samples[0], Rgb::new(0, 0, 0));
        assert_eq!(samples[4], Rgb::new(255, 255, 255));

        let empty_grad = ColorGradient::new();
        let empty_samples = empty_grad.sample_even(3);
        assert!(empty_samples.is_empty());

        let grad = ColorGradient::new();
        let samples = grad.sample_even(0);
        assert!(samples.is_empty());
    }

    #[test]
    fn test_hsv_rgb_roundtrip() {
        let test_colors = [
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
            (255, 0, 255),
            (0, 255, 255),
            (128, 128, 128),
            (64, 128, 192),
        ];

        for (r, g, b) in test_colors {
            let (h, s, v) = rgb_to_hsv(r, g, b);
            let (r2, g2, b2) = hsv_to_rgb(h, s, v);
            assert!(
                (r as i32 - r2 as i32).abs() <= 1,
                "Red mismatch for ({}, {}, {})",
                r,
                g,
                b
            );
            assert!(
                (g as i32 - g2 as i32).abs() <= 1,
                "Green mismatch for ({}, {}, {})",
                r,
                g,
                b
            );
            assert!(
                (b as i32 - b2 as i32).abs() <= 1,
                "Blue mismatch for ({}, {}, {})",
                r,
                g,
                b
            );
        }
    }
}
