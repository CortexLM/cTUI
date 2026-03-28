//! List component for displaying selectable items
//!
//! This module provides a `List` component that renders a scrollable list of items
//! with keyboard navigation and selection support.

use crate::text::Line;
use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};
use std::collections::HashSet;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    #[default]
    Single,
    Multiple,
}

/// A single item in a list.
///
/// Each item contains text content and an optional style.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListItem {
    /// The text content of the item
    content: Line,
    /// Optional style for this item (overrides list base style)
    style: Option<Style>,
}

impl ListItem {
    /// Creates a new list item with the given content
    pub fn new(content: impl Into<Line>) -> Self {
        Self {
            content: content.into(),
            style: None,
        }
    }

    /// Sets the style for this item
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Returns the content of this item
    pub fn content(&self) -> String {
        self.content.content()
    }

    pub fn content_str(&self) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Owned(self.content.content())
    }

    /// Returns the style of this item
    pub fn style_ref(&self) -> Option<&Style> {
        self.style.as_ref()
    }

    /// Returns the display width of this item
    pub fn width(&self) -> usize {
        self.content.width()
    }
}

impl From<&str> for ListItem {
    fn from(s: &str) -> Self {
        Self::new(Line::from(s))
    }
}

impl From<String> for ListItem {
    fn from(s: String) -> Self {
        Self::new(Line::from(s))
    }
}

impl From<Line> for ListItem {
    fn from(line: Line) -> Self {
        Self::new(line)
    }
}

/// A scrollable list of items with selection support.
///
/// The `List` component renders a collection of items, supporting:
/// - Keyboard navigation (up/down)
/// - Selection highlighting
/// - Scrolling for long lists
/// - Custom item rendering via styles
///
/// # Example
///
/// ```
/// use ctui_components::{List, ListItem};
/// use ctui_core::style::{Style, Color};
///
/// let items = vec![
///     ListItem::new("First"),
///     ListItem::new("Second"),
///     ListItem::new("Third"),
/// ];
///
/// let list = List::new(items)
///     .highlight_style(Style::new().fg(Color::Yellow).bg(Color::Blue));
/// ```
#[derive(Clone, Debug)]
pub struct List {
    items: Vec<ListItem>,
    selected: Option<usize>,
    selected_indices: HashSet<usize>,
    selection_mode: SelectionMode,
    highlight_style: Style,
    multi_select_style: Style,
    scroll_offset: usize,
    style: Style,
}

impl Default for List {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            selected_indices: HashSet::new(),
            selection_mode: SelectionMode::Single,
            highlight_style: Style::default(),
            multi_select_style: Style::default(),
            scroll_offset: 0,
            style: Style::default(),
        }
    }
}

impl List {
    pub fn new(items: impl Into<Vec<ListItem>>) -> Self {
        Self {
            items: items.into(),
            ..Self::default()
        }
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    pub fn multi_select_style(mut self, style: Style) -> Self {
        self.multi_select_style = style;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    pub fn select(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn selected_indices(&self) -> &HashSet<usize> {
        &self.selected_indices
    }

    pub fn get_selected_indices(&self) -> Vec<usize> {
        self.selected_indices.iter().copied().collect()
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selected_indices.contains(&index) || self.selected == Some(index)
    }

    pub fn toggle_select(&mut self, index: usize) {
        if self.selection_mode == SelectionMode::Multiple {
            if self.selected_indices.contains(&index) {
                self.selected_indices.remove(&index);
            } else {
                self.selected_indices.insert(index);
            }
        }
        self.selected = Some(index);
    }

    pub fn select_all(&mut self) {
        if self.selection_mode == SelectionMode::Multiple {
            self.selected_indices = (0..self.items.len()).collect();
        }
    }

    pub fn select_none(&mut self) {
        self.selected_indices.clear();
    }

    pub fn select_range(&mut self, start: usize, end: usize) {
        if self.selection_mode == SelectionMode::Multiple {
            for i in start..=end.min(self.items.len().saturating_sub(1)) {
                self.selected_indices.insert(i);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn items(&self) -> &[ListItem] {
        &self.items
    }

    pub fn select_and_scroll(&mut self, index: Option<usize>, visible_height: usize) {
        self.selected = index;
        if let Some(idx) = index {
            self.scroll_to(idx, visible_height);
        }
    }

    /// Scrolls to make the given index visible
    pub fn scroll_to(&mut self, index: usize, visible_height: usize) {
        if visible_height == 0 {
            return;
        }

        // Ensure the selected item is visible
        if index < self.scroll_offset {
            // Item is above the viewport - scroll up
            self.scroll_offset = index;
        } else if index >= self.scroll_offset + visible_height {
            // Item is below the viewport - scroll down
            self.scroll_offset = index - visible_height + 1;
        }
        // Else: item is already visible, no scroll needed
    }

    /// Selects the next item (down)
    pub fn next(&mut self, visible_height: usize) {
        if self.items.is_empty() {
            return;
        }

        let new_index = match self.selected {
            Some(idx) => {
                if idx + 1 < self.items.len() {
                    idx + 1
                } else {
                    idx // Stay at last item
                }
            }
            None => 0, // Select first item if none selected
        };

        self.select_and_scroll(Some(new_index), visible_height);
    }

    /// Selects the previous item (up)
    pub fn previous(&mut self, visible_height: usize) {
        if self.items.is_empty() {
            return;
        }

        let new_index = match self.selected {
            Some(idx) => {
                if idx > 0 {
                    idx - 1
                } else {
                    0 // Stay at first item
                }
            }
            None => 0, // Select first item if none selected
        };

        self.select_and_scroll(Some(new_index), visible_height);
    }

    /// Scrolls by the given number of items (positive = down, negative = up)
    pub fn scroll_by(&mut self, delta: i16) {
        if delta >= 0 {
            self.scroll_offset = self
                .scroll_offset
                .saturating_add(delta as usize)
                .min(self.items.len().saturating_sub(1));
        } else {
            self.scroll_offset = self.scroll_offset.saturating_sub((-delta) as usize);
        }
    }

    fn render_item(&self, item: &ListItem, y: u16, area: Rect, buf: &mut Buffer, item_idx: usize) {
        let content = item.content();
        let content_width = UnicodeWidthStr::width(content.as_str());

        let is_cursor = self.selected == Some(item_idx);
        let is_multi_selected = self.selected_indices.contains(&item_idx);

        let style = if is_cursor {
            self.highlight_style
        } else if is_multi_selected {
            self.multi_select_style
        } else if let Some(ref s) = item.style {
            Style {
                fg: if s.fg != ctui_core::style::Color::Reset {
                    s.fg
                } else {
                    self.style.fg
                },
                bg: if s.bg != ctui_core::style::Color::Reset {
                    s.bg
                } else {
                    self.style.bg
                },
                modifier: self.style.modifier | s.modifier,
            }
        } else {
            self.style
        };

        let prefix = if self.selection_mode == SelectionMode::Multiple {
            if is_multi_selected {
                "[x] "
            } else {
                "[ ] "
            }
        } else {
            ""
        };

        let display_content = format!("{}{}", prefix, content.as_str());
        let chars: Vec<char> = display_content.chars().collect();
        for (i, ch) in chars.iter().enumerate() {
            let x = area.x + i as u16;
            if x >= area.x + area.width {
                break;
            }

            buf.modify_cell(x, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(style);
            });
        }

        for i in chars.len()..area.width as usize {
            let x = area.x + i as u16;
            buf.modify_cell(x, y, |cell| {
                cell.symbol = " ".to_string();
                cell.set_style(style);
            });
        }
    }
}

/// Props for creating a List
pub struct ListProps {
    pub items: Vec<ListItem>,
    pub highlight_style: Style,
    pub style: Style,
}

impl ListProps {
    /// Creates new list props with the given items
    pub fn new(items: impl Into<Vec<ListItem>>) -> Self {
        Self {
            items: items.into(),
            highlight_style: Style::default(),
            style: Style::default(),
        }
    }

    /// Sets the highlight style
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Sets the base style
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Component for List {
    type Props = ListProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            items: props.items,
            selected: None,
            selected_indices: HashSet::new(),
            selection_mode: SelectionMode::Single,
            highlight_style: props.highlight_style,
            multi_select_style: Style::default(),
            scroll_offset: 0,
            style: props.style,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.items.is_empty() {
            return;
        }

        let visible_count = area.height as usize;
        let end_idx = (self.scroll_offset + visible_count).min(self.items.len());

        for (i, item_idx) in (self.scroll_offset..end_idx).enumerate() {
            let y = area.y + i as u16;
            if y >= area.y + area.height {
                break;
            }

            let item = &self.items[item_idx];
            self.render_item(item, y, area, buf, item_idx);
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::style::Color;
    use ctui_core::Buffer;
    use insta::assert_snapshot;

    fn render_to_string(list: &List, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        list.render(Rect::new(0, 0, width, height), &mut buf);

        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = buf.get(x, y) { output.push_str(&cell.symbol); }
            }
            if y < height - 1 {
                output.push('\n');
            }
        }
        output
    }

    #[test]
    fn snapshot_list_empty() {
        let list = List::new(Vec::<ListItem>::new());
        let result = render_to_string(&list, 15, 5);
        assert_snapshot!("list_empty", result);
    }

    #[test]
    fn snapshot_list_single_item() {
        let list = List::new(vec![ListItem::new("Single Item")]);
        let result = render_to_string(&list, 20, 3);
        assert_snapshot!("list_single_item", result);
    }

    #[test]
    fn snapshot_list_multiple_items() {
        let items = vec![
            ListItem::new("First"),
            ListItem::new("Second"),
            ListItem::new("Third"),
            ListItem::new("Fourth"),
        ];
        let list = List::new(items);
        let result = render_to_string(&list, 15, 4);
        assert_snapshot!("list_multiple_items", result);
    }

    #[test]
    fn snapshot_list_with_selection_first() {
        let items = vec![
            ListItem::new("Item A"),
            ListItem::new("Item B"),
            ListItem::new("Item C"),
        ];
        let list = List::new(items)
            .select(Some(0))
            .highlight_style(Style::new().bg(Color::Blue));
        let result = render_to_string(&list, 15, 3);
        assert_snapshot!("list_selection_first", result);
    }

    #[test]
    fn snapshot_list_with_selection_middle() {
        let items = vec![
            ListItem::new("Item A"),
            ListItem::new("Item B"),
            ListItem::new("Item C"),
        ];
        let list = List::new(items)
            .select(Some(1))
            .highlight_style(Style::new().bg(Color::Blue));
        let result = render_to_string(&list, 15, 3);
        assert_snapshot!("list_selection_middle", result);
    }

    #[test]
    fn snapshot_list_with_selection_last() {
        let items = vec![
            ListItem::new("Item A"),
            ListItem::new("Item B"),
            ListItem::new("Item C"),
        ];
        let list = List::new(items)
            .select(Some(2))
            .highlight_style(Style::new().bg(Color::Blue));
        let result = render_to_string(&list, 15, 3);
        assert_snapshot!("list_selection_last", result);
    }

    #[test]
    fn snapshot_list_scrolled() {
        let items: Vec<ListItem> = (0..10)
            .map(|i| ListItem::new(format!("Item {}", i)))
            .collect();
        let mut list = List::new(items);
        list.scroll_offset = 5;
        let result = render_to_string(&list, 15, 3);
        assert_snapshot!("list_scrolled", result);
    }

    #[test]
    fn snapshot_list_item_truncated() {
        let items = vec![ListItem::new(
            "This is a very long item that should be truncated",
        )];
        let list = List::new(items);
        let result = render_to_string(&list, 10, 1);
        assert_snapshot!("list_item_truncated", result);
    }

    #[test]
    fn test_list_item_new() {
        let item = ListItem::new("Test");
        assert_eq!(item.content(), "Test");
        assert!(item.style.is_none());
        assert_eq!(item.width(), 4);
    }

    #[test]
    fn test_list_item_styled() {
        let item = ListItem::new("Test").style(Style::new().fg(Color::Red));
        assert!(item.style.is_some());
        assert_eq!(item.style_ref().unwrap().fg, Color::Red);
    }

    #[test]
    fn test_list_new() {
        let items = vec![
            ListItem::new("One"),
            ListItem::new("Two"),
            ListItem::new("Three"),
        ];
        let list = List::new(items.clone());

        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        assert!(list.selected().is_none());
        assert_eq!(list.scroll_offset(), 0);
    }

    #[test]
    fn test_list_select() {
        let list = List::new(vec![ListItem::new("Test")]).select(Some(0));
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn test_list_scroll_offset() {
        let items: Vec<ListItem> = (0..20)
            .map(|i| ListItem::new(format!("Item {}", i)))
            .collect();
        let mut list = List::new(items);

        // Select item at index 15 with viewport height of 5
        list.select_and_scroll(Some(15), 5);
        assert_eq!(list.selected(), Some(15));
        // Should scroll so item 15 is visible (at bottom of viewport)
        assert_eq!(list.scroll_offset(), 11);
    }

    #[test]
    fn test_list_navigation_next() {
        let items = vec![
            ListItem::new("One"),
            ListItem::new("Two"),
            ListItem::new("Three"),
        ];
        let mut list = List::new(items);

        // No selection initially
        assert!(list.selected().is_none());

        // Next selects first item
        list.next(3);
        assert_eq!(list.selected(), Some(0));

        // Next again
        list.next(3);
        assert_eq!(list.selected(), Some(1));

        // Next at last item stays there
        list.next(3);
        assert_eq!(list.selected(), Some(2));
        list.next(3);
        assert_eq!(list.selected(), Some(2)); // Still at last
    }

    #[test]
    fn test_list_navigation_previous() {
        let items = vec![
            ListItem::new("One"),
            ListItem::new("Two"),
            ListItem::new("Three"),
        ];
        let mut list = List::new(items);

        // Start at last
        list.selected = Some(2);

        // Previous
        list.previous(3);
        assert_eq!(list.selected(), Some(1));

        // Previous again
        list.previous(3);
        assert_eq!(list.selected(), Some(0));

        // Previous at first stays there
        list.previous(3);
        assert_eq!(list.selected(), Some(0)); // Still at first
    }

    #[test]
    fn test_list_scroll_by() {
        let items: Vec<ListItem> = (0..20)
            .map(|i| ListItem::new(format!("Item {}", i)))
            .collect();
        let mut list = List::new(items);

        // Scroll down by 5
        list.scroll_by(5);
        assert_eq!(list.scroll_offset(), 5);

        // Scroll up by 2
        list.scroll_by(-2);
        assert_eq!(list.scroll_offset(), 3);

        // Can't scroll past 0
        list.scroll_by(-10);
        assert_eq!(list.scroll_offset(), 0);
    }

    #[test]
    fn test_list_scroll_to_visible() {
        let items: Vec<ListItem> = (0..20)
            .map(|i| ListItem::new(format!("Item {}", i)))
            .collect();
        let mut list = List::new(items);

        // Scroll to item 2 - already visible with initial offset 0
        list.scroll_to(2, 5);
        assert_eq!(list.scroll_offset(), 0); // No scroll needed

        // Scroll to item 7 - outside viewport
        list.scroll_to(7, 5);
        assert_eq!(list.scroll_offset(), 3); // 7 - 5 + 1 = 3
    }

    #[test]
    fn test_render_empty_list() {
        let list = List::new(Vec::<ListItem>::new());
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        list.render(Rect::new(0, 0, 10, 5), &mut buf);

        // Should render nothing (no panic)
        assert_eq!(buf.get(0, 0).unwrap().symbol, " ");
    }

    #[test]
    fn test_render_basic() {
        let items = vec![
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3"),
        ];
        let list = List::new(items);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 3));

        list.render(Rect::new(0, 0, 10, 3), &mut buf);

        assert!(buf.get(0, 0).unwrap().symbol.starts_with('I'));
        assert!(buf.get(0, 1).unwrap().symbol.starts_with('I'));
        assert!(buf.get(0, 2).unwrap().symbol.starts_with('I'));
    }

    #[test]
    fn test_render_with_selection() {
        use ctui_core::style::Color;

        let items = vec![ListItem::new("First"), ListItem::new("Second")];
        let list = List::new(items)
            .select(Some(0))
            .highlight_style(Style::new().bg(Color::Blue));

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 2));
        list.render(Rect::new(0, 0, 10, 2), &mut buf);

        // First item should have blue background (selected)
        assert_eq!(buf.get(0, 0).unwrap().bg, Color::Blue);
        // Second item should have default background
        assert_eq!(buf.get(0, 1).unwrap().bg, Color::Reset);
    }

    #[test]
    fn test_render_scrolled() {
        let items: Vec<ListItem> = (0..10)
            .map(|i| ListItem::new(format!("Item {}", i)))
            .collect();
        let mut list = List::new(items);
        list.scroll_offset = 5;

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 3));
        list.render(Rect::new(0, 0, 10, 3), &mut buf);

        // Should show items 5, 6, 7
        assert!(buf.get(0, 0).unwrap().symbol.starts_with('I')); // Item 5
        assert!(buf.get(0, 1).unwrap().symbol.starts_with('I')); // Item 6
        assert!(buf.get(0, 2).unwrap().symbol.starts_with('I')); // Item 7
    }

    #[test]
    fn test_render_selection_in_scrolled_list() {
        use ctui_core::style::Color;

        let items: Vec<ListItem> = (0..10)
            .map(|i| ListItem::new(format!("Item {}", i)))
            .collect();
        let mut list = List::new(items).highlight_style(Style::new().bg(Color::Red));
        list.scroll_offset = 5;
        list.selected = Some(6); // Item at row 1 in viewport

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 3));
        list.render(Rect::new(0, 0, 10, 3), &mut buf);

        // Row 1 (item 6) should be highlighted
        assert_eq!(buf.get(0, 1).unwrap().bg, Color::Red);
        // Other rows not highlighted
        assert_eq!(buf.get(0, 0).unwrap().bg, Color::Reset);
        assert_eq!(buf.get(0, 2).unwrap().bg, Color::Reset);
    }

    #[test]
    fn test_props_creation() {
        let props = ListProps::new(vec![ListItem::new("Test")])
            .highlight_style(Style::new().fg(Color::Yellow))
            .style(Style::new().bg(Color::Black));

        assert_eq!(props.items.len(), 1);
        assert_eq!(props.highlight_style.fg, Color::Yellow);
        assert_eq!(props.style.bg, Color::Black);
    }

    #[test]
    fn test_component_create() {
        let props = ListProps::new(vec![ListItem::new("One"), ListItem::new("Two")]);
        let list = List::create(props);

        assert_eq!(list.len(), 2);
        assert!(list.selected().is_none());
    }

    #[test]
    fn test_list_item_from_str() {
        let item: ListItem = "Hello".into();
        assert_eq!(item.content(), "Hello");
    }

    #[test]
    fn test_list_item_from_string() {
        let item: ListItem = String::from("World").into();
        assert_eq!(item.content(), "World");
    }
}
