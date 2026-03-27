//! Theme validation and accessibility checking
//!
//! Provides validation for theme completeness and WCAG contrast checking.

use crate::color::{Color, NamedColor, Rgb};
use crate::theme::Theme;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// WCAG conformance level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum WcagLevel {
    /// WCAG AA (minimum contrast 4.5:1 for normal text, 3:1 for large text)
    #[default]
    AA,
    /// WCAG AAA (minimum contrast 7:1 for normal text, 4.5:1 for large text)
    AAA,
}


impl WcagLevel {
    /// Returns the minimum contrast ratio for normal text
    #[must_use]
    pub const fn min_contrast_normal(&self) -> f64 {
        match self {
            Self::AA => 4.5,
            Self::AAA => 7.0,
        }
    }

    /// Returns the minimum contrast ratio for large text
    #[must_use]
    pub const fn min_contrast_large(&self) -> f64 {
        match self {
            Self::AA => 3.0,
            Self::AAA => 4.5,
        }
    }
}

/// Validation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the theme passed validation
    pub valid: bool,
    /// Validation errors
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    /// Returns true if validation passed (no errors)
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Validation error types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationError {
    /// Theme name is empty
    EmptyName,
    /// Required color token is missing
    MissingColor(String),
    /// Color contrast is insufficient
    InsufficientContrast {
        /// Foreground color name
        foreground: String,
        /// Background color name
        background: String,
        /// Actual contrast ratio
        ratio: String,
        /// Required contrast ratio
        required: String,
        /// WCAG level that was not met
        level: WcagLevel,
    },
}

/// Validation warning types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationWarning {
    /// Optional token is missing
    MissingOptionalToken(String),
    /// Color may not display well on all terminals
    TerminalCompatibility(String),
    /// Custom token may conflict with standard
    CustomTokenConflict(String),
}

/// Theme validator
#[derive(Debug, Clone, Default)]
pub struct ThemeValidator {
    /// WCAG level to check against
    pub wcag_level: WcagLevel,
    /// Required color tokens
    pub required_colors: HashSet<String>,
    /// Check contrast ratios
    pub check_contrast: bool,
}

impl ThemeValidator {
    /// Creates a new validator with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            wcag_level: WcagLevel::AA,
            required_colors: Self::default_required_colors(),
            check_contrast: true,
        }
    }

    /// Sets the WCAG level
    #[must_use]
    pub const fn wcag_level(mut self, level: WcagLevel) -> Self {
        self.wcag_level = level;
        self
    }

    /// Enables or disables contrast checking
    #[must_use]
    pub const fn check_contrast(mut self, check: bool) -> Self {
        self.check_contrast = check;
        self
    }

    /// Returns the default required color tokens
    fn default_required_colors() -> HashSet<String> {
        [
            "primary",
            "secondary",
            "background",
            "surface",
            "text",
            "success",
            "warning",
            "error",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    /// Validates a theme
    #[must_use]
    pub fn validate(&self, theme: &Theme) -> ValidationResult {
        let mut errors = Vec::new();
        let warnings = Vec::new();

        if theme.name.is_empty() {
            errors.push(ValidationError::EmptyName);
        }

        for color in &self.required_colors {
            if theme.color(color).is_none() {
                errors.push(ValidationError::MissingColor(color.clone()));
            }
        }

        if self.check_contrast {
            self.check_contrast_ratios(theme, &mut errors);
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Checks contrast ratios between theme colors
    fn check_contrast_ratios(&self, theme: &Theme, errors: &mut Vec<ValidationError>) {
        let required = self.wcag_level.min_contrast_normal();

        let checks = [
            ("text", "background"),
            ("text_muted", "background"),
            ("primary", "background"),
            ("success", "background"),
            ("warning", "background"),
            ("error", "background"),
        ];

        for (fg, bg) in checks {
            if let (Some(fg_color), Some(bg_color)) = (theme.color(fg), theme.color(bg)) {
                let ratio = contrast_ratio(fg_color, bg_color);
                if ratio < required {
                    errors.push(ValidationError::InsufficientContrast {
                        foreground: fg.to_string(),
                        background: bg.to_string(),
                        ratio: format!("{ratio:.2}"),
                        required: format!("{required}"),
                        level: self.wcag_level,
                    });
                }
            }
        }
    }
}

/// Calculates the contrast ratio between two colors
#[must_use]
pub fn contrast_ratio(color1: &Color, color2: &Color) -> f64 {
    let lum1 = relative_luminance(color1);
    let lum2 = relative_luminance(color2);

    let lighter = lum1.max(lum2);
    let darker = lum1.min(lum2);

    (lighter + 0.05) / (darker + 0.05)
}

/// Calculates relative luminance of a color (WCAG formula)
#[must_use]
pub fn relative_luminance(color: &Color) -> f64 {
    let rgb = match color {
        Color::Rgb(rgb) => *rgb,
        Color::Named(named) => named_to_rgb(named),
        Color::Indexed { index } => indexed_to_rgb(*index),
    };

    let r = srgb_to_linear(rgb.r);
    let g = srgb_to_linear(rgb.g);
    let b = srgb_to_linear(rgb.b);

    0.0722f64.mul_add(b, 0.7152f64.mul_add(g, 0.2126 * r))
}

/// Converts sRGB component to linear
fn srgb_to_linear(value: u8) -> f64 {
    let v = f64::from(value) / 255.0;
    if v <= 0.03928 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

/// Converts named color to RGB
const fn named_to_rgb(named: &NamedColor) -> Rgb {
    match named {
        NamedColor::Default => Rgb::new(255, 255, 255),
        NamedColor::Black => Rgb::new(0, 0, 0),
        NamedColor::Red => Rgb::new(170, 0, 0),
        NamedColor::Green => Rgb::new(0, 170, 0),
        NamedColor::Yellow => Rgb::new(170, 85, 0),
        NamedColor::Blue => Rgb::new(0, 0, 170),
        NamedColor::Magenta => Rgb::new(170, 0, 170),
        NamedColor::Cyan => Rgb::new(0, 170, 170),
        NamedColor::White => Rgb::new(170, 170, 170),
        NamedColor::BrightBlack => Rgb::new(85, 85, 85),
        NamedColor::BrightRed => Rgb::new(255, 85, 85),
        NamedColor::BrightGreen => Rgb::new(85, 255, 85),
        NamedColor::BrightYellow => Rgb::new(255, 255, 85),
        NamedColor::BrightBlue => Rgb::new(85, 85, 255),
        NamedColor::BrightMagenta => Rgb::new(255, 85, 255),
        NamedColor::BrightCyan => Rgb::new(85, 255, 255),
        NamedColor::BrightWhite => Rgb::new(255, 255, 255),
    }
}

/// Converts indexed color to RGB
fn indexed_to_rgb(index: u8) -> Rgb {
    if index < 16 {
        let named = match index {
            0 => NamedColor::Black,
            1 => NamedColor::Red,
            2 => NamedColor::Green,
            3 => NamedColor::Yellow,
            4 => NamedColor::Blue,
            5 => NamedColor::Magenta,
            6 => NamedColor::Cyan,
            7 => NamedColor::White,
            8 => NamedColor::BrightBlack,
            9 => NamedColor::BrightRed,
            10 => NamedColor::BrightGreen,
            11 => NamedColor::BrightYellow,
            12 => NamedColor::BrightBlue,
            13 => NamedColor::BrightMagenta,
            14 => NamedColor::BrightCyan,
            15 => NamedColor::BrightWhite,
            _ => unreachable!(),
        };
        named_to_rgb(&named)
    } else if index < 232 {
        let n = index - 16;
        let r = (n / 36) * 51;
        let g = ((n % 36) / 6) * 51;
        let b = (n % 6) * 51;
        Rgb::new(r, g, b)
    } else {
        let gray = (index - 232) * 10 + 8;
        Rgb::new(gray, gray, gray)
    }
}

/// Accessibility audit result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessibilityAudit {
    /// Overall score (0-100)
    pub score: u8,
    /// Contrast issues found
    pub contrast_issues: Vec<ContrastIssue>,
    /// WCAG level achieved (if any)
    pub wcag_level_achieved: Option<WcagLevel>,
}

/// A contrast issue found during audit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContrastIssue {
    /// Foreground color name
    pub foreground: String,
    /// Background color name
    pub background: String,
    /// Actual contrast ratio
    pub ratio: f64,
    /// Required contrast ratio
    pub required: f64,
    /// Whether this is critical
    pub critical: bool,
}

/// Performs an accessibility audit on a theme
#[must_use]
pub fn audit_accessibility(theme: &Theme) -> AccessibilityAudit {
    let mut issues = Vec::new();
    let mut aa_passes = true;
    let mut aaa_passes = true;

    let checks = [
        ("text", "background", true),
        ("text_muted", "background", false),
        ("primary", "background", true),
        ("secondary", "background", false),
        ("accent", "background", false),
        ("success", "background", true),
        ("warning", "background", true),
        ("error", "background", true),
        ("info", "background", false),
    ];

    for (fg, bg, critical) in checks {
        if let (Some(fg_color), Some(bg_color)) = (theme.color(fg), theme.color(bg)) {
            let ratio = contrast_ratio(fg_color, bg_color);

            let aa_required = WcagLevel::AA.min_contrast_normal();
            let aaa_required = WcagLevel::AAA.min_contrast_normal();

            if ratio < aa_required && critical {
                aa_passes = false;
            }
            if ratio < aaa_required && critical {
                aaa_passes = false;
            }

            if ratio < aa_required {
                issues.push(ContrastIssue {
                    foreground: fg.to_string(),
                    background: bg.to_string(),
                    ratio,
                    required: aa_required,
                    critical,
                });
            }
        }
    }

    let wcag_level_achieved = if aaa_passes {
        Some(WcagLevel::AAA)
    } else if aa_passes {
        Some(WcagLevel::AA)
    } else {
        None
    };

    let critical_count = issues.iter().filter(|i| i.critical).count();
    let _total_count = checks.iter().filter(|(_, _, c)| *c).count();

    let score = if issues.is_empty() {
        100
    } else {
        let base_score =
            ((issues.len() - critical_count) as f64).mul_add(-5.0, (critical_count as f64).mul_add(-15.0, 100.0));
        base_score.max(0.0) as u8
    };

    AccessibilityAudit {
        score,
        contrast_issues: issues,
        wcag_level_achieved,
    }
}

/// Extension trait for Color to provide accessibility methods
pub trait ColorExt {
    /// Returns the relative luminance of this color
    fn luminance(&self) -> f64;
    /// Returns the contrast ratio with another color
    fn contrast_with(&self, other: &Color) -> f64;
    /// Returns true if this color passes WCAG AA contrast with the other color
    fn passes_aa(&self, other: &Color) -> bool;
    /// Returns true if this color passes WCAG AAA contrast with the other color
    fn passes_aaa(&self, other: &Color) -> bool;
}

impl ColorExt for Color {
    fn luminance(&self) -> f64 {
        relative_luminance(self)
    }

    fn contrast_with(&self, other: &Color) -> f64 {
        contrast_ratio(self, other)
    }

    fn passes_aa(&self, other: &Color) -> bool {
        contrast_ratio(self, other) >= WcagLevel::AA.min_contrast_normal()
    }

    fn passes_aaa(&self, other: &Color) -> bool {
        contrast_ratio(self, other) >= WcagLevel::AAA.min_contrast_normal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wcag_level_defaults() {
        let level = WcagLevel::default();
        assert_eq!(level.min_contrast_normal(), 4.5);
        assert_eq!(level.min_contrast_large(), 3.0);
    }

    #[test]
    fn test_contrast_ratio_black_white() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        let ratio = contrast_ratio(&black, &white);
        assert!(ratio >= 20.0);
    }

    #[test]
    fn test_contrast_ratio_same_color() {
        let color = Color::rgb(128, 128, 128);
        let ratio = contrast_ratio(&color, &color);
        assert!((ratio - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_relative_luminance() {
        let black = Color::rgb(0, 0, 0);
        assert_eq!(relative_luminance(&black), 0.0);

        let white = Color::rgb(255, 255, 255);
        assert!((relative_luminance(&white) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_theme_validator_valid() {
        let theme = Theme::dracula();
        let validator = ThemeValidator::new().check_contrast(false);
        let result = validator.validate(&theme);
        assert!(result.is_valid());
    }

    #[test]
    fn test_theme_validator_empty_name() {
        let theme = Theme {
            name: String::new(),
            ..Theme::default()
        };
        let validator = ThemeValidator::new().check_contrast(false);
        let result = validator.validate(&theme);
        assert!(!result.is_valid());
        assert!(result.errors.contains(&ValidationError::EmptyName));
    }

    #[test]
    fn test_accessibility_audit() {
        let theme = Theme::nord();
        let audit = audit_accessibility(&theme);
        assert!(audit.score > 50);
    }

    #[test]
    fn test_color_ext_trait() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);

        assert!(black.passes_aa(&white));
        assert!(black.passes_aaa(&white));
        assert_eq!(black.luminance(), 0.0);
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult {
            valid: true,
            errors: vec![],
            warnings: vec![],
        };
        assert!(valid.is_valid());

        let invalid = ValidationResult {
            valid: false,
            errors: vec![ValidationError::EmptyName],
            warnings: vec![],
        };
        assert!(!invalid.is_valid());
    }
}
