//! Checkbox widgets for boolean selection in terminal UIs.
//!
//! This module provides widgets for rendering checkboxes and checkbox groups
//! in terminal applications. Checkboxes allow users to select multiple options
//! independently, unlike radio buttons which enforce mutually exclusive selection.
//!
//! # Widgets
//!
//! - [`Checkbox`]: A single checkbox with an optional label
//! - [`CheckboxGroup`]: A horizontal group of checkboxes rendered inline
//!
//! # Example
//!
//! \`\`\`rust
//! use ctui_components::{Checkbox, CheckboxGroup, Widget};
//! use ctui_core::{Buffer, Rect};
//!
//! // Single checkbox
//! let checkbox = Checkbox::new()
//!     .label("Accept terms and conditions")
//!     .checked(false);
//!
//! // Checkbox group for multiple selections
//! let group = CheckboxGroup::new()
//!     .items(vec![("Option A", true), ("Option B", false)])
//!     .spacing(2);
//! \`\`\`

use crate::text::Line;
use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};

/// A single checkbox widget with an optional label.
///
/// The checkbox can be in a checked or unchecked state, with customizable
/// characters for each state. By default, uses Unicode checkbox characters
/// (☐ for unchecked, ☑ for checked).
///
/// # Example
///
/// \`\`\`rust
/// use ctui_components::Checkbox;
/// use ctui_core::style::{Style, Color};
///
/// let checkbox = Checkbox::new()
///     .label("Enable notifications")
///     .checked(true)
///     .checked_style(Style::new().fg(Color::Green));
/// \`\`\`
#[derive(Clone, Debug, Default)]
pub struct Checkbox {
    /// Whether the checkbox is currently checked.
    checked: bool,
    /// Optional label text displayed next to the checkbox.
    label: Option<Line>,
    /// Style for the unchecked state.
    style: Style,
    /// Style for the checked state.
    checked_style: Style,
    /// Character displayed when unchecked (default: ☐).
    unchecked_char: char,
    /// Character displayed when checked (default: ☑).
    checked_char: char,
}

impl Checkbox {
    /// Creates a new unchecked checkbox with default settings.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    ///
    /// let checkbox = Checkbox::new();
    /// assert!(!checkbox.is_checked());
    /// \`\`\`
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

    /// Sets the checked state of the checkbox.
    ///
    /// # Arguments
    ///
    /// * \`checked\` - Whether the checkbox should be checked.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    ///
    /// let checkbox = Checkbox::new().checked(true);
    /// assert!(checkbox.is_checked());
    /// \`\`\`
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets the label text displayed next to the checkbox.
    ///
    /// # Arguments
    ///
    /// * \`label\` - The label text (can be a string or [`Line`]).
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    ///
    /// let checkbox = Checkbox::new().label("Remember me");
    /// \`\`\`
    pub fn label(mut self, label: impl Into<Line>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the style for the unchecked state.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The style to apply when unchecked.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    /// use ctui_core::style::{Style, Color};
    ///
    /// let checkbox = Checkbox::new()
    ///     .style(Style::new().fg(Color::Gray));
    /// \`\`\`
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style for the checked state.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The style to apply when checked.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    /// use ctui_core::style::{Style, Color};
    ///
    /// let checkbox = Checkbox::new()
    ///     .checked(true)
    ///     .checked_style(Style::new().fg(Color::Green));
    /// \`\`\`
    pub fn checked_style(mut self, style: Style) -> Self {
        self.checked_style = style;
        self
    }

    /// Sets the character displayed when unchecked.
    ///
    /// # Arguments
    ///
    /// * \`ch\` - The character to use (default: ☐).
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    ///
    /// let checkbox = Checkbox::new().unchecked_char('□');
    /// \`\`\`
    pub fn unchecked_char(mut self, ch: char) -> Self {
        self.unchecked_char = ch;
        self
    }

    /// Sets the character displayed when checked.
    ///
    /// # Arguments
    ///
    /// * \`ch\` - The character to use (default: ☑).
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    ///
    /// let checkbox = Checkbox::new().checked_char('■');
    /// \`\`\`
    pub fn checked_char(mut self, ch: char) -> Self {
        self.checked_char = ch;
        self
    }

    /// Toggles the checked state.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    ///
    /// let mut checkbox = Checkbox::new();
    /// checkbox.toggle();  // Now checked
    /// assert!(checkbox.is_checked());
    /// checkbox.toggle();  // Now unchecked
    /// assert!(!checkbox.is_checked());
    /// \`\`\`
    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }

    /// Sets the checked state directly.
    ///
    /// # Arguments
    ///
    /// * \`checked\` - The new checked state.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    ///
    /// let mut checkbox = Checkbox::new();
    /// checkbox.set_checked(true);
    /// assert!(checkbox.is_checked());
    /// \`\`\`
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Returns whether the checkbox is currently checked.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Checkbox;
    ///
    /// let checkbox = Checkbox::new().checked(true);
    /// assert!(checkbox.is_checked());
    /// \`\`\`
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

        buf.modify_cell(area.x, area.y, |cell| {
            cell.symbol = checkbox_char.to_string();
            cell.set_style(style);
        });

        if let Some(ref label) = self.label {
            let label_start = area.x + 2;
            let label_text = label.content();

            for (i, ch) in label_text.chars().enumerate() {
                let x = label_start + i as u16;
                if x >= area.x + area.width {
                    break;
                }
                buf.modify_cell(x, area.y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(self.style);
                });
            }
        }
    }
}

/// A horizontal group of checkboxes rendered inline.
///
/// Useful for displaying multiple related options that can all be toggled
/// independently. All checkboxes are rendered on a single line with optional
/// spacing between them.
///
/// # Example
///
/// \`\`\`rust
/// use ctui_components::CheckboxGroup;
///
/// let group = CheckboxGroup::new()
///     .items(vec![
///         ("Notifications", true),
///         ("Email alerts", false),
///         ("SMS alerts", true),
///     ])
///     .spacing(2);
///
/// // Get all checked items
/// let selected: Vec<&str> = group.checked_items();
/// \`\`\`
#[derive(Clone, Debug, Default)]
pub struct CheckboxGroup {
    /// The checkbox items as (label, checked) pairs.
    checkboxes: Vec<(String, bool)>,
    /// Style for unchecked items.
    style: Style,
    /// Style for checked items.
    checked_style: Style,
    /// Horizontal spacing between checkboxes.
    spacing: u16,
}

impl CheckboxGroup {
    /// Creates a new empty checkbox group.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::CheckboxGroup;
    ///
    /// let group = CheckboxGroup::new();
    /// assert!(group.checked_indices().is_empty());
    /// \`\`\`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the checkbox items from a slice of (label, checked) pairs.
    ///
    /// # Arguments
    ///
    /// * \`items\` - Vector of (label, initial_checked_state) tuples.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::CheckboxGroup;
    ///
    /// let group = CheckboxGroup::new()
    ///     .items(vec![("Option A", true), ("Option B", false)]);
    /// \`\`\`
    pub fn items(mut self, items: Vec<(&str, bool)>) -> Self {
        self.checkboxes = items.into_iter().map(|(s, b)| (s.to_string(), b)).collect();
        self
    }

    /// Sets the style for unchecked checkboxes.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The style to apply to unchecked items.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style for checked checkboxes.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The style to apply to checked items.
    pub fn checked_style(mut self, style: Style) -> Self {
        self.checked_style = style;
        self
    }

    /// Sets the horizontal spacing between checkboxes.
    ///
    /// # Arguments
    ///
    /// * \`spacing\` - Number of spaces between each checkbox (default: 0).
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::CheckboxGroup;
    ///
    /// let group = CheckboxGroup::new()
    ///     .items(vec![("A", true), ("B", false)])
    ///     .spacing(3);
    /// \`\`\`
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Toggles the checked state of a checkbox by index.
    ///
    /// # Arguments
    ///
    /// * \`index\` - The zero-based index of the checkbox to toggle.
    ///
    /// # Panics
    ///
    /// Does nothing if the index is out of bounds (no panic).
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::CheckboxGroup;
    ///
    /// let mut group = CheckboxGroup::new()
    ///     .items(vec![("A", false), ("B", true)]);
    /// group.toggle(0);  // "A" is now checked
    /// assert_eq!(group.checked_indices(), vec![0, 1]);
    /// \`\`\`
    pub fn toggle(&mut self, index: usize) {
        if let Some((_, checked)) = self.checkboxes.get_mut(index) {
            *checked = !*checked;
        }
    }

    /// Returns the indices of all checked items.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::CheckboxGroup;
    ///
    /// let group = CheckboxGroup::new()
    ///     .items(vec![("A", true), ("B", false), ("C", true)]);
    /// assert_eq!(group.checked_indices(), vec![0, 2]);
    /// \`\`\`
    pub fn checked_indices(&self) -> Vec<usize> {
        self.checkboxes
            .iter()
            .enumerate()
            .filter_map(|(i, (_, checked))| if *checked { Some(i) } else { None })
            .collect()
    }

    /// Returns the labels of all checked items.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::CheckboxGroup;
    ///
    /// let group = CheckboxGroup::new()
    ///     .items(vec![("Apple", true), ("Banana", false), ("Cherry", true)]);
    /// assert_eq!(group.checked_items(), vec!["Apple", "Cherry"]);
    /// \`\`\`
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

            buf.modify_cell(x, area.y, |cell| {
                cell.symbol = checkbox_char.to_string();
                cell.set_style(style);
            });
            x += 1;

            x += 1;

            for ch in label.chars() {
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
