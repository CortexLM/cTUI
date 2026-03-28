//! Radio button widgets for mutually exclusive selection in terminal UIs.
//!
//! This module provides widgets for rendering radio button groups where users
//! can select exactly one option from a list. Unlike checkboxes which allow
//! multiple selections, radio buttons enforce that only one item can be
//! selected at a time.
//!
//! # Widgets
//!
//! - [`RadioItem`]: A single radio option with label and value
//! - [`RadioGroup`]: A group of radio items with single selection
//!
//! # Example
//!
//! \`\`\`rust
//! use ctui_components::{RadioItem, RadioGroup, Widget};
//!
//! let radio = RadioGroup::new()
//!     .items(vec![
//!         RadioItem::new("Red"),
//!         RadioItem::new("Green"),
//!         RadioItem::new("Blue"),
//!     ])
//!     .selected(Some(0))
//!     .vertical(true);
//!
//! // Navigate between options
//! let mut radio = radio;
//! radio.select_next();  // Select "Green"
//! assert_eq!(radio.get_selected_value(), Some("Green"));
//! \`\`\`

use crate::text::Line;
use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};

/// A single radio button item with label and value.
///
/// Radio items are used to populate a [`RadioGroup`]. Each item has a
/// display label and an optional value that can differ from the label.
///
/// # Example
///
/// \`\`\`rust
/// use ctui_components::RadioItem;
///
/// let item = RadioItem::new("United States")
///     .value("US");  // Label shows "United States", value returns "US"
/// \`\`\`
#[derive(Clone, Debug)]
pub struct RadioItem {
    /// The display label for this radio item.
    label: Line,
    /// The value returned when this item is selected.
    value: String,
}

impl RadioItem {
    /// Creates a new radio item with the given label.
    ///
    /// The value defaults to the same string as the label.
    ///
    /// # Arguments
    ///
    /// * \`label\` - The display text for this item.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioItem;
    ///
    /// let item = RadioItem::new("Option 1");
    /// \`\`\`
    pub fn new(label: impl Into<String>) -> Self {
        let label_str = label.into();
        Self {
            value: label_str.clone(),
            label: Line::from(label_str),
        }
    }

    /// Sets a custom value for this radio item.
    ///
    /// The value is returned by [`RadioGroup::get_selected_value`] and can
    /// differ from the display label.
    ///
    /// # Arguments
    ///
    /// * \`value\` - The value to return when this item is selected.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioItem;
    ///
    /// let item = RadioItem::new("Dark Mode")
    ///     .value("dark");
    /// \`\`\`
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Returns the content of this item's label.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioItem;
    ///
    /// let item = RadioItem::new("My Label");
    /// assert_eq!(item.label_content(), "My Label");
    /// \`\`\`
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

/// A radio button group widget with mutual exclusivity.
///
/// Renders a list of radio items where only one can be selected at a time.
/// Supports both vertical and horizontal layouts, with configurable spacing
/// and custom selection characters.
///
/// # Example
///
/// \`\`\`rust
/// use ctui_components::{RadioItem, RadioGroup};
/// use ctui_core::style::{Style, Color};
///
/// let radio = RadioGroup::new()
///     .items(vec![
///         RadioItem::new("Light"),
///         RadioItem::new("Dark"),
///         RadioItem::new("System"),
///     ])
///     .selected(Some(1))
///     .selected_style(Style::new().fg(Color::Cyan))
///     .vertical(true)
///     .spacing(1);
/// \`\`\`
#[derive(Clone, Debug, Default)]
pub struct RadioGroup {
    /// The list of radio items.
    items: Vec<RadioItem>,
    /// Index of the currently selected item (None = no selection).
    selected: Option<usize>,
    /// Style for unselected items.
    style: Style,
    /// Style for the selected item.
    selected_style: Style,
    /// Whether items are stacked vertically (true) or horizontal (false).
    vertical: bool,
    /// Spacing between items (in cells).
    spacing: u16,
    /// Character for selected radio button (default: ◉).
    selected_char: char,
    /// Character for unselected radio button (default: ◯).
    unselected_char: char,
}

impl RadioGroup {
    /// Creates a new empty radio group with default settings.
    ///
    /// Default settings:
    /// - Vertical layout
    /// - Unicode radio characters (◉ / ◯)
    /// - 2-cell spacing
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioGroup;
    ///
    /// let radio = RadioGroup::new();
    /// assert!(radio.is_empty());
    /// \`\`\`
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

    /// Sets all radio items at once.
    ///
    /// # Arguments
    ///
    /// * \`items\` - Vector of [`RadioItem`] to display.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let radio = RadioGroup::new()
    ///     .items(vec![
    ///         RadioItem::new("Option A"),
    ///         RadioItem::new("Option B"),
    ///     ]);
    /// \`\`\`
    pub fn items(mut self, items: Vec<RadioItem>) -> Self {
        self.items = items;
        self
    }

    /// Adds a single radio item to the group.
    ///
    /// # Arguments
    ///
    /// * \`item\` - The [`RadioItem`] to add.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let radio = RadioGroup::new()
    ///     .item(RadioItem::new("First"))
    ///     .item(RadioItem::new("Second"));
    /// \`\`\`
    pub fn item(mut self, item: RadioItem) -> Self {
        self.items.push(item);
        self
    }

    /// Sets the initially selected item by index.
    ///
    /// # Arguments
    ///
    /// * \`index\` - The zero-based index to select, or \`None\` for no selection.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let radio = RadioGroup::new()
    ///     .items(vec![RadioItem::new("A"), RadioItem::new("B")])
    ///     .selected(Some(0));
    /// \`\`\`
    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }

    /// Sets the style for unselected radio items.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The [`Style`] to apply to unselected items.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioGroup;
    /// use ctui_core::style::{Style, Color};
    ///
    /// let radio = RadioGroup::new()
    ///     .style(Style::new().fg(Color::Gray));
    /// \`\`\`
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style for the selected radio item.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The [`Style`] to apply to the selected item.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioGroup;
    /// use ctui_core::style::{Style, Color};
    ///
    /// let radio = RadioGroup::new()
    ///     .selected_style(Style::new().fg(Color::Yellow));
    /// \`\`\`
    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// Sets whether the radio items are laid out vertically.
    ///
    /// When \`true\`, items are stacked vertically (each on its own line).
    /// When \`false\`, items are laid out horizontally.
    ///
    /// # Arguments
    ///
    /// * \`vertical\` - \`true\` for vertical layout, \`false\` for horizontal.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioGroup;
    ///
    /// let vertical_radio = RadioGroup::new().vertical(true);
    /// let horizontal_radio = RadioGroup::new().vertical(false);
    /// \`\`\`
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// Convenience method for horizontal layout.
    ///
    /// Equivalent to \`.vertical(false)\`.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioGroup;
    ///
    /// let horizontal_radio = RadioGroup::new().horizontal();
    /// \`\`\`
    pub fn horizontal(mut self) -> Self {
        self.vertical = false;
        self
    }

    /// Sets the spacing between radio items.
    ///
    /// For vertical layout, this is the number of empty lines between items.
    /// For horizontal layout, this is the number of spaces between items.
    ///
    /// # Arguments
    ///
    /// * \`spacing\` - Number of cells/lines between items.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioGroup;
    ///
    /// let radio = RadioGroup::new()
    ///     .vertical(true)
    ///     .spacing(1);  // 1 empty line between items
    /// \`\`\`
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Selects an item by its index.
    ///
    /// # Arguments
    ///
    /// * \`index\` - The zero-based index to select.
    ///
    /// # Note
    ///
    /// Does nothing if index is out of bounds.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let mut radio = RadioGroup::new()
    ///     .items(vec![RadioItem::new("A"), RadioItem::new("B")]);
    /// radio.select(1);
    /// assert_eq!(radio.get_selected(), Some(1));
    /// \`\`\`
    pub fn select(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected = Some(index);
        }
    }

    /// Selects the next item in the group (wraps around).
    ///
    /// If nothing is selected, selects the first item.
    /// If the last item is selected, wraps to the first.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let mut radio = RadioGroup::new()
    ///     .items(vec![RadioItem::new("A"), RadioItem::new("B"), RadioItem::new("C")]);
    /// radio.select_next();  // Selects A
    /// radio.select_next();  // Selects B
    /// radio.select_next();  // Selects C
    /// radio.select_next();  // Wraps to A
    /// \`\`\`
    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        });
    }

    /// Selects the previous item in the group (wraps around).
    ///
    /// If nothing is selected, selects the first item.
    /// If the first item is selected, wraps to the last.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let mut radio = RadioGroup::new()
    ///     .items(vec![RadioItem::new("A"), RadioItem::new("B")])
    ///     .selected(Some(0));
    /// radio.select_prev();  // Wraps to B
    /// assert_eq!(radio.get_selected(), Some(1));
    /// \`\`\`
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

    /// Returns the index of the selected item, or \`None\` if nothing selected.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let radio = RadioGroup::new()
    ///     .items(vec![RadioItem::new("A")])
    ///     .selected(Some(0));
    /// assert_eq!(radio.get_selected(), Some(0));
    /// \`\`\`
    pub fn get_selected(&self) -> Option<usize> {
        self.selected
    }

    /// Returns a reference to the selected [`RadioItem`], if any.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let radio = RadioGroup::new()
    ///     .items(vec![RadioItem::new("A"), RadioItem::new("B")])
    ///     .selected(Some(1));
    /// let selected = radio.get_selected_item().unwrap();
    /// assert_eq!(selected.label_content(), "B");
    /// \`\`\`
    pub fn get_selected_item(&self) -> Option<&RadioItem> {
        self.selected.and_then(|i| self.items.get(i))
    }

    /// Returns the value of the selected item, if any.
    ///
    /// This returns the value set via [`RadioItem::value`], which may differ
    /// from the display label.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let radio = RadioGroup::new()
    ///     .items(vec![
    ///         RadioItem::new("Option A").value("opt_a"),
    ///         RadioItem::new("Option B").value("opt_b"),
    ///     ])
    ///     .selected(Some(0));
    /// assert_eq!(radio.get_selected_value(), Some("opt_a"));
    /// \`\`\`
    pub fn get_selected_value(&self) -> Option<&str> {
        self.get_selected_item().map(|i| i.value.as_str())
    }

    /// Returns the number of radio items in the group.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::{RadioItem, RadioGroup};
    ///
    /// let radio = RadioGroup::new()
    ///     .items(vec![RadioItem::new("A"), RadioItem::new("B")]);
    /// assert_eq!(radio.len(), 2);
    /// \`\`\`
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns \`true\` if the group has no items.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::RadioGroup;
    ///
    /// let radio = RadioGroup::new();
    /// assert!(radio.is_empty());
    /// \`\`\`
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
