use crate::{BorderType, Borders, Padding};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Title {
    content: String,
    alignment: Alignment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitlePosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl Title {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            alignment: Alignment::default(),
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn position(self, position: TitlePosition) -> PositionedTitle {
        PositionedTitle {
            title: self,
            position,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionedTitle {
    title: Title,
    position: TitlePosition,
}

impl PositionedTitle {
    pub fn new(content: impl Into<String>, position: TitlePosition) -> Self {
        Self {
            title: Title::new(content),
            position,
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.title.alignment = alignment;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block {
    borders: Borders,
    border_type: BorderType,
    title: Option<Title>,
    title_bottom: Option<Title>,
    title_left: Option<Title>,
    title_right: Option<Title>,
    padding: Padding,
    style: Style,
}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(Title::new(title));
        self
    }

    pub fn title_with_alignment(mut self, title: impl Into<String>, alignment: Alignment) -> Self {
        self.title = Some(Title::new(title).alignment(alignment));
        self
    }

    pub fn title_bottom(mut self, title: impl Into<String>) -> Self {
        self.title_bottom = Some(Title::new(title));
        self
    }

    pub fn title_bottom_with_alignment(
        mut self,
        title: impl Into<String>,
        alignment: Alignment,
    ) -> Self {
        self.title_bottom = Some(Title::new(title).alignment(alignment));
        self
    }

    pub fn title_left(mut self, title: impl Into<String>) -> Self {
        self.title_left = Some(Title::new(title));
        self
    }

    pub fn title_right(mut self, title: impl Into<String>) -> Self {
        self.title_right = Some(Title::new(title));
        self
    }

    pub fn titles(mut self, titles: impl IntoIterator<Item = PositionedTitle>) -> Self {
        for positioned in titles {
            match positioned.position {
                TitlePosition::Top => self.title = Some(positioned.title),
                TitlePosition::Bottom => self.title_bottom = Some(positioned.title),
                TitlePosition::Left => self.title_left = Some(positioned.title),
                TitlePosition::Right => self.title_right = Some(positioned.title),
            }
        }
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn inner(&self, area: Rect) -> Rect {
        let mut x = area.x;
        let mut y = area.y;
        let mut width = area.width;
        let mut height = area.height;

        if self.borders.contains(Borders::LEFT) && width > 0 {
            x = x.saturating_add(1);
            width = width.saturating_sub(1);
        }
        if self.borders.contains(Borders::RIGHT) && width > 0 {
            width = width.saturating_sub(1);
        }
        if self.borders.contains(Borders::TOP) && height > 0 {
            y = y.saturating_add(1);
            height = height.saturating_sub(1);
        }
        if self.borders.contains(Borders::BOTTOM) && height > 0 {
            height = height.saturating_sub(1);
        }

        width = width
            .saturating_sub(self.padding.left)
            .saturating_sub(self.padding.right);
        height = height
            .saturating_sub(self.padding.top)
            .saturating_sub(self.padding.bottom);
        x = x.saturating_add(self.padding.left);
        y = y.saturating_add(self.padding.top);

        Rect::new(x, y, width, height)
    }

    fn render_top_border(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 {
            return;
        }

        let chars = self.border_type.chars();
        let has_left = self.borders.contains(Borders::LEFT);
        let has_right = self.borders.contains(Borders::RIGHT);

        let left_pos = area.x;
        let right_pos = area.x + area.width.saturating_sub(1);
        let top_y = area.y;

        if has_left {
            buf.modify_cell(left_pos, top_y, |cell| {
                cell.symbol = chars.top_left.to_string();
                cell.set_style(self.style);
            });
        }

        if has_right && area.width > 1 {
            buf.modify_cell(right_pos, top_y, |cell| {
                cell.symbol = chars.top_right.to_string();
                cell.set_style(self.style);
            });
        }

        let start_x = if has_left { left_pos + 1 } else { left_pos };
        let end_x = if has_right { right_pos } else { right_pos + 1 };

        for x in start_x..end_x {
            buf.modify_cell(x, top_y, |cell| {
                cell.symbol = chars.horizontal.to_string();
                cell.set_style(self.style);
            });
        }
    }

    fn render_bottom_border(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let chars = self.border_type.chars();
        let has_left = self.borders.contains(Borders::LEFT);
        let has_right = self.borders.contains(Borders::RIGHT);

        let left_pos = area.x;
        let right_pos = area.x + area.width.saturating_sub(1);
        let bottom_y = area.y + area.height.saturating_sub(1);

        if has_left {
            buf.modify_cell(left_pos, bottom_y, |cell| {
                cell.symbol = chars.bottom_left.to_string();
                cell.set_style(self.style);
            });
        }

        if has_right && area.width > 1 {
            buf.modify_cell(right_pos, bottom_y, |cell| {
                cell.symbol = chars.bottom_right.to_string();
                cell.set_style(self.style);
            });
        }

        let start_x = if has_left { left_pos + 1 } else { left_pos };
        let end_x = if has_right { right_pos } else { right_pos + 1 };

        for x in start_x..end_x {
            buf.modify_cell(x, bottom_y, |cell| {
                cell.symbol = chars.horizontal.to_string();
                cell.set_style(self.style);
            });
        }
    }

    fn render_left_border(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }

        let chars = self.border_type.chars();
        let has_top = self.borders.contains(Borders::TOP);
        let has_bottom = self.borders.contains(Borders::BOTTOM);

        let left_x = area.x;
        let start_y = if has_top { area.y + 1 } else { area.y };
        let end_y = if has_bottom {
            area.y + area.height.saturating_sub(1)
        } else {
            area.y + area.height
        };

        for y in start_y..end_y {
            buf.modify_cell(left_x, y, |cell| {
                cell.symbol = chars.vertical.to_string();
                cell.set_style(self.style);
            });
        }
    }

    fn render_right_border(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let chars = self.border_type.chars();
        let has_top = self.borders.contains(Borders::TOP);
        let has_bottom = self.borders.contains(Borders::BOTTOM);

        let right_x = area.x + area.width.saturating_sub(1);
        let start_y = if has_top { area.y + 1 } else { area.y };
        let end_y = if has_bottom {
            area.y + area.height.saturating_sub(1)
        } else {
            area.y + area.height
        };

        for y in start_y..end_y {
            buf.modify_cell(right_x, y, |cell| {
                cell.symbol = chars.vertical.to_string();
                cell.set_style(self.style);
            });
        }
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let title = match &self.title {
            Some(t) => t,
            None => return,
        };

        if area.width <= 2 {
            return;
        }

        let has_left = self.borders.contains(Borders::LEFT);
        let has_right = self.borders.contains(Borders::RIGHT);

        let start_x = if has_left { area.x + 1 } else { area.x };
        let end_x = if has_right {
            area.x + area.width.saturating_sub(1)
        } else {
            area.x + area.width
        };

        let available_width = end_x.saturating_sub(start_x) as usize;
        if available_width == 0 {
            return;
        }

        let title_text = format!(" {} ", title.content);
        let title_len = title_text.chars().count();

        let title_x = match title.alignment {
            Alignment::Left => start_x,
            Alignment::Right => end_x.saturating_sub(title_len as u16),
            Alignment::Center => start_x + (available_width.saturating_sub(title_len) / 2) as u16,
        };

        for (i, ch) in title_text.chars().enumerate() {
            let x = title_x + i as u16;
            if x >= end_x {
                break;
            }
            buf.modify_cell(x, area.y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }
    }

    fn render_title_bottom(&self, area: Rect, buf: &mut Buffer) {
        let title = match &self.title_bottom {
            Some(t) => t,
            None => return,
        };

        if area.width <= 2 || area.height == 0 {
            return;
        }

        let has_left = self.borders.contains(Borders::LEFT);
        let has_right = self.borders.contains(Borders::RIGHT);
        let bottom_y = area.y + area.height.saturating_sub(1);

        let start_x = if has_left { area.x + 1 } else { area.x };
        let end_x = if has_right {
            area.x + area.width.saturating_sub(1)
        } else {
            area.x + area.width
        };

        let available_width = end_x.saturating_sub(start_x) as usize;
        if available_width == 0 {
            return;
        }

        let title_text = format!(" {} ", title.content);
        let title_len = title_text.chars().count();

        let title_x = match title.alignment {
            Alignment::Left => start_x,
            Alignment::Right => end_x.saturating_sub(title_len as u16),
            Alignment::Center => start_x + (available_width.saturating_sub(title_len) / 2) as u16,
        };

        for (i, ch) in title_text.chars().enumerate() {
            let x = title_x + i as u16;
            if x >= end_x {
                break;
            }
            buf.modify_cell(x, bottom_y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }
    }

    fn render_title_left(&self, area: Rect, buf: &mut Buffer) {
        let title = match &self.title_left {
            Some(t) => t,
            None => return,
        };

        if area.height <= 2 {
            return;
        }

        let has_top = self.borders.contains(Borders::TOP);
        let has_bottom = self.borders.contains(Borders::BOTTOM);

        let start_y = if has_top { area.y + 1 } else { area.y };
        let end_y = if has_bottom {
            area.y + area.height.saturating_sub(1)
        } else {
            area.y + area.height
        };

        let available_height = end_y.saturating_sub(start_y) as usize;
        if available_height == 0 {
            return;
        }

        let title_text = title.content.clone();
        let title_len = title_text.chars().count();

        let start_offset = match title.alignment {
            Alignment::Left => 0,
            Alignment::Right => available_height.saturating_sub(title_len),
            Alignment::Center => (available_height.saturating_sub(title_len)) / 2,
        };

        for (i, ch) in title_text.chars().enumerate() {
            let y = start_y + start_offset as u16 + i as u16;
            if y >= end_y {
                break;
            }
            buf.modify_cell(area.x, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }
    }

    fn render_title_right(&self, area: Rect, buf: &mut Buffer) {
        let title = match &self.title_right {
            Some(t) => t,
            None => return,
        };

        if area.width == 0 || area.height <= 2 {
            return;
        }

        let has_top = self.borders.contains(Borders::TOP);
        let has_bottom = self.borders.contains(Borders::BOTTOM);
        let right_x = area.x + area.width.saturating_sub(1);

        let start_y = if has_top { area.y + 1 } else { area.y };
        let end_y = if has_bottom {
            area.y + area.height.saturating_sub(1)
        } else {
            area.y + area.height
        };

        let available_height = end_y.saturating_sub(start_y) as usize;
        if available_height == 0 {
            return;
        }

        let title_text = title.content.clone();
        let title_len = title_text.chars().count();

        let start_offset = match title.alignment {
            Alignment::Left => 0,
            Alignment::Right => available_height.saturating_sub(title_len),
            Alignment::Center => (available_height.saturating_sub(title_len)) / 2,
        };

        for (i, ch) in title_text.chars().enumerate() {
            let y = start_y + start_offset as u16 + i as u16;
            if y >= end_y {
                break;
            }
            buf.modify_cell(right_x, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            });
        }
    }
}

impl Component for Block {
    type Props = ();
    type State = ();

    fn create(_props: Self::Props) -> Self {
        Self::new()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        if self.borders.contains(Borders::TOP) {
            self.render_top_border(area, buf);
            if self.title.is_some() {
                self.render_title(area, buf);
            }
        }

        if self.borders.contains(Borders::BOTTOM) {
            self.render_bottom_border(area, buf);
            if self.title_bottom.is_some() {
                self.render_title_bottom(area, buf);
            }
        }

        if self.borders.contains(Borders::LEFT) {
            self.render_left_border(area, buf);
            if self.title_left.is_some() {
                self.render_title_left(area, buf);
            }
        }

        if self.borders.contains(Borders::RIGHT) {
            self.render_right_border(area, buf);
            if self.title_right.is_some() {
                self.render_title_right(area, buf);
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
    use ctui_core::Buffer;
    use insta::assert_snapshot;

    fn render_to_string(block: &Block, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        block.render(Rect::new(0, 0, width, height), &mut buf);

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
    fn test_no_borders() {
        let block = Block::new();
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("no_borders", result);
    }

    #[test]
    fn test_border_all_plain() {
        let block = Block::new().borders(Borders::ALL);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_all_plain", result);
    }

    #[test]
    fn test_border_all_rounded() {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_all_rounded", result);
    }

    #[test]
    fn test_border_all_double() {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_all_double", result);
    }

    #[test]
    fn test_border_all_thick() {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_all_thick", result);
    }

    #[test]
    fn test_border_left_only() {
        let block = Block::new().borders(Borders::LEFT);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_left_only", result);
    }

    #[test]
    fn test_border_right_only() {
        let block = Block::new().borders(Borders::RIGHT);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_right_only", result);
    }

    #[test]
    fn test_border_top_only() {
        let block = Block::new().borders(Borders::TOP);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_top_only", result);
    }

    #[test]
    fn test_border_bottom_only() {
        let block = Block::new().borders(Borders::BOTTOM);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_bottom_only", result);
    }

    #[test]
    fn test_border_top_bottom() {
        let block = Block::new().borders(Borders::TOP | Borders::BOTTOM);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_top_bottom", result);
    }

    #[test]
    fn test_border_left_right() {
        let block = Block::new().borders(Borders::LEFT | Borders::RIGHT);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_left_right", result);
    }

    #[test]
    fn test_border_with_title_left() {
        let block = Block::new().borders(Borders::ALL).title("Title");
        let result = render_to_string(&block, 15, 5);
        assert_snapshot!("border_with_title_left", result);
    }

    #[test]
    fn test_border_with_title_center() {
        let block = Block::new()
            .borders(Borders::ALL)
            .title_with_alignment("Title", Alignment::Center);
        let result = render_to_string(&block, 15, 5);
        assert_snapshot!("border_with_title_center", result);
    }

    #[test]
    fn test_border_with_title_right() {
        let block = Block::new()
            .borders(Borders::ALL)
            .title_with_alignment("Title", Alignment::Right);
        let result = render_to_string(&block, 15, 5);
        assert_snapshot!("border_with_title_right", result);
    }

    #[test]
    fn test_border_with_padding() {
        let block = Block::new()
            .borders(Borders::ALL)
            .padding(Padding::uniform(1));
        let inner = block.inner(Rect::new(0, 0, 10, 5));
        assert_eq!(inner, Rect::new(2, 2, 6, 1));
    }

    #[test]
    fn test_border_with_asymmetric_padding() {
        let block = Block::new()
            .borders(Borders::ALL)
            .padding(Padding::new(2, 1, 1, 2));
        let inner = block.inner(Rect::new(0, 0, 10, 5));
        assert_eq!(inner, Rect::new(3, 2, 5, 0));
    }

    #[test]
    fn test_inner_area_no_border() {
        let block = Block::new();
        let inner = block.inner(Rect::new(0, 0, 10, 5));
        assert_eq!(inner, Rect::new(0, 0, 10, 5));
    }

    #[test]
    fn test_inner_area_all_borders() {
        let block = Block::new().borders(Borders::ALL);
        let inner = block.inner(Rect::new(0, 0, 10, 5));
        assert_eq!(inner, Rect::new(1, 1, 8, 3));
    }

    #[test]
    fn test_border_combinations() {
        let block = Block::new().borders(Borders::TOP | Borders::LEFT);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_top_left", result);
    }

    #[test]
    fn test_small_area() {
        let block = Block::new().borders(Borders::ALL);
        let result = render_to_string(&block, 3, 3);
        assert_snapshot!("small_area", result);
    }

    #[test]
    fn test_minimal_area() {
        let block = Block::new().borders(Borders::ALL);
        let result = render_to_string(&block, 2, 2);
        assert_snapshot!("minimal_area", result);
    }

    #[test]
    fn test_empty_area() {
        let block = Block::new().borders(Borders::ALL);
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 1));
        block.render(Rect::new(0, 0, 0, 0), &mut buf);
        assert_eq!(buf.get(0, 0).unwrap().symbol, " ");
    }

    #[test]
    fn test_border_with_title_bottom() {
        let block = Block::new()
            .borders(Borders::ALL)
            .title_bottom("Bottom")
            .title("Top");
        let result = render_to_string(&block, 15, 5);
        assert_snapshot!("border_with_title_bottom", result);
    }

    #[test]
    fn test_border_with_title_bottom_center() {
        let block = Block::new()
            .borders(Borders::ALL)
            .title_bottom_with_alignment("Bottom", Alignment::Center);
        let result = render_to_string(&block, 15, 5);
        assert_snapshot!("border_with_title_bottom_center", result);
    }

    #[test]
    fn test_title_on_left_border() {
        let block = Block::new().borders(Borders::ALL).title_left("Left");
        let result = render_to_string(&block, 10, 8);
        assert_snapshot!("border_title_on_left", result);
    }

    #[test]
    fn test_title_on_right_border() {
        let block = Block::new().borders(Borders::ALL).title_right("Right");
        let result = render_to_string(&block, 10, 8);
        assert_snapshot!("border_title_on_right", result);
    }

    #[test]
    fn test_titles_on_all_sides() {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Top")
            .title_bottom("Bottom")
            .title_left("L")
            .title_right("R");
        let result = render_to_string(&block, 15, 10);
        assert_snapshot!("border_titles_all_sides", result);
    }

    #[test]
    fn test_positioned_titles() {
        let block = Block::new().borders(Borders::ALL).titles([
            Title::new("T").position(TitlePosition::Top),
            Title::new("B").position(TitlePosition::Bottom),
        ]);
        let result = render_to_string(&block, 10, 5);
        assert_snapshot!("border_positioned_titles", result);
    }
}
