//! Per-component theming system
//!
//! This module provides a trait-based approach for component-specific theming,
//! allowing each component type to define its own theming requirements while
//! supporting inheritance and override patterns.

use crate::color::Color;
use crate::style::{BorderStyle, Modifier, Spacing, Style};
use crate::theme::Theme;
use serde::{Deserialize, Serialize};

/// Trait for component-specific theming
///
/// Components implement this trait to define their default styling
/// and to enable theme inheritance patterns.
pub trait ComponentTheme: Clone + Default {
    /// Returns the style for this component in the given theme
    fn style(&self, theme: &Theme) -> Style;

    /// Returns the style for a specific state (hover, focus, disabled, etc.)
    fn style_for_state(&self, theme: &Theme, state: ComponentState) -> Style {
        let base = self.style(theme);
        match state {
            ComponentState::Normal => base,
            ComponentState::Hover => self.hover_style(theme, base),
            ComponentState::Focus => self.focus_style(theme, base),
            ComponentState::Active => self.active_style(theme, base),
            ComponentState::Disabled => self.disabled_style(theme, base),
        }
    }

    /// Override to customize hover state
    fn hover_style(&self, _theme: &Theme, base: Style) -> Style {
        base
    }

    /// Override to customize focus state
    fn focus_style(&self, theme: &Theme, base: Style) -> Style {
        base.bg(theme.colors.focus)
    }

    /// Override to customize active/pressed state
    fn active_style(&self, _theme: &Theme, base: Style) -> Style {
        base.add_modifier(Modifier::BOLD)
    }

    /// Override to customize disabled state
    fn disabled_style(&self, _theme: &Theme, base: Style) -> Style {
        base.add_modifier(Modifier::DIM)
    }

    /// Merge with parent theme, child takes precedence
    fn merge_with_parent(&self, parent: &Style, child: &Style) -> Style {
        parent.merge(child)
    }
}

/// Component interaction states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ComponentState {
    /// Normal/ resting state
    #[default]
    Normal,
    /// Mouse hover state
    Hover,
    /// Focus state (keyboard or click)
    Focus,
    /// Active/pressed state
    Active,
    /// Disabled state
    Disabled,
}


/// Block component theme (containers, panels, cards)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockTheme {
    /// Base style
    #[serde(default)]
    pub base: Style,
    /// Border style
    #[serde(default)]
    pub border: BorderStyle,
    /// Border color override
    #[serde(default)]
    pub border_color: Option<Color>,
    /// Title style
    #[serde(default)]
    pub title_style: Style,
    /// Padding inside the block
    #[serde(default)]
    pub padding: Spacing,
}

impl Default for BlockTheme {
    fn default() -> Self {
        Self {
            base: Style::new(),
            border: BorderStyle::Single,
            border_color: None,
            title_style: Style::new().add_modifier(Modifier::BOLD),
            padding: Spacing::all(1),
        }
    }
}

impl ComponentTheme for BlockTheme {
    fn style(&self, theme: &Theme) -> Style {
        let mut style = self.base.clone();
        style.bg = if self.base.bg.is_default() {
            theme.colors.surface
        } else {
            self.base.bg
        };
        style.fg = if self.base.fg.is_default() {
            theme.colors.text
        } else {
            self.base.fg
        };
        style.border_color = self.border_color.unwrap_or(theme.colors.border);
        style.border_style = self.border;
        style.padding = self.padding;
        style
    }

    fn focus_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            border_color: theme.colors.focus,
            ..base
        }
    }
}

/// Paragraph/text component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParagraphTheme {
    /// Base text style
    #[serde(default)]
    pub base: Style,
    /// Alignment (left, center, right)
    #[serde(default)]
    pub alignment: TextAlignment,
    /// Wrap behavior
    #[serde(default)]
    pub wrap: bool,
    /// Line height multiplier (x100)
    #[serde(default = "default_line_height")]
    pub line_height: u16,
}

const fn default_line_height() -> u16 {
    100 // 1.0x
}

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextAlignment {
    /// Left alignment (default)
    #[default]
    Left,
    /// Center alignment
    Center,
    /// Right alignment
    Right,
}

impl Default for ParagraphTheme {
    fn default() -> Self {
        Self {
            base: Style::new(),
            alignment: TextAlignment::Left,
            wrap: true,
            line_height: 100,
        }
    }
}

impl ComponentTheme for ParagraphTheme {
    fn style(&self, theme: &Theme) -> Style {
        let mut style = self.base.clone();
        style.fg = if self.base.fg.is_default() {
            theme.colors.text
        } else {
            self.base.fg
        };
        style
    }

    fn disabled_style(&self, _theme: &Theme, base: Style) -> Style {
        Style {
            fg: _theme.colors.text_muted,
            ..base
        }
        .add_modifier(Modifier::DIM)
    }
}

/// List component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListTheme {
    /// Base list style
    #[serde(default)]
    pub base: Style,
    /// Item style
    #[serde(default)]
    pub item: Style,
    /// Selected item style
    #[serde(default)]
    pub selected: Style,
    /// Highlighted/active item style
    #[serde(default)]
    pub highlighted: Style,
    /// Bullet/indicator character
    #[serde(default = "default_bullet")]
    pub bullet: char,
    /// Spacing between items
    #[serde(default)]
    pub item_spacing: u16,
}

const fn default_bullet() -> char {
    '•'
}

impl Default for ListTheme {
    fn default() -> Self {
        Self {
            base: Style::new(),
            item: Style::new(),
            selected: Style::new().add_modifier(Modifier::BOLD),
            highlighted: Style::new(),
            bullet: '•',
            item_spacing: 0,
        }
    }
}

impl ComponentTheme for ListTheme {
    fn style(&self, theme: &Theme) -> Style {
        let mut style = self.base.clone();
        style.fg = if self.base.fg.is_default() {
            theme.colors.text
        } else {
            self.base.fg
        };
        style
    }

    fn hover_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            fg: theme.colors.accent,
            ..base
        }
    }

    fn focus_style(&self, theme: &Theme, _base: Style) -> Style {
        self.selected
            .clone()
            .fg(theme.colors.text)
            .bg(theme.colors.primary)
    }
}

/// Button component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ButtonTheme {
    /// Base button style
    #[serde(default)]
    pub base: Style,
    /// Primary action style
    #[serde(default)]
    pub primary: Style,
    /// Secondary action style
    #[serde(default)]
    pub secondary: Style,
    /// Danger/destructive action style
    #[serde(default)]
    pub danger: Style,
    /// Button size variants
    #[serde(default)]
    pub sizes: ButtonSizes,
}

/// Button size variants
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ButtonSizes {
    /// Small padding
    #[serde(default = "default_size_sm")]
    pub small: Spacing,
    /// Medium padding (default)
    #[serde(default = "default_size_md")]
    pub medium: Spacing,
    /// Large padding
    #[serde(default = "default_size_lg")]
    pub large: Spacing,
}

const fn default_size_sm() -> Spacing {
    Spacing::horizontal_vertical(2, 1)
}

const fn default_size_md() -> Spacing {
    Spacing::horizontal_vertical(4, 1)
}

const fn default_size_lg() -> Spacing {
    Spacing::horizontal_vertical(6, 2)
}

impl Default for ButtonSizes {
    fn default() -> Self {
        Self {
            small: default_size_sm(),
            medium: default_size_md(),
            large: default_size_lg(),
        }
    }
}

impl Default for ButtonTheme {
    fn default() -> Self {
        Self {
            base: Style::new(),
            primary: Style::new().border(BorderStyle::Rounded),
            secondary: Style::new().border(BorderStyle::Rounded),
            danger: Style::new().border(BorderStyle::Rounded),
            sizes: ButtonSizes::default(),
        }
    }
}

impl ComponentTheme for ButtonTheme {
    fn style(&self, theme: &Theme) -> Style {
        self.base
            .clone()
            .fg(theme.colors.text)
            .bg(theme.colors.surface)
            .border(BorderStyle::Rounded)
            .border_color(theme.colors.border)
            .padding(self.sizes.medium)
    }

    fn hover_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            bg: theme.colors.primary,
            fg: theme.colors.background,
            ..base
        }
    }

    fn active_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            bg: theme.colors.accent,
            ..base
        }
        .add_modifier(Modifier::BOLD)
    }

    fn focus_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            border_color: theme.colors.focus,
            ..base
        }
        .add_modifier(Modifier::BOLD)
    }

    fn disabled_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            fg: theme.colors.text_muted,
            bg: theme.colors.surface,
            ..base
        }
        .add_modifier(Modifier::DIM)
    }
}

impl ButtonTheme {
    /// Get style for primary variant
    #[must_use]
    pub fn primary_style(&self, theme: &Theme) -> Style {
        self.primary
            .clone()
            .fg(theme.colors.background)
            .bg(theme.colors.primary)
            .border(BorderStyle::Rounded)
            .padding(self.sizes.medium)
    }

    /// Get style for danger variant
    #[must_use]
    pub fn danger_style(&self, theme: &Theme) -> Style {
        self.danger
            .clone()
            .fg(theme.colors.background)
            .bg(theme.colors.error)
            .border(BorderStyle::Rounded)
            .padding(self.sizes.medium)
    }
}

/// Input field component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputTheme {
    /// Base input style
    #[serde(default)]
    pub base: Style,
    /// Placeholder style
    #[serde(default)]
    pub placeholder: Style,
    /// Cursor style
    #[serde(default)]
    pub cursor: Style,
    /// Selection style
    #[serde(default)]
    pub selection: Style,
    /// Placeholder text
    #[serde(default)]
    pub placeholder_text: Option<String>,
}

impl Default for InputTheme {
    fn default() -> Self {
        Self {
            base: Style::new().border(BorderStyle::Single),
            placeholder: Style::new().add_modifier(Modifier::DIM),
            cursor: Style::new().add_modifier(Modifier::REVERSED),
            selection: Style::new().add_modifier(Modifier::REVERSED),
            placeholder_text: None,
        }
    }
}

impl ComponentTheme for InputTheme {
    fn style(&self, theme: &Theme) -> Style {
        self.base
            .clone()
            .fg(theme.colors.text)
            .bg(theme.colors.background)
            .border_color(theme.colors.border)
            .padding(Spacing::horizontal_vertical(1, 0))
    }

    fn focus_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            border_color: theme.colors.focus,
            ..base
        }
    }

    fn disabled_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            fg: theme.colors.text_muted,
            bg: theme.colors.surface,
            border_color: theme.colors.text_muted,
            ..base
        }
        .add_modifier(Modifier::DIM)
    }
}

/// Table component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableTheme {
    /// Base table style
    #[serde(default)]
    pub base: Style,
    /// Header row style
    #[serde(default)]
    pub header: Style,
    /// Row style (alternating colors)
    #[serde(default)]
    pub row: Style,
    /// Selected row style
    #[serde(default)]
    pub selected: Style,
    /// Cell spacing
    #[serde(default = "default_cell_spacing")]
    pub cell_spacing: u16,
    /// Column separator
    #[serde(default)]
    pub column_separator: char,
}

const fn default_cell_spacing() -> u16 {
    2
}

impl Default for TableTheme {
    fn default() -> Self {
        Self {
            base: Style::new(),
            header: Style::new().add_modifier(Modifier::BOLD),
            row: Style::new(),
            selected: Style::new().add_modifier(Modifier::BOLD),
            cell_spacing: 2,
            column_separator: '│',
        }
    }
}

impl ComponentTheme for TableTheme {
    fn style(&self, theme: &Theme) -> Style {
        self.base.clone().fg(theme.colors.text)
    }

    fn focus_style(&self, theme: &Theme, _base: Style) -> Style {
        self.selected
            .clone()
            .fg(theme.colors.text)
            .bg(theme.colors.primary)
    }
}

/// Progress bar component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressTheme {
    /// Bar style
    #[serde(default)]
    pub bar: Style,
    /// Background/track style
    #[serde(default)]
    pub track: Style,
    /// Label style
    #[serde(default)]
    pub label: Style,
    /// Filled character
    #[serde(default = "default_bar_char")]
    pub bar_char: char,
    /// Empty/track character
    #[serde(default = "default_track_char")]
    pub track_char: char,
}

const fn default_bar_char() -> char {
    '█'
}

const fn default_track_char() -> char {
    '░'
}

impl Default for ProgressTheme {
    fn default() -> Self {
        Self {
            bar: Style::new(),
            track: Style::new().add_modifier(Modifier::DIM),
            label: Style::new(),
            bar_char: '█',
            track_char: '░',
        }
    }
}

impl ComponentTheme for ProgressTheme {
    fn style(&self, theme: &Theme) -> Style {
        self.bar
            .clone()
            .fg(theme.colors.primary)
            .bg(theme.colors.surface)
    }
}

impl ProgressTheme {
    /// Get the track (background) style
    #[must_use]
    pub fn track_style(&self, theme: &Theme) -> Style {
        self.track
            .clone()
            .fg(theme.colors.text_muted)
            .bg(theme.colors.surface)
    }
}

/// Modal/dialog component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalTheme {
    /// Overlay/backdrop style
    #[serde(default)]
    pub overlay: Style,
    /// Dialog box style
    #[serde(default)]
    pub dialog: Style,
    /// Title style
    #[serde(default)]
    pub title: Style,
    /// Content style
    #[serde(default)]
    pub content: Style,
    /// Footer style
    #[serde(default)]
    pub footer: Style,
}

impl Default for ModalTheme {
    fn default() -> Self {
        Self {
            overlay: Style::new()
                .bg(Color::default_color())
                .add_modifier(Modifier::DIM),
            dialog: Style::new().border(BorderStyle::Rounded),
            title: Style::new().add_modifier(Modifier::BOLD),
            content: Style::new(),
            footer: Style::new(),
        }
    }
}

impl ComponentTheme for ModalTheme {
    fn style(&self, theme: &Theme) -> Style {
        self.dialog
            .clone()
            .fg(theme.colors.text)
            .bg(theme.colors.surface)
            .border_color(theme.colors.border)
            .padding(Spacing::all(2))
    }

    fn focus_style(&self, theme: &Theme, base: Style) -> Style {
        Style {
            border_color: theme.colors.primary,
            ..base
        }
    }
}

/// Scrollbar component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrollbarTheme {
    /// Thumb style
    #[serde(default)]
    pub thumb: Style,
    /// Track style
    #[serde(default)]
    pub track: Style,
    /// Thumb character
    #[serde(default = "default_thumb_char")]
    pub thumb_char: char,
    /// Track character
    #[serde(default = "default_track_scroll_char")]
    pub track_char: char,
}

const fn default_thumb_char() -> char {
    '█'
}

const fn default_track_scroll_char() -> char {
    '░'
}

impl Default for ScrollbarTheme {
    fn default() -> Self {
        Self {
            thumb: Style::new(),
            track: Style::new().add_modifier(Modifier::DIM),
            thumb_char: '█',
            track_char: '░',
        }
    }
}

impl ComponentTheme for ScrollbarTheme {
    fn style(&self, theme: &Theme) -> Style {
        self.thumb
            .clone()
            .fg(theme.colors.text_muted)
            .bg(theme.colors.surface)
    }
}

/// Tabs component theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabsTheme {
    /// Base tabs style
    #[serde(default)]
    pub base: Style,
    /// Active tab style
    #[serde(default)]
    pub active: Style,
    /// Inactive tab style
    #[serde(default)]
    pub inactive: Style,
    /// Tab separator style
    #[serde(default)]
    pub separator: char,
}

impl Default for TabsTheme {
    fn default() -> Self {
        Self {
            base: Style::new(),
            active: Style::new().border(BorderStyle::Single),
            inactive: Style::new(),
            separator: '│',
        }
    }
}

impl ComponentTheme for TabsTheme {
    fn style(&self, theme: &Theme) -> Style {
        self.base.clone().fg(theme.colors.text)
    }
}

impl TabsTheme {
    /// Get the active tab style
    #[must_use]
    pub fn active_style(&self, theme: &Theme) -> Style {
        self.active
            .clone()
            .fg(theme.colors.text)
            .bg(theme.colors.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Get the inactive tab style
    #[must_use]
    pub fn inactive_style(&self, theme: &Theme) -> Style {
        self.inactive
            .clone()
            .fg(theme.colors.text_muted)
            .bg(theme.colors.surface)
    }
}

/// Comprehensive component themes container
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub struct AllComponentThemes {
    /// Block/panel theme
    #[serde(default)]
    pub block: BlockTheme,
    /// Paragraph theme
    #[serde(default)]
    pub paragraph: ParagraphTheme,
    /// List theme
    #[serde(default)]
    pub list: ListTheme,
    /// Button theme
    #[serde(default)]
    pub button: ButtonTheme,
    /// Input theme
    #[serde(default)]
    pub input: InputTheme,
    /// Table theme
    #[serde(default)]
    pub table: TableTheme,
    /// Progress bar theme
    #[serde(default)]
    pub progress: ProgressTheme,
    /// Modal theme
    #[serde(default)]
    pub modal: ModalTheme,
    /// Scrollbar theme
    #[serde(default)]
    pub scrollbar: ScrollbarTheme,
    /// Tabs theme
    #[serde(default)]
    pub tabs: TabsTheme,
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn test_block_theme() {
        let theme = Theme::dark();
        let block_theme = BlockTheme::default();
        let style = block_theme.style(&theme);
        assert_eq!(style.border_style, BorderStyle::Single);
    }

    #[test]
    fn test_button_theme_states() {
        let theme = Theme::dracula();
        let button_theme = ButtonTheme::default();

        let hover = button_theme.style_for_state(&theme, ComponentState::Hover);
        let focus = button_theme.style_for_state(&theme, ComponentState::Focus);

        assert!(hover.bg == theme.colors.primary);
        assert!(focus.border_color == theme.colors.focus);
    }

    #[test]
    fn test_list_theme() {
        let theme = Theme::nord();
        let list_theme = ListTheme::default();
        let style = list_theme.style(&theme);
        assert!(style.fg == theme.colors.text);
    }

    #[test]
    fn test_component_theme_serialization() {
        let themes = AllComponentThemes::default();
        let json = serde_json::to_string(&themes).unwrap();
        let deserialized: AllComponentThemes = serde_json::from_str(&json).unwrap();
        assert_eq!(themes.button.sizes.medium, deserialized.button.sizes.medium);
    }

    #[test]
    fn test_paragraph_theme_alignment() {
        let para = ParagraphTheme {
            alignment: TextAlignment::Center,
            ..Default::default()
        };
        assert_eq!(para.alignment, TextAlignment::Center);
    }

    #[test]
    fn test_input_theme() {
        let theme = Theme::dark();
        let input_theme = InputTheme::default();
        let style = input_theme.style(&theme);
        assert_eq!(style.border_style, BorderStyle::Single);
    }

    #[test]
    fn test_progress_theme() {
        let theme = Theme::gruvbox();
        let progress_theme = ProgressTheme::default();
        let _style = progress_theme.style(&theme);
        assert_eq!(progress_theme.bar_char, '█');
        assert_eq!(progress_theme.track_char, '░');
    }

    #[test]
    fn test_tabs_theme() {
        let theme = Theme::nord();
        let tabs_theme = TabsTheme::default();
        let active = tabs_theme.active_style(&theme);
        let inactive = tabs_theme.inactive_style(&theme);

        assert!(active.modifiers.contains(Modifier::BOLD));
        assert!(!inactive.modifiers.contains(Modifier::BOLD));
    }
}
