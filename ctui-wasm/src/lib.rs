//! WebAssembly backend support for cTUI.
//!
//! This crate provides WASM bindings for running cTUI applications
//! in the browser using HTML5 Canvas.

#![deny(unsafe_code)]

pub mod backend;
pub mod events;
pub mod renderer;

pub use backend::CanvasBackend;
pub use events::{keyboard_event_to_key, mouse_event_to_mouse, wheel_event_to_scroll};
pub use renderer::{RenderLoop, WebRenderer, create_frame_callback};

// Re-export useful types from ctui-core for convenience
pub use ctui_core::{Cell, Color, Rect};

// Re-export event types for convenience
pub use ctui_core::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
};
