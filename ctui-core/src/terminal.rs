//! Terminal management with double buffering and lifecycle hooks

use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io;
use std::ops::{Deref, DerefMut};

use crate::backend::Backend;
use crate::backend::{CursorConfig, CursorStyle};
use crate::buffer::Buffer;
use crate::cell::Cell;
use crate::geometry::Rect;
use crate::style::Color;

/// Cache hit rate metrics for layout caching
#[derive(Debug, Clone, Default)]
pub struct CacheMetrics {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
}

impl CacheMetrics {
    /// Returns the total number of cache operations
    #[must_use]
    pub const fn total(&self) -> u64 {
        self.hits + self.misses
    }

    /// Returns the cache hit rate as a percentage (0-100)
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }
}

/// A cache for layout computation results
///
/// Stores the last layout result along with the area and constraints
/// that produced it. This allows skipping recomputation when
/// the same layout is requested repeatedly.
#[derive(Debug, Clone, Default)]
pub struct LayoutCache {
    /// The area that was cached
    area: Rect,
    /// Hash of the cached constraints
    constraints_hash: u64,
    /// The cached layout result
    result: Vec<Rect>,
    /// Cache metrics
    metrics: CacheMetrics,
}

impl LayoutCache {
    /// Creates a new empty layout cache
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Hashes a slice of constraints for cache key
    fn hash_constraints<T: Hash>(constraints: &[T]) -> u64 {
        let mut hasher = DefaultHasher::new();
        constraints.hash(&mut hasher);
        hasher.finish()
    }

    /// Attempts to get a cached layout result
    ///
    /// Returns `Some` if the cache contains a valid result for the given
    /// area and constraints. Returns `None` if the cache needs to be
    /// recomputed.
    pub fn get(&mut self, area: Rect, constraints: &[impl Hash]) -> Option<&[Rect]> {
        let hash = Self::hash_constraints(constraints);
        if self.area == area && self.constraints_hash == hash && !self.result.is_empty() {
            self.metrics.hits += 1;
            Some(&self.result)
        } else {
            self.metrics.misses += 1;
            None
        }
    }

    /// Stores a layout result in the cache
    pub fn store(&mut self, area: Rect, constraints: &[impl Hash], result: Vec<Rect>) {
        self.area = area;
        self.constraints_hash = Self::hash_constraints(constraints);
        self.result = result;
    }

    /// Invalidates the cache, clearing the stored result
    pub fn invalidate(&mut self) {
        self.result.clear();
    }

    /// Returns the cache metrics
    #[must_use]
    pub const fn metrics(&self) -> &CacheMetrics {
        &self.metrics
    }

    /// Returns true if the cache has a valid entry
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !self.result.is_empty()
    }
}

/// A widget that can be rendered to a frame
///
/// # Z-Index
///
/// Widgets support z-index layering for render ordering. Higher z-index values
/// are rendered on top of lower values. The default z-index is 0.
pub trait Widget {
    /// Renders the widget to the given buffer within the specified area
    fn render(&self, area: Rect, buffer: &mut Buffer);

    /// Returns the z-index for layer order during rendering.
    ///
    /// Higher values are rendered on top of lower values.
    /// Default is 0 (bottom layer).
    #[inline]
    fn z_index(&self) -> i32 {
        0
    }
}

/// A terminal instance that manages rendering state with double buffering
pub struct Terminal<B: Backend> {
    backend: B,
    buffers: [Buffer; 2],
    current: usize,
    mounted_components: HashSet<TypeId>,
    cleanup_fns: HashMap<TypeId, Vec<Box<dyn FnOnce()>>>,
    /// Layout computation cache for performance optimization
    layout_cache: LayoutCache,
}

impl<B: Backend> Drop for Terminal<B> {
    fn drop(&mut self) {
        self.run_all_cleanups();
    }
}

/// A pending render operation with its z-index for layering
struct PendingRender {
    z_index: i32,
    area: Rect,
    buffer: Buffer,
}

/// A frame represents a single render pass
///
/// # Z-Index Layering
///
/// Widgets rendered via `render_widget` are tracked with their z-index.
/// When the frame is dropped, all pending renders are sorted by z-index
/// (lowest first) and applied to the main buffer. Higher z-index values
/// render on top of lower values.
pub struct Frame<'a> {
    buffer: &'a mut Buffer,
    pending: Vec<PendingRender>,
}

/// Result of a completed render operation
#[derive(Debug)]
pub struct CompletedFrame {
    /// The area that was rendered
    pub area: Rect,
    /// The buffer that was rendered
    pub buffer: Buffer,
}

impl<B: Backend> Terminal<B> {
    /// Creates a new terminal with the given backend
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to get the terminal size.
    pub fn new(backend: B) -> io::Result<Self> {
        let size = backend.size()?;
        let area = Rect::new(0, 0, size.width, size.height);

        Ok(Self {
            backend,
            buffers: [Buffer::empty(area), Buffer::empty(area)],
            current: 0,
            mounted_components: HashSet::new(),
            cleanup_fns: HashMap::new(),
            layout_cache: LayoutCache::new(),
        })
    }

    fn run_all_cleanups(&mut self) {
        let cleanups = std::mem::take(&mut self.cleanup_fns);
        for (_, fns) in cleanups {
            for cleanup in fns {
                cleanup();
            }
        }
    }

    const fn current_buffer(&self) -> &Buffer {
        &self.buffers[self.current]
    }

    const fn prev_buffer(&self) -> &Buffer {
        &self.buffers[1 - self.current]
    }

    const fn prev_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[1 - self.current]
    }

    const fn swap_buffers(&mut self) {
        self.current = 1 - self.current;
    }

    /// Draws the frame using the provided render function
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to draw the content.
    pub fn draw<F>(&mut self, render_fn: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame),
    {
        self.prev_buffer_mut().reset();

        let completed_buffer;
        {
            let mut frame = Frame {
                buffer: self.prev_buffer_mut(),
                pending: Vec::new(),
            };
            render_fn(&mut frame);
            completed_buffer = frame.buffer.clone();
        }

        let current = self.current_buffer().clone();
        let prev = self.prev_buffer().clone();
        self.backend.draw(current.diff(&prev))?;

        self.swap_buffers();

        Ok(CompletedFrame {
            area: completed_buffer.area,
            buffer: completed_buffer,
        })
    }

    /// Flushes pending output to the backend
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to flush.
    pub fn flush(&mut self) -> io::Result<()> {
        self.backend.flush()
    }

    /// Returns a reference to the backend
    #[must_use]
    pub const fn backend(&self) -> &B {
        &self.backend
    }

    /// Returns a mutable reference to the backend
    pub const fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Returns the terminal size
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to get the terminal size.
    pub fn size(&self) -> io::Result<Rect> {
        self.backend.size()
    }

    /// Resizes buffers to match backend size
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to get the terminal size.
    pub fn resize(&mut self) -> io::Result<()> {
        let size = self.backend.size()?;
        let area = Rect::new(0, 0, size.width, size.height);

        self.buffers[0].resize(area);
        self.buffers[1].resize(area);

        // Invalidate layout cache on resize
        self.layout_cache.invalidate();

        Ok(())
    }

    /// Returns a reference to the layout cache
    #[must_use]
    pub const fn layout_cache(&self) -> &LayoutCache {
        &self.layout_cache
    }

    /// Returns a mutable reference to the layout cache
    pub const fn layout_cache_mut(&mut self) -> &mut LayoutCache {
        &mut self.layout_cache
    }

    /// Returns the layout cache metrics
    #[must_use]
    pub const fn cache_metrics(&self) -> &CacheMetrics {
        self.layout_cache.metrics()
    }

    /// Clears the terminal
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to clear.
    pub fn clear(&mut self) -> io::Result<()> {
        self.buffers[0].reset();
        self.buffers[1].reset();
        self.backend.clear()
    }

    /// Clears the entire screen and both buffers
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to clear.
    pub fn clear_screen(&mut self) -> io::Result<()> {
        self.clear()
    }

    /// Clears a specific region of the screen
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to clear the region.
    pub fn clear_region(&mut self, area: Rect) -> io::Result<()> {
        for y in area.y..area.y.saturating_add(area.height) {
            for x in area.x..area.x.saturating_add(area.width) {
                self.buffers[self.current].set(x, y, Cell::default());
            }
        }
        {
            let prev = self.prev_buffer_mut();
            for y in area.y..area.y.saturating_add(area.height) {
                for x in area.x..area.x.saturating_add(area.width) {
                    prev.set(x, y, Cell::default());
                }
            }
        }
        self.backend.clear_region(area)
    }

    /// Hides the cursor
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to hide the cursor.
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.backend.hide_cursor()
    }

    /// Shows the cursor
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to show the cursor.
    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.backend.show_cursor()
    }

    /// Sets the cursor position
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to set the cursor position.
    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.backend.set_cursor(x, y)
    }

    /// Begins synchronized output mode for flicker-free rendering
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to begin synchronized output.
    pub fn begin_synchronized_output(&mut self) -> io::Result<()> {
        self.backend.begin_synchronized_output()
    }

    /// Ends synchronized output mode
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to end synchronized output.
    pub fn end_synchronized_output(&mut self) -> io::Result<()> {
        self.backend.end_synchronized_output()
    }

    /// Returns true if synchronized output is supported
    #[must_use]
    pub fn supports_synchronized_output(&self) -> bool {
        self.backend.supports_synchronized_output()
    }

    /// Scrolls the terminal content up by n lines
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to scroll.
    pub fn scroll_up(&mut self, n: u16) -> io::Result<()> {
        self.backend.scroll_up(n)
    }

    /// Scrolls the terminal content down by n lines
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to scroll.
    pub fn scroll_down(&mut self, n: u16) -> io::Result<()> {
        self.backend.scroll_down(n)
    }

    /// Sets the terminal window title
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to set the title.
    pub fn set_title(&mut self, title: &str) -> io::Result<()> {
        self.backend.set_title(title)
    }

    /// Enters alternate screen mode
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to enter alternate screen.
    pub fn enter_alternate_screen(&mut self) -> io::Result<()> {
        self.backend.enter_alternate_screen()
    }

    /// Leaves alternate screen mode
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to leave alternate screen.
    pub fn leave_alternate_screen(&mut self) -> io::Result<()> {
        self.backend.leave_alternate_screen()
    }

    /// Returns true if currently in alternate screen mode
    #[must_use]
    pub fn is_alternate_screen(&self) -> bool {
        self.backend.is_alternate_screen()
    }

    /// Returns the terminal area
    #[must_use]
    pub const fn area(&self) -> Rect {
        self.current_buffer().area
    }

    /// Sets the cursor style (block, underline, bar) with optional blinking
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to set the cursor style.
    pub fn set_cursor_style(&mut self, style: CursorStyle, blinking: bool) -> io::Result<()> {
        let config = CursorConfig::new(style, blinking);
        self.backend.set_cursor_style(config)
    }

    /// Sets the cursor style using a `CursorConfig`
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to set the cursor style.
    pub fn set_cursor_config(&mut self, config: CursorConfig) -> io::Result<()> {
        self.backend.set_cursor_style(config)
    }

    /// Sets the default background color for the terminal
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to set the background color.
    pub fn set_background_color(&mut self, color: Color) -> io::Result<()> {
        self.backend.set_background_color(color)
    }

    /// Enables mouse event capture
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to enable mouse capture.
    pub fn enable_mouse_capture(&mut self) -> io::Result<()> {
        self.backend.enable_mouse_capture()
    }

    /// Disables mouse event capture
    ///
    /// # Errors
    ///
    /// Returns an error if the backend fails to disable mouse capture.
    pub fn disable_mouse_capture(&mut self) -> io::Result<()> {
        self.backend.disable_mouse_capture()
    }

    // ===== Lifecycle Management =====

    /// Checks if a component type is mounted
    #[must_use]
    pub fn is_mounted<T: 'static>(&self) -> bool {
        self.mounted_components.contains(&TypeId::of::<T>())
    }

    /// Mounts a component, returns true if newly mounted
    pub fn mount_component<T: 'static>(&mut self) -> bool {
        self.mounted_components.insert(TypeId::of::<T>())
    }

    /// Unmounts a component and runs its cleanup functions
    /// Returns true if the component was mounted
    #[must_use]
    pub fn unmount_component<T: 'static>(&mut self) -> bool {
        let type_id = TypeId::of::<T>();
        let removed = self.mounted_components.remove(&type_id);
        if removed {
            if let Some(cleanups) = self.cleanup_fns.remove(&type_id) {
                for cleanup in cleanups {
                    cleanup();
                }
            }
        }
        removed
    }

    /// Registers a cleanup function for a component
    /// Cleanup is run when the component is unmounted or terminal is dropped
    pub fn add_cleanup<T: 'static, F: FnOnce() + 'static>(&mut self, cleanup: F) {
        let type_id = TypeId::of::<T>();
        self.cleanup_fns
            .entry(type_id)
            .or_default()
            .push(Box::new(cleanup));
    }

    /// Runs an effect and registers its optional cleanup function
    /// The effect closure returns `Option<Box<dyn FnOnce()>>` - Some for cleanup, None for no cleanup
    pub fn use_effect<T: 'static, F, C>(&mut self, effect: F)
    where
        F: FnOnce() -> Option<C>,
        C: FnOnce() + 'static,
    {
        if let Some(cleanup) = effect() {
            self.add_cleanup::<T, _>(cleanup);
        }
    }

    /// Runs all cleanup functions for a component without unmounting
    pub fn run_cleanups<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        if let Some(cleanups) = self.cleanup_fns.remove(&type_id) {
            for cleanup in cleanups {
                cleanup();
            }
        }
    }

    /// Clears all mounted components and runs all cleanups
    pub fn clear_lifecycle(&mut self) {
        self.mounted_components.clear();
        self.run_all_cleanups();
    }
}

impl Frame<'_> {
    /// Renders a widget in the given area, respecting z-index layering.
    ///
    /// Widgets are buffered and sorted by z-index before being applied to
    /// the main buffer. Higher z-index values render on top of lower values.
    pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
        let z_index = widget.z_index();
        let mut layer_buffer = Buffer::empty(area);
        widget.render(area, &mut layer_buffer);
        self.pending.push(PendingRender {
            z_index,
            area,
            buffer: layer_buffer,
        });
    }

    /// Immediately applies all pending renders to the buffer.
    ///
    /// Sorts pending renders by z-index (lowest first) and merges them
    /// into the main buffer. After this call, the pending queue is empty.
    pub fn flush(&mut self) {
        self.pending.sort_by_key(|r| r.z_index);
        for render in self.pending.drain(..) {
            // Copy each cell from the layer buffer to the main buffer
            for y in render.area.y..render.area.y.saturating_add(render.area.height) {
                for x in render.area.x..render.area.x.saturating_add(render.area.width) {
                    if let Some(cell) = render.buffer.get(x, y) {
                        self.buffer.set(x, y, cell);
                    }
                }
            }
        }
    }

    /// Returns the frame area
    #[must_use]
    pub const fn area(&self) -> Rect {
        self.buffer.area
    }

    /// Returns a mutable reference to the buffer
    pub const fn buffer_mut(&mut self) -> &mut Buffer {
        self.buffer
    }
}

impl Deref for Frame<'_> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl DerefMut for Frame<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer
    }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        self.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::test::TestBackend;

    fn test_terminal() -> Terminal<TestBackend> {
        let backend = TestBackend::new(80, 24);
        Terminal::new(backend).unwrap()
    }

    #[test]
    fn test_terminal_new() {
        let terminal = test_terminal();
        assert_eq!(terminal.area().width, 80);
        assert_eq!(terminal.area().height, 24);
        assert_eq!(terminal.current_buffer().len(), 80 * 24);
    }

    #[test]
    fn test_terminal_draw() {
        let mut terminal = test_terminal();

        let result = terminal
            .draw(|f| {
                f.buffer_mut().modify_cell(0, 0, |cell| {
                    cell.symbol = "A".to_string();
                });
                f.buffer_mut().modify_cell(1, 0, |cell| {
                    cell.symbol = "B".to_string();
                });
            })
            .unwrap();

        assert_eq!(result.area.width, 80);
        assert_eq!(result.area.height, 24);
        assert_eq!(terminal.current_buffer().get(0, 0).unwrap().symbol, "A");
        assert_eq!(terminal.current_buffer().get(1, 0).unwrap().symbol, "B");
    }

    #[test]
    fn test_terminal_flush() {
        let mut terminal = test_terminal();
        terminal.flush().unwrap();
    }

    #[test]
    fn test_terminal_buffer_swap() {
        let mut terminal = test_terminal();

        terminal
            .draw(|f| {
                f.buffer_mut().modify_cell(0, 0, |cell| {
                    cell.symbol = "X".to_string();
                });
            })
            .unwrap();

        terminal
            .draw(|f| {
                f.buffer_mut().modify_cell(0, 0, |cell| {
                    cell.symbol = "Y".to_string();
                });
            })
            .unwrap();

        assert_eq!(terminal.current_buffer().get(0, 0).unwrap().symbol, "Y");
        assert_eq!(terminal.prev_buffer().get(0, 0).unwrap().symbol, "X");
    }

    #[test]
    fn test_frame_area() {
        let terminal = test_terminal();
        let buffer = terminal.current_buffer().clone();
        let frame = Frame {
            buffer: &mut buffer.clone(),
            pending: Vec::new(),
        };

        assert_eq!(frame.area().width, 80);
        assert_eq!(frame.area().height, 24);
    }

    #[test]
    fn test_frame_render_widget() {
        let terminal = test_terminal();
        let buffer = terminal.current_buffer().clone();
        let mut frame = Frame {
            buffer: &mut buffer.clone(),
            pending: Vec::new(),
        };

        struct TestWidget;
        impl Widget for TestWidget {
            fn render(&self, area: Rect, buffer: &mut Buffer) {
                for y in area.y..area.y + area.height {
                    for x in area.x..area.x + area.width {
                        buffer.modify_cell(x, y, |cell| {
                            cell.symbol = "X".to_string();
                        });
                    }
                }
            }
        }

        let widget_area = Rect::new(5, 5, 10, 5);
        frame.render_widget(TestWidget, widget_area);
        frame.flush();  // Apply pending renders before checking

        assert_eq!(frame.buffer.get(5, 5).unwrap().symbol, "X");
        assert_eq!(frame.buffer.get(14, 9).unwrap().symbol, "X");
        assert_eq!(frame.buffer.get(0, 0).unwrap().symbol, " ");
    }

    #[test]
    fn test_frame_z_index_ordering() {
        let terminal = test_terminal();
        let buffer = terminal.current_buffer().clone();
        let mut frame = Frame {
            buffer: &mut buffer.clone(),
            pending: Vec::new(),
        };

        // Widget with higher z-index (renders on top)
        struct TopWidget;
        impl Widget for TopWidget {
            fn render(&self, area: Rect, buffer: &mut Buffer) {
                for y in area.y..area.y + area.height {
                    for x in area.x..area.x + area.width {
                        buffer.modify_cell(x, y, |cell| {
                            cell.symbol = "T".to_string();
                        });
                    }
                }
            }
            fn z_index(&self) -> i32 {
                10
            }
        }

        // Widget with lower z-index (renders first, gets overwritten)
        struct BottomWidget;
        impl Widget for BottomWidget {
            fn render(&self, area: Rect, buffer: &mut Buffer) {
                for y in area.y..area.y + area.height {
                    for x in area.x..area.x + area.width {
                        buffer.modify_cell(x, y, |cell| {
                            cell.symbol = "B".to_string();
                        });
                    }
                }
            }
            fn z_index(&self) -> i32 {
                1
            }
        }

        let widget_area = Rect::new(0, 0, 5, 5);

        // Render higher z-index first, then lower z-index
        frame.render_widget(TopWidget, widget_area);
        frame.render_widget(BottomWidget, widget_area);
        frame.flush();

        // Higher z-index should win - "T" should be visible
        assert_eq!(frame.buffer.get(0, 0).unwrap().symbol, "T");
        assert_eq!(frame.buffer.get(2, 2).unwrap().symbol, "T");
    }

    #[test]
    fn test_z_index_default_is_zero() {
        // Verify default z_index is 0
        struct DefaultWidget;
        impl Widget for DefaultWidget {
            fn render(&self, _area: Rect, _buffer: &mut Buffer) {}
        }
        let widget = DefaultWidget;
        assert_eq!(widget.z_index(), 0);
    }

    #[test]
    fn test_terminal_backend_access() {
        let terminal = test_terminal();
        assert_eq!(terminal.backend().size().width, 80);

        let mut terminal = terminal;
        assert_eq!(terminal.backend_mut().size().width, 80);
    }

    #[test]
    fn test_double_buffering() {
        let mut terminal = test_terminal();

        terminal
            .draw(|f| {
                f.buffer_mut().modify_cell(0, 0, |cell| {
                    cell.symbol = "A".to_string();
                });
            })
            .unwrap();

        terminal
            .draw(|f| {
                f.buffer_mut().modify_cell(0, 0, |cell| {
                    cell.symbol = "B".to_string();
                });
            })
            .unwrap();

        assert_eq!(terminal.current_buffer().get(0, 0).unwrap().symbol, "B");
        assert_eq!(terminal.prev_buffer().get(0, 0).unwrap().symbol, "A");
    }

    #[test]
    fn test_diff_draws_changes() {
        let mut terminal = test_terminal();

        terminal
            .draw(|f| {
                f.buffer_mut().modify_cell(0, 0, |cell| {
                    cell.symbol = "A".to_string();
                });
            })
            .unwrap();

        let backend = terminal.backend();
        assert!(backend.buffer().get(0, 0).unwrap().symbol == "A");
    }

    #[test]
    fn test_completed_frame() {
        let mut terminal = test_terminal();

        let frame = terminal
            .draw(|f| {
                f.buffer_mut().modify_cell(10, 5, |cell| {
                    cell.symbol = "Z".to_string();
                });
            })
            .unwrap();

        assert_eq!(frame.area.width, 80);
        assert_eq!(frame.area.height, 24);
        assert_eq!(frame.buffer.get(10, 5).unwrap().symbol, "Z");
    }

    #[test]
    fn test_clear() {
        let mut terminal = test_terminal();

        terminal
            .draw(|f| {
                f.buffer_mut().modify_cell(0, 0, |cell| {
                    cell.symbol = "X".to_string();
                });
            })
            .unwrap();

        terminal.clear().unwrap();

        assert_eq!(terminal.current_buffer().get(0, 0).unwrap().symbol, " ");
        assert_eq!(terminal.prev_buffer().get(0, 0).unwrap().symbol, " ");
    }

    // ===== Lifecycle Tests =====

    struct TestComponentA;
    struct TestComponentB;

    #[test]
    fn test_mount_component() {
        let mut terminal = test_terminal();
        assert!(!terminal.is_mounted::<TestComponentA>());

        let newly_mounted = terminal.mount_component::<TestComponentA>();
        assert!(newly_mounted);
        assert!(terminal.is_mounted::<TestComponentA>());

        let already_mounted = terminal.mount_component::<TestComponentA>();
        assert!(!already_mounted);
    }

    #[test]
    fn test_unmount_component() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut terminal = test_terminal();
        terminal.mount_component::<TestComponentA>();

        let cleanup_count = Arc::new(AtomicUsize::new(0));
        let cleanup_count_clone = cleanup_count.clone();
        terminal.add_cleanup::<TestComponentA, _>(move || {
            cleanup_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        let was_mounted = terminal.unmount_component::<TestComponentA>();
        assert!(was_mounted);
        assert!(!terminal.is_mounted::<TestComponentA>());
        assert_eq!(cleanup_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multiply_mounted_components() {
        let mut terminal = test_terminal();

        terminal.mount_component::<TestComponentA>();
        terminal.mount_component::<TestComponentB>();

        assert!(terminal.is_mounted::<TestComponentA>());
        assert!(terminal.is_mounted::<TestComponentB>());

        let _ = terminal.unmount_component::<TestComponentA>();
        assert!(!terminal.is_mounted::<TestComponentA>());
        assert!(terminal.is_mounted::<TestComponentB>());
    }

    #[test]
    fn test_cleanup_runs_on_unmount() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut terminal = test_terminal();
        terminal.mount_component::<TestComponentA>();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        terminal.add_cleanup::<TestComponentA, _>(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        terminal.unmount_component::<TestComponentA>();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multiply_cleanups_same_component() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut terminal = test_terminal();
        terminal.mount_component::<TestComponentA>();

        let counter = Arc::new(AtomicUsize::new(0));

        let c1 = counter.clone();
        terminal.add_cleanup::<TestComponentA, _>(move || {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let c2 = counter.clone();
        terminal.add_cleanup::<TestComponentA, _>(move || {
            c2.fetch_add(10, Ordering::SeqCst);
        });

        terminal.unmount_component::<TestComponentA>();
        assert_eq!(counter.load(Ordering::SeqCst), 11);
    }

    #[test]
    fn test_clear_lifecycle() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut terminal = test_terminal();
        terminal.mount_component::<TestComponentA>();
        terminal.mount_component::<TestComponentB>();

        let counter = Arc::new(AtomicUsize::new(0));
        let c1 = counter.clone();
        terminal.add_cleanup::<TestComponentA, _>(move || {
            c1.fetch_add(1, Ordering::SeqCst);
        });
        let c2 = counter.clone();
        terminal.add_cleanup::<TestComponentB, _>(move || {
            c2.fetch_add(1, Ordering::SeqCst);
        });

        terminal.clear_lifecycle();

        assert!(!terminal.is_mounted::<TestComponentA>());
        assert!(!terminal.is_mounted::<TestComponentB>());
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_run_cleanups_without_unmount() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut terminal = test_terminal();
        terminal.mount_component::<TestComponentA>();

        let counter = Arc::new(AtomicUsize::new(0));
        let c1 = counter.clone();
        terminal.add_cleanup::<TestComponentA, _>(move || {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        terminal.run_cleanups::<TestComponentA>();

        assert!(terminal.is_mounted::<TestComponentA>());
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_use_effect_with_cleanup() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut terminal = test_terminal();
        terminal.mount_component::<TestComponentA>();

        let counter = Arc::new(AtomicUsize::new(0));
        let effect_counter = counter.clone();
        let cleanup_counter = counter.clone();

        terminal.use_effect::<TestComponentA, _, _>(move || {
            effect_counter.fetch_add(100, Ordering::SeqCst);
            Some(move || {
                cleanup_counter.fetch_add(1, Ordering::SeqCst);
            })
        });

        assert_eq!(counter.load(Ordering::SeqCst), 100);

        terminal.unmount_component::<TestComponentA>();
        assert_eq!(counter.load(Ordering::SeqCst), 101);
    }

    #[test]
    fn test_use_effect_without_cleanup() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut terminal = test_terminal();
        terminal.mount_component::<TestComponentA>();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        terminal.use_effect::<TestComponentA, _, fn()>(move || {
            counter_clone.fetch_add(50, Ordering::SeqCst);
            None
        });

        assert_eq!(counter.load(Ordering::SeqCst), 50);

        terminal.unmount_component::<TestComponentA>();
        assert_eq!(counter.load(Ordering::SeqCst), 50);
    }

    // ===== Layout Cache Tests =====

    #[test]
    fn test_layout_cache_new() {
        let cache = LayoutCache::new();
        assert!(!cache.is_valid());
        assert_eq!(cache.metrics().hits, 0);
        assert_eq!(cache.metrics().misses, 0);
    }

    #[test]
    fn test_layout_cache_store_and_get() {
        let mut cache = LayoutCache::new();
        let area = Rect::new(0, 0, 80, 24);
        let constraints = [1u32, 2, 3];
        let result = vec![
            Rect::new(0, 0, 20, 24),
            Rect::new(20, 0, 40, 24),
            Rect::new(60, 0, 20, 24),
        ];

        cache.store(area, &constraints, result.clone());
        assert!(cache.is_valid());

        let cached = cache.get(area, &constraints);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap(), result);
    }

    #[test]
    fn test_layout_cache_miss() {
        let mut cache = LayoutCache::new();
        let area = Rect::new(0, 0, 80, 24);
        let constraints = [1u32, 2, 3];

        let cached = cache.get(area, &constraints);
        assert!(cached.is_none());
        assert_eq!(cache.metrics().misses, 1);
        assert_eq!(cache.metrics().hits, 0);
    }

    #[test]
    fn test_layout_cache_hit() {
        let mut cache = LayoutCache::new();
        let area = Rect::new(0, 0, 80, 24);
        let constraints = [1u32, 2, 3];
        let result = vec![Rect::new(0, 0, 80, 24)];

        cache.store(area, &constraints, result);

        // First call should be a hit
        let _ = cache.get(area, &constraints);
        assert_eq!(cache.metrics().hits, 1);
        assert_eq!(cache.metrics().misses, 0);
    }

    #[test]
    fn test_layout_cache_invalidation() {
        let mut cache = LayoutCache::new();
        let area = Rect::new(0, 0, 80, 24);
        let constraints = [1u32, 2, 3];
        let result = vec![Rect::new(0, 0, 80, 24)];

        cache.store(area, &constraints, result);
        assert!(cache.is_valid());

        cache.invalidate();
        assert!(!cache.is_valid());

        // After invalidation, should miss
        let cached = cache.get(area, &constraints);
        assert!(cached.is_none());
    }

    #[test]
    fn test_layout_cache_different_area_miss() {
        let mut cache = LayoutCache::new();
        let area1 = Rect::new(0, 0, 80, 24);
        let area2 = Rect::new(0, 0, 100, 30);
        let constraints = [1u32, 2, 3];
        let result = vec![Rect::new(0, 0, 80, 24)];

        cache.store(area1, &constraints, result);

        // Different area should miss
        let cached = cache.get(area2, &constraints);
        assert!(cached.is_none());
        assert_eq!(cache.metrics().misses, 1);
    }

    #[test]
    fn test_layout_cache_different_constraints_miss() {
        let mut cache = LayoutCache::new();
        let area = Rect::new(0, 0, 80, 24);
        let constraints1 = [1u32, 2, 3];
        let constraints2 = [4u32, 5, 6];
        let result = vec![Rect::new(0, 0, 80, 24)];

        cache.store(area, &constraints1, result);

        // Different constraints should miss
        let cached = cache.get(area, &constraints2);
        assert!(cached.is_none());
        assert_eq!(cache.metrics().misses, 1);
    }

    #[test]
    fn test_layout_cache_hit_rate() {
        let mut metrics = CacheMetrics::default();
        assert_eq!(metrics.hit_rate(), 0.0);

        metrics.hits = 3;
        metrics.misses = 1;
        assert_eq!(metrics.hit_rate(), 75.0);

        metrics.hits = 0;
        metrics.misses = 5;
        assert_eq!(metrics.hit_rate(), 0.0);
    }

    #[test]
    fn test_terminal_has_layout_cache() {
        let terminal = test_terminal();
        assert!(terminal.layout_cache().metrics().total() == 0);
    }

    #[test]
    fn test_terminal_cache_metrics() {
        let mut terminal = test_terminal();
        let area = Rect::new(0, 0, 80, 24);
        let constraints = [1u32];

        // Access cache mutably and store
        terminal
            .layout_cache_mut()
            .store(area, &constraints, vec![area]);

        // Check metrics through terminal
        let metrics = terminal.cache_metrics();
        assert_eq!(metrics.hits, 0);
        assert_eq!(metrics.misses, 0);
    }

    #[test]
    fn test_terminal_resize_invalidates_cache() {
        use crate::backend::test::TestBackend;

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let area = Rect::new(0, 0, 80, 24);
        let constraints = [1u32];
        terminal
            .layout_cache_mut()
            .store(area, &constraints, vec![area]);
        assert!(terminal.layout_cache().is_valid());

        // Resize should invalidate cache
        terminal.resize().unwrap();
        assert!(!terminal.layout_cache().is_valid());
    }
}
