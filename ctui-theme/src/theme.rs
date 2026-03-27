//! Theme tokens and definitions

use crate::color::Color;
use crate::style::{BorderStyle, Modifier, Spacing, Style};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Font weight options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FontWeight {
    /// Thin weight (100)
    Thin,
    /// Extra light weight (200)
    ExtraLight,
    /// Light weight (300)
    Light,
    /// Normal/regular weight (400)
    #[default]
    Normal,
    /// Medium weight (500)
    Medium,
    /// Semi-bold weight (600)
    SemiBold,
    /// Bold weight (700)
    Bold,
    /// Extra bold weight (800)
    ExtraBold,
    /// Black weight (900)
    Black,
}

impl FontWeight {
    /// Returns the numeric weight value
    #[must_use]
    pub const fn weight(&self) -> u16 {
        match self {
            Self::Thin => 100,
            Self::ExtraLight => 200,
            Self::Light => 300,
            Self::Normal => 400,
            Self::Medium => 500,
            Self::SemiBold => 600,
            Self::Bold => 700,
            Self::ExtraBold => 800,
            Self::Black => 900,
        }
    }
}

/// Font rendering hints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FontRendering {
    /// Default rendering
    #[default]
    Normal,
    /// Optimized for sharpness
    Crisp,
    /// Optimized for smooth edges
    Smooth,
    /// Monochrome rendering
    Monochrome,
}

/// Typography configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Typography {
    /// Font family name (for reference)
    pub family: String,
    /// Base font size (relative units)
    pub size: u16,
    /// Line height multiplier
    pub line_height: u16,
    /// Font weight
    #[serde(default)]
    pub weight: FontWeight,
    /// Letter spacing (0 = normal)
    #[serde(default)]
    pub letter_spacing: i16,
    /// Font rendering hint
    #[serde(default)]
    pub rendering: FontRendering,
    /// Default modifiers
    #[serde(default)]
    pub modifiers: Modifier,
}

impl Typography {
    /// Creates new typography settings
    #[must_use]
    pub fn new(family: &str, size: u16, line_height: u16) -> Self {
        Self {
            family: family.to_string(),
            size,
            line_height,
            weight: FontWeight::default(),
            letter_spacing: 0,
            rendering: FontRendering::default(),
            modifiers: Modifier::default(),
        }
    }

    /// Sets the font weight
    #[must_use]
    pub const fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Sets letter spacing
    #[must_use]
    pub const fn letter_spacing(mut self, spacing: i16) -> Self {
        self.letter_spacing = spacing;
        self
    }

    /// Sets font rendering hint
    #[must_use]
    pub const fn rendering(mut self, rendering: FontRendering) -> Self {
        self.rendering = rendering;
        self
    }

    /// Adds a modifier
    #[must_use]
    pub fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers |= modifier;
        self
    }
}

impl Default for Typography {
    fn default() -> Self {
        Self::new("monospace", 12, 16)
    }
}

/// Border configuration tokens
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BorderTokens {
    /// Default border style
    #[serde(default)]
    pub style: BorderStyle,
    /// Border radius (0 = no radius)
    #[serde(default)]
    pub radius: u16,
    /// Default border width
    #[serde(default = "default_border_width")]
    pub width: u16,
}

const fn default_border_width() -> u16 {
    1
}

impl Default for BorderTokens {
    fn default() -> Self {
        Self {
            style: BorderStyle::Single,
            radius: 0,
            width: 1,
        }
    }
}

/// Spacing scale tokens
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpacingTokens {
    /// Extra small spacing (usually 2)
    #[serde(default = "default_xs")]
    pub xs: u16,
    /// Small spacing (usually 4)
    #[serde(default = "default_sm")]
    pub sm: u16,
    /// Medium spacing (usually 8)
    #[serde(default = "default_md")]
    pub md: u16,
    /// Large spacing (usually 16)
    #[serde(default = "default_lg")]
    pub lg: u16,
    /// Extra large spacing (usually 24)
    #[serde(default = "default_xl")]
    pub xl: u16,
    /// 2x extra large spacing (usually 32)
    #[serde(default = "default_2xl")]
    pub x2: u16,
    /// 3x extra large spacing (usually 48)
    #[serde(default = "default_3xl")]
    pub x3: u16,
}

const fn default_xs() -> u16 {
    2
}
const fn default_sm() -> u16 {
    4
}
const fn default_md() -> u16 {
    8
}
const fn default_lg() -> u16 {
    16
}
const fn default_xl() -> u16 {
    24
}
const fn default_2xl() -> u16 {
    32
}
const fn default_3xl() -> u16 {
    48
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self {
            xs: default_xs(),
            sm: default_sm(),
            md: default_md(),
            lg: default_lg(),
            xl: default_xl(),
            x2: default_2xl(),
            x3: default_3xl(),
        }
    }
}

impl SpacingTokens {
    /// Creates a Spacing value from a size key
    #[must_use]
    pub fn spacing(&self, size: &str) -> Spacing {
        let value = match size {
            "xs" => self.xs,
            "sm" => self.sm,
            "md" => self.md,
            "lg" => self.lg,
            "xl" => self.xl,
            "2xl" | "x2" => self.x2,
            "3xl" | "x3" => self.x3,
            _ => self.md,
        };
        Spacing::all(value)
    }
}

/// Color palette for a theme
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Primary brand color
    pub primary: Color,
    /// Secondary brand color
    #[serde(default)]
    pub secondary: Color,
    /// Accent color for highlights
    #[serde(default)]
    pub accent: Color,
    /// Background color
    #[serde(default)]
    pub background: Color,
    /// Surface color (cards, panels)
    #[serde(default)]
    pub surface: Color,
    /// Text color
    #[serde(default)]
    pub text: Color,
    /// Muted text color
    #[serde(default)]
    pub text_muted: Color,
    /// Success color
    #[serde(default)]
    pub success: Color,
    /// Warning color
    #[serde(default)]
    pub warning: Color,
    /// Error color
    #[serde(default)]
    pub error: Color,
    /// Info color
    #[serde(default)]
    pub info: Color,
    /// Border color
    #[serde(default)]
    pub border: Color,
    /// Focus ring color
    #[serde(default)]
    pub focus: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        use crate::color::NamedColor;
        Self {
            primary: Color::named(NamedColor::Blue),
            secondary: Color::named(NamedColor::BrightBlack),
            accent: Color::named(NamedColor::Cyan),
            background: Color::named(NamedColor::Default),
            surface: Color::named(NamedColor::Default),
            text: Color::named(NamedColor::Default),
            text_muted: Color::named(NamedColor::BrightBlack),
            success: Color::named(NamedColor::Green),
            warning: Color::named(NamedColor::Yellow),
            error: Color::named(NamedColor::Red),
            info: Color::named(NamedColor::Cyan),
            border: Color::named(NamedColor::BrightBlack),
            focus: Color::named(NamedColor::Blue),
        }
    }
}

/// Component-specific styles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ComponentStyles {
    /// Button styles
    #[serde(default)]
    pub button: Style,
    /// Input field styles
    #[serde(default)]
    pub input: Style,
    /// Label styles
    #[serde(default)]
    pub label: Style,
    /// Header styles
    #[serde(default)]
    pub header: Style,
    /// Footer styles
    #[serde(default)]
    pub footer: Style,
    /// Sidebar styles
    #[serde(default)]
    pub sidebar: Style,
    /// Panel/Card styles
    #[serde(default)]
    pub panel: Style,
    /// List item styles
    #[serde(default)]
    pub list_item: Style,
    /// Selected item styles
    #[serde(default)]
    pub selected: Style,
}

/// Complete theme definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme author
    #[serde(default)]
    pub author: String,
    /// Theme description
    #[serde(default)]
    pub description: String,
    /// Theme version
    #[serde(default)]
    pub version: String,
    /// Color palette
    #[serde(default)]
    pub colors: ColorPalette,
    /// Spacing scale
    #[serde(default)]
    pub spacing: SpacingTokens,
    /// Typography settings
    #[serde(default)]
    pub typography: Typography,
    /// Border configuration
    #[serde(default)]
    pub borders: BorderTokens,
    /// Component styles
    #[serde(default)]
    pub components: ComponentStyles,
    /// Custom color tokens
    #[serde(default)]
    pub custom_colors: HashMap<String, Color>,
    /// Custom styles
    #[serde(default)]
    pub custom_styles: HashMap<String, Style>,
}

impl Theme {
    /// Creates a new theme with the given name
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }

    /// Gets a color by name (from palette or custom colors)
    #[must_use]
    pub fn color(&self, name: &str) -> Option<&Color> {
        match name {
            "primary" => Some(&self.colors.primary),
            "secondary" => Some(&self.colors.secondary),
            "accent" => Some(&self.colors.accent),
            "background" => Some(&self.colors.background),
            "surface" => Some(&self.colors.surface),
            "text" => Some(&self.colors.text),
            "text_muted" => Some(&self.colors.text_muted),
            "success" => Some(&self.colors.success),
            "warning" => Some(&self.colors.warning),
            "error" => Some(&self.colors.error),
            "info" => Some(&self.colors.info),
            "border" => Some(&self.colors.border),
            "focus" => Some(&self.colors.focus),
            _ => self.custom_colors.get(name),
        }
    }

    /// Gets a style by name (from components or custom styles)
    #[must_use]
    pub fn style(&self, name: &str) -> Option<&Style> {
        match name {
            "button" => Some(&self.components.button),
            "input" => Some(&self.components.input),
            "label" => Some(&self.components.label),
            "header" => Some(&self.components.header),
            "footer" => Some(&self.components.footer),
            "sidebar" => Some(&self.components.sidebar),
            "panel" => Some(&self.components.panel),
            "list_item" => Some(&self.components.list_item),
            "selected" => Some(&self.components.selected),
            _ => self.custom_styles.get(name),
        }
    }

    /// Gets spacing value by key
    #[must_use]
    pub fn spacing(&self, key: &str) -> Spacing {
        self.spacing.spacing(key)
    }

    /// Creates a dark theme preset
    #[must_use]
    pub fn dark() -> Self {
        use crate::color::NamedColor;
        Self {
            name: "dark".to_string(),
            author: "cTUI".to_string(),
            description: "Dark theme preset".to_string(),
            colors: ColorPalette {
                primary: Color::named(NamedColor::Blue),
                secondary: Color::named(NamedColor::BrightBlack),
                accent: Color::named(NamedColor::Cyan),
                background: Color::named(NamedColor::Black),
                surface: Color::named(NamedColor::BrightBlack),
                text: Color::named(NamedColor::White),
                text_muted: Color::named(NamedColor::BrightBlack),
                success: Color::named(NamedColor::Green),
                warning: Color::named(NamedColor::Yellow),
                error: Color::named(NamedColor::Red),
                info: Color::named(NamedColor::Cyan),
                border: Color::named(NamedColor::BrightBlack),
                focus: Color::named(NamedColor::Blue),
            },
            ..Self::default()
        }
    }

    /// Creates a light theme preset
    #[must_use]
    pub fn light() -> Self {
        use crate::color::NamedColor;
        Self {
            name: "light".to_string(),
            author: "cTUI".to_string(),
            description: "Light theme preset".to_string(),
            colors: ColorPalette {
                primary: Color::named(NamedColor::Blue),
                secondary: Color::named(NamedColor::BrightBlack),
                accent: Color::named(NamedColor::Cyan),
                background: Color::named(NamedColor::White),
                surface: Color::named(NamedColor::BrightWhite),
                text: Color::named(NamedColor::Black),
                text_muted: Color::named(NamedColor::BrightBlack),
                success: Color::named(NamedColor::Green),
                warning: Color::named(NamedColor::Yellow),
                error: Color::named(NamedColor::Red),
                info: Color::named(NamedColor::Cyan),
                border: Color::named(NamedColor::BrightBlack),
                focus: Color::named(NamedColor::Blue),
            },
            ..Self::default()
        }
    }

    /// Creates a Dracula-inspired theme
    #[must_use]
    pub fn dracula() -> Self {
        let bg = Color::rgb(40, 42, 54);
        let current_line = Color::rgb(68, 71, 90);
        let foreground = Color::rgb(248, 248, 242);

        Self {
            name: "dracula".to_string(),
            author: "cTUI".to_string(),
            description: "Dracula-inspired theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(189, 147, 249),  // purple
                secondary: Color::rgb(98, 114, 164), // comment
                accent: Color::rgb(255, 121, 198),   // pink
                background: bg,
                surface: current_line,
                text: foreground,
                text_muted: Color::rgb(98, 114, 164),
                success: Color::rgb(80, 250, 123),  // green
                warning: Color::rgb(255, 184, 108), // orange
                error: Color::rgb(255, 85, 85),     // red
                info: Color::rgb(139, 233, 253),    // cyan
                border: current_line,
                focus: Color::rgb(189, 147, 249),
            },
            ..Self::default()
        }
    }

    /// Creates a Nord-inspired theme
    #[must_use]
    pub fn nord() -> Self {
        Self {
            name: "nord".to_string(),
            author: "cTUI".to_string(),
            description: "Nord-inspired theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(136, 192, 208),    // nord8
                secondary: Color::rgb(76, 86, 106),    // nord3
                accent: Color::rgb(163, 190, 214),     // nord4
                background: Color::rgb(46, 52, 64),    // nord0
                surface: Color::rgb(59, 66, 82),       // nord1
                text: Color::rgb(236, 239, 244),       // nord6
                text_muted: Color::rgb(143, 188, 187), // nord7
                success: Color::rgb(163, 190, 214),    // nord4
                warning: Color::rgb(235, 203, 139),    // nord13
                error: Color::rgb(191, 97, 106),       // nord11
                info: Color::rgb(136, 192, 208),       // nord8
                border: Color::rgb(76, 86, 106),       // nord3
                focus: Color::rgb(136, 192, 208),      // nord8
            },
            ..Self::default()
        }
    }

    /// Creates a Gruvbox-inspired theme
    #[must_use]
    pub fn gruvbox() -> Self {
        Self {
            name: "gruvbox".to_string(),
            author: "cTUI".to_string(),
            description: "Gruvbox-inspired theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(254, 128, 25),     // orange
                secondary: Color::rgb(124, 111, 100),  // gray
                accent: Color::rgb(230, 159, 12),      // yellow
                background: Color::rgb(40, 40, 40),    // bg0
                surface: Color::rgb(60, 56, 54),       // bg1
                text: Color::rgb(235, 219, 178),       // fg0
                text_muted: Color::rgb(189, 174, 147), // fg2
                success: Color::rgb(152, 151, 26),     // green
                warning: Color::rgb(250, 189, 47),     // yellow
                error: Color::rgb(251, 73, 52),        // red
                info: Color::rgb(142, 192, 124),       // green bright
                border: Color::rgb(102, 92, 84),       // bg3
                focus: Color::rgb(214, 93, 14),        // orange
            },
            ..Self::default()
        }
    }

    /// Creates a Gruvbox Light theme
    #[must_use]
    pub fn gruvbox_light() -> Self {
        Self {
            name: "gruvbox_light".to_string(),
            author: "cTUI".to_string(),
            description: "Gruvbox Light theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(175, 58, 3),       // orange
                secondary: Color::rgb(124, 111, 100),  // gray
                accent: Color::rgb(184, 140, 8),       // yellow
                background: Color::rgb(251, 241, 199), // bg0
                surface: Color::rgb(242, 229, 188),    // bg1
                text: Color::rgb(60, 56, 54),          // fg0
                text_muted: Color::rgb(124, 111, 100), // fg2
                success: Color::rgb(121, 116, 14),     // green
                warning: Color::rgb(184, 140, 8),      // yellow
                error: Color::rgb(204, 36, 29),        // red
                info: Color::rgb(46, 125, 50),         // green bright
                border: Color::rgb(186, 175, 144),     // bg3
                focus: Color::rgb(175, 58, 3),         // orange
            },
            ..Self::default()
        }
    }

    /// Creates a Tokyo Night theme
    #[must_use]
    pub fn tokyo_night() -> Self {
        Self {
            name: "tokyo_night".to_string(),
            author: "cTUI".to_string(),
            description: "Tokyo Night theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(119, 148, 213),  // blue
                secondary: Color::rgb(83, 88, 103),  // comment
                accent: Color::rgb(158, 206, 106),   // green
                background: Color::rgb(26, 27, 38),  // bg
                surface: Color::rgb(35, 38, 52),     // bg_dark
                text: Color::rgb(192, 202, 245),     // fg
                text_muted: Color::rgb(83, 88, 103), // comment
                success: Color::rgb(158, 206, 106),  // green
                warning: Color::rgb(255, 158, 67),   // orange
                error: Color::rgb(255, 121, 121),    // red
                info: Color::rgb(119, 148, 213),     // blue
                border: Color::rgb(61, 68, 88),      // border
                focus: Color::rgb(119, 148, 213),    // blue
            },
            ..Self::default()
        }
    }

    /// Creates an One Dark theme
    #[must_use]
    pub fn one_dark() -> Self {
        Self {
            name: "one_dark".to_string(),
            author: "cTUI".to_string(),
            description: "One Dark theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(97, 175, 239),   // blue
                secondary: Color::rgb(92, 99, 112),  // mono-3
                accent: Color::rgb(198, 120, 221),   // purple
                background: Color::rgb(40, 44, 52),  // bg
                surface: Color::rgb(48, 52, 60),     // bg +10
                text: Color::rgb(171, 178, 191),     // fg
                text_muted: Color::rgb(92, 99, 112), // mono-3
                success: Color::rgb(152, 195, 121),  // green
                warning: Color::rgb(229, 192, 123),  // yellow
                error: Color::rgb(224, 108, 117),    // red
                info: Color::rgb(97, 175, 239),      // blue
                border: Color::rgb(66, 72, 82),      // gutter
                focus: Color::rgb(97, 175, 239),     // blue
            },
            ..Self::default()
        }
    }

    /// Creates a Solarized Dark theme
    #[must_use]
    pub fn solarized_dark() -> Self {
        Self {
            name: "solarized_dark".to_string(),
            author: "cTUI".to_string(),
            description: "Solarized Dark theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(38, 139, 210),     // blue
                secondary: Color::rgb(101, 123, 131),  // base01
                accent: Color::rgb(203, 75, 22),       // orange
                background: Color::rgb(0, 43, 54),     // base03
                surface: Color::rgb(7, 54, 66),        // base02
                text: Color::rgb(253, 246, 227),       // base3
                text_muted: Color::rgb(101, 123, 131), // base01
                success: Color::rgb(133, 153, 0),      // green
                warning: Color::rgb(181, 137, 0),      // yellow
                error: Color::rgb(220, 50, 47),        // red
                info: Color::rgb(42, 161, 152),        // cyan
                border: Color::rgb(88, 110, 117),      // base01
                focus: Color::rgb(211, 54, 130),       // magenta
            },
            ..Self::default()
        }
    }

    /// Creates a Solarized Light theme
    #[must_use]
    pub fn solarized_light() -> Self {
        Self {
            name: "solarized_light".to_string(),
            author: "cTUI".to_string(),
            description: "Solarized Light theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(38, 139, 210),     // blue
                secondary: Color::rgb(101, 123, 131),  // base01
                accent: Color::rgb(203, 75, 22),       // orange
                background: Color::rgb(253, 246, 227), // base3
                surface: Color::rgb(238, 232, 213),    // base2
                text: Color::rgb(0, 43, 54),           // base03
                text_muted: Color::rgb(101, 123, 131), // base01
                success: Color::rgb(133, 153, 0),      // green
                warning: Color::rgb(181, 137, 0),      // yellow
                error: Color::rgb(220, 50, 47),        // red
                info: Color::rgb(42, 161, 152),        // cyan
                border: Color::rgb(147, 161, 161),     // base1
                focus: Color::rgb(211, 54, 130),       // magenta
            },
            ..Self::default()
        }
    }

    /// Creates a Monokai theme
    #[must_use]
    pub fn monokai() -> Self {
        Self {
            name: "monokai".to_string(),
            author: "cTUI".to_string(),
            description: "Monokai theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(102, 217, 239),   // cyan
                secondary: Color::rgb(117, 113, 94),  // comment
                accent: Color::rgb(249, 38, 114),     // pink
                background: Color::rgb(39, 40, 34),   // bg
                surface: Color::rgb(49, 50, 44),      // bg lighter
                text: Color::rgb(248, 248, 242),      // fg
                text_muted: Color::rgb(117, 113, 94), // comment
                success: Color::rgb(166, 226, 46),    // green
                warning: Color::rgb(230, 219, 116),   // yellow
                error: Color::rgb(249, 38, 114),      // red
                info: Color::rgb(102, 217, 239),      // cyan
                border: Color::rgb(70, 72, 65),       // border
                focus: Color::rgb(253, 151, 31),      // orange
            },
            ..Self::default()
        }
    }

    /// Creates a Catppuccin Latte theme (Light)
    #[must_use]
    pub fn catppuccin_latte() -> Self {
        Self {
            name: "catppuccin_latte".to_string(),
            author: "cTUI".to_string(),
            description: "Catppuccin Latte theme (Light)".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(30, 102, 245),     // blue
                secondary: Color::rgb(144, 140, 170),  // surface2
                accent: Color::rgb(234, 118, 203),     // pink
                background: Color::rgb(239, 241, 245), // base
                surface: Color::rgb(230, 233, 239),    // mantle
                text: Color::rgb(76, 79, 105),         // text
                text_muted: Color::rgb(92, 95, 119),   // subtext0
                success: Color::rgb(64, 160, 43),      // green
                warning: Color::rgb(223, 142, 29),     // yellow
                error: Color::rgb(210, 15, 57),        // red
                info: Color::rgb(4, 165, 229),         // sky
                border: Color::rgb(204, 208, 218),     // surface1
                focus: Color::rgb(30, 102, 245),       // blue
            },
            ..Self::default()
        }
    }

    /// Creates a Catppuccin Frappé theme
    #[must_use]
    pub fn catppuccin_frappe() -> Self {
        Self {
            name: "catppuccin_frappe".to_string(),
            author: "cTUI".to_string(),
            description: "Catppuccin Frappé theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(140, 170, 238),    // blue
                secondary: Color::rgb(129, 140, 162),  // surface2
                accent: Color::rgb(244, 184, 228),     // pink
                background: Color::rgb(48, 52, 70),    // base
                surface: Color::rgb(57, 61, 82),       // mantle
                text: Color::rgb(198, 206, 239),       // text
                text_muted: Color::rgb(166, 173, 200), // subtext0
                success: Color::rgb(166, 209, 137),    // green
                warning: Color::rgb(230, 193, 113),    // yellow
                error: Color::rgb(231, 130, 132),      // red
                info: Color::rgb(153, 209, 219),       // sky
                border: Color::rgb(81, 88, 112),       // surface1
                focus: Color::rgb(140, 170, 238),      // blue
            },
            ..Self::default()
        }
    }

    /// Creates a Catppuccin Macchiato theme
    #[must_use]
    pub fn catppuccin_macchiato() -> Self {
        Self {
            name: "catppuccin_macchiato".to_string(),
            author: "cTUI".to_string(),
            description: "Catppuccin Macchiato theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(138, 173, 244),    // blue
                secondary: Color::rgb(125, 137, 162),  // surface2
                accent: Color::rgb(245, 169, 184),     // pink
                background: Color::rgb(36, 39, 58),    // base
                surface: Color::rgb(45, 48, 70),       // mantle
                text: Color::rgb(202, 211, 245),       // text
                text_muted: Color::rgb(169, 177, 214), // subtext0
                success: Color::rgb(166, 218, 149),    // green
                warning: Color::rgb(238, 212, 159),    // yellow
                error: Color::rgb(238, 153, 160),      // red
                info: Color::rgb(145, 215, 227),       // sky
                border: Color::rgb(73, 77, 100),       // surface1
                focus: Color::rgb(138, 173, 244),      // blue
            },
            ..Self::default()
        }
    }

    /// Creates a Catppuccin Mocha theme (Dark)
    #[must_use]
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "catppuccin_mocha".to_string(),
            author: "cTUI".to_string(),
            description: "Catppuccin Mocha theme (Dark)".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(137, 180, 250),    // blue
                secondary: Color::rgb(125, 137, 162),  // surface2
                accent: Color::rgb(245, 194, 231),     // pink
                background: Color::rgb(30, 30, 46),    // base
                surface: Color::rgb(41, 42, 60),       // mantle
                text: Color::rgb(205, 214, 244),       // text
                text_muted: Color::rgb(172, 182, 222), // subtext0
                success: Color::rgb(166, 227, 161),    // green
                warning: Color::rgb(249, 226, 175),    // yellow
                error: Color::rgb(243, 139, 168),      // red
                info: Color::rgb(148, 226, 213),       // sky
                border: Color::rgb(69, 71, 90),        // surface1
                focus: Color::rgb(137, 180, 250),      // blue
            },
            ..Self::default()
        }
    }

    /// Creates a Nord Light theme
    #[must_use]
    pub fn nord_light() -> Self {
        Self {
            name: "nord_light".to_string(),
            author: "cTUI".to_string(),
            description: "Nord Light theme".to_string(),
            colors: ColorPalette {
                primary: Color::rgb(94, 129, 172),     // nord10
                secondary: Color::rgb(216, 222, 233),  // nord4
                accent: Color::rgb(180, 142, 173),     // nord15
                background: Color::rgb(236, 239, 244), // nord6
                surface: Color::rgb(229, 233, 240),    // nord5
                text: Color::rgb(46, 52, 64),          // nord0
                text_muted: Color::rgb(129, 142, 157), // nord8
                success: Color::rgb(72, 182, 133),     // nord14
                warning: Color::rgb(235, 203, 139),    // nord13
                error: Color::rgb(191, 97, 106),       // nord11
                info: Color::rgb(136, 192, 208),       // nord8
                border: Color::rgb(193, 202, 215),     // nord4
                focus: Color::rgb(94, 129, 172),       // nord10
            },
            ..Self::default()
        }
    }

    /// Returns a list of all available built-in theme names
    #[must_use]
    pub const fn available_themes() -> &'static [&'static str] {
        &[
            "dark",
            "light",
            "dracula",
            "nord",
            "nord_light",
            "gruvbox",
            "gruvbox_light",
            "tokyo_night",
            "one_dark",
            "solarized_dark",
            "solarized_light",
            "monokai",
            "catppuccin_latte",
            "catppuccin_frappe",
            "catppuccin_macchiato",
            "catppuccin_mocha",
        ]
    }

    /// Loads a built-in theme by name
    #[must_use]
    pub fn by_name(name: &str) -> Option<Self> {
        match name {
            "dark" => Some(Self::dark()),
            "light" => Some(Self::light()),
            "dracula" => Some(Self::dracula()),
            "nord" => Some(Self::nord()),
            "nord_light" => Some(Self::nord_light()),
            "gruvbox" => Some(Self::gruvbox()),
            "gruvbox_light" => Some(Self::gruvbox_light()),
            "tokyo_night" => Some(Self::tokyo_night()),
            "one_dark" => Some(Self::one_dark()),
            "solarized_dark" => Some(Self::solarized_dark()),
            "solarized_light" => Some(Self::solarized_light()),
            "monokai" => Some(Self::monokai()),
            "catppuccin_latte" => Some(Self::catppuccin_latte()),
            "catppuccin_frappe" => Some(Self::catppuccin_frappe()),
            "catppuccin_macchiato" => Some(Self::catppuccin_macchiato()),
            "catppuccin_mocha" => Some(Self::catppuccin_mocha()),
            _ => None,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            author: String::new(),
            description: String::new(),
            version: String::new(),
            colors: ColorPalette::default(),
            spacing: SpacingTokens::default(),
            typography: Typography::default(),
            borders: BorderTokens::default(),
            components: ComponentStyles::default(),
            custom_colors: HashMap::new(),
            custom_styles: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typography_new() {
        let typo = Typography::new("mono", 14, 20);
        assert_eq!(typo.family, "mono");
        assert_eq!(typo.size, 14);
        assert_eq!(typo.line_height, 20);
    }

    #[test]
    fn test_typography_with_modifier() {
        let typo = Typography::new("mono", 12, 16).with_modifier(Modifier::BOLD);
        assert!(typo.modifiers.contains(Modifier::BOLD));
    }

    #[test]
    fn test_spacing_tokens_default() {
        let spacing = SpacingTokens::default();
        assert_eq!(spacing.xs, 2);
        assert_eq!(spacing.sm, 4);
        assert_eq!(spacing.md, 8);
        assert_eq!(spacing.lg, 16);
        assert_eq!(spacing.xl, 24);
    }

    #[test]
    fn test_spacing_tokens_spacing() {
        let tokens = SpacingTokens::default();
        assert_eq!(tokens.spacing("xs"), Spacing::all(2));
        assert_eq!(tokens.spacing("md"), Spacing::all(8));
    }

    #[test]
    fn test_theme_new() {
        let theme = Theme::new("test");
        assert_eq!(theme.name, "test");
    }

    #[test]
    fn test_theme_get_color() {
        let theme = Theme::dark();
        assert!(theme.color("primary").is_some());
        assert!(theme.color("nonexistent").is_none());
    }

    #[test]
    fn test_theme_get_style() {
        let theme = Theme::default();
        assert!(theme.style("button").is_some());
        assert!(theme.style("nonexistent").is_none());
    }

    #[test]
    fn test_theme_presets() {
        let dark = Theme::dark();
        assert_eq!(dark.name, "dark");

        let light = Theme::light();
        assert_eq!(light.name, "light");

        let dracula = Theme::dracula();
        assert_eq!(dracula.name, "dracula");

        let nord = Theme::nord();
        assert_eq!(nord.name, "nord");

        let gruvbox = Theme::gruvbox();
        assert_eq!(gruvbox.name, "gruvbox");

        let gruvbox_light = Theme::gruvbox_light();
        assert_eq!(gruvbox_light.name, "gruvbox_light");

        let tokyo_night = Theme::tokyo_night();
        assert_eq!(tokyo_night.name, "tokyo_night");

        let one_dark = Theme::one_dark();
        assert_eq!(one_dark.name, "one_dark");

        let solarized_dark = Theme::solarized_dark();
        assert_eq!(solarized_dark.name, "solarized_dark");

        let solarized_light = Theme::solarized_light();
        assert_eq!(solarized_light.name, "solarized_light");

        let monokai = Theme::monokai();
        assert_eq!(monokai.name, "monokai");

        let catppuccin_latte = Theme::catppuccin_latte();
        assert_eq!(catppuccin_latte.name, "catppuccin_latte");

        let catppuccin_frappe = Theme::catppuccin_frappe();
        assert_eq!(catppuccin_frappe.name, "catppuccin_frappe");

        let catppuccin_macchiato = Theme::catppuccin_macchiato();
        assert_eq!(catppuccin_macchiato.name, "catppuccin_macchiato");

        let catppuccin_mocha = Theme::catppuccin_mocha();
        assert_eq!(catppuccin_mocha.name, "catppuccin_mocha");

        let nord_light = Theme::nord_light();
        assert_eq!(nord_light.name, "nord_light");
    }

    #[test]
    fn test_theme_available_themes() {
        let themes = Theme::available_themes();
        assert!(themes.len() >= 16);
        assert!(themes.contains(&"dark"));
        assert!(themes.contains(&"tokyo_night"));
        assert!(themes.contains(&"catppuccin_mocha"));
    }

    #[test]
    fn test_theme_by_name() {
        assert!(Theme::by_name("dracula").is_some());
        assert!(Theme::by_name("nonexistent").is_none());
        assert_eq!(Theme::by_name("nord").unwrap().name, "nord");
    }

    #[test]
    fn test_font_weight() {
        assert_eq!(FontWeight::Thin.weight(), 100);
        assert_eq!(FontWeight::Normal.weight(), 400);
        assert_eq!(FontWeight::Bold.weight(), 700);
        assert_eq!(FontWeight::Black.weight(), 900);
    }

    #[test]
    fn test_typography_weight() {
        let typo = Typography::new("mono", 12, 16).weight(FontWeight::Bold);
        assert_eq!(typo.weight, FontWeight::Bold);
    }

    #[test]
    fn test_typography_letter_spacing() {
        let typo = Typography::new("mono", 12, 16).letter_spacing(2);
        assert_eq!(typo.letter_spacing, 2);
    }

    #[test]
    fn test_theme_custom_colors() {
        let mut theme = Theme::default();
        theme
            .custom_colors
            .insert("brand".to_string(), Color::rgb(255, 128, 64));

        assert!(theme.color("brand").is_some());
    }

    #[test]
    fn test_theme_serde() {
        let theme = Theme::dark();
        let json = serde_json::to_string(&theme).unwrap();
        let deserialized: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme.name, deserialized.name);
    }

    #[test]
    fn test_color_palette_default() {
        let palette = ColorPalette::default();
        assert!(!palette.primary.is_default());
        assert!(palette.background.is_default());
    }
}
