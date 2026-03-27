use crate::text::Line;
use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};

#[derive(Clone, Debug, Default)]
pub struct Checkbox {
    checked: bool,
    label: Option<Line>,
    style: Style,
    checked_style: Style,
    unchecked_char: char,
    checked_char: char,
}

impl Checkbox {
    pub fn new() -> Self {
        Self {
            checked: false,
            label: None,
            style: Style::default(),
            checked_style: Style::default(),
            unchecked_char: '☐',
            checked_char: '☑',
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn label(mut self, label: impl Into<Line>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn checked_style(mut self, style: Style) -> Self {
        self.checked_style = style;
        self
    }

    pub fn unchecked_char(mut self, ch: char) -> Self {
        self.unchecked_char = ch;
        self
    }

    pub fn checked_char(mut self, ch: char) -> Self {
        self.checked_char = ch;
        self
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl Widget for Checkbox {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let style = if self.checked {
            self.checked_style
        } else {
            self.style
        };
        let checkbox_char = if self.checked {
            self.checked_char
        } else {
            self.unchecked_char
        };

        if let Some(cell) = buf.get_mut(area.x, area.y) {
            cell.symbol = checkbox_char.to_string();
            cell.set_style(style);
        }

        if let Some(ref label) = self.label {
            let label_start = area.x + 2;
            let label_text = label.content();

            for (i, ch) in label_text.chars().enumerate() {
                let x = label_start + i as u16;
                if x >= area.x + area.width {
                    break;
                }
                if let Some(cell) = buf.get_mut(x, area.y) {
                    cell.symbol = ch.to_string();
                    cell.set_style(self.style);
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CheckboxGroup {
    checkboxes: Vec<(String, bool)>,
    style: Style,
    checked_style: Style,
    spacing: u16,
}

impl CheckboxGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(mut self, items: Vec<(&str, bool)>) -> Self {
        self.checkboxes = items.into_iter().map(|(s, b)| (s.to_string(), b)).collect();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn checked_style(mut self, style: Style) -> Self {
        self.checked_style = style;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn toggle(&mut self, index: usize) {
        if let Some((_, checked)) = self.checkboxes.get_mut(index) {
            *checked = !*checked;
        }
    }

    pub fn checked_indices(&self) -> Vec<usize> {
        self.checkboxes
            .iter()
            .enumerate()
            .filter_map(|(i, (_, checked))| if *checked { Some(i) } else { None })
            .collect()
    }

    pub fn checked_items(&self) -> Vec<&str> {
        self.checkboxes
            .iter()
            .filter(|(_, checked)| *checked)
            .map(|(label, _)| label.as_str())
            .collect()
    }
}

impl Widget for CheckboxGroup {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.checkboxes.is_empty() {
            return;
        }

        let mut x = area.x;

        for (label, checked) in &self.checkboxes {
            let style = if *checked {
                self.checked_style
            } else {
                self.style
            };
            let checkbox_char = if *checked { '☑' } else { '☐' };

            if x >= area.x + area.width {
                break;
            }

            if let Some(cell) = buf.get_mut(x, area.y) {
                cell.symbol = checkbox_char.to_string();
                cell.set_style(style);
            }
            x += 1;

            x += 1;

            for ch in label.chars() {
                if x >= area.x + area.width {
                    break;
                }
                if let Some(cell) = buf.get_mut(x, area.y) {
                    cell.symbol = ch.to_string();
                    cell.set_style(self.style);
                }
                x += 1;
            }

            x += self.spacing;
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
    fn test_checkbox_unchecked() {
        let checkbox = Checkbox::new().label("Accept terms");
        let result = checkbox.render_to_string(15, 1);
        assert_snapshot!("checkbox_unchecked", result);
    }

    #[test]
    fn test_checkbox_checked() {
        let checkbox = Checkbox::new()
            .checked(true)
            .label("Accept terms")
            .checked_style(Style::new().fg(Color::Green));
        let result = checkbox.render_to_string(15, 1);
        assert_snapshot!("checkbox_checked", result);
    }

    #[test]
    fn test_checkbox_no_label() {
        let checkbox = Checkbox::new().checked(true);
        let result = checkbox.render_to_string(5, 1);
        assert_snapshot!("checkbox_no_label", result);
    }

    #[test]
    fn test_checkbox_group() {
        let group = CheckboxGroup::new()
            .items(vec![
                ("Option A", true),
                ("Option B", false),
                ("Option C", true),
            ])
            .spacing(2);
        let result = group.render_to_string(30, 1);
        assert_snapshot!("checkbox_group", result);
    }

    #[test]
    fn test_checkbox_toggle() {
        let mut checkbox = Checkbox::new();
        assert!(!checkbox.is_checked());
        checkbox.toggle();
        assert!(checkbox.is_checked());
        checkbox.toggle();
        assert!(!checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_group_checked_indices() {
        let group = CheckboxGroup::new().items(vec![("A", true), ("B", false), ("C", true)]);
        assert_eq!(group.checked_indices(), vec![0, 2]);
        assert_eq!(group.checked_items(), vec!["A", "C"]);
    }
}
