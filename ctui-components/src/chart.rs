//! ASCII chart component for data visualization.
//!
//! Renders data as ASCII bar charts in the terminal.

use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

#[derive(Clone, Debug)]
pub struct DataPoint {
    pub label: String,
    pub value: f64,
    pub style: Option<Style>,
}

impl DataPoint {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            style: None,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum ChartType {
    #[default]
    Bar,
    Line,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum BarOrientation {
    Horizontal,
    Vertical,
}

impl Default for BarOrientation {
    fn default() -> Self {
        BarOrientation::Vertical
    }
}

const BAR_CHARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

#[derive(Clone, Debug)]
pub struct Chart {
    data: Vec<DataPoint>,
    chart_type: ChartType,
    orientation: BarOrientation,
    style: Style,
    show_labels: bool,
    show_values: bool,
    max_value: Option<f64>,
    min_value: Option<f64>,
}

impl Default for Chart {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            chart_type: ChartType::default(),
            orientation: BarOrientation::default(),
            style: Style::default(),
            show_labels: true,
            show_values: false,
            max_value: None,
            min_value: None,
        }
    }
}

impl Chart {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data(mut self, data: Vec<DataPoint>) -> Self {
        self.data = data;
        self
    }

    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    pub fn max_value(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }

    pub fn min_value(mut self, min: f64) -> Self {
        self.min_value = Some(min);
        self
    }

    pub fn data_ref(&self) -> &[DataPoint] {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<DataPoint>) {
        self.data = data;
    }

    fn get_value_range(&self) -> (f64, f64) {
        if self.data.is_empty() {
            return (0.0, 1.0);
        }

        let min = self.min_value.unwrap_or_else(|| {
            self.data
                .iter()
                .map(|d| d.value)
                .fold(f64::INFINITY, f64::min)
        });

        let max = self.max_value.unwrap_or_else(|| {
            self.data
                .iter()
                .map(|d| d.value)
                .fold(f64::NEG_INFINITY, f64::max)
        });

        let min = if min == max { min - 1.0 } else { min };
        (min, max)
    }

    fn value_to_bar_height(
        &self,
        value: f64,
        min: f64,
        max: f64,
        available_height: usize,
    ) -> usize {
        if max <= min {
            return 0;
        }
        let normalized = (value - min) / (max - min);
        (normalized * available_height as f64).round() as usize
    }

    fn render_horizontal_bars(&self, area: Rect, buf: &mut Buffer) {
        if self.data.is_empty() || area.height == 0 {
            return;
        }

        let (min, max) = self.get_value_range();
        let bar_width = if self.show_labels {
            area.width.saturating_sub(10)
        } else {
            area.width
        };

        for (i, point) in self.data.iter().enumerate() {
            let y = area.y + i as u16;
            if y >= area.y + area.height {
                break;
            }

            let label_width = if self.show_labels {
                let label: Vec<char> = point.label.chars().take(8).collect();
                for (j, ch) in label.iter().enumerate() {
                    if let Some(cell) = buf.get_mut(area.x + j as u16, y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.style);
                    }
                }
                label.len() + 1
            } else {
                0
            };

            let normalized = if max > min {
                (point.value - min) / (max - min)
            } else {
                0.0
            };

            let filled_width = (normalized * bar_width as f64).round() as usize;

            for x in label_width..(label_width + filled_width).min(bar_width as usize) {
                if let Some(cell) = buf.get_mut(area.x + x as u16, y) {
                    cell.symbol = '█'.to_string();
                    cell.set_style(point.style.unwrap_or(self.style));
                }
            }

            if self.show_values && filled_width < bar_width as usize {
                let value_str = format!("{:.1}", point.value);
                for (j, ch) in value_str.chars().enumerate() {
                    let x_pos = label_width + filled_width + j;
                    if x_pos >= bar_width as usize {
                        break;
                    }
                    if let Some(cell) = buf.get_mut(area.x + x_pos as u16, y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.style);
                    }
                }
            }
        }
    }

    fn render_vertical_bars(&self, area: Rect, buf: &mut Buffer) {
        if self.data.is_empty() || area.height == 0 {
            return;
        }

        let (min, max) = self.get_value_range();
        let chart_height = if self.show_labels {
            area.height.saturating_sub(1)
        } else {
            area.height
        };
        let bar_width = (area.width as usize / self.data.len())
            .max(1)
            .saturating_sub(1);
        let spacing = 1;

        for (i, point) in self.data.iter().enumerate() {
            let bar_height = self.value_to_bar_height(point.value, min, max, chart_height as usize);
            let x_start = area.x + (i * (bar_width + spacing)) as u16;

            if x_start >= area.x + area.width {
                break;
            }

            for row in 0..bar_height.min(chart_height as usize) {
                let y = area.y + chart_height - 1 - row as u16;
                for col in 0..bar_width {
                    let x = x_start + col as u16;
                    if x < area.x + area.width {
                        if let Some(cell) = buf.get_mut(x, y) {
                            cell.symbol = '█'.to_string();
                            cell.set_style(point.style.unwrap_or(self.style));
                        }
                    }
                }
            }

            if self.show_labels && chart_height < area.height {
                let label: Vec<char> = point.label.chars().take(bar_width).collect();
                for (j, ch) in label.iter().enumerate() {
                    if j >= bar_width {
                        break;
                    }
                    if let Some(cell) = buf.get_mut(x_start + j as u16, area.y + area.height - 1) {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.style);
                    }
                }
            }
        }
    }

    fn render_line_chart(&self, area: Rect, buf: &mut Buffer) {
        if self.data.len() < 2 || area.width < 2 {
            return;
        }

        let (min, max) = self.get_value_range();
        let chart_height = area.height;
        let data_len = self.data.len();

        for i in 0..(data_len - 1) {
            let x1 = area.x + (i * (area.width as usize - 1) / (data_len - 1)) as u16;
            let x2 = area.x + ((i + 1) * (area.width as usize - 1) / (data_len - 1)) as u16;

            let v1 = self.data[i].value;
            let v2 = self.data[i + 1].value;

            let bar_h1 = self.value_to_bar_height(v1, min, max, chart_height as usize) as u16;
            let bar_h2 = self.value_to_bar_height(v2, min, max, chart_height as usize) as u16;

            let y1 = area.y + chart_height.saturating_sub(1).saturating_sub(bar_h1);
            let y2 = area.y + chart_height.saturating_sub(1).saturating_sub(bar_h2);

            if let Some(cell) = buf.get_mut(x1, y1) {
                cell.symbol = '●'.to_string();
                cell.set_style(self.style);
            }

            let dx = (x2 as i32 - x1 as i32).abs();
            let dy = (y2 as i32 - y1 as i32).abs();
            let steps = dx.max(dy).max(1);

            for step in 1..=steps {
                let t = step as f64 / steps as f64;
                let x = (x1 as f64 + (x2 as f64 - x1 as f64) * t).round() as u16;
                let y = (y1 as f64 + (y2 as f64 - y1 as f64) * t).round() as u16;

                if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
                {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.symbol = '·'.to_string();
                        cell.set_style(self.style);
                    }
                }
            }
        }

        if let Some(last) = self.data.last() {
            let x = area.x + area.width.saturating_sub(1);
            let bar_h =
                self.value_to_bar_height(last.value, min, max, chart_height as usize) as u16;
            let y = area.y + chart_height.saturating_sub(1).saturating_sub(bar_h);

            if let Some(cell) = buf.get_mut(x, y) {
                cell.symbol = '●'.to_string();
                cell.set_style(self.style);
            }
        }
    }
}

pub struct ChartProps {
    pub data: Vec<DataPoint>,
    pub chart_type: ChartType,
    pub orientation: BarOrientation,
    pub style: Style,
    pub show_labels: bool,
    pub show_values: bool,
    pub max_value: Option<f64>,
    pub min_value: Option<f64>,
}

impl ChartProps {
    pub fn new(data: Vec<DataPoint>) -> Self {
        Self {
            data,
            chart_type: ChartType::default(),
            orientation: BarOrientation::default(),
            style: Style::default(),
            show_labels: true,
            show_values: false,
            max_value: None,
            min_value: None,
        }
    }

    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    pub fn max_value(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }

    pub fn min_value(mut self, min: f64) -> Self {
        self.min_value = Some(min);
        self
    }
}

impl Component for Chart {
    type Props = ChartProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            data: props.data,
            chart_type: props.chart_type,
            orientation: props.orientation,
            style: props.style,
            show_labels: props.show_labels,
            show_values: props.show_values,
            max_value: props.max_value,
            min_value: props.min_value,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.data.is_empty() {
            return;
        }

        match self.chart_type {
            ChartType::Bar => match self.orientation {
                BarOrientation::Horizontal => self.render_horizontal_bars(area, buf),
                BarOrientation::Vertical => self.render_vertical_bars(area, buf),
            },
            ChartType::Line => self.render_line_chart(area, buf),
        }
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

    fn render_to_string(chart: &Chart, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        chart.render(Rect::new(0, 0, width, height), &mut buf);

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
    fn snapshot_chart_vertical_bars() {
        let data = vec![
            DataPoint::new("A", 30.0),
            DataPoint::new("B", 60.0),
            DataPoint::new("C", 90.0),
            DataPoint::new("D", 45.0),
        ];
        let chart = Chart::new()
            .data(data)
            .orientation(BarOrientation::Vertical);
        let result = render_to_string(&chart, 20, 10);
        assert_snapshot!("chart_vertical_bars", result);
    }

    #[test]
    fn snapshot_chart_horizontal_bars() {
        let data = vec![
            DataPoint::new("ItemA", 30.0),
            DataPoint::new("ItemB", 60.0),
            DataPoint::new("ItemC", 90.0),
        ];
        let chart = Chart::new()
            .data(data)
            .orientation(BarOrientation::Horizontal);
        let result = render_to_string(&chart, 30, 5);
        assert_snapshot!("chart_horizontal_bars", result);
    }

    #[test]
    fn snapshot_chart_line() {
        let data = vec![
            DataPoint::new("1", 10.0),
            DataPoint::new("2", 50.0),
            DataPoint::new("3", 30.0),
            DataPoint::new("4", 80.0),
            DataPoint::new("5", 40.0),
        ];
        let chart = Chart::new().data(data).chart_type(ChartType::Line);
        let result = render_to_string(&chart, 20, 8);
        assert_snapshot!("chart_line", result);
    }

    #[test]
    fn snapshot_chart_with_values() {
        let data = vec![DataPoint::new("A", 25.0), DataPoint::new("B", 75.0)];
        let chart = Chart::new()
            .data(data)
            .orientation(BarOrientation::Horizontal)
            .show_values(true);
        let result = render_to_string(&chart, 20, 3);
        assert_snapshot!("chart_with_values", result);
    }

    #[test]
    fn test_data_point_new() {
        let point = DataPoint::new("Test", 42.0);
        assert_eq!(point.label, "Test");
        assert_eq!(point.value, 42.0);
        assert!(point.style.is_none());
    }

    #[test]
    fn test_data_point_styled() {
        let point = DataPoint::new("Test", 42.0).style(Style::new().fg(Color::Red));
        assert!(point.style.is_some());
    }

    #[test]
    fn test_chart_default() {
        let chart = Chart::new();
        assert!(chart.data.is_empty());
        assert_eq!(chart.chart_type, ChartType::Bar);
        assert_eq!(chart.orientation, BarOrientation::Vertical);
    }

    #[test]
    fn test_chart_set_data() {
        let mut chart = Chart::new();
        chart.set_data(vec![DataPoint::new("A", 10.0)]);
        assert_eq!(chart.data_ref().len(), 1);
    }

    #[test]
    fn test_chart_value_range() {
        let chart = Chart::new().data(vec![
            DataPoint::new("A", 10.0),
            DataPoint::new("B", 50.0),
            DataPoint::new("C", 30.0),
        ]);

        let (min, max) = chart.get_value_range();
        assert_eq!(min, 10.0);
        assert_eq!(max, 50.0);
    }

    #[test]
    fn test_chart_fixed_range() {
        let chart = Chart::new()
            .data(vec![DataPoint::new("A", 10.0)])
            .min_value(0.0)
            .max_value(100.0);

        let (min, max) = chart.get_value_range();
        assert_eq!(min, 0.0);
        assert_eq!(max, 100.0);
    }

    #[test]
    fn test_chart_props() {
        let props = ChartProps::new(vec![DataPoint::new("Test", 50.0)])
            .chart_type(ChartType::Line)
            .show_labels(false)
            .show_values(true);

        let chart = Chart::create(props);
        assert_eq!(chart.chart_type, ChartType::Line);
        assert!(!chart.show_labels);
        assert!(chart.show_values);
    }

    #[test]
    fn test_chart_empty_render() {
        let chart = Chart::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        chart.render(Rect::new(0, 0, 10, 5), &mut buf);
        // Should not panic
    }

    #[test]
    fn test_chart_render_zero_area() {
        let chart = Chart::new().data(vec![DataPoint::new("A", 10.0)]);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        chart.render(Rect::new(0, 0, 0, 0), &mut buf);
        // Should not panic
    }
}
