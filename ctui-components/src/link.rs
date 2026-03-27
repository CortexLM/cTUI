use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

const OSC8_START: &str = "\x1b]8;;";
const OSC8_END: &str = "\x1b\\";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    url: String,
    text: String,
    style: Style,
    hover_style: Style,
    is_hovered: bool,
}

impl Link {
    pub fn new(url: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            text: text.into(),
            style: Style::default(),
            hover_style: Style::default(),
            is_hovered: false,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
    }

    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    pub fn osc8_sequence(&self) -> String {
        format!("{}{}\x07{}{}", OSC8_START, self.url, self.text, OSC8_END)
    }

    pub fn render_osc8(&self) -> String {
        format!("\x1b]8;;{}\x07{}", self.url, self.text)
    }

    pub fn render_with_style(&self) -> String {
        let style_prefix = String::new();
        format!("{}{}\x07{}", style_prefix, OSC8_START, self.render_osc8())
    }

    pub fn contains_point(&self, x: u16, y: u16, area: Rect) -> bool {
        if y != area.y {
            return false;
        }
        let text_len = self.text.chars().count() as u16;
        x >= area.x && x < area.x + text_len
    }

    pub fn width(&self) -> u16 {
        self.text.chars().count() as u16
    }
}

pub fn format_link(url: &str, text: &str) -> String {
    format!("\x1b]8;;{}\x07{}", url, text)
}

pub fn link_with_id(id: &str, url: &str, text: &str) -> String {
    format!("{}id={},{}\x07{}\x1b]8;;\x07", OSC8_START, id, url, text)
}

#[derive(Debug, Clone)]
pub struct LinkProps {
    pub url: String,
    pub text: String,
    pub style: Style,
    pub hover_style: Style,
}

impl LinkProps {
    pub fn new(url: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            text: text.into(),
            style: Style::default(),
            hover_style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }
}

impl Component for Link {
    type Props = LinkProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            url: props.url,
            text: props.text,
            style: props.style,
            hover_style: props.hover_style,
            is_hovered: false,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }

        let style = if self.is_hovered {
            self.hover_style
        } else {
            self.style
        };

        for (i, ch) in self.text.chars().enumerate() {
            let x = area.x + i as u16;
            if x >= area.x + area.width {
                break;
            }
            if let Some(cell) = buf.get_mut(x, area.y) {
                cell.symbol = ch.to_string();
                cell.set_style(style);
            }
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

pub struct LinkList {
    links: Vec<Link>,
    spacing: u16,
    vertical: bool,
}

impl LinkList {
    pub fn new() -> Self {
        Self {
            links: Vec::new(),
            spacing: 1,
            vertical: false,
        }
    }

    pub fn add(mut self, url: impl Into<String>, text: impl Into<String>) -> Self {
        self.links.push(Link::new(url, text));
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn links(&self) -> &[Link] {
        &self.links
    }

    pub fn link_at(&self, x: u16, y: u16, area: Rect) -> Option<&Link> {
        if self.vertical {
            let index = y.saturating_sub(area.y) as usize;
            self.links.get(index)
        } else {
            let mut current_x = area.x;
            for link in &self.links {
                let link_width = link.width();
                if x >= current_x && x < current_x + link_width && y == area.y {
                    return Some(link);
                }
                current_x += link_width + self.spacing;
            }
            None
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.links.is_empty() {
            return;
        }

        if self.vertical {
            for (i, link) in self.links.iter().enumerate() {
                let y = area.y + i as u16;
                if y >= area.y + area.height {
                    break;
                }
                let link_area = Rect::new(area.x, y, area.width, 1);
                link.render(link_area, buf);
            }
        } else {
            let mut current_x = area.x;
            for link in &self.links {
                let link_width = link.width();
                if current_x >= area.x + area.width {
                    break;
                }
                let link_area = Rect::new(current_x, area.y, link_width.min(area.width), 1);
                link.render(link_area, buf);
                current_x += link_width + self.spacing;
            }
        }
    }
}

impl Default for LinkList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_new() {
        let link = Link::new("https://example.com", "Example");
        assert_eq!(link.get_url(), "https://example.com");
        assert_eq!(link.get_text(), "Example");
        assert!(!link.is_hovered());
    }

    #[test]
    fn test_link_builder() {
        let link = Link::new("url", "text")
            .style(Style::default())
            .hover_style(Style::default());

        assert_eq!(link.get_url(), "url");
        assert_eq!(link.get_text(), "text");
    }

    #[test]
    fn test_link_hover() {
        let mut link = Link::new("url", "text");
        assert!(!link.is_hovered());

        link.set_hovered(true);
        assert!(link.is_hovered());

        link.set_hovered(false);
        assert!(!link.is_hovered());
    }

    #[test]
    fn test_link_osc8_sequence() {
        let link = Link::new("https://example.com", "Click here");
        let seq = link.osc8_sequence();
        assert!(seq.contains("\x1b]8;;"));
        assert!(seq.contains("https://example.com"));
        assert!(seq.contains("Click here"));
        assert!(seq.contains("\x1b\\"));
    }

    #[test]
    fn test_link_width() {
        let link = Link::new("url", "Hello World");
        assert_eq!(link.width(), 11);
    }

    #[test]
    fn test_link_contains_point() {
        let link = Link::new("url", "Hello");
        let area = Rect::new(10, 5, 20, 1);

        assert!(link.contains_point(10, 5, area));
        assert!(link.contains_point(12, 5, area));
        assert!(link.contains_point(14, 5, area));
        assert!(!link.contains_point(15, 5, area));
        assert!(!link.contains_point(10, 6, area));
        assert!(!link.contains_point(9, 5, area));
    }

    #[test]
    fn test_format_link() {
        let link = format_link("https://example.com", "Example");
        assert!(link.contains("\x1b]8;;"));
        assert!(link.contains("https://example.com"));
        assert!(link.contains("Example"));
    }

    #[test]
    fn test_link_with_id() {
        let link = link_with_id("link-1", "https://example.com", "Example");
        assert!(link.contains("id=link-1"));
    }

    #[test]
    fn test_link_list_new() {
        let list = LinkList::new();
        assert!(list.links().is_empty());
    }

    #[test]
    fn test_link_list_add() {
        let list = LinkList::new().add("url1", "Link 1").add("url2", "Link 2");

        assert_eq!(list.links().len(), 2);
        assert_eq!(list.links()[0].get_text(), "Link 1");
        assert_eq!(list.links()[1].get_text(), "Link 2");
    }

    #[test]
    fn test_link_list_spacing() {
        let list = LinkList::new().spacing(2).add("url", "text");

        assert_eq!(list.spacing, 2);
    }

    #[test]
    fn test_link_list_vertical() {
        let list = LinkList::new()
            .vertical(true)
            .add("url1", "Link 1")
            .add("url2", "Link 2");

        assert!(list.vertical);
    }

    #[test]
    fn test_link_list_link_at_horizontal() {
        let list = LinkList::new()
            .add("url1", "AAAAA")
            .add("url2", "BBBBB")
            .spacing(1);

        let area = Rect::new(0, 0, 20, 1);

        let link = list.link_at(2, 0, area);
        assert!(link.is_some());
        assert_eq!(link.unwrap().get_text(), "AAAAA");

        let link = list.link_at(7, 0, area);
        assert!(link.is_some());
        assert_eq!(link.unwrap().get_text(), "BBBBB");

        let link = list.link_at(0, 1, area);
        assert!(link.is_none());
    }

    #[test]
    fn test_link_list_link_at_vertical() {
        let list = LinkList::new()
            .vertical(true)
            .add("url1", "Link 1")
            .add("url2", "Link 2");

        let area = Rect::new(0, 0, 20, 5);

        let link = list.link_at(0, 0, area);
        assert!(link.is_some());
        assert_eq!(link.unwrap().get_text(), "Link 1");

        let link = list.link_at(0, 1, area);
        assert!(link.is_some());
        assert_eq!(link.unwrap().get_text(), "Link 2");

        let link = list.link_at(0, 5, area);
        assert!(link.is_none());
    }

    #[test]
    fn test_link_props() {
        let props = LinkProps::new("https://example.com", "Example");
        let link = Link::create(props);

        assert_eq!(link.get_url(), "https://example.com");
        assert_eq!(link.get_text(), "Example");
    }

    #[test]
    fn test_link_render() {
        let link = Link::new("url", "Hello");
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        link.render(Rect::new(0, 0, 10, 1), &mut buf);

        assert_eq!(buf[(0, 0)].symbol, "H");
        assert_eq!(buf[(1, 0)].symbol, "e");
        assert_eq!(buf[(2, 0)].symbol, "l");
        assert_eq!(buf[(3, 0)].symbol, "l");
        assert_eq!(buf[(4, 0)].symbol, "o");
    }
}
