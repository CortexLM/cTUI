//! Tabs component for tabbed navigation.

use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

#[derive(Clone, Debug)]
pub struct Tab {
    label: String,
    style: Style,
}

impl Tab {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

impl From<&str> for Tab {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Tab {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum TabAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug)]
pub struct Tabs {
    tabs: Vec<Tab>,
    selected: usize,
    alignment: TabAlignment,
    style: Style,
    selected_style: Style,
    highlight_style: Style,
    divider: String,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            selected: 0,
            alignment: TabAlignment::default(),
            style: Style::default(),
            selected_style: Style::default(),
            highlight_style: Style::default(),
            divider: "│".to_string(),
        }
    }
}

impl Tabs {
    pub fn new(tabs: impl Into<Vec<Tab>>) -> Self {
        let tabs = tabs.into();
        Self {
            tabs,
            ..Self::default()
        }
    }

    pub fn titles(titles: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let tabs: Vec<Tab> = titles.into_iter().map(|t| Tab::new(t)).collect();
        Self::new(tabs)
    }

    pub fn with_selected(mut self, index: usize) -> Self {
        self.selected = index.min(self.tabs.len().saturating_sub(1));
        self
    }

    pub fn alignment(mut self, alignment: TabAlignment) -> Self {
        self.alignment = alignment;
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

    pub fn divider(mut self, divider: impl Into<String>) -> Self {
        self.divider = divider.into();
        self
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn select(&mut self, index: usize) {
        if !self.tabs.is_empty() {
            self.selected = index.min(self.tabs.len() - 1);
        }
    }

    pub fn select_next(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.tabs.len();
    }

    pub fn select_prev(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        self.selected = if self.selected == 0 {
            self.tabs.len() - 1
        } else {
            self.selected - 1
        };
    }

    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    pub fn select_last(&mut self) {
        if !self.tabs.is_empty() {
            self.selected = self.tabs.len() - 1;
        }
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
    }

    pub fn remove_tab(&mut self, index: usize) -> Option<Tab> {
        if index < self.tabs.len() {
            let removed = self.tabs.remove(index);
            if self.selected >= self.tabs.len() && !self.tabs.is_empty() {
                self.selected = self.tabs.len() - 1;
            }
            Some(removed)
        } else {
            None
        }
    }

    pub fn current_label(&self) -> Option<&str> {
        self.tabs.get(self.selected).map(|t| t.label())
    }

    fn calculate_tabs_width(&self) -> usize {
        let tab_widths: usize = self.tabs.iter().map(|t| t.label.chars().count()).sum();
        let divider_count = self.tabs.len().saturating_sub(1);
        let divider_width = self.divider.chars().count() * divider_count;
        let spacing = self.tabs.len() * 2;
        tab_widths + divider_width + spacing
    }

    fn render_tab(&self, tab: &Tab, x: u16, y: u16, is_selected: bool, buf: &mut Buffer) -> u16 {
        let prefix = " ";
        let suffix = " ";

        for (i, ch) in prefix.chars().enumerate() {
            buf.modify_cell(x + i as u16, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(if is_selected {
                self.selected_style
                } else {
                self.style
                });
            });
        }

        let label_start = x + prefix.len() as u16;
        for (i, ch) in tab.label.chars().enumerate() {
            buf.modify_cell(label_start + i as u16, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(tab.style);
            });
        }

        let suffix_start = label_start + tab.label.chars().count() as u16;
        for (i, ch) in suffix.chars().enumerate() {
            buf.modify_cell(suffix_start + i as u16, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(if is_selected {
                self.selected_style
                } else {
                self.style
                });
            });
        }

        (prefix.len() + tab.label.chars().count() + suffix.len()) as u16
    }

    fn render_divider(&self, x: u16, y: u16, buf: &mut Buffer) -> u16 {
        for (i, ch) in self.divider.chars().enumerate() {
            buf.modify_cell(x + i as u16, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }
        self.divider.chars().count() as u16
    }
}

pub struct TabsProps {
    pub tabs: Vec<Tab>,
    pub selected: usize,
    pub alignment: TabAlignment,
    pub style: Style,
    pub selected_style: Style,
    pub highlight_style: Style,
    pub divider: String,
}

impl TabsProps {
    pub fn new(tabs: Vec<Tab>) -> Self {
        Self {
            tabs,
            selected: 0,
            alignment: TabAlignment::default(),
            style: Style::default(),
            selected_style: Style::default(),
            highlight_style: Style::default(),
            divider: "│".to_string(),
        }
    }

    pub fn titles(titles: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let tabs: Vec<Tab> = titles.into_iter().map(|t| Tab::new(t)).collect();
        Self::new(tabs)
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    pub fn alignment(mut self, alignment: TabAlignment) -> Self {
        self.alignment = alignment;
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

    pub fn divider(mut self, divider: impl Into<String>) -> Self {
        self.divider = divider.into();
        self
    }
}

impl Component for Tabs {
    type Props = TabsProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        let tabs_len = props.tabs.len();
        Self {
            tabs: props.tabs,
            selected: props.selected.min(tabs_len.saturating_sub(1)),
            alignment: props.alignment,
            style: props.style,
            selected_style: props.selected_style,
            highlight_style: props.highlight_style,
            divider: props.divider,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.tabs.is_empty() {
            return;
        }

        let total_width = self.calculate_tabs_width() as u16;
        let start_x = match self.alignment {
            TabAlignment::Left => area.x,
            TabAlignment::Center => area.x + (area.width.saturating_sub(total_width) / 2),
            TabAlignment::Right => area.x + area.width.saturating_sub(total_width),
        };

        let mut current_x = start_x;

        for (i, tab) in self.tabs.iter().enumerate() {
            if i > 0 {
                current_x += self.render_divider(current_x, area.y, buf);
            }

            let is_selected = i == self.selected;

            if i == self.selected && self.highlight_style != Style::default() {
                let tab_width = tab.label.chars().count() as u16 + 2;
                for x in current_x..current_x + tab_width {
                    buf.modify_cell(x, area.y, |cell| {
                        let merged = Style {
                        fg: if self.highlight_style.fg != ctui_core::style::Color::Reset {
                        self.highlight_style.fg
                        } else {
                        cell.fg
                        },
                        bg: if self.highlight_style.bg != ctui_core::style::Color::Reset {
                        self.highlight_style.bg
                        } else {
                        cell.bg
                        },
                        modifier: cell.modifier | self.highlight_style.modifier,
                        };
                        cell.set_style(merged);
                    });
                }
            }

            current_x += self.render_tab(tab, current_x, area.y, is_selected, buf);

            if current_x >= area.x + area.width {
                break;
            }
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

    fn render_to_string(tabs: &Tabs, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        tabs.render(Rect::new(0, 0, width, height), &mut buf);

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
    fn snapshot_tabs_basic() {
        let tabs = Tabs::titles(["File", "Edit", "View", "Help"]);
        let result = render_to_string(&tabs, 40, 1);
        assert_snapshot!("tabs_basic", result);
    }

    #[test]
    fn snapshot_tabs_selected() {
        let tabs = Tabs::titles(["Tab1", "Tab2", "Tab3"]).with_selected(1);
        let result = render_to_string(&tabs, 30, 1);
        assert_snapshot!("tabs_selected", result);
    }

    #[test]
    fn snapshot_tabs_centered() {
        let tabs = Tabs::titles(["A", "B", "C"]).alignment(TabAlignment::Center);
        let result = render_to_string(&tabs, 30, 1);
        assert_snapshot!("tabs_centered", result);
    }

    #[test]
    fn snapshot_tabs_right_aligned() {
        let tabs = Tabs::titles(["One", "Two", "Three"]).alignment(TabAlignment::Right);
        let result = render_to_string(&tabs, 30, 1);
        assert_snapshot!("tabs_right_aligned", result);
    }

    #[test]
    fn snapshot_tabs_custom_divider() {
        let tabs = Tabs::titles(["X", "Y", "Z"]).divider(" | ");
        let result = render_to_string(&tabs, 25, 1);
        assert_snapshot!("tabs_custom_divider", result);
    }

    #[test]
    fn test_tabs_new() {
        let tabs = Tabs::titles(["A", "B", "C"]);
        assert_eq!(tabs.len(), 3);
        assert!(!tabs.is_empty());
        assert_eq!(tabs.selected(), 0);
    }

    #[test]
    fn test_tabs_select() {
        let mut tabs = Tabs::titles(["A", "B", "C"]);
        tabs.select(2);
        assert_eq!(tabs.selected(), 2);

        tabs.select(5);
        assert_eq!(tabs.selected(), 2); // Clamped to last

        tabs.select_first();
        assert_eq!(tabs.selected(), 0);

        tabs.select_last();
        assert_eq!(tabs.selected(), 2);
    }

    #[test]
    fn test_tabs_navigation() {
        let mut tabs = Tabs::titles(["A", "B", "C"]);

        tabs.select_next();
        assert_eq!(tabs.selected(), 1);

        tabs.select_next();
        assert_eq!(tabs.selected(), 2);

        tabs.select_next();
        assert_eq!(tabs.selected(), 0); // Wraps

        tabs.select_prev();
        assert_eq!(tabs.selected(), 2); // Wraps back

        tabs.select_prev();
        assert_eq!(tabs.selected(), 1);
    }

    #[test]
    fn test_tabs_add_remove() {
        let mut tabs = Tabs::titles(["A", "B"]);
        tabs.add_tab(Tab::new("C"));
        assert_eq!(tabs.len(), 3);

        let removed = tabs.remove_tab(1);
        assert_eq!(
            removed.map(|t| t.label().to_string()),
            Some("B".to_string())
        );
        assert_eq!(tabs.len(), 2);
    }

    #[test]
    fn test_tabs_remove_selected() {
        let mut tabs = Tabs::titles(["A", "B", "C"]);
        tabs.select(2);
        tabs.remove_tab(2);
        assert_eq!(tabs.selected(), 1); // Adjusted
    }

    #[test]
    fn test_tabs_current_label() {
        let tabs = Tabs::titles(["Alpha", "Beta", "Gamma"]).with_selected(1);
        assert_eq!(tabs.current_label(), Some("Beta"));
    }

    #[test]
    fn test_tab_from_str() {
        let tab: Tab = "Label".into();
        assert_eq!(tab.label(), "Label");
    }

    #[test]
    fn test_tab_from_string() {
        let tab: Tab = String::from("Label").into();
        assert_eq!(tab.label(), "Label");
    }

    #[test]
    fn test_tabs_props() {
        let props = TabsProps::titles(["A", "B", "C"])
            .selected(2)
            .alignment(TabAlignment::Center);

        let tabs = Tabs::create(props);
        assert_eq!(tabs.selected(), 2);
        assert_eq!(tabs.alignment, TabAlignment::Center);
    }

    #[test]
    fn test_tabs_empty() {
        let tabs = Tabs::new(Vec::<Tab>::new());
        assert!(tabs.is_empty());
        assert_eq!(tabs.len(), 0);
    }

    #[test]
    fn test_render_empty_tabs() {
        let tabs = Tabs::new(Vec::<Tab>::new());
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
        tabs.render(Rect::new(0, 0, 20, 1), &mut buf);
    }

    #[test]
    fn test_render_zero_area() {
        let tabs = Tabs::titles(["A", "B"]);
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
        tabs.render(Rect::new(0, 0, 0, 0), &mut buf);
    }
}
