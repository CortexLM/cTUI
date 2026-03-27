//! Theme transition and animation system
//!
//! Provides smooth theme switching with interpolation, timing functions,
//! and dark/light mode toggle support.

use crate::color::{Color, Rgb};
use crate::theme::Theme;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Easing function for transitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum Easing {
    /// Linear interpolation
    Linear,
    /// Ease-in (slow start)
    EaseIn,
    /// Ease-out (slow end)
    EaseOut,
    /// Ease-in-out (slow start and end)
    #[default]
    EaseInOut,
    /// Cubic ease-in
    CubicIn,
    /// Cubic ease-out
    CubicOut,
    /// Cubic ease-in-out
    CubicInOut,
    /// Quadratic ease-in
    QuadIn,
    /// Quadratic ease-out
    QuadOut,
    /// Quadratic ease-in-out
    QuadInOut,
    /// Exponential ease-out
    ExpoOut,
    /// Elastic bounce
    ElasticOut,
}


impl Easing {
    /// Applies the easing function to a progress value (0.0 - 1.0)
    #[must_use]
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => (1.0 - t).mul_add(-(1.0 - t), 1.0),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0f64).mul_add(t, 2.0).powi(2) / 2.0
                }
            }
            Self::CubicIn => t.powi(3),
            Self::CubicOut => 1.0 - (1.0 - t).powi(3),
            Self::CubicInOut => {
                if t < 0.5 {
                    4.0 * t.powi(3)
                } else {
                    1.0 - (-2.0f64).mul_add(t, 2.0).powi(3) / 2.0
                }
            }
            Self::QuadIn => t.powi(2),
            Self::QuadOut => (1.0 - t).mul_add(-(1.0 - t), 1.0),
            Self::QuadInOut => {
                if t < 0.5 {
                    2.0 * t.powi(2)
                } else {
                    1.0 - (-2.0f64).mul_add(t, 2.0).powi(2) / 2.0
                }
            }
            Self::ExpoOut => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - (-10.0 * t).exp2()
                }
            }
            Self::ElasticOut => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    (-10.0 * t).exp2().mul_add(((t - s) * std::f64::consts::TAU / p).sin(), 1.0)
                }
            }
        }
    }
}

/// Theme transition configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeTransition {
    /// Transition duration
    #[serde(default = "default_duration")]
    pub duration: Duration,
    /// Easing function
    #[serde(default)]
    pub easing: Easing,
    /// Whether to animate colors
    #[serde(default = "default_true")]
    pub animate_colors: bool,
    /// Whether to animate spacing
    #[serde(default = "default_true")]
    pub animate_spacing: bool,
}

const fn default_duration() -> Duration {
    Duration::from_millis(300)
}

const fn default_true() -> bool {
    true
}

impl Default for ThemeTransition {
    fn default() -> Self {
        Self {
            duration: default_duration(),
            easing: Easing::default(),
            animate_colors: true,
            animate_spacing: true,
        }
    }
}

impl ThemeTransition {
    /// Creates a new transition with the given duration
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            ..Self::default()
        }
    }

    /// Sets the easing function
    #[must_use]
    pub const fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Creates a fast transition (150ms)
    #[must_use]
    pub fn fast() -> Self {
        Self::new(Duration::from_millis(150))
    }

    /// Creates a slow transition (500ms)
    #[must_use]
    pub fn slow() -> Self {
        Self::new(Duration::from_millis(500))
    }

    /// Creates an instant transition (no animation)
    #[must_use]
    pub fn instant() -> Self {
        Self::new(Duration::ZERO)
    }
}

/// Theme interpolator for smooth transitions between themes
pub struct ThemeInterpolator {
    /// Source theme
    pub from: Theme,
    /// Target theme
    pub to: Theme,
    /// Transition config
    pub transition: ThemeTransition,
    /// Current progress (0.0 - 1.0)
    pub progress: f64,
}

impl ThemeInterpolator {
    /// Creates a new interpolator between two themes
    #[must_use]
    pub const fn new(from: Theme, to: Theme, transition: ThemeTransition) -> Self {
        Self {
            from,
            to,
            transition,
            progress: 0.0,
        }
    }

    /// Sets the progress (0.0 - 1.0)
    #[must_use]
    pub const fn with_progress(mut self, progress: f64) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Gets the interpolated theme at current progress
    #[must_use]
    pub fn interpolate(&self) -> Theme {
        let t = self.transition.easing.apply(self.progress);

        let colors = crate::theme::ColorPalette {
            primary: self.interpolate_color(&self.from.colors.primary, &self.to.colors.primary, t),
            secondary: self.interpolate_color(
                &self.from.colors.secondary,
                &self.to.colors.secondary,
                t,
            ),
            accent: self.interpolate_color(&self.from.colors.accent, &self.to.colors.accent, t),
            background: self.interpolate_color(
                &self.from.colors.background,
                &self.to.colors.background,
                t,
            ),
            surface: self.interpolate_color(&self.from.colors.surface, &self.to.colors.surface, t),
            text: self.interpolate_color(&self.from.colors.text, &self.to.colors.text, t),
            text_muted: self.interpolate_color(
                &self.from.colors.text_muted,
                &self.to.colors.text_muted,
                t,
            ),
            success: self.interpolate_color(&self.from.colors.success, &self.to.colors.success, t),
            warning: self.interpolate_color(&self.from.colors.warning, &self.to.colors.warning, t),
            error: self.interpolate_color(&self.from.colors.error, &self.to.colors.error, t),
            info: self.interpolate_color(&self.from.colors.info, &self.to.colors.info, t),
            border: self.interpolate_color(&self.from.colors.border, &self.to.colors.border, t),
            focus: self.interpolate_color(&self.from.colors.focus, &self.to.colors.focus, t),
        };

        Theme {
            name: format!("{}->{}@{}", self.from.name, self.to.name, self.progress),
            colors,
            ..self.to.clone()
        }
    }

    /// Interpolates between two colors
    fn interpolate_color(&self, from: &Color, to: &Color, t: f64) -> Color {
        if self.transition.animate_colors {
            let from_rgb = self.color_to_rgb(from);
            let to_rgb = self.color_to_rgb(to);

            Color::rgb(
                Self::lerp(from_rgb.r, to_rgb.r, t),
                Self::lerp(from_rgb.g, to_rgb.g, t),
                Self::lerp(from_rgb.b, to_rgb.b, t),
            )
        } else if t < 0.5 {
            *from
        } else {
            *to
        }
    }

    /// Linear interpolation between two values
    fn lerp(a: u8, b: u8, t: f64) -> u8 {
        (f64::from(b) - f64::from(a)).mul_add(t, f64::from(a)).round() as u8
    }

    /// Converts a color to RGB
    fn color_to_rgb(&self, color: &Color) -> Rgb {
        match color {
            Color::Rgb(rgb) => *rgb,
            Color::Named(named) => named_color_to_rgb(named),
            Color::Indexed { index } => indexed_color_to_rgb(*index),
        }
    }
}

/// Converts a named color to RGB
const fn named_color_to_rgb(named: &crate::color::NamedColor) -> Rgb {
    use crate::color::NamedColor;
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

/// Converts an indexed color (256-color) to RGB
fn indexed_color_to_rgb(index: u8) -> Rgb {
    if index < 16 {
        let named = match index {
            0 => crate::color::NamedColor::Black,
            1 => crate::color::NamedColor::Red,
            2 => crate::color::NamedColor::Green,
            3 => crate::color::NamedColor::Yellow,
            4 => crate::color::NamedColor::Blue,
            5 => crate::color::NamedColor::Magenta,
            6 => crate::color::NamedColor::Cyan,
            7 => crate::color::NamedColor::White,
            8 => crate::color::NamedColor::BrightBlack,
            9 => crate::color::NamedColor::BrightRed,
            10 => crate::color::NamedColor::BrightGreen,
            11 => crate::color::NamedColor::BrightYellow,
            12 => crate::color::NamedColor::BrightBlue,
            13 => crate::color::NamedColor::BrightMagenta,
            14 => crate::color::NamedColor::BrightCyan,
            15 => crate::color::NamedColor::BrightWhite,
            _ => unreachable!(),
        };
        named_color_to_rgb(&named)
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

/// Dark/Light mode toggle with smooth transition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ColorMode {
    /// Dark mode
    #[default]
    Dark,
    /// Light mode
    Light,
}


impl ColorMode {
    /// Toggles between dark and light mode
    #[must_use]
    pub const fn toggle(&self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    /// Returns true if dark mode
    #[must_use]
    pub const fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Returns true if light mode
    #[must_use]
    pub const fn is_light(&self) -> bool {
        matches!(self, Self::Light)
    }
}

/// Dark/light mode transition handler
pub struct ModeTransition {
    /// Current mode
    pub current_mode: ColorMode,
    /// Dark theme
    pub dark_theme: Theme,
    /// Light theme
    pub light_theme: Theme,
    /// Transition configuration
    pub transition: ThemeTransition,
    /// Animation state
    pub interpolator: Option<ThemeInterpolator>,
}

impl ModeTransition {
    /// Creates a new mode transition handler
    #[must_use]
    pub fn new(dark_theme: Theme, light_theme: Theme) -> Self {
        Self {
            current_mode: ColorMode::Dark,
            dark_theme,
            light_theme,
            transition: ThemeTransition::default(),
            interpolator: None,
        }
    }

    /// Gets the current theme
    #[must_use]
    pub const fn current_theme(&self) -> &Theme {
        match self.current_mode {
            ColorMode::Dark => &self.dark_theme,
            ColorMode::Light => &self.light_theme,
        }
    }

    /// Toggles the mode with smooth transition
    #[must_use]
    pub fn toggle(&mut self) -> ThemeInterpolator {
        let (from, to) = match self.current_mode {
            ColorMode::Dark => (&self.dark_theme, &self.light_theme),
            ColorMode::Light => (&self.light_theme, &self.dark_theme),
        };

        self.current_mode = self.current_mode.toggle();

        ThemeInterpolator::new(from.clone(), to.clone(), self.transition.clone())
    }

    /// Toggles instantly without transition
    pub const fn toggle_instant(&mut self) {
        self.current_mode = self.current_mode.toggle();
    }

    /// Sets the transition duration
    #[must_use]
    pub const fn with_duration(mut self, duration: Duration) -> Self {
        self.transition.duration = duration;
        self
    }

    /// Sets the easing function
    #[must_use]
    pub const fn with_easing(mut self, easing: Easing) -> Self {
        self.transition.easing = easing;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let easing = Easing::Linear;
        assert!((easing.apply(0.5) - 0.5).abs() < 0.001);
        assert!((easing.apply(0.0) - 0.0).abs() < 0.001);
        assert!((easing.apply(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_easing_ease_in() {
        let easing = Easing::EaseIn;
        assert!(easing.apply(0.5) < 0.5);
    }

    #[test]
    fn test_easing_ease_out() {
        let easing = Easing::EaseOut;
        assert!(easing.apply(0.5) > 0.5);
    }

    #[test]
    fn test_theme_transition_defaults() {
        let transition = ThemeTransition::default();
        assert_eq!(transition.duration, Duration::from_millis(300));
        assert!(transition.animate_colors);
    }

    #[test]
    fn test_theme_transition_presets() {
        let fast = ThemeTransition::fast();
        assert_eq!(fast.duration, Duration::from_millis(150));

        let slow = ThemeTransition::slow();
        assert_eq!(slow.duration, Duration::from_millis(500));

        let instant = ThemeTransition::instant();
        assert_eq!(instant.duration, Duration::ZERO);
    }

    #[test]
    fn test_color_mode_toggle() {
        let mode = ColorMode::Dark;
        assert!(mode.is_dark());
        assert!(!mode.is_light());

        let toggled = mode.toggle();
        assert!(toggled.is_light());
        assert!(!toggled.is_dark());
    }

    #[test]
    fn test_theme_interpolator() {
        let from = Theme::dark();
        let to = Theme::light();
        let interpolator =
            ThemeInterpolator::new(from, to, ThemeTransition::instant()).with_progress(0.5);

        let interpolated = interpolator.interpolate();
        assert!(!interpolated.name.is_empty());
    }

    #[test]
    fn test_mode_transition() {
        let mut mode = ModeTransition::new(Theme::dark(), Theme::light());

        assert!(mode.current_theme().name == "dark");
        mode.toggle_instant();
        assert!(mode.current_theme().name == "light");
    }

    #[test]
    fn test_named_color_to_rgb() {
        let rgb = named_color_to_rgb(&crate::color::NamedColor::Red);
        assert_eq!(rgb.r, 170);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn test_indexed_color_to_rgb() {
        let rgb = indexed_color_to_rgb(16);
        assert_eq!(rgb.r, 0);
    }
}
