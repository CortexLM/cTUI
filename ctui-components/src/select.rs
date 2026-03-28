use crate::text::Line;
use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Debug)]
pub struct SelectItem {
    content: Line,
    value: String,
}

impl SelectItem {
    pub fn new(content: impl Into<String>) -> Self {
        let content_str = content.into();
        Self {
            value: content_str.clone(),
            content: Line::from(content_str),
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn content_str(&self) -> String {
        self.content.content()
    }
}

impl From<&str> for SelectItem {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for SelectItem {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Select {
    items: Vec<SelectItem>,
    selected: Option<usize>,
    open: bool,
    highlighted: Option<usize>,
    style: Style,
    selected_style: Style,
    highlight_style: Style,
    placeholder: Option<String>,
    max_height: u16,
}

impl Select {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(mut self, items: Vec<SelectItem>) -> Self {
        self.items = items;
        self
    }

    pub fn item(mut self, item: SelectItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = height;
        self
    }

    pub fn highlighted(mut self, idx: usize) -> Self {
        self.highlighted = Some(idx);
        self
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open && self.highlighted.is_none() {
            self.highlighted = Some(0);
        }
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn highlight_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.highlighted = Some(match self.highlighted {
            Some(i) => (i + 1).min(self.items.len() - 1),
            None => 0,
        });
    }

    pub fn highlight_prev(&mut self) {
        self.highlighted = Some(match self.highlighted {
            Some(i) => i.saturating_sub(1),
            None => 0,
        });
    }

    pub fn select_highlighted(&mut self) {
        self.selected = self.highlighted;
        self.open = false;
    }

    pub fn get_selected(&self) -> Option<&SelectItem> {
        self.selected.and_then(|i| self.items.get(i))
    }
}

impl Widget for Select {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let display_text = if let Some(idx) = self.selected {
            self.items
                .get(idx)
                .map(|i| i.content_str())
                .unwrap_or_default()
        } else {
            self.placeholder
                .clone()
                .unwrap_or_else(|| "Select...".to_string())
        };

        let truncated: String = display_text
            .chars()
            .take(area.width.saturating_sub(3) as usize)
            .collect();

        for (i, ch) in truncated.chars().enumerate() {
            buf.modify_cell(area.x + i as u16, area.y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }

        let arrow_x = area.x + area.width.saturating_sub(2);
        buf.modify_cell(arrow_x, area.y, |cell| {
            cell.symbol = if self.open { "▲" } else { "▼" }.to_string();
            cell.set_style(self.style);
        });

        if self.open && !self.items.is_empty() {
            let dropdown_height = self.max_height.min(self.items.len() as u16 + 1).max(1);
            let dropdown_y = area.y + 1;

            for (i, item) in self.items.iter().take(dropdown_height as usize).enumerate() {
                let y = dropdown_y + i as u16;
                if y >= area.y + area.height + dropdown_height {
                    break;
                }

                let is_highlighted = self.highlighted == Some(i);
                let is_selected = self.selected == Some(i);
                let style = if is_highlighted {
                    self.highlight_style
                } else if is_selected {
                    self.selected_style
                } else {
                    self.style
                };

                let item_text = item.content_str();
                for (j, ch) in item_text.chars().take(area.width as usize).enumerate() {
                    buf.modify_cell(area.x + j as u16, y, |cell| {
                        cell.symbol = ch.to_string();
                        cell.set_style(style);
                    });
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ComboBox {
    input: String,
    cursor: usize,
    items: Vec<SelectItem>,
    filtered_items: Vec<usize>,
    selected: Option<usize>,
    open: bool,
    highlighted: Option<usize>,
    style: Style,
}

impl ComboBox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(mut self, items: Vec<SelectItem>) -> Self {
        self.items = items;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn set_input(&mut self, input: impl Into<String>) {
        self.input = input.into();
        self.cursor = self.input.len();
        self.filter_items();
    }

    fn filter_items(&mut self) {
        self.filtered_items.clear();
        let input_lower = self.input.to_lowercase();
        for (i, item) in self.items.iter().enumerate() {
            if item.content_str().to_lowercase().contains(&input_lower) {
                self.filtered_items.push(i);
            }
        }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open {
            self.filter_items();
        }
    }

    pub fn select_highlighted(&mut self) {
        if let Some(filtered_idx) = self.highlighted {
            if let Some(&item_idx) = self.filtered_items.get(filtered_idx) {
                self.selected = Some(item_idx);
                self.input = self.items[item_idx].content_str();
                self.cursor = self.input.len();
                self.open = false;
            }
        }
    }
}

impl Widget for ComboBox {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let display: String = self
            .input
            .chars()
            .take(area.width.saturating_sub(1) as usize)
            .collect();

        for (i, ch) in display.chars().enumerate() {
            buf.modify_cell(area.x + i as u16, area.y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }

        buf.modify_cell(area.x + self.cursor as u16, area.y, |cell| { cell.symbol = "│".to_string(); });

        if self.open && !self.filtered_items.is_empty() {
            let dropdown_y = area.y + 1;
            for (i, &item_idx) in self.filtered_items.iter().enumerate().take(5) {
                let y = dropdown_y + i as u16;
                if let Some(item) = self.items.get(item_idx) {
                    let item_text = item.content_str();
                    for (j, ch) in item_text.chars().take(area.width as usize).enumerate() {
                        buf.modify_cell(area.x + j as u16, y, |cell| {
                            cell.symbol = ch.to_string();
                            cell.set_style(self.style);
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WidgetExt;
    use insta::assert_snapshot;

    #[test]
    fn test_select_closed() {
        let select = Select::new()
            .items(vec![
                SelectItem::new("Option 1"),
                SelectItem::new("Option 2"),
            ])
            .selected(Some(0));
        let result = select.render_to_string(15, 1);
        assert_snapshot!("select_closed", result);
    }

    #[test]
    fn test_select_open() {
        let select = Select::new()
            .items(vec![
                SelectItem::new("Apple"),
                SelectItem::new("Banana"),
                SelectItem::new("Cherry"),
            ])
            .open(true)
            .highlighted(1);
        let result = select.render_to_string(15, 6);
        assert_snapshot!("select_open", result);
    }

    #[test]
    fn test_select_placeholder() {
        let select = Select::new()
            .items(vec![SelectItem::new("Item")])
            .placeholder("Choose...");
        let result = select.render_to_string(15, 1);
        assert_snapshot!("select_placeholder", result);
    }

    #[test]
    fn test_combo_box() {
        let combo = ComboBox::new()
            .items(vec![SelectItem::new("Apple"), SelectItem::new("Banana")])
            .open(true);
        let result = combo.render_to_string(15, 6);
        assert_snapshot!("combo_box", result);
    }
}
