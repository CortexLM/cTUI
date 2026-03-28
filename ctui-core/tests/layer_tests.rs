//! Layer system tests for z-index behavior
//!
//! Tests verify:
//! - Z-index ordering: higher z renders on top of lower z
//! - Default z-index is 0
//! - Negative z-index works as background layer

use ctui_core::backend::test::TestBackend;
use ctui_core::buffer::Buffer;
use ctui_core::cell::Cell;
use ctui_core::geometry::Rect;
use ctui_core::terminal::{Terminal, Widget};

// =============================================================================
// Test Widgets
// =============================================================================

/// Widget that renders its symbol with a specific z-index
struct LayerWidget {
    symbol: String,
    z: i32,
}

impl LayerWidget {
    fn new(symbol: char, z: i32) -> Self {
        Self {
            symbol: symbol.to_string(),
            z,
        }
    }
}

impl Widget for LayerWidget {
    fn render(&self, area: Rect, buffer: &mut Buffer) {
        for y in area.y..area.y.saturating_add(area.height) {
            for x in area.x..area.x.saturating_add(area.width) {
                buffer.set(x, y, Cell::new(&self.symbol));
            }
        }
    }

    fn z_index(&self) -> i32 {
        self.z
    }
}

/// Widget with default z-index (0)
struct DefaultZIndexWidget {
    symbol: String,
}

impl DefaultZIndexWidget {
    fn new(symbol: char) -> Self {
        Self {
            symbol: symbol.to_string(),
        }
    }
}

impl Widget for DefaultZIndexWidget {
    fn render(&self, area: Rect, buffer: &mut Buffer) {
        for y in area.y..area.y.saturating_add(area.height) {
            for x in area.x..area.x.saturating_add(area.width) {
                buffer.set(x, y, Cell::new(&self.symbol));
            }
        }
    }
    // Uses default z_index() which returns 0
}

// =============================================================================
// Test 1: Z-Index Ordering - Higher z renders on top
// =============================================================================

#[test]
fn test_z_index_ordering() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            // Render widgets in arbitrary order - z-index should dictate final output
            f.render_widget(LayerWidget::new('A', 1), area);  // z=1 (bottom)
            f.render_widget(LayerWidget::new('B', 2), area);  // z=2 (middle)
            f.render_widget(LayerWidget::new('C', 3), area);  // z=3 (top)
            f.flush(); // Manually flush before frame drop
        })
        .unwrap();

    // Highest z-index (C) should be visible
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "C");
    assert_eq!(result.buffer.get(5, 2).unwrap().symbol, "C");
}

#[test]
fn test_z_index_ordering_reversed_render_order() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            // Render in reverse order - highest z first
            f.render_widget(LayerWidget::new('C', 3), area);  // z=3 (top)
            f.render_widget(LayerWidget::new('A', 1), area);  // z=1 (bottom)
            f.render_widget(LayerWidget::new('B', 2), area);  // z=2 (middle)
            f.flush();
        })
        .unwrap();

    // Highest z-index (C) should still be visible regardless of render order
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "C");
}

#[test]
fn test_z_index_large_values() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            f.render_widget(LayerWidget::new('L', 1000), area);
            f.render_widget(LayerWidget::new('H', 10000), area);
            f.flush();
        })
        .unwrap();

    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "H");
}

#[test]
fn test_z_index_with_zero_on_top_of_negative() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            f.render_widget(LayerWidget::new('N', -1), area);  // Negative (bottom)
            f.render_widget(LayerWidget::new('Z', 0), area);   // Zero (top)
            f.flush();
        })
        .unwrap();

    // z=0 should be on top of z=-1
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "Z");
}

#[test]
fn test_z_index_positive_on_top_of_zero() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            f.render_widget(LayerWidget::new('Z', 0), area);
            f.render_widget(LayerWidget::new('P', 1), area);
            f.flush();
        })
        .unwrap();

    // z=1 should be on top of z=0
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "P");
}

// =============================================================================
// Test 2: Default z-index is 0
// =============================================================================

#[test]
fn test_z_index_default_is_zero() {
    let widget = DefaultZIndexWidget::new('X');
    assert_eq!(widget.z_index(), 0);
}

#[test]
fn test_default_z_index_vs_explicit_zero() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            // Default z-index (0) vs explicit z=0 - should be equal priority
            f.render_widget(DefaultZIndexWidget::new('D'), area);
            f.render_widget(LayerWidget::new('E', 0), area);
            f.flush();
        })
        .unwrap();

    // Either could win since same z-index - just verify one rendered
    let symbol = &result.buffer.get(0, 0).unwrap().symbol;
    assert!(symbol == "D" || symbol == "E", "Expected D or E, got {}", symbol);
}

#[test]
fn test_default_z_index_on_top_of_negative() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            // Negative z should render behind default (z=0)
            f.render_widget(LayerWidget::new('N', -1), area);
            f.render_widget(DefaultZIndexWidget::new('D'), area);
            f.flush();
        })
        .unwrap();

    // Default z=0 should be on top
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "D");
}

#[test]
fn test_default_z_index_behind_positive() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            f.render_widget(DefaultZIndexWidget::new('D'), area);
            f.render_widget(LayerWidget::new('P', 5), area);
            f.flush();
        })
        .unwrap();

    // Positive z should be on top of default z=0
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "P");
}

// =============================================================================
// Test 3: Negative z-index works as background
// =============================================================================

#[test]
fn test_negative_z_index_background() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            // Negative z should render first (background)
            f.render_widget(LayerWidget::new('F', 0), area);
            f.render_widget(LayerWidget::new('B', -5), area);  // Background
            f.flush();
        })
        .unwrap();

    // z=0 should be on top of z=-5
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "F");
}

#[test]
fn test_negative_z_index_stack() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            // Multiple negative layers
            f.render_widget(LayerWidget::new('T', -1), area);
            f.render_widget(LayerWidget::new('M', -5), area);
            f.render_widget(LayerWidget::new('B', -10), area);
            f.render_widget(LayerWidget::new('A', 0), area);
            f.flush();
        })
        .unwrap();

    // z=0 should be on top
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "A");
}

#[test]
fn test_negative_z_index_is_behind_positive() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            f.render_widget(LayerWidget::new('P', 10), area);  // Positive
            f.render_widget(LayerWidget::new('Z', 0), area);    // Zero
            f.render_widget(LayerWidget::new('N', -10), area); // Negative
            f.flush();
        })
        .unwrap();

    // Positive z should be on top
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "P");
}

#[test]
fn test_mixed_z_indices_ordering() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            // Mix of negative, zero, and positive z-indices
            f.render_widget(LayerWidget::new('A', -100), area);
            f.render_widget(LayerWidget::new('B', -1), area);
            f.render_widget(LayerWidget::new('C', 0), area);
            f.render_widget(LayerWidget::new('D', 1), area);
            f.render_widget(LayerWidget::new('E', 50), area);
            f.render_widget(LayerWidget::new('F', 100), area);
            f.flush();
        })
        .unwrap();

    // Highest z (F=100) should be visible
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "F");
}

#[test]
fn test_very_negative_z_index() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            f.render_widget(LayerWidget::new('D', -10000), area);
            f.render_widget(LayerWidget::new('T', -1), area);
            f.flush();
        })
        .unwrap();

    // z=-1 should be above z=-10000
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "T");
}

// =============================================================================
// Integration: Verify many layers order correctly
// =============================================================================

#[test]
fn test_layer_sort_order_correct() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 10, 5);
    let result = terminal
        .draw(|f| {
            // Render 5 layers in random order
            f.render_widget(LayerWidget::new('3', 3), area);
            f.render_widget(LayerWidget::new('1', 1), area);
            f.render_widget(LayerWidget::new('5', 5), area);
            f.render_widget(LayerWidget::new('2', 2), area);
            f.render_widget(LayerWidget::new('4', 4), area);
            f.flush();
        })
        .unwrap();

    // z=5 should be on top
    assert_eq!(result.buffer.get(0, 0).unwrap().symbol, "5");
}
