//! Shadow and elevation system for layered UI elements
//!
//! Provides elevation levels and shadow rendering for visual depth hierarchy.

use crate::color::Color;
use serde::{Deserialize, Serialize};

/// Elevation level (Material Design-inspired scale 0-24)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Elevation {
    /// No elevation (flat)
    #[default]
    Level0,
    /// 1dp elevation - subtle lift
    Level1,
    /// 2dp elevation - cards
    Level2,
    /// 3dp elevation - raised buttons
    Level3,
    /// 4dp elevation - menus
    Level4,
    /// 6dp elevation - snackbars
    Level6,
    /// 8dp elevation - floating action buttons
    Level8,
    /// 12dp elevation - modal dialogs
    Level12,
    /// 16dp elevation - popovers
    Level16,
    /// 24dp elevation - drawers, modals
    Level24,
}


impl Elevation {
    /// Returns the dp (density-independent pixel) value
    #[must_use]
    pub const fn dp(&self) -> u8 {
        match self {
            Self::Level0 => 0,
            Self::Level1 => 1,
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Level4 => 4,
            Self::Level6 => 6,
            Self::Level8 => 8,
            Self::Level12 => 12,
            Self::Level16 => 16,
            Self::Level24 => 24,
        }
    }

    /// Returns the z-index for layering
    #[must_use]
    pub const fn z_index(&self) -> u16 {
        match self {
            Self::Level0 => 0,
            Self::Level1 => 10,
            Self::Level2 => 20,
            Self::Level3 => 30,
            Self::Level4 => 40,
            Self::Level6 => 60,
            Self::Level8 => 80,
            Self::Level12 => 120,
            Self::Level16 => 160,
            Self::Level24 => 240,
        }
    }

    /// Returns the shadow definition for this elevation
    #[must_use]
    pub const fn shadow(&self) -> Shadow {
        match self {
            Self::Level0 => Shadow::none(),
            Self::Level1 => Shadow::new(0, 1, 3, 1, 0.12),
            Self::Level2 => Shadow::new(0, 2, 4, 2, 0.14),
            Self::Level3 => Shadow::new(0, 3, 6, 3, 0.16),
            Self::Level4 => Shadow::new(0, 4, 8, 4, 0.18),
            Self::Level6 => Shadow::new(0, 6, 10, 6, 0.20),
            Self::Level8 => Shadow::new(0, 8, 12, 8, 0.22),
            Self::Level12 => Shadow::new(0, 12, 18, 12, 0.24),
            Self::Level16 => Shadow::new(0, 16, 24, 16, 0.26),
            Self::Level24 => Shadow::new(0, 24, 38, 24, 0.30),
        }
    }
}

/// Box shadow definition
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Shadow {
    /// Horizontal offset (usually 0 for terminal)
    pub x: i16,
    /// Vertical offset (positive = down)
    pub y: i16,
    /// Blur radius (spread)
    pub blur: u16,
    /// Spread radius
    pub spread: u16,
    /// Shadow opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Shadow color (default: black)
    #[serde(default)]
    pub color: Color,
}

impl Default for Shadow {
    fn default() -> Self {
        Self::none()
    }
}

impl Shadow {
    /// Creates a new shadow
    #[must_use]
    pub const fn new(x: i16, y: i16, blur: u16, spread: u16, opacity: f32) -> Self {
        Self {
            x,
            y,
            blur,
            spread,
            opacity,
            color: Color::black(),
        }
    }

    /// No shadow
    #[must_use]
    pub const fn none() -> Self {
        Self::new(0, 0, 0, 0, 0.0)
    }

    /// Small shadow for subtle elevation
    #[must_use]
    pub const fn small() -> Self {
        Self::new(0, 1, 2, 0, 0.1)
    }

    /// Medium shadow for normal elevation
    #[must_use]
    pub const fn medium() -> Self {
        Self::new(0, 2, 4, 0, 0.15)
    }

    /// Large shadow for high elevation
    #[must_use]
    pub const fn large() -> Self {
        Self::new(0, 4, 8, 0, 0.2)
    }

    /// Sets the shadow color
    #[must_use]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Returns the shadow intensity level (0-100) for terminal rendering
    /// This converts the blur and opacity to a printable intensity
    #[must_use]
    pub fn intensity(&self) -> u8 {
        if self.opacity <= 0.0 {
            return 0;
        }

        let base = (self.opacity * 100.0) as u8;
        let blur_factor = (f32::from(self.blur) * 0.02).min(1.0);
        ((f32::from(base) * (1.0 - blur_factor)) as u8).min(100)
    }
}

/// Elevation tokens for a theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElevationTokens {
    /// Default elevation
    #[serde(default)]
    pub default: Elevation,
    /// Elevation for raised elements (buttons)
    #[serde(default = "default_raised")]
    pub raised: Elevation,
    /// Elevation for overlay elements (modals)
    #[serde(default = "default_overlay")]
    pub overlay: Elevation,
    /// Elevation for sticky elements
    #[serde(default = "default_sticky")]
    pub sticky: Elevation,
    /// Default shadow color
    #[serde(default)]
    pub shadow_color: Color,
}

const fn default_raised() -> Elevation {
    Elevation::Level2
}

const fn default_overlay() -> Elevation {
    Elevation::Level24
}

const fn default_sticky() -> Elevation {
    Elevation::Level8
}

impl Default for ElevationTokens {
    fn default() -> Self {
        Self {
            default: Elevation::Level0,
            raised: default_raised(),
            overlay: default_overlay(),
            sticky: default_sticky(),
            shadow_color: Color::black(),
        }
    }
}

impl ElevationTokens {
    /// Gets the shadow for a given elevation
    #[must_use]
    pub const fn shadow_for(&self, elevation: Elevation) -> Shadow {
        elevation.shadow().color(self.shadow_color)
    }

    /// Gets the z-index for a given elevation
    #[must_use]
    pub const fn z_index_for(&self, elevation: Elevation) -> u16 {
        elevation.z_index()
    }
}

/// Z-index layer constants for layout ordering
pub mod z_index {
    /// Base layer (background)
    pub const BASE: u16 = 0;
    /// Content layer
    pub const CONTENT: u16 = 10;
    /// Elevated content (cards)
    pub const ELEVATED: u16 = 50;
    /// Sticky headers
    pub const STICKY: u16 = 100;
    /// Dropdowns and popovers
    pub const DROPDOWN: u16 = 200;
    /// Modals and dialogs
    pub const MODAL: u16 = 500;
    /// Notifications/toasts
    pub const NOTIFICATION: u16 = 700;
    /// Tooltip layer
    pub const TOOLTIP: u16 = 900;
    /// Maximum z-index
    pub const MAX: u16 = 1000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elevation_dp() {
        assert_eq!(Elevation::Level0.dp(), 0);
        assert_eq!(Elevation::Level2.dp(), 2);
        assert_eq!(Elevation::Level24.dp(), 24);
    }

    #[test]
    fn test_elevation_z_index() {
        assert_eq!(Elevation::Level0.z_index(), 0);
        assert_eq!(Elevation::Level24.z_index(), 240);
    }

    #[test]
    fn test_elevation_shadow() {
        let shadow = Elevation::Level2.shadow();
        assert_eq!(shadow.y, 2);
        assert!(shadow.opacity > 0.0);
    }

    #[test]
    fn test_shadow_defaults() {
        let shadow = Shadow::none();
        assert_eq!(shadow.opacity, 0.0);

        let small = Shadow::small();
        assert!(small.opacity > 0.0);
    }

    #[test]
    fn test_shadow_intensity() {
        let none = Shadow::none();
        assert_eq!(none.intensity(), 0);

        let medium = Shadow::medium();
        assert!(medium.intensity() > 0);
    }

    #[test]
    fn test_elevation_tokens_default() {
        let tokens = ElevationTokens::default();
        assert_eq!(tokens.default, Elevation::Level0);
        assert_eq!(tokens.raised, Elevation::Level2);
    }

    #[test]
    fn test_elevation_tokens_shadow_for() {
        let tokens = ElevationTokens::default();
        let shadow = tokens.shadow_for(Elevation::Level4);
        assert_eq!(shadow.blur, 8);
    }

    #[test]
    fn test_z_index_constants() {
        assert!(z_index::BASE < z_index::CONTENT);
        assert!(z_index::CONTENT < z_index::MODAL);
        assert!(z_index::MODAL < z_index::TOOLTIP);
    }

    #[test]
    fn test_elevation_ordering() {
        assert!(Elevation::Level0.z_index() < Elevation::Level1.z_index());
        assert!(Elevation::Level8.z_index() < Elevation::Level24.z_index());
    }
}
