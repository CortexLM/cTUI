//! Integration tests for the layout system

use ctui_core::backend::test::TestBackend;
use ctui_core::{Buffer, Cell, Rect, Terminal, Widget};
use ctui_layout::{
    AlignContent, AlignItems, Constraint, FlexChild, FlexDirection, FlexLayout, JustifyContent,
    Layout, Margin,
};

fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
    Rect::new(x, y, width, height)
}

#[test]
fn test_row_layout_basic() {
    let area = rect(0, 0, 80, 24);
    let layout = Layout::row();
    let rects = layout.split(
        area,
        &[
            Constraint::Length(20),
            Constraint::Length(30),
            Constraint::Fill,
        ],
    );

    assert_eq!(rects.len(), 3);
    assert_eq!(rects[0], rect(0, 0, 20, 24));
    assert_eq!(rects[1], rect(20, 0, 30, 24));
    assert!(rects[2].width > 0);
}

#[test]
fn test_column_layout_basic() {
    let area = rect(0, 0, 80, 24);
    let layout = Layout::column();
    let rects = layout.split(
        area,
        &[
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Fill,
        ],
    );

    assert_eq!(rects.len(), 3);
    assert_eq!(rects[0], rect(0, 0, 80, 8));
    assert_eq!(rects[1], rect(0, 8, 80, 8));
}

#[test]
fn test_layout_with_gap() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row().gap(5);
    let rects = layout.split(area, &[Constraint::Length(30), Constraint::Length(30)]);

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0], rect(0, 0, 30, 24));
    assert_eq!(rects[1], rect(35, 0, 30, 24));
}

#[test]
fn test_justify_content_center() {
    let area = rect(0, 0, 80, 24);
    let layout = Layout::row().justify_content(JustifyContent::Center);
    let rects = layout.split(area, &[Constraint::Length(20)]);

    assert_eq!(rects.len(), 1);
    assert_eq!(rects[0].x, 30);
    assert_eq!(rects[0].width, 20);
}

#[test]
fn test_justify_content_end() {
    let area = rect(0, 0, 80, 24);
    let layout = Layout::row().justify_content(JustifyContent::End);
    let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(10)]);

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0].x, 50);
    assert_eq!(rects[1].x, 70);
}

#[test]
fn test_justify_content_space_between() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row().justify_content(JustifyContent::SpaceBetween);
    let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(20)]);

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0].x, 0);
    assert_eq!(rects[1].x, 80);
}

#[test]
fn test_justify_content_space_around() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row().justify_content(JustifyContent::SpaceAround);
    let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(20)]);

    assert_eq!(rects.len(), 2);
    assert!(rects[0].x > 0);
    assert!(rects[1].x > rects[0].x + rects[0].width);
}

#[test]
fn test_align_items_variants() {
    let area = rect(0, 0, 80, 24);

    let layout_start = Layout::row().align_items(AlignItems::Start);
    let rects = layout_start.split(area, &[Constraint::Length(20)]);
    assert_eq!(rects[0].height, 24);

    let layout_center = Layout::row().align_items(AlignItems::Center);
    let rects = layout_center.split(area, &[Constraint::Length(20)]);
    assert_eq!(rects[0].height, 24);

    let layout_end = Layout::row().align_items(AlignItems::End);
    let rects = layout_end.split(area, &[Constraint::Length(20)]);
    assert_eq!(rects[0].height, 24);

    let layout_stretch = Layout::row().align_items(AlignItems::Stretch);
    let rects = layout_stretch.split(area, &[Constraint::Length(20)]);
    assert_eq!(rects[0].height, 24);
}

#[test]
fn test_percentage_constraint() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row();
    let rects = layout.split(
        area,
        &[Constraint::Percentage(25), Constraint::Percentage(75)],
    );

    assert_eq!(rects[0].width, 25);
    assert_eq!(rects[1].width, 75);
}

#[test]
fn test_min_constraint() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row();
    let rects = layout.split(area, &[Constraint::Min(20), Constraint::Min(20)]);

    assert_eq!(rects.len(), 2);
    assert!(rects[0].width >= 20);
    assert!(rects[1].width >= 20);
}

#[test]
fn test_max_constraint() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row();
    let rects = layout.split(area, &[Constraint::Max(30)]);

    assert_eq!(rects.len(), 1);
    assert!(rects[0].width <= 30);
}

#[test]
fn test_ratio_constraint() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row();
    let rects = layout.split(area, &[Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)]);

    assert_eq!(rects.len(), 2);
    assert!(rects[0].width < rects[1].width);
}

#[test]
fn test_fill_constraint() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row();
    let rects = layout.split(area, &[Constraint::Length(30), Constraint::Fill]);

    assert_eq!(rects[0].width, 30);
    assert!(rects[1].width > 0);
}

#[test]
fn test_range_constraint() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row();
    let rects = layout.split(
        area,
        &[Constraint::Range { min: 20, max: 40 }, Constraint::Fill],
    );

    assert!(rects[0].width >= 20);
    assert!(rects[0].width <= 40);
}

#[test]
fn test_portion_constraint() {
    let area = rect(0, 0, 90, 24);
    let layout = Layout::row();
    let rects = layout.split(area, &[Constraint::Portion(1), Constraint::Portion(2)]);

    assert_eq!(rects.len(), 2);
}

#[test]
fn test_mixed_constraints() {
    let area = rect(0, 0, 100, 24);
    let layout = Layout::row();
    let rects = layout.split(
        area,
        &[
            Constraint::Length(30),
            Constraint::Percentage(20),
            Constraint::Min(10),
            Constraint::Fill,
        ],
    );

    assert_eq!(rects.len(), 4);
    assert_eq!(rects[0].width, 30);
    assert_eq!(rects[1].width, 20);
}

#[test]
fn test_offset_origin_preserved() {
    let area = rect(10, 5, 80, 24);
    let layout = Layout::row().gap(2);
    let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(30)]);

    assert_eq!(rects[0].x, 10);
    assert_eq!(rects[0].y, 5);
    assert_eq!(rects[1].x, 32);
    assert_eq!(rects[1].y, 5);
}

#[test]
fn test_flex_child_basic() {
    let child = FlexChild::new(Constraint::Length(20));
    assert_eq!(child.constraint, Constraint::Length(20));
    assert_eq!(child.flex_grow, 0);
    assert_eq!(child.flex_shrink, 1);
}

#[test]
fn test_flex_child_fill() {
    let child = FlexChild::fill();
    assert_eq!(child.constraint, Constraint::Fill);
}

#[test]
fn test_flex_child_fixed() {
    let child = FlexChild::fixed(50);
    assert_eq!(child.constraint, Constraint::Length(50));
}

#[test]
fn test_flex_child_builder() {
    let child = FlexChild::fill().grow(2).shrink(0).order(1).basis(100);

    assert_eq!(child.flex_grow, 2);
    assert_eq!(child.flex_shrink, 0);
    assert_eq!(child.order, 1);
    assert_eq!(child.flex_basis, Some(100));
}

#[test]
fn test_split_with_children_ordering() {
    let area = rect(0, 0, 60, 24);
    let layout = Layout::row();

    let children = vec![
        FlexChild::fixed(20).order(2),
        FlexChild::fixed(20).order(0),
        FlexChild::fixed(20).order(1),
    ];

    let rects = layout.split_with_children(area, &children);
    assert_eq!(rects.len(), 3);
}

#[test]
fn test_margin_uniform() {
    let m = Margin::uniform(10);
    assert_eq!(m.top, 10);
    assert_eq!(m.right, 10);
    assert_eq!(m.bottom, 10);
    assert_eq!(m.left, 10);
}

#[test]
fn test_margin_symmetric() {
    let m = Margin::symmetric(5, 10);
    assert_eq!(m.top, 5);
    assert_eq!(m.bottom, 5);
    assert_eq!(m.left, 10);
    assert_eq!(m.right, 10);
}

#[test]
fn test_margin_horizontal_vertical() {
    let m = Margin::new(1, 2, 3, 4);
    assert_eq!(m.horizontal(), 6);
    assert_eq!(m.vertical(), 4);
}

#[test]
fn test_margin_zero() {
    let m = Margin::zero();
    assert_eq!(m.top, 0);
    assert_eq!(m.right, 0);
    assert_eq!(m.bottom, 0);
    assert_eq!(m.left, 0);
}

#[test]
fn test_flex_layout_wrap() {
    let layout = Layout::row().wrap(true).align_content(AlignContent::Center);
    assert!(layout.flex_wrap);
    assert_eq!(layout.align_content, AlignContent::Center);
}

#[test]
fn test_split_wrapped_basic() {
    let area = rect(0, 0, 20, 24);
    let layout = Layout::row().wrap(true);

    let constraints = vec![
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ];

    let lines = layout.split_wrapped(area, &constraints);
    assert!(!lines.is_empty());
}

#[test]
fn test_column_gap() {
    let area = rect(0, 0, 80, 40);
    let layout = Layout::column().gap(4);
    let rects = layout.split(area, &[Constraint::Length(10), Constraint::Length(10)]);

    assert_eq!(rects[1].y, rects[0].y + rects[0].height + 4);
}

#[test]
fn test_render_with_layout() {
    struct ColoredWidget {
        id: char,
    }

    impl Widget for ColoredWidget {
        fn render(&self, area: Rect, buffer: &mut Buffer) {
            for y in area.y..area.y.saturating_add(area.height) {
                for x in area.x..area.x.saturating_add(area.width) {
                    buffer.modify_cell(x, y, |cell| { cell.symbol = self.id.to_string(); });
                }
            }
        }
    }

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let layout = Layout::row();
    let areas = layout.split(
        Rect::new(0, 0, 80, 24),
        &[
            Constraint::Length(20),
            Constraint::Length(30),
            Constraint::Fill,
        ],
    );

    terminal
        .draw(|f| {
            ColoredWidget { id: 'A' }.render(areas[0], f.buffer_mut());
            ColoredWidget { id: 'B' }.render(areas[1], f.buffer_mut());
            ColoredWidget { id: 'C' }.render(areas[2], f.buffer_mut());
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(0, 0).unwrap().symbol, "A");
    assert_eq!(backend.buffer().get(20, 0).unwrap().symbol, "B");
    assert_eq!(backend.buffer().get(50, 0).unwrap().symbol, "C");
}

#[test]
fn test_layout_with_terminal_cache() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let area = terminal.area();
    let constraints = vec![
        Constraint::Length(20),
        Constraint::Length(30),
        Constraint::Fill,
    ];

    terminal
        .layout_cache_mut()
        .store(area, &constraints, Layout::row().split(area, &constraints));

    let cached = terminal.layout_cache_mut().get(area, &constraints);
    assert!(cached.is_some());
}

#[test]
fn test_empty_constraints() {
    let area = rect(0, 0, 80, 24);
    let layout = Layout::row();
    let rects = layout.split(area, &[]);

    assert!(rects.is_empty());
}

#[test]
fn test_single_child_fills_cross_axis() {
    let area = rect(0, 0, 80, 24);
    let layout = Layout::row();
    let rects = layout.split(area, &[Constraint::Length(20)]);

    assert_eq!(rects[0].height, 24);
}

#[test]
fn test_nested_layouts() {
    let outer_area = rect(0, 0, 80, 24);
    let outer = Layout::column();
    let rows = outer.split(
        outer_area,
        &[
            Constraint::Length(8),
            Constraint::Fill,
            Constraint::Length(8),
        ],
    );

    let inner = Layout::row();
    let cols = inner.split(
        rows[1],
        &[
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ],
    );

    assert_eq!(rows.len(), 3);
    assert_eq!(cols.len(), 3);
}

#[test]
fn test_flex_direction_default() {
    let layout = FlexLayout::new();
    assert_eq!(layout.direction, FlexDirection::Row);
}

#[test]
fn test_justify_content_default() {
    let layout = FlexLayout::new();
    assert_eq!(layout.justify_content, JustifyContent::Start);
}

#[test]
fn test_align_items_default() {
    let layout = FlexLayout::new();
    assert_eq!(layout.align_items, AlignItems::Start);
}
