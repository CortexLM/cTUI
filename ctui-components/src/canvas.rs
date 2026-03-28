use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub enum Shape {
    Line {
        start: Point,
        end: Point,
        style: Style,
    },
    Rectangle {
        top_left: Point,
        bottom_right: Point,
        style: Style,
        filled: bool,
    },
    Circle {
        center: Point,
        radius: f64,
        style: Style,
        filled: bool,
    },
}

impl Shape {
    pub fn line(start: Point, end: Point) -> Self {
        Shape::Line {
            start,
            end,
            style: Style::default(),
        }
    }

    pub fn line_styled(start: Point, end: Point, style: Style) -> Self {
        Shape::Line { start, end, style }
    }

    pub fn rectangle(top_left: Point, bottom_right: Point) -> Self {
        Shape::Rectangle {
            top_left,
            bottom_right,
            style: Style::default(),
            filled: false,
        }
    }

    pub fn rectangle_filled(top_left: Point, bottom_right: Point) -> Self {
        Shape::Rectangle {
            top_left,
            bottom_right,
            style: Style::default(),
            filled: true,
        }
    }

    pub fn rectangle_styled(
        top_left: Point,
        bottom_right: Point,
        style: Style,
        filled: bool,
    ) -> Self {
        Shape::Rectangle {
            top_left,
            bottom_right,
            style,
            filled,
        }
    }

    pub fn circle(center: Point, radius: f64) -> Self {
        Shape::Circle {
            center,
            radius,
            style: Style::default(),
            filled: false,
        }
    }

    pub fn circle_filled(center: Point, radius: f64) -> Self {
        Shape::Circle {
            center,
            radius,
            style: Style::default(),
            filled: true,
        }
    }

    pub fn circle_styled(center: Point, radius: f64, style: Style, filled: bool) -> Self {
        Shape::Circle {
            center,
            radius,
            style,
            filled,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Canvas {
    shapes: Vec<Shape>,
    background: Option<char>,
    background_style: Style,
}

impl Canvas {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shapes.push(shape);
        self
    }

    pub fn line(mut self, start: Point, end: Point) -> Self {
        self.shapes.push(Shape::line(start, end));
        self
    }

    pub fn rectangle(mut self, top_left: Point, bottom_right: Point) -> Self {
        self.shapes.push(Shape::rectangle(top_left, bottom_right));
        self
    }

    pub fn circle(mut self, center: Point, radius: f64) -> Self {
        self.shapes.push(Shape::circle(center, radius));
        self
    }

    pub fn background(mut self, ch: char, style: Style) -> Self {
        self.background = Some(ch);
        self.background_style = style;
        self
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    fn draw_line(&self, start: &Point, end: &Point, style: &Style, area: Rect, buf: &mut Buffer) {
        let x0 = (start.x * area.width as f64) as i32;
        let y0 = (start.y * area.height as f64) as i32;
        let x1 = (end.x * area.width as f64) as i32;
        let y1 = (end.y * area.height as f64) as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            let buf_x = area.x + x.clamp(0, area.width as i32 - 1) as u16;
            let buf_y = area.y + y.clamp(0, area.height as i32 - 1) as u16;

            buf.modify_cell(buf_x, buf_y, |cell| {
                cell.symbol = "█".to_string();
                cell.set_style(*style);
            });

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_rectangle(
        &self,
        tl: &Point,
        br: &Point,
        style: &Style,
        filled: bool,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let x0 = (tl.x * area.width as f64) as u16;
        let y0 = (tl.y * area.height as f64) as u16;
        let x1 = (br.x * area.width as f64).min(area.width as f64 - 1.0) as u16;
        let y1 = (br.y * area.height as f64).min(area.height as f64 - 1.0) as u16;

        if filled {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    let buf_x = area.x + x;
                    let buf_y = area.y + y;
                    buf.modify_cell(buf_x, buf_y, |cell| {
                        cell.symbol = "█".to_string();
                        cell.set_style(*style);
                    });
                }
            }
        } else {
            for x in x0..=x1 {
                buf.modify_cell(area.x + x, area.y + y0, |cell| {
                    cell.symbol = "─".to_string();
                    cell.set_style(*style);
                });
                if y0 != y1 {
                    buf.modify_cell(area.x + x, area.y + y1, |cell| {
                        cell.symbol = "─".to_string();
                        cell.set_style(*style);
                    });
                }
            }
            for y in y0..=y1 {
                buf.modify_cell(area.x + x0, area.y + y, |cell| {
                    cell.symbol = "│".to_string();
                    cell.set_style(*style);
                });
                if x0 != x1 {
                    buf.modify_cell(area.x + x1, area.y + y, |cell| {
                        cell.symbol = "│".to_string();
                        cell.set_style(*style);
                    });
                }
            }
            buf.modify_cell(area.x + x0, area.y + y0, |cell| { cell.symbol = "┌".to_string(); });
            buf.modify_cell(area.x + x1, area.y + y0, |cell| { cell.symbol = "┐".to_string(); });
            buf.modify_cell(area.x + x0, area.y + y1, |cell| { cell.symbol = "└".to_string(); });
            buf.modify_cell(area.x + x1, area.y + y1, |cell| { cell.symbol = "┘".to_string(); });
        }
    }

    fn draw_circle(
        &self,
        center: &Point,
        radius: f64,
        style: &Style,
        filled: bool,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let cx = (center.x * area.width as f64) as i32;
        let cy = (center.y * area.height as f64) as i32;
        let r = (radius * area.width.min(area.height) as f64) as i32;

        if filled {
            for y in cy - r..=cy + r {
                for x in cx - r..=cx + r {
                    let dx = x - cx;
                    let dy = y - cy;
                    if dx * dx + dy * dy <= r * r {
                        let buf_x = area.x + x.clamp(0, area.width as i32 - 1) as u16;
                        let buf_y = area.y + y.clamp(0, area.height as i32 - 1) as u16;
                        buf.modify_cell(buf_x, buf_y, |cell| {
                            cell.symbol = "█".to_string();
                            cell.set_style(*style);
                        });
                    }
                }
            }
        } else {
            let mut x = r;
            let mut y = 0;
            let mut err = 0;

            while x >= y {
                for (dx, dy) in [
                    (x, y),
                    (y, x),
                    (-y, x),
                    (-x, y),
                    (-x, -y),
                    (-y, -x),
                    (y, -x),
                    (x, -y),
                ] {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && py >= 0 {
                        let buf_x = area.x + (px as u16).min(area.width - 1);
                        let buf_y = area.y + (py as u16).min(area.height - 1);
                        buf.modify_cell(buf_x, buf_y, |cell| {
                            cell.symbol = "●".to_string();
                            cell.set_style(*style);
                        });
                    }
                }

                y += 1;
                err += 1 + 2 * y;
                if 2 * (err - x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2 * x;
                }
            }
        }
    }
}

impl Widget for Canvas {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        if let Some(ch) = self.background {
            for y in area.y..area.y + area.height {
                for x in area.x..area.x + area.width {
                    buf.modify_cell(x, y, |cell| {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.background_style);
                    });
                }
            }
        }

        for shape in &self.shapes {
            match shape {
                Shape::Line { start, end, style } => {
                    self.draw_line(start, end, style, area, buf);
                }
                Shape::Rectangle {
                    top_left,
                    bottom_right,
                    style,
                    filled,
                } => {
                    self.draw_rectangle(top_left, bottom_right, style, *filled, area, buf);
                }
                Shape::Circle {
                    center,
                    radius,
                    style,
                    filled,
                } => {
                    self.draw_circle(center, *radius, style, *filled, area, buf);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WidgetExt;
    use insta::assert_snapshot;

    #[test]
    fn test_canvas_line() {
        let canvas = Canvas::new().line(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let result = canvas.render_to_string(10, 10);
        assert_snapshot!("canvas_line", result);
    }

    #[test]
    fn test_canvas_rectangle() {
        let canvas = Canvas::new().rectangle(Point::new(0.1, 0.1), Point::new(0.9, 0.9));
        let result = canvas.render_to_string(15, 10);
        assert_snapshot!("canvas_rectangle", result);
    }

    #[test]
    fn test_canvas_circle() {
        let canvas = Canvas::new().circle(Point::new(0.5, 0.5), 0.4);
        let result = canvas.render_to_string(15, 15);
        assert_snapshot!("canvas_circle", result);
    }

    #[test]
    fn test_canvas_multiple_shapes() {
        let canvas = Canvas::new()
            .rectangle(Point::new(0.0, 0.0), Point::new(1.0, 1.0))
            .line(Point::new(0.0, 0.0), Point::new(1.0, 1.0))
            .circle(Point::new(0.5, 0.5), 0.2);
        let result = canvas.render_to_string(20, 15);
        assert_snapshot!("canvas_multiple", result);
    }
}
