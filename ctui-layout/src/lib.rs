//! cTUI Layout - Flexbox-inspired layout engine for terminal UIs
// Suppress pedantic lints
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
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(dead_code)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::unnecessary_min_or_max)]
//!
//! This crate provides a simplified flexbox layout system designed for
//! terminal UI applications. It supports:
//! - Flex direction (row, column)
//! - Justify content (main axis alignment)
//! - Align items (cross axis alignment)
//! - Gap between children
//! - Grid layout with rows/columns
//! - Absolute positioning with z-index
//! - Layout validation
//!
//! # Example
//!
//! ```
//! use ctui_layout::{Layout, FlexDirection, JustifyContent, AlignItems, Constraint};
//! use ctui_core::Rect;
//!
//! let area = Rect::new(0, 0, 80, 24);
//! let layout = Layout::flex()
//!     .direction(FlexDirection::Row)
//!     .justify_content(JustifyContent::SpaceBetween)
//!     .align_items(AlignItems::Center)
//!     .gap(1);
//!
//! let constraints = vec![Constraint::Length(20), Constraint::Min(10), Constraint::Length(20)];
//! let rects = layout.split(area, &constraints);
//! ```

mod absolute;
mod constraint;
mod flex;
mod grid;
mod validation;

#[cfg(feature = "taffy-layout")]
mod taffy_engine;

pub use absolute::{AbsoluteItem, AbsoluteLayout, StackingContext, ZIndex, DEFAULT_Z_INDEX};
pub use constraint::Constraint;
pub use flex::{
    AlignContent, AlignItems, FlexChild, FlexDirection, FlexLayout, JustifyContent, Layout, Margin,
};
pub use grid::{Grid, GridAlignment, GridPosition, GridTrack};
pub use validation::{
    no_overlapping_rects, rect_fits_in_container, LayoutValidationError, LayoutValidator,
    ValidationResult,
};

#[cfg(feature = "taffy-layout")]
pub use taffy_engine::TaffyLayoutEngine;

pub use ctui_core::Rect;
