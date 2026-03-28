//! WebAssembly backend support for cTUI.
//!
//! This crate provides WASM bindings for running cTUI applications
//! in the browser using HTML5 Canvas.

#![deny(unsafe_code)]

pub mod backend;

pub use backend::CanvasBackend;

// Re-export useful types from ctui-core for convenience
pub use ctui_core::{Cell, Color, Rect};
