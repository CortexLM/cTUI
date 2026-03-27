//! Integration tests for cTUI framework
// Suppress pedantic lints
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
//!
//! This crate contains comprehensive integration tests that verify
//! the interaction between multiple cTUI components.
//!
//! # Test Modules
//!
//! - `render_integration` - Tests the full rendering pipeline
//! - `component_integration` - Tests component lifecycle
//! - `layout_integration` - Tests the layout system
//! - `animation_integration` - Tests the animation system
//! - `theme_integration` - Tests the theming system
//! - `e2e_examples` - End-to-end tests based on example apps

// Empty lib - this crate only contains integration tests
