use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};

const SPARKLINE_CHARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

#[derive(Clone, Debug, Default)]
pub struct Sparkline {
    data: Vec<f64>,
    style: Style,
    max: Option<f64>,
    min: Option<f64>,
}

impl Sparkline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data(mut self, data: Vec<f64>) -> Self {
        self.data = data;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    fn normalize_value(&self, value: f64, min_val: f64, max_val: f64) -> usize {
        if max_val <= min_val {
            return SPARKLINE_CHARS.len() / 2;
        }
        let normalized = (value - min_val) / (max_val - min_val);
        let index = (normalized * (SPARKLINE_CHARS.len() - 1) as f64).round() as usize;
        index.min(SPARKLINE_CHARS.len() - 1)
    }
}

impl Widget for Sparkline {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.data.is_empty() {
            return;
        }

        let min_val = self
            .min
            .unwrap_or_else(|| self.data.iter().cloned().fold(f64::INFINITY, f64::min));
        let max_val = self
            .max
            .unwrap_or_else(|| self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max));

        let data_to_render: Vec<f64> = if self.data.len() > area.width as usize {
            let skip = self.data.len() - area.width as usize;
            self.data.iter().skip(skip).cloned().collect()
        } else {
            self.data.clone()
        };

        let start_x = area.x + (area.width.saturating_sub(data_to_render.len() as u16));

        for (i, value) in data_to_render.iter().enumerate() {
            let char_index = self.normalize_value(*value, min_val, max_val);
            let ch = SPARKLINE_CHARS[char_index];

            let x = start_x + i as u16;
            if x < area.x + area.width {
                buf.modify_cell(x, area.y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(self.style);
                });
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BarSparkline {
    data: Vec<f64>,
    style: Style,
    max: Option<f64>,
}

impl Default for BarSparkline {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            style: Style::default(),
            max: None,
        }
    }
}

impl BarSparkline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data(mut self, data: Vec<f64>) -> Self {
        self.data = data;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }
}

impl Widget for BarSparkline {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.data.is_empty() {
            return;
        }

        let max_val = self
            .max
            .unwrap_or_else(|| self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max));

        if max_val <= 0.0 {
            return;
        }

        let bar_width = (area.width / self.data.len() as u16).max(1);

        for (i, value) in self.data.iter().enumerate() {
            let height_ratio = (value / max_val).min(1.0);
            let bar_height = (height_ratio * area.height as f64).round() as u16;

            let bar_x = area.x + (i as u16 * bar_width).min(area.width - bar_width);

            for y in 0..area.height {
                let buf_y = area.y + area.height - 1 - y;
                let is_filled = y < bar_height;

                for dx in 0..bar_width {
                    let buf_x = bar_x + dx;
                    if buf_x < area.x + area.width {
                        buf.modify_cell(buf_x, buf_y, |cell| {
                            cell.symbol = if is_filled { "█" } else { " " }.to_string();
                            cell.set_style(self.style);
                        });
                    }
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
    fn test_sparkline_empty() {
        let sparkline = Sparkline::new();
        let result = sparkline.render_to_string(10, 1);
        assert_snapshot!("sparkline_empty", result);
    }

    #[test]
    fn test_sparkline_basic() {
        let sparkline = Sparkline::new().data(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = sparkline.render_to_string(10, 1);
        assert_snapshot!("sparkline_basic", result);
    }

    #[test]
    fn test_sparkline_full_range() {
        let sparkline = Sparkline::new().data(vec![0.0, 25.0, 50.0, 75.0, 100.0]);
        let result = sparkline.render_to_string(10, 1);
        assert_snapshot!("sparkline_full_range", result);
    }

    #[test]
    fn test_sparkline_with_bounds() {
        let sparkline = Sparkline::new()
            .data(vec![50.0, 60.0, 70.0, 80.0])
            .min(0.0)
            .max(100.0);
        let result = sparkline.render_to_string(8, 1);
        assert_snapshot!("sparkline_with_bounds", result);
    }

    #[test]
    fn test_bar_sparkline() {
        let sparkline = BarSparkline::new().data(vec![10.0, 30.0, 20.0, 40.0, 15.0]);
        let result = sparkline.render_to_string(15, 5);
        assert_snapshot!("bar_sparkline", result);
    }
}
