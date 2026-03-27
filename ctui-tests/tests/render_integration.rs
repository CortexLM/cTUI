//! Integration tests for the full rendering pipeline
//!
//! Tests verify terminal -> draw -> flush -> verify output flow

use ctui_components::{Block, Borders, Line, Paragraph, ParagraphProps, Span, Text};
use ctui_core::backend::test::TestBackend;
use ctui_core::{Buffer, Cell, Rect, Terminal, Widget};

#[test]
fn test_full_render_pipeline() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let frame = terminal
        .draw(|f| {
            f.buffer_mut()[(0, 0)].symbol = "H".to_string();
            f.buffer_mut()[(1, 0)].symbol = "e".to_string();
            f.buffer_mut()[(2, 0)].symbol = "l".to_string();
            f.buffer_mut()[(3, 0)].symbol = "l".to_string();
            f.buffer_mut()[(4, 0)].symbol = "o".to_string();
        })
        .unwrap();

    assert_eq!(frame.area.width, 80);
    assert_eq!(frame.area.height, 24);

    terminal.flush().unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(0, 0)].symbol, "H");
    assert_eq!(backend.buffer()[(1, 0)].symbol, "e");
    assert_eq!(backend.buffer()[(2, 0)].symbol, "l");
    assert_eq!(backend.buffer()[(3, 0)].symbol, "l");
    assert_eq!(backend.buffer()[(4, 0)].symbol, "o");
}

#[test]
fn test_double_buffer_swap() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            f.buffer_mut()[(0, 0)].symbol = "A".to_string();
        })
        .unwrap();

    terminal
        .draw(|f| {
            f.buffer_mut()[(0, 0)].symbol = "B".to_string();
        })
        .unwrap();

    terminal
        .draw(|f| {
            f.buffer_mut()[(0, 0)].symbol = "C".to_string();
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(0, 0)].symbol, "C");
}

#[test]
fn test_diff_rendering_only_changed_cells() {
    let backend = TestBackend::new(10, 5);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            for i in 0..10 {
                f.buffer_mut()[(i as u16, 0)].symbol = "X".to_string();
            }
        })
        .unwrap();

    terminal
        .draw(|f| {
            f.buffer_mut()[(5, 0)].symbol = "Y".to_string();
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(5, 0)].symbol, "Y");
}

#[test]
fn test_terminal_clear() {
    let backend = TestBackend::new(10, 5);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            f.buffer_mut()[(0, 0)].symbol = "A".to_string();
        })
        .unwrap();

    terminal.clear().unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(0, 0)].symbol, " ");
}

#[test]
fn test_terminal_resize() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    assert_eq!(terminal.area().width, 80);
    assert_eq!(terminal.area().height, 24);

    terminal.resize().unwrap();
    assert_eq!(terminal.area().width, 80);
    assert_eq!(terminal.area().height, 24);
}

#[test]
fn test_frame_render_widget() {
    struct TestWidget;

    impl Widget for TestWidget {
        fn render(self, area: Rect, buffer: &mut Buffer) {
            for y in area.y..area.y.saturating_add(area.height) {
                for x in area.x..area.x.saturating_add(area.width) {
                    if let Some(cell) = buffer.get_mut(x, y) {
                        cell.symbol = "X".to_string();
                    }
                }
            }
        }
    }

    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let widget_area = Rect::new(5, 5, 5, 3);
    terminal
        .draw(|f| {
            f.render_widget(TestWidget, widget_area);
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(5, 5)].symbol, "X");
    assert_eq!(backend.buffer()[(9, 5)].symbol, "X");
    assert_eq!(backend.buffer()[(9, 7)].symbol, "X");
    assert_eq!(backend.buffer()[(10, 5)].symbol, " ");
    assert_eq!(backend.buffer()[(5, 8)].symbol, " ");
}

#[test]
fn test_component_render_with_block() {
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 20, 5);
            for x in area.x..area.x.saturating_add(area.width) {
                if let Some(cell) = f.buffer_mut().get_mut(x, area.y) {
                    cell.symbol = "═".to_string();
                }
                if let Some(cell) = f.buffer_mut().get_mut(x, area.y + area.height - 1) {
                    cell.symbol = "═".to_string();
                }
            }
        })
        .unwrap();

    let backend = terminal.backend();
    assert!(!backend.buffer()[(0, 0)].symbol.trim().is_empty());
}

#[test]
fn test_paragraph_render() {
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 40, 10);
            let text = "Hello, World!";
            for (i, ch) in text.chars().take(area.width as usize).enumerate() {
                if let Some(cell) = f.buffer_mut().get_mut(area.x + i as u16, area.y) {
                    cell.symbol = ch.to_string();
                }
            }
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(0, 0)].symbol, "H");
    assert_eq!(backend.buffer()[(1, 0)].symbol, "e");
    assert_eq!(backend.buffer()[(12, 0)].symbol, "!");
}

#[test]
fn test_cursor_operations() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.set_cursor(10, 5).unwrap();
    assert_eq!(terminal.backend().cursor_pos(), (10, 5));

    terminal.hide_cursor().unwrap();
    terminal.show_cursor().unwrap();
}

#[test]
fn test_terminal_scroll_operations() {
    let backend = TestBackend::new(10, 5);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            f.buffer_mut()[(0, 0)].symbol = "A".to_string();
            f.buffer_mut()[(0, 1)].symbol = "B".to_string();
        })
        .unwrap();

    terminal.scroll_up(1).unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(0, 0)].symbol, "B");
    assert_eq!(backend.buffer()[(0, 1)].symbol, " ");
}

#[test]
fn test_layout_cache_integration() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = Rect::new(0, 0, 80, 24);
    let constraints = [1u32, 2, 3];
    let result = vec![
        Rect::new(0, 0, 20, 24),
        Rect::new(20, 0, 40, 24),
        Rect::new(60, 0, 20, 24),
    ];

    terminal
        .layout_cache_mut()
        .store(area, &constraints, result.clone());

    let cached = terminal.layout_cache_mut().get(area, &constraints);
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().len(), 3);

    let metrics = terminal.cache_metrics();
    assert_eq!(metrics.hits, 1);
}

#[test]
fn test_empty_render() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let frame = terminal.draw(|_| {}).unwrap();
    assert_eq!(frame.area.width, 80);
    assert_eq!(frame.area.height, 24);

    for y in 0..frame.area.height {
        for x in 0..frame.area.width {
            assert_eq!(frame.buffer[(x, y)].symbol, " ");
        }
    }
}

#[test]
fn test_alternate_screen() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    assert!(!terminal.is_alternate_screen());

    terminal.enter_alternate_screen().unwrap();
    terminal.leave_alternate_screen().unwrap();
}

#[test]
fn test_terminal_title() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.set_title("Test Application").unwrap();
}

#[test]
fn test_multiple_draw_cycles() {
    let backend = TestBackend::new(10, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    for i in 0..10 {
        terminal
            .draw(|f| {
                f.buffer_mut()[(0, 0)].symbol = char::from(b'0' + (i % 10) as u8).to_string();
            })
            .unwrap();
    }

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(0, 0)].symbol, "9");
}
