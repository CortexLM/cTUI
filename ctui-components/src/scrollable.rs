//! Scrollable region component for managing viewport scrolling.

use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum ScrollbarVisibility {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Clone, Debug)]
pub struct Scrollable {
    content_width: usize,
    content_height: usize,
    scroll_x: u16,
    scroll_y: u16,
    scrollbar_visibility: ScrollbarVisibility,
    scrollbar_style: Style,
    track_style: Style,
    thumb_char: char,
    track_char: char,
}

impl Default for Scrollable {
    fn default() -> Self {
        Self {
            content_width: 0,
            content_height: 0,
            scroll_x: 0,
            scroll_y: 0,
            scrollbar_visibility: ScrollbarVisibility::default(),
            scrollbar_style: Style::default(),
            track_style: Style::default(),
            thumb_char: '█',
            track_char: '░',
        }
    }
}

impl Scrollable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content_size(mut self, width: usize, height: usize) -> Self {
        self.content_width = width;
        self.content_height = height;
        self
    }

    pub fn scroll(mut self, x: u16, y: u16) -> Self {
        self.scroll_x = x;
        self.scroll_y = y;
        self
    }

    pub fn scrollbar_visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.scrollbar_visibility = visibility;
        self
    }

    pub fn scrollbar_style(mut self, style: Style) -> Self {
        self.scrollbar_style = style;
        self
    }

    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = style;
        self
    }

    pub fn thumb_char(mut self, ch: char) -> Self {
        self.thumb_char = ch;
        self
    }

    pub fn track_char(mut self, ch: char) -> Self {
        self.track_char = ch;
        self
    }

    pub fn scroll_x(&self) -> u16 {
        self.scroll_x
    }

    pub fn scroll_y(&self) -> u16 {
        self.scroll_y
    }

    pub fn set_content_size(&mut self, width: usize, height: usize) {
        self.content_width = width;
        self.content_height = height;
        self.clamp_scroll();
    }

    pub fn set_scroll(&mut self, x: u16, y: u16) {
        self.scroll_x = x;
        self.scroll_y = y;
        self.clamp_scroll();
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll_y = self.scroll_y.saturating_sub(amount);
        self.clamp_scroll();
    }

    pub fn scroll_down(&mut self, amount: u16, viewport_height: u16) {
        self.scroll_y = self.scroll_y.saturating_add(amount);
        self.clamp_scroll_with_viewport(viewport_height);
    }

    pub fn scroll_left(&mut self, amount: u16) {
        self.scroll_x = self.scroll_x.saturating_sub(amount);
        self.clamp_scroll();
    }

    pub fn scroll_right(&mut self, amount: u16, viewport_width: u16) {
        self.scroll_x = self.scroll_x.saturating_add(amount);
        self.clamp_scroll_with_viewport_width(viewport_width);
    }

    pub fn scroll_to_top(&mut self) {
        self.scroll_y = 0;
    }

    pub fn scroll_to_bottom(&mut self, viewport_height: u16) {
        if self.content_height > viewport_height as usize {
            self.scroll_y = (self.content_height - viewport_height as usize) as u16;
        }
    }

    pub fn scroll_to_left(&mut self) {
        self.scroll_x = 0;
    }

    pub fn scroll_to_right(&mut self, viewport_width: u16) {
        if self.content_width > viewport_width as usize {
            self.scroll_x = (self.content_width - viewport_width as usize) as u16;
        }
    }

    pub fn clamp_scroll(&mut self) {
        let max_x = self.content_width.saturating_sub(1);
        let max_y = self.content_height.saturating_sub(1);
        self.scroll_x = self.scroll_x.min(max_x as u16);
        self.scroll_y = self.scroll_y.min(max_y as u16);
    }

    fn clamp_scroll_with_viewport(&mut self, viewport_height: u16) {
        if self.content_height > viewport_height as usize {
            let max_y = (self.content_height - viewport_height as usize) as u16;
            self.scroll_y = self.scroll_y.min(max_y);
        } else {
            self.scroll_y = 0;
        }
    }

    fn clamp_scroll_with_viewport_width(&mut self, viewport_width: u16) {
        if self.content_width > viewport_width as usize {
            let max_x = (self.content_width - viewport_width as usize) as u16;
            self.scroll_x = self.scroll_x.min(max_x);
        } else {
            self.scroll_x = 0;
        }
    }

    fn should_show_vertical_scrollbar(&self, viewport_height: u16) -> bool {
        match self.scrollbar_visibility {
            ScrollbarVisibility::Always => true,
            ScrollbarVisibility::Never => false,
            ScrollbarVisibility::Auto => self.content_height > viewport_height as usize,
        }
    }

    fn should_show_horizontal_scrollbar(&self, viewport_width: u16) -> bool {
        match self.scrollbar_visibility {
            ScrollbarVisibility::Always => true,
            ScrollbarVisibility::Never => false,
            ScrollbarVisibility::Auto => self.content_width > viewport_width as usize,
        }
    }

    fn render_vertical_scrollbar(&self, area: Rect, buf: &mut Buffer, viewport_height: usize) {
        if area.height == 0 {
            return;
        }

        let x = area.x + area.width.saturating_sub(1);

        if self.content_height <= viewport_height {
            for y in area.y..area.y + area.height {
                buf.modify_cell(x, y, |cell| {
                    cell.symbol = self.track_char.to_string();
                    cell.set_style(self.track_style);
                });
            }
            return;
        }

        let scroll_ratio =
            self.scroll_y as f64 / (self.content_height - viewport_height).max(1) as f64;
        let thumb_height = ((viewport_height * viewport_height) / self.content_height).max(1);
        let thumb_start = (scroll_ratio * (viewport_height - thumb_height) as f64).round() as usize;

        for (i, y) in (area.y..area.y + area.height).enumerate() {
            buf.modify_cell(x, y, |cell| {
                if i >= thumb_start && i < thumb_start + thumb_height {
                cell.symbol = self.thumb_char.to_string();
                cell.set_style(self.scrollbar_style);
                } else {
                cell.symbol = self.track_char.to_string();
                cell.set_style(self.track_style);
                }
            });
        }
    }

    fn render_horizontal_scrollbar(&self, area: Rect, buf: &mut Buffer, viewport_width: usize) {
        if area.width == 0 {
            return;
        }

        let y = area.y + area.height.saturating_sub(1);
        let _scrollbar_width = area.width.saturating_sub(0) as usize;

        if self.content_width <= viewport_width {
            for x in area.x..area.x + area.width {
                buf.modify_cell(x, y, |cell| {
                    cell.symbol = self.track_char.to_string();
                    cell.set_style(self.track_style);
                });
            }
            return;
        }

        let scroll_ratio =
            self.scroll_x as f64 / (self.content_width - viewport_width).max(1) as f64;
        let thumb_width = ((viewport_width * viewport_width) / self.content_width).max(1);
        let thumb_start = (scroll_ratio * (viewport_width - thumb_width) as f64).round() as usize;

        for (i, x) in (area.x..area.x + area.width).enumerate() {
            buf.modify_cell(x, y, |cell| {
                if i >= thumb_start && i < thumb_start + thumb_width {
                cell.symbol = self.thumb_char.to_string();
                cell.set_style(self.scrollbar_style);
                } else {
                cell.symbol = self.track_char.to_string();
                cell.set_style(self.track_style);
                }
            });
        }
    }

    pub fn get_viewport_area(&self, area: Rect) -> Rect {
        let show_vertical = self.should_show_vertical_scrollbar(area.height);
        let show_horizontal = self.should_show_horizontal_scrollbar(area.width);

        let mut width = area.width;
        let mut height = area.height;

        if show_vertical {
            width = width.saturating_sub(1);
        }
        if show_horizontal {
            height = height.saturating_sub(1);
        }

        Rect::new(area.x, area.y, width, height)
    }
}

pub struct ScrollableProps {
    pub content_width: usize,
    pub content_height: usize,
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub scrollbar_visibility: ScrollbarVisibility,
    pub scrollbar_style: Style,
    pub track_style: Style,
}

impl ScrollableProps {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            content_width: width,
            content_height: height,
            scroll_x: 0,
            scroll_y: 0,
            scrollbar_visibility: ScrollbarVisibility::default(),
            scrollbar_style: Style::default(),
            track_style: Style::default(),
        }
    }

    pub fn scroll(mut self, x: u16, y: u16) -> Self {
        self.scroll_x = x;
        self.scroll_y = y;
        self
    }

    pub fn scrollbar_visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.scrollbar_visibility = visibility;
        self
    }

    pub fn scrollbar_style(mut self, style: Style) -> Self {
        self.scrollbar_style = style;
        self
    }

    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = style;
        self
    }
}

impl Component for Scrollable {
    type Props = ScrollableProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            content_width: props.content_width,
            content_height: props.content_height,
            scroll_x: props.scroll_x,
            scroll_y: props.scroll_y,
            scrollbar_visibility: props.scrollbar_visibility,
            scrollbar_style: props.scrollbar_style,
            track_style: props.track_style,
            thumb_char: '█',
            track_char: '░',
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let viewport_height = area.height as usize;
        let viewport_width = area.width as usize;

        if self.should_show_vertical_scrollbar(area.height) {
            self.render_vertical_scrollbar(
                area,
                buf,
                viewport_height.saturating_sub(
                    if self.should_show_horizontal_scrollbar(area.width) {
                        1
                    } else {
                        0
                    },
                ),
            );
        }

        if self.should_show_horizontal_scrollbar(area.width) {
            self.render_horizontal_scrollbar(
                area,
                buf,
                viewport_width.saturating_sub(
                    if self.should_show_vertical_scrollbar(area.height) {
                        1
                    } else {
                        0
                    },
                ),
            );
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::Buffer;
    use insta::assert_snapshot;

    fn render_to_string(scrollable: &Scrollable, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        scrollable.render(Rect::new(0, 0, width, height), &mut buf);

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
    fn snapshot_scrollable_vertical() {
        let scrollable = Scrollable::new()
            .content_size(50, 100)
            .scroll(0, 50)
            .scrollbar_visibility(ScrollbarVisibility::Always);
        let result = render_to_string(&scrollable, 20, 10);
        assert_snapshot!("scrollable_vertical", result);
    }

    #[test]
    fn snapshot_scrollable_horizontal() {
        let scrollable = Scrollable::new()
            .content_size(100, 20)
            .scroll(50, 0)
            .scrollbar_visibility(ScrollbarVisibility::Always);
        let result = render_to_string(&scrollable, 20, 10);
        assert_snapshot!("scrollable_horizontal", result);
    }

    #[test]
    fn snapshot_scrollable_both() {
        let scrollable = Scrollable::new()
            .content_size(100, 100)
            .scroll(30, 50)
            .scrollbar_visibility(ScrollbarVisibility::Always);
        let result = render_to_string(&scrollable, 20, 10);
        assert_snapshot!("scrollable_both", result);
    }

    #[test]
    fn test_scrollable_default() {
        let scrollable = Scrollable::new();
        assert_eq!(scrollable.scroll_x(), 0);
        assert_eq!(scrollable.scroll_y(), 0);
        assert_eq!(scrollable.content_width, 0);
        assert_eq!(scrollable.content_height, 0);
    }

    #[test]
    fn test_scrollable_scroll() {
        let scrollable = Scrollable::new().content_size(100, 100).scroll(10, 20);
        assert_eq!(scrollable.scroll_x(), 10);
        assert_eq!(scrollable.scroll_y(), 20);
    }

    #[test]
    fn test_scrollable_scroll_up_down() {
        let mut scrollable = Scrollable::new().content_size(100, 100);

        scrollable.scroll_down(10, 20);
        assert_eq!(scrollable.scroll_y(), 10);

        scrollable.scroll_up(5);
        assert_eq!(scrollable.scroll_y(), 5);
    }

    #[test]
    fn test_scrollable_scroll_left_right() {
        let mut scrollable = Scrollable::new().content_size(100, 100);

        scrollable.scroll_right(10, 20);
        assert_eq!(scrollable.scroll_x(), 10);

        scrollable.scroll_left(5);
        assert_eq!(scrollable.scroll_x(), 5);
    }

    #[test]
    fn test_scrollable_scroll_to_edges() {
        let mut scrollable = Scrollable::new().content_size(100, 100);

        scrollable.scroll_to_bottom(20);
        assert_eq!(scrollable.scroll_y(), 80);

        scrollable.scroll_to_right(20);
        assert_eq!(scrollable.scroll_x(), 80);

        scrollable.scroll_to_top();
        assert_eq!(scrollable.scroll_y(), 0);

        scrollable.scroll_to_left();
        assert_eq!(scrollable.scroll_x(), 0);
    }

    #[test]
    fn test_scrollable_clamp() {
        let mut scrollable = Scrollable::new().content_size(50, 50);
        scrollable.scroll_x = 100;
        scrollable.scroll_y = 100;
        scrollable.clamp_scroll();

        assert!(scrollable.scroll_x() < 50);
        assert!(scrollable.scroll_y() < 50);
    }

    #[test]
    fn test_scrollable_visibility_auto() {
        let scrollable = Scrollable::new()
            .content_size(10, 10)
            .scrollbar_visibility(ScrollbarVisibility::Auto);

        assert!(!scrollable.should_show_vertical_scrollbar(20));
        assert!(scrollable.should_show_vertical_scrollbar(5));

        assert!(!scrollable.should_show_horizontal_scrollbar(20));
        assert!(scrollable.should_show_horizontal_scrollbar(5));
    }

    #[test]
    fn test_scrollable_visibility_always_never() {
        let scrollable = Scrollable::new().scrollbar_visibility(ScrollbarVisibility::Always);
        assert!(scrollable.should_show_vertical_scrollbar(100));
        assert!(scrollable.should_show_horizontal_scrollbar(100));

        let scrollable = Scrollable::new().scrollbar_visibility(ScrollbarVisibility::Never);
        assert!(!scrollable.should_show_vertical_scrollbar(1));
        assert!(!scrollable.should_show_horizontal_scrollbar(1));
    }

    #[test]
    fn test_scrollable_props() {
        let props = ScrollableProps::new(100, 100)
            .scroll(10, 20)
            .scrollbar_visibility(ScrollbarVisibility::Always);

        let scrollable = Scrollable::create(props);
        assert_eq!(scrollable.content_width, 100);
        assert_eq!(scrollable.content_height, 100);
        assert_eq!(scrollable.scroll_x(), 10);
        assert_eq!(scrollable.scroll_y(), 20);
    }

    #[test]
    fn test_viewport_area() {
        let scrollable = Scrollable::new()
            .content_size(100, 100)
            .scrollbar_visibility(ScrollbarVisibility::Always);

        let area = Rect::new(0, 0, 20, 10);
        let viewport = scrollable.get_viewport_area(area);

        assert_eq!(viewport.width, 19);
        assert_eq!(viewport.height, 9);
    }

    #[test]
    fn test_render_empty_area() {
        let scrollable = Scrollable::new().content_size(100, 100);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        scrollable.render(Rect::new(0, 0, 0, 0), &mut buf);
    }
}
