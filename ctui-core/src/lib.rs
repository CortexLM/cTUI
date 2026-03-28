//! cTUI Core - Terminal rendering primitives
//!
//! This crate provides the low-level building blocks for terminal UI rendering:
//! - Buffer and Cell types for screen representation
//! - Backend trait for terminal abstraction
//! - Terminal struct for managing the terminal state
//! - Geometry primitives for layout calculations
//! - Style types for colors and modifiers
//! - Component trait for declarative UI elements
//! - Props system for component configuration
//! - State management with dispatch pattern
//! - Event system for input handling

// Suppress pedantic lints that don't add value in this codebase
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::must_use_candidate)]
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
#![allow(clippy::option_if_let_else)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::type_complexity)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::double_must_use)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::float_cmp)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::unused_self)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(dead_code)]
#![allow(clippy::derive_partial_eq_without_eq)]

pub mod backend;
pub mod buffer;
pub mod cell;
pub mod component;
pub mod event;
pub mod geometry;
pub mod packed_cell;
pub mod props;
pub mod render_loop;
pub mod renderable;
pub mod state;
pub mod style;
pub mod symbol_table;
pub mod terminal;
pub mod unicode;

pub use buffer::Buffer;
pub use cell::Cell;
pub use component::{Cmd, Component, Msg};
pub use event::{Event, EventHandler, FnEventHandler, KeyCode, KeyEvent, KeyModifiers, MouseEvent, ResizeEvent};
pub use geometry::{Position, Rect, Size};
pub use packed_cell::PackedCell;
pub use props::{DefaultProps, Props};
pub use state::State;
pub use style::{Color, Modifier, Style};
pub use symbol_table::{SymbolId, SymbolTable};
pub use terminal::{Frame, Terminal, Widget};
pub use unicode::{display_width, UnicodeCompat};
