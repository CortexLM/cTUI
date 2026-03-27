use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};

#[derive(Clone, Debug, Default)]
pub struct Slider {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    style: Style,
    track_style: Style,
    thumb_style: Style,
    label: Option<String>,
    show_value: bool,
    orientation: Orientation,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Slider {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            style: Style::default(),
            track_style: Style::default(),
            thumb_style: Style::default(),
            label: None,
            show_value: false,
            orientation: Orientation::Horizontal,
        }
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = style;
        self
    }

    pub fn thumb_style(mut self, style: Style) -> Self {
        self.thumb_style = style;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(self.min, self.max);
    }

    pub fn increment(&mut self) {
        self.value = (self.value + self.step).min(self.max);
    }

    pub fn decrement(&mut self) {
        self.value = (self.value - self.step).max(self.min);
    }

    pub fn percent(&self) -> f64 {
        if self.max <= self.min {
            return 0.0;
        }
        (self.value - self.min) / (self.max - self.min)
    }

    pub fn set_percent(&mut self, percent: f64) {
        self.value = self.min + percent * (self.max - self.min);
        self.value = self.value.clamp(self.min, self.max);
    }
}

impl Widget for Slider {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let percent = self.percent();

        if self.orientation == Orientation::Horizontal {
            let filled_width = ((area.width as f64) * percent) as u16;
            let thumb_pos = filled_width.min(area.width - 1);

            for x in 0..area.width {
                let buf_x = area.x + x;
                if let Some(cell) = buf.get_mut(buf_x, area.y) {
                    if x == thumb_pos {
                        cell.symbol = "●".to_string();
                        cell.set_style(self.thumb_style);
                    } else if x < filled_width {
                        cell.symbol = "█".to_string();
                        cell.set_style(self.style);
                    } else {
                        cell.symbol = "░".to_string();
                        cell.set_style(self.track_style);
                    }
                }
            }

            if self.show_value || self.label.is_some() {
                let display = if self.show_value {
                    format!("{:.1}", self.value)
                } else if let Some(ref label) = self.label {
                    format!("{}: {:.1}", label, self.value)
                } else {
                    String::new()
                };

                if !display.is_empty() && area.height > 1 {
                    let label_x = area.x + (area.width.saturating_sub(display.len() as u16)) / 2;
                    let label_y = area.y + 1;

                    for (i, ch) in display.chars().enumerate() {
                        let x_pos = label_x + i as u16;
                        if x_pos < area.x + area.width {
                            if let Some(cell) = buf.get_mut(x_pos, label_y) {
                                cell.symbol = ch.to_string();
                                cell.set_style(self.style);
                            }
                        }
                    }
                }
            }
        } else {
            let filled_height = ((area.height as f64) * percent) as u16;
            let thumb_pos = area.height - 1 - filled_height.min(area.height - 1);

            for y in 0..area.height {
                let buf_y = area.y + y;
                if let Some(cell) = buf.get_mut(area.x, buf_y) {
                    if y == thumb_pos {
                        cell.symbol = "●".to_string();
                        cell.set_style(self.thumb_style);
                    } else if y > thumb_pos {
                        cell.symbol = "█".to_string();
                        cell.set_style(self.style);
                    } else {
                        cell.symbol = "░".to_string();
                        cell.set_style(self.track_style);
                    }
                }
            }

            if area.width > 1 {
                for y in 0..area.height {
                    let buf_y = area.y + y;
                    for x in 1..area.width.min(3) {
                        if let Some(cell) = buf.get_mut(area.x + x, buf_y) {
                            cell.symbol = "│".to_string();
                            cell.set_style(self.track_style);
                        }
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
    use ctui_core::style::Color;
    use insta::assert_snapshot;

    #[test]
    fn test_slider_zero() {
        let slider = Slider::new().value(0.0).min(0.0).max(100.0);
        let result = slider.render_to_string(20, 1);
        assert_snapshot!("slider_zero", result);
    }

    #[test]
    fn test_slider_half() {
        let slider = Slider::new()
            .value(50.0)
            .min(0.0)
            .max(100.0)
            .style(Style::new().fg(Color::Blue));
        let result = slider.render_to_string(20, 1);
        assert_snapshot!("slider_half", result);
    }

    #[test]
    fn test_slider_full() {
        let slider = Slider::new().value(100.0).min(0.0).max(100.0);
        let result = slider.render_to_string(20, 1);
        assert_snapshot!("slider_full", result);
    }

    #[test]
    fn test_slider_with_value() {
        let slider = Slider::new().value(75.0).show_value(true);
        let result = slider.render_to_string(20, 2);
        assert_snapshot!("slider_with_value", result);
    }

    #[test]
    fn test_slider_with_label() {
        let slider = Slider::new().value(42.0).label("Volume").show_value(true);
        let result = slider.render_to_string(20, 2);
        assert_snapshot!("slider_with_label", result);
    }

    #[test]
    fn test_slider_vertical() {
        let slider = Slider::new().value(60.0).vertical();
        let result = slider.render_to_string(3, 10);
        assert_snapshot!("slider_vertical", result);
    }

    #[test]
    fn test_slider_navigation() {
        let mut slider = Slider::new().value(50.0).min(0.0).max(100.0).step(10.0);

        slider.increment();
        assert_eq!(slider.get_value(), 60.0);

        slider.increment();
        slider.increment();
        slider.increment();
        slider.increment();
        slider.increment();
        assert_eq!(slider.get_value(), 100.0);

        slider.increment();
        assert_eq!(slider.get_value(), 100.0);

        slider.decrement();
        assert_eq!(slider.get_value(), 90.0);
    }

    #[test]
    fn test_slider_percent() {
        let slider = Slider::new().value(25.0).min(0.0).max(100.0);
        assert_eq!(slider.percent(), 0.25);

        let slider = Slider::new().value(50.0).min(0.0).max(200.0);
        assert_eq!(slider.percent(), 0.25);

        let mut slider = Slider::new().min(0.0).max(100.0);
        slider.set_percent(0.75);
        assert_eq!(slider.get_value(), 75.0);
    }
}
