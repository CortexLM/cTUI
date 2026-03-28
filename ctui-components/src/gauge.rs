use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};
use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct Gauge {
    value: f64,
    max: f64,
    label: Option<String>,
    style: Style,
    gauge_style: Style,
    start_angle: f64,
    end_angle: f64,
}

impl Default for Gauge {
    fn default() -> Self {
        Self {
            value: 0.0,
            max: 100.0,
            label: None,
            style: Style::default(),
            gauge_style: Style::default(),
            start_angle: -PI * 0.75,
            end_angle: PI * 0.75,
        }
    }
}

impl Gauge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(0.0, self.max);
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = max.max(0.0);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn gauge_style(mut self, style: Style) -> Self {
        self.gauge_style = style;
        self
    }

    pub fn angle_range(mut self, start: f64, end: f64) -> Self {
        self.start_angle = start;
        self.end_angle = end;
        self
    }

    pub fn percent(&self) -> f64 {
        if self.max == 0.0 {
            0.0
        } else {
            self.value / self.max
        }
    }

    fn in_arc(x: f64, y: f64, outer_r: f64, inner_r: f64, start: f64, end: f64) -> bool {
        let dist_sq = x * x + y * y;
        let outer_r_sq = outer_r * outer_r;
        let inner_r_sq = inner_r * inner_r;

        if dist_sq > outer_r_sq || dist_sq < inner_r_sq {
            return false;
        }

        let angle = y.atan2(x);
        let angle = if angle < 0.0 { angle + 2.0 * PI } else { angle };

        let start = if start < 0.0 { start + 2.0 * PI } else { start };
        let end = if end < 0.0 { end + 2.0 * PI } else { end };

        if start <= end {
            angle >= start && angle <= end
        } else {
            angle >= start || angle <= end
        }
    }

    fn angle_in_range(&self, angle: f64, percent: f64) -> bool {
        let range = self.end_angle - self.start_angle;
        let filled_range = range * percent;
        let filled_end = self.start_angle + filled_range;

        let normalized_angle = if angle < 0.0 { angle + 2.0 * PI } else { angle };
        let normalized_start = if self.start_angle < 0.0 {
            self.start_angle + 2.0 * PI
        } else {
            self.start_angle
        };
        let normalized_end = if filled_end < 0.0 {
            filled_end + 2.0 * PI
        } else {
            filled_end
        };

        if normalized_start <= normalized_end {
            normalized_angle >= normalized_start && normalized_angle <= normalized_end
        } else {
            normalized_angle >= normalized_start || normalized_angle <= normalized_end
        }
    }
}

impl Widget for Gauge {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let center_x = area.x + area.width / 2;
        let center_y = area.y + area.height / 2;

        let size = area.width.min(area.height * 2).min(20) as usize;
        let outer_r = size as f64 / 2.0;
        let inner_r = outer_r * 0.6;

        let percent = self.percent();

        for y_offset in 0..=size / 2 {
            for x_offset in 0..=size {
                let dx = (x_offset as f64 - outer_r) / 2.0;
                let dy = y_offset as f64 - outer_r / 2.0;

                let dist = (dx * dx * 4.0 + dy * dy).sqrt();

                if dist <= outer_r && dist >= inner_r * 0.8 {
                    let angle = (dy).atan2(dx * 2.0);

                    let is_filled = self.angle_in_range(angle, percent);
                    let style = if is_filled {
                        self.gauge_style
                    } else {
                        self.style
                    };

                    let buf_x = center_x.saturating_sub((outer_r - x_offset as f64) as u16);
                    let buf_y = center_y.saturating_sub((outer_r / 2.0 - y_offset as f64) as u16);

                    if buf_x >= area.x
                        && buf_x < area.x + area.width
                        && buf_y >= area.y
                        && buf_y < area.y + area.height
                    {
                        buf.modify_cell(buf_x, buf_y, |cell| {
                            cell.symbol = if is_filled { "█" } else { "░" }.to_string();
                            cell.set_style(style);
                        });
                    }

                    let mirror_x = center_x + (x_offset as f64 - outer_r) as u16;
                    if mirror_x < area.x + area.width {
                        buf.modify_cell(mirror_x, buf_y, |cell| {
                            cell.symbol = if is_filled { "█" } else { "░" }.to_string();
                            cell.set_style(style);
                        });
                    }
                }
            }
        }

        let pct_text = format!("{:.0}%", percent * 100.0);
        let label = self.label.as_deref().unwrap_or(&pct_text);
        let label_x = center_x.saturating_sub(label.len() as u16 / 2);
        let label_y = center_y;

        for (i, ch) in label.chars().enumerate() {
            let x = label_x + i as u16;
            if x < area.x + area.width {
                buf.modify_cell(x, label_y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(self.style);
                });
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LinearGauge {
    value: f64,
    max: f64,
    label: Option<String>,
    style: Style,
    filled_style: Style,
    show_percent: bool,
}

impl LinearGauge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn filled_style(mut self, style: Style) -> Self {
        self.filled_style = style;
        self
    }

    pub fn show_percent(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }

    pub fn percent(&self) -> f64 {
        if self.max == 0.0 {
            0.0
        } else {
            self.value / self.max
        }
    }
}

impl Widget for LinearGauge {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let percent = self.percent().clamp(0.0, 1.0);
        let filled_width = ((area.width as f64) * percent) as u16;

        for x in 0..area.width {
            let buf_x = area.x + x;
            buf.modify_cell(buf_x, area.y, |cell| {
                if x < filled_width {
                cell.symbol = "█".to_string();
                cell.set_style(self.filled_style);
                } else {
                cell.symbol = "░".to_string();
                cell.set_style(self.style);
                }
            });
        }

        let label = if self.show_percent {
            format!("{:.0}%", percent * 100.0)
        } else if let Some(ref l) = self.label {
            l.clone()
        } else {
            String::new()
        };

        if !label.is_empty() && area.height > 1 {
            let label_x = area.x + (area.width.saturating_sub(label.len() as u16)) / 2;
            let label_y = area.y + 1;

            for (i, ch) in label.chars().enumerate() {
                let x_pos = label_x + i as u16;
                if x_pos < area.x + area.width {
                    buf.modify_cell(x_pos, label_y, |cell| {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.style);
                    });
                }
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
    fn test_gauge_empty() {
        let gauge = Gauge::new().value(0.0);
        let result = gauge.render_to_string(15, 8);
        assert_snapshot!("gauge_empty", result);
    }

    #[test]
    fn test_gauge_half() {
        let gauge = Gauge::new()
            .value(50.0)
            .max(100.0)
            .gauge_style(Style::new().fg(Color::Green));
        let result = gauge.render_to_string(15, 8);
        assert_snapshot!("gauge_half", result);
    }

    #[test]
    fn test_gauge_full() {
        let gauge = Gauge::new().value(100.0).max(100.0);
        let result = gauge.render_to_string(15, 8);
        assert_snapshot!("gauge_full", result);
    }

    #[test]
    fn test_gauge_with_label() {
        let gauge = Gauge::new().value(75.0).max(100.0).label("Speed");
        let result = gauge.render_to_string(15, 8);
        assert_snapshot!("gauge_with_label", result);
    }

    #[test]
    fn test_linear_gauge() {
        let gauge = LinearGauge::new().value(60.0).max(100.0).show_percent(true);
        let result = gauge.render_to_string(20, 2);
        assert_snapshot!("linear_gauge", result);
    }
}
