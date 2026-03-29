//! `WebRenderer` trait for browser rendering loop integration.
//!
//! Provides a trait-based abstraction for rendering cTUI content
//! in the browser with requestAnimationFrame-based animation loops.

use wasm_bindgen::prelude::*;

/// Trait for implementing a browser-based rendering loop.
///
/// Implement this trait to create a renderable component that can
/// be driven by the browser's `requestAnimationFrame` API.
pub trait WebRenderer {
    /// Render the current frame.
    ///
    /// Called on each animation frame. Implementations should:
    /// - Update state if needed
    /// - Render to the backend
    /// - Return Ok(()) on success
    ///
    /// # Errors
    /// Returns a `JsValue` error if rendering fails.
    fn render(&mut self) -> Result<(), JsValue>;

    /// Handle canvas resize events.
    ///
    /// Called when the browser window or canvas is resized.
    /// Implementations should update their layout and internal state.
    ///
    /// # Arguments
    /// * `width` - New width in cells/columns
    /// * `height` - New height in cells/rows
    fn on_resize(&mut self, width: u16, height: u16);
}

/// Helper for managing a requestAnimationFrame-based render loop.
///
/// This struct manages the animation frame callback lifecycle,
/// providing a clean interface for starting and stopping rendering.
pub struct RenderLoop {
    running: bool,
}

impl RenderLoop {
    /// Create a new render loop instance.
    #[must_use]
    pub const fn new() -> Self {
        Self { running: false }
    }

    /// Check if the render loop is currently running.
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.running
    }

    /// Stop the render loop.
    pub const fn stop(&mut self) {
        self.running = false;
    }
}

impl Default for RenderLoop {
    fn default() -> Self {
        Self::new()
    }
}

/// Create an animation frame callback closure.
///
/// Returns a closure suitable for use with requestAnimationFrame.
/// The caller is responsible for managing the closure's lifetime.
///
/// # Arguments
/// * `render_fn` - Function to call on each frame (receives timestamp)
///
/// # Example
/// ```ignore
/// let callback = create_frame_callback(|ts| {
///     // render logic here
/// });
/// window.request_animation_frame(callback.as_ref().unchecked_ref());
/// ```
#[must_use]
pub fn create_frame_callback<F>(mut render_fn: F) -> Closure<dyn FnMut(f64)>
where
    F: FnMut(f64) + 'static,
{
    Closure::wrap(Box::new(move |timestamp: f64| {
        render_fn(timestamp);
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_loop_creation() {
        let loop_instance = RenderLoop::new();
        assert!(!loop_instance.is_running());
    }

    #[test]
    fn test_render_loop_default() {
        let loop_instance = RenderLoop::default();
        assert!(!loop_instance.is_running());
    }

    #[test]
    fn test_render_loop_stop() {
        let mut loop_instance = RenderLoop::new();
        loop_instance.stop();
        assert!(!loop_instance.is_running());
    }

    #[test]
    fn test_stop_idempotent() {
        let mut lp = RenderLoop::new();
        lp.stop();
        lp.stop();
        assert!(!lp.is_running());
    }

    #[test]
    fn test_new_default_eq() {
        let a = RenderLoop::new();
        let b = RenderLoop::default();
        assert_eq!(a.is_running(), b.is_running());
    }

    #[test]
    fn test_is_running_const() {
        const LP: RenderLoop = RenderLoop::new();
        assert!(!LP.is_running());
    }

    #[test]
    fn test_stop_sequence() {
        let mut lp = RenderLoop::new();
        assert!(!lp.is_running());
        lp.stop();
        assert!(!lp.is_running());
    }

    #[test]
    fn test_multiple_cycles() {
        let mut lp = RenderLoop::new();
        for _ in 0..5 {
            lp.stop();
        }
        assert!(!lp.is_running());
    }
}

    #[test]
    fn test_new_is_const() {
        const _: RenderLoop = RenderLoop::new();
    }

    #[test]
    fn test_default_is_not_running() {
        assert!(!RenderLoop::default().is_running());
    }

    #[test]
    fn test_new_is_not_running() {
        assert!(!RenderLoop::new().is_running());
    }

    #[test]
    fn test_stop_on_default() {
        let mut lp = RenderLoop::default();
        lp.stop();
        assert!(!lp.is_running());
    }

    #[test]
    fn test_is_running_returns_false() {
        let lp = RenderLoop::new();
        assert_eq!(lp.is_running(), false);
    }

    #[test]
    fn test_stop_changes_nothing() {
        let mut lp = RenderLoop::new();
        let before = lp.is_running();
        lp.stop();
        assert_eq!(lp.is_running(), before);
    }

    #[test]
    fn test_truth_table() {
        let mut lp = RenderLoop::new();
        assert!(!lp.is_running());
        lp.stop();
        assert!(!lp.is_running());
        lp.stop();
        assert!(!lp.is_running());
    }
