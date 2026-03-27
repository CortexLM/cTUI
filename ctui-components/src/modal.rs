//! Modal/Dialog component for overlay UI.

use crate::block::Block;
use crate::{BorderType, Borders};
use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Medium,
    Large,
    Custom(u16, u16),
}

impl ModalSize {
    pub fn dimensions(&self, max_width: u16, max_height: u16) -> (u16, u16) {
        match self {
            ModalSize::Small => {
                let w = (max_width / 3).max(20).min(max_width);
                let h = (max_height / 3).max(5).min(max_height);
                (w, h)
            }
            ModalSize::Medium => {
                let w = (max_width / 2).max(30).min(max_width);
                let h = (max_height / 2).max(10).min(max_height);
                (w, h)
            }
            ModalSize::Large => {
                let w = (max_width * 3 / 4).max(40).min(max_width);
                let h = (max_height * 3 / 4).max(15).min(max_height);
                (w, h)
            }
            ModalSize::Custom(w, h) => ((*w).min(max_width), (*h).min(max_height)),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ModalAlignment {
    Top,
    Center,
    Bottom,
}

impl Default for ModalAlignment {
    fn default() -> Self {
        ModalAlignment::Center
    }
}

#[derive(Clone, Debug)]
pub struct ModalButton {
    label: String,
    style: Style,
    action: ModalAction,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ModalAction {
    Close,
    Confirm,
    Custom,
}

impl ModalButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: Style::default(),
            action: ModalAction::Close,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn action(mut self, action: ModalAction) -> Self {
        self.action = action;
        self
    }

    pub fn confirm(label: impl Into<String>) -> Self {
        Self::new(label).action(ModalAction::Confirm)
    }

    pub fn close(label: impl Into<String>) -> Self {
        Self::new(label).action(ModalAction::Close)
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug)]
pub struct Modal {
    title: Option<String>,
    content: String,
    size: ModalSize,
    alignment: ModalAlignment,
    buttons: Vec<ModalButton>,
    focused_button: Option<usize>,
    style: Style,
    title_style: Style,
    content_style: Style,
    backdrop_style: Style,
    show_backdrop: bool,
    border_type: BorderType,
    close_on_escape: bool,
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            title: None,
            content: String::new(),
            size: ModalSize::default(),
            alignment: ModalAlignment::default(),
            buttons: vec![ModalButton::close("Close")],
            focused_button: Some(0),
            style: Style::default(),
            title_style: Style::default(),
            content_style: Style::default(),
            backdrop_style: Style::default(),
            show_backdrop: true,
            border_type: BorderType::Plain,
            close_on_escape: true,
        }
    }
}

impl Modal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn size(mut self, size: ModalSize) -> Self {
        self.size = size;
        self
    }

    pub fn alignment(mut self, alignment: ModalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn buttons(mut self, buttons: Vec<ModalButton>) -> Self {
        self.buttons = buttons;
        self.focused_button = if !self.buttons.is_empty() {
            Some(0)
        } else {
            None
        };
        self
    }

    pub fn add_button(mut self, button: ModalButton) -> Self {
        self.buttons.push(button);
        if self.focused_button.is_none() {
            self.focused_button = Some(0);
        }
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    pub fn content_style(mut self, style: Style) -> Self {
        self.content_style = style;
        self
    }

    pub fn backdrop_style(mut self, style: Style) -> Self {
        self.backdrop_style = style;
        self
    }

    pub fn show_backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn focused_button(&self) -> Option<usize> {
        self.focused_button
    }

    pub fn focus_next(&mut self) {
        if self.buttons.is_empty() {
            return;
        }
        match self.focused_button {
            Some(idx) => {
                self.focused_button = Some((idx + 1) % self.buttons.len());
            }
            None => {
                self.focused_button = Some(0);
            }
        }
    }

    pub fn focus_prev(&mut self) {
        if self.buttons.is_empty() {
            return;
        }
        match self.focused_button {
            Some(0) => {
                self.focused_button = Some(self.buttons.len() - 1);
            }
            Some(idx) => {
                self.focused_button = Some(idx - 1);
            }
            None => {
                self.focused_button = Some(self.buttons.len() - 1);
            }
        }
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
    }

    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    fn calculate_modal_rect(&self, area: Rect) -> Rect {
        let (modal_width, modal_height) = self.size.dimensions(area.width, area.height);

        let x = (area.width.saturating_sub(modal_width)) / 2;

        let y = match self.alignment {
            ModalAlignment::Top => 2,
            ModalAlignment::Center => (area.height.saturating_sub(modal_height)) / 2,
            ModalAlignment::Bottom => area.height.saturating_sub(modal_height + 2),
        };

        Rect::new(area.x + x, area.y + y, modal_width, modal_height)
    }

    fn render_backdrop(&self, area: Rect, buf: &mut Buffer) {
        if !self.show_backdrop {
            return;
        }

        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.get_mut(x, y) {
                    cell.set_style(self.backdrop_style);
                }
            }
        }
    }

    fn render_buttons(&self, modal_area: Rect, buf: &mut Buffer) {
        if self.buttons.is_empty() {
            return;
        }

        let button_row = modal_area.y + modal_area.height.saturating_sub(2);
        let total_width: usize = self
            .buttons
            .iter()
            .map(|b| b.label.len() + 4)
            .sum::<usize>()
            + (self.buttons.len().saturating_sub(1) * 2);
        let start_x = modal_area.x + (modal_area.width.saturating_sub(total_width as u16)) / 2;

        let mut current_x = start_x;
        for (i, button) in self.buttons.iter().enumerate() {
            let is_focused = self.focused_button == Some(i);
            let button_text = format!("[ {} ]", button.label);
            let button_width = button_text.len() as u16;

            for (j, ch) in button_text.chars().enumerate() {
                if let Some(cell) = buf.get_mut(current_x + j as u16, button_row) {
                    cell.symbol = ch.to_string();
                    cell.set_style(if is_focused {
                        Style {
                            modifier: ctui_core::style::Modifier::REVERSED,
                            ..button.style
                        }
                    } else {
                        button.style
                    });
                }
            }

            current_x += button_width + 2;
        }
    }

    fn render_content(&self, modal_area: Rect, buf: &mut Buffer) {
        let content_y = modal_area.y + 2;
        let content_height = modal_area.height.saturating_sub(5);

        let lines: Vec<&str> = self.content.lines().take(content_height as usize).collect();

        for (i, line) in lines.iter().enumerate() {
            let y = content_y + i as u16;
            if y >= modal_area.y + modal_area.height.saturating_sub(3) {
                break;
            }

            for (j, ch) in line
                .chars()
                .take(modal_area.width.saturating_sub(2) as usize)
                .enumerate()
            {
                if let Some(cell) = buf.get_mut(modal_area.x + 1 + j as u16, y) {
                    cell.symbol = ch.to_string();
                    cell.set_style(self.content_style);
                }
            }
        }
    }
}

pub struct ModalProps {
    pub title: Option<String>,
    pub content: String,
    pub size: ModalSize,
    pub alignment: ModalAlignment,
    pub buttons: Vec<ModalButton>,
    pub style: Style,
    pub title_style: Style,
    pub content_style: Style,
    pub backdrop_style: Style,
    pub show_backdrop: bool,
    pub border_type: BorderType,
}

impl ModalProps {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            title: None,
            content: content.into(),
            size: ModalSize::default(),
            alignment: ModalAlignment::default(),
            buttons: vec![ModalButton::close("Close")],
            style: Style::default(),
            title_style: Style::default(),
            content_style: Style::default(),
            backdrop_style: Style::default(),
            show_backdrop: true,
            border_type: BorderType::Plain,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn size(mut self, size: ModalSize) -> Self {
        self.size = size;
        self
    }

    pub fn alignment(mut self, alignment: ModalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn buttons(mut self, buttons: Vec<ModalButton>) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }
}

impl Component for Modal {
    type Props = ModalProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            title: props.title,
            content: props.content,
            size: props.size,
            alignment: props.alignment,
            buttons: props.buttons,
            focused_button: Some(0),
            style: props.style,
            title_style: Style::default(),
            content_style: props.content_style,
            backdrop_style: props.backdrop_style,
            show_backdrop: props.show_backdrop,
            border_type: props.border_type,
            close_on_escape: true,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_backdrop(area, buf);

        let modal_rect = self.calculate_modal_rect(area);

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(self.border_type)
            .style(self.style);

        if let Some(ref title) = self.title {
            block.clone().title(title).render(modal_rect, buf);
        } else {
            block.render(modal_rect, buf);
        }

        self.render_content(modal_rect, buf);
        self.render_buttons(modal_rect, buf);
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

    fn render_to_string(modal: &Modal, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        modal.render(Rect::new(0, 0, width, height), &mut buf);

        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                output.push_str(&buf[(x, y)].symbol);
            }
            if y < height - 1 {
                output.push('\n');
            }
        }
        output
    }

    #[test]
    fn snapshot_modal_basic() {
        let modal = Modal::new()
            .title("Confirmation")
            .content("Are you sure you want to continue?")
            .size(ModalSize::Small);
        let result = render_to_string(&modal, 50, 20);
        assert_snapshot!("modal_basic", result);
    }

    #[test]
    fn snapshot_modal_multiple_buttons() {
        let modal = Modal::new()
            .title("Delete Item")
            .content("This action cannot be undone.")
            .buttons(vec![
                ModalButton::close("Cancel"),
                ModalButton::confirm("Delete"),
            ])
            .size(ModalSize::Medium);
        let result = render_to_string(&modal, 60, 25);
        assert_snapshot!("modal_multiple_buttons", result);
    }

    #[test]
    fn snapshot_modal_multiline() {
        let modal = Modal::new()
            .title("About")
            .content("This is a multi-line\nmodal dialog example\nwith three lines of text.")
            .size(ModalSize::Large)
            .buttons(vec![ModalButton::new("OK")]);
        let result = render_to_string(&modal, 60, 25);
        assert_snapshot!("modal_multiline", result);
    }

    #[test]
    fn snapshot_modal_no_title() {
        let modal = Modal::new()
            .content("Simple message dialog")
            .size(ModalSize::Small);
        let result = render_to_string(&modal, 40, 15);
        assert_snapshot!("modal_no_title", result);
    }

    #[test]
    fn test_modal_size_dimensions() {
        let size = ModalSize::Small;
        let (w, h) = size.dimensions(80, 24);
        assert!(w >= 20);
        assert!(h >= 5);

        let size = ModalSize::Medium;
        let (w, h) = size.dimensions(80, 24);
        assert!(w >= 30);
        assert!(h >= 10);

        let size = ModalSize::Large;
        let (w, h) = size.dimensions(80, 24);
        assert!(w >= 40);
        assert!(h >= 15);

        let size = ModalSize::Custom(30, 10);
        let (w, h) = size.dimensions(80, 24);
        assert_eq!(w, 30);
        assert_eq!(h, 10);
    }

    #[test]
    fn test_modal_size_clamp() {
        let size = ModalSize::Custom(100, 50);
        let (w, h) = size.dimensions(50, 20);
        assert_eq!(w, 50);
        assert_eq!(h, 20);
    }

    #[test]
    fn test_modal_button_new() {
        let button = ModalButton::new("Test");
        assert_eq!(button.label(), "Test");
        assert_eq!(button.action, ModalAction::Close);
    }

    #[test]
    fn test_modal_button_actions() {
        let confirm = ModalButton::confirm("Yes");
        assert_eq!(confirm.action, ModalAction::Confirm);

        let close = ModalButton::close("No");
        assert_eq!(close.action, ModalAction::Close);
    }

    #[test]
    fn test_modal_default() {
        let modal = Modal::new();
        assert!(modal.title.is_none());
        assert_eq!(modal.content, "");
        assert_eq!(modal.size, ModalSize::Medium);
        assert_eq!(modal.alignment, ModalAlignment::Center);
        assert!(modal.close_on_escape);
    }

    #[test]
    fn test_modal_focus_navigation() {
        let mut modal = Modal::new().buttons(vec![
            ModalButton::new("A"),
            ModalButton::new("B"),
            ModalButton::new("C"),
        ]);

        assert_eq!(modal.focused_button(), Some(0));

        modal.focus_next();
        assert_eq!(modal.focused_button(), Some(1));

        modal.focus_next();
        assert_eq!(modal.focused_button(), Some(2));

        modal.focus_next();
        assert_eq!(modal.focused_button(), Some(0)); // Wraps

        modal.focus_prev();
        assert_eq!(modal.focused_button(), Some(2)); // Wraps back
    }

    #[test]
    fn test_modal_setters() {
        let mut modal = Modal::new();
        modal.set_title("New Title");
        modal.set_content("New Content");

        assert_eq!(modal.title, Some("New Title".to_string()));
        assert_eq!(modal.content, "New Content");
    }

    #[test]
    fn test_modal_props() {
        let props = ModalProps::new("Test content")
            .title("Test Title")
            .size(ModalSize::Large)
            .alignment(ModalAlignment::Top);
        let modal = Modal::create(props);

        assert_eq!(modal.title, Some("Test Title".to_string()));
        assert_eq!(modal.content, "Test content");
        assert_eq!(modal.size, ModalSize::Large);
        assert_eq!(modal.alignment, ModalAlignment::Top);
    }

    #[test]
    fn test_modal_calculate_rect() {
        let modal = Modal::new().size(ModalSize::Custom(30, 10));
        let area = Rect::new(0, 0, 80, 24);
        let rect = modal.calculate_modal_rect(area);

        assert_eq!(rect.width, 30);
        assert_eq!(rect.height, 10);
        assert!(rect.x < 80);
        assert!(rect.y < 24);
    }

    #[test]
    fn test_render_empty_modal() {
        let modal = Modal::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 15));
        modal.render(Rect::new(0, 0, 40, 15), &mut buf);
    }
}
