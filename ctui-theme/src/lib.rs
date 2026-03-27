//! Theming system for cTUI
//!
//! This crate provides a comprehensive theming system including:
//! - Color types (Named, Indexed, RGB)
//! - Style properties (fg, bg, modifiers, padding, margin)
//! - Theme tokens (colors, spacing, typography, borders)
//! - Component themes (Block, Paragraph, List, Button, etc.)
//! - Elevation and shadow system
//! - Theme transitions and animations
//! - Built-in themes (Tokyo Night, Dracula, Catppuccin, etc.)
//! - Theme validation and accessibility checking

// Suppress pedantic lints that don't add value in this codebase
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::use_self)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::type_complexity)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::double_must_use)]
#![allow(clippy::float_cmp)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::unused_self)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(dead_code)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::unnecessary_min_or_max)]

pub mod color;
pub mod component;
pub mod elevation;
pub mod loader;
pub mod style;
pub mod theme;
pub mod transition;
pub mod validation;

pub use color::{Color, NamedColor, Rgb};
pub use component::{
    AllComponentThemes, BlockTheme, ButtonSizes, ButtonTheme, ComponentState, ComponentTheme,
    InputTheme, ListTheme, ModalTheme, ParagraphTheme, ProgressTheme, ScrollbarTheme, TableTheme,
    TabsTheme, TextAlignment,
};
pub use elevation::{z_index, Elevation, ElevationTokens, Shadow};
pub use loader::{ThemeLoadError, ThemeLoader};
pub use style::{BorderChars, BorderStyle, Modifier, Spacing, Style};
pub use theme::{
    BorderTokens, ColorPalette, ComponentStyles, FontRendering, FontWeight, SpacingTokens, Theme,
    Typography,
};
pub use transition::{ColorMode, Easing, ModeTransition, ThemeInterpolator, ThemeTransition};
pub use validation::{
    audit_accessibility, contrast_ratio, relative_luminance, AccessibilityAudit, ColorExt,
    ContrastIssue, ThemeValidator, ValidationError, ValidationResult, ValidationWarning, WcagLevel,
};
