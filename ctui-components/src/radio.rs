use crate::text::Line;
use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};

#[derive(Clone, Debug)]
pub struct RadioItem {
    label: Line,
    value: String,
}

impl RadioItem {
    pub fn new(label: impl Into<String>) -> Self {
        let label_str = label.into();
        Self {
            value: label_str.clone(),
            label: Line::from(label_str),
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn label_content(&self) -> String {
        self.label.content()
    }
}

impl From<&str> for RadioItem {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for RadioItem {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[derive(Clone, Debug, Default)]
pub struct RadioGroup {
    items: Vec<RadioItem>,
    selected: Option<usize>,
    style: Style,
    selected_style: Style,
    vertical: bool,
    spacing: u16,
    selected_char: char,
    unselected_char: char,
}

impl RadioGroup {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            style: Style::default(),
            selected_style: Style::default(),
            vertical: true,
            spacing: 2,
            selected_char: '◉',
            unselected_char: '◯',
        }
    }

    pub fn items(mut self, items: Vec<RadioItem>) -> Self {
        self.items = items;
        self
    }

    pub fn item(mut self, item: RadioItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
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

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.vertical = false;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn select(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected = Some(index);
        }
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        });
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        });
    }

    pub fn get_selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn get_selected_item(&self) -> Option<&RadioItem> {
        self.selected.and_then(|i| self.items.get(i))
    }

    pub fn get_selected_value(&self) -> Option<&str> {
        self.get_selected_item().map(|i| i.value.as_str())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Widget for RadioGroup {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.items.is_empty() {
            return;
        }

        if self.vertical {
            for (i, item) in self.items.iter().enumerate() {
                let y = area.y + i as u16;
                if y >= area.y + area.height {
                    break;
                }

                let is_selected = self.selected == Some(i);
                let style = if is_selected {
                    self.selected_style
                } else {
                    self.style
                };
                let radio_char = if is_selected {
                    self.selected_char
                } else {
                    self.unselected_char
                };

                buf.modify_cell(area.x, y, |cell| {
                    cell.symbol = radio_char.to_string();
                    cell.set_style(style);
                });

                let label_start = area.x + 2;
                for (j, ch) in item.label_content().chars().enumerate() {
                    let x = label_start + j as u16;
                    if x >= area.x + area.width {
                        break;
                    }
                    buf.modify_cell(x, y, |cell| {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.style);
                    });
                }
            }
        } else {
            let mut x = area.x;

            for (i, item) in self.items.iter().enumerate() {
                if x >= area.x + area.width {
                    break;
                }

                let is_selected = self.selected == Some(i);
                let style = if is_selected {
                    self.selected_style
                } else {
                    self.style
                };
                let radio_char = if is_selected {
                    self.selected_char
                } else {
                    self.unselected_char
                };

                buf.modify_cell(x, area.y, |cell| {
                    cell.symbol = radio_char.to_string();
                    cell.set_style(style);
                });
                x += 2;

                for ch in item.label_content().chars() {
                    if x >= area.x + area.width {
                        break;
                    }
                    buf.modify_cell(x, area.y, |cell| {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.style);
                    });
                    x += 1;
                }

                x += self.spacing;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WidgetExt;
    use ctui_core::style::Color;
    use insta::assert_snapshot;

    #[test]
    fn test_radio_group_vertical() {
        let radio = RadioGroup::new()
            .items(vec![
                RadioItem::new("Option A"),
                RadioItem::new("Option B"),
                RadioItem::new("Option C"),
            ])
            .selected(Some(1))
            .selected_style(Style::new().fg(Color::Yellow));
        let result = radio.render_to_string(15, 5);
        assert_snapshot!("radio_group_vertical", result);
    }

    #[test]
    fn test_radio_group_horizontal() {
        let radio = RadioGroup::new()
            .items(vec![
                RadioItem::new("Red"),
                RadioItem::new("Green"),
                RadioItem::new("Blue"),
            ])
            .selected(Some(0))
            .horizontal()
            .spacing(3);
        let result = radio.render_to_string(25, 1);
        assert_snapshot!("radio_group_horizontal", result);
    }

    #[test]
    fn test_radio_group_none_selected() {
        let radio =
            RadioGroup::new().items(vec![RadioItem::new("Choice 1"), RadioItem::new("Choice 2")]);
        let result = radio.render_to_string(15, 3);
        assert_snapshot!("radio_group_none_selected", result);
    }

    #[test]
    fn test_radio_group_navigation() {
        let mut radio = RadioGroup::new().items(vec![
            RadioItem::new("A"),
            RadioItem::new("B"),
            RadioItem::new("C"),
        ]);

        assert!(radio.get_selected().is_none());
        radio.select_next();
        assert_eq!(radio.get_selected(), Some(0));
        radio.select_next();
        assert_eq!(radio.get_selected(), Some(1));
        radio.select_next();
        assert_eq!(radio.get_selected(), Some(2));
        radio.select_next();
        assert_eq!(radio.get_selected(), Some(0));

        radio.select_prev();
        assert_eq!(radio.get_selected(), Some(2));
        radio.select_prev();
        assert_eq!(radio.get_selected(), Some(1));
    }

    #[test]
    fn test_radio_group_get_value() {
        let radio = RadioGroup::new()
            .items(vec![
                RadioItem::new("First").value("first-value"),
                RadioItem::new("Second").value("second-value"),
            ])
            .selected(Some(1));

        assert_eq!(radio.get_selected_value(), Some("second-value"));
    }
}
