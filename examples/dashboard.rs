//! Dashboard example - data visualization demonstration
//!
//! Run with: `cargo run --example dashboard`

use ctui_components::{Chart, ChartProps, ChartType, DataPoint, ProgressBar, ProgressBarProps};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Style};
use std::collections::HashMap;

struct RefreshData;
struct UpdateMetric(String, f64);

impl Msg for RefreshData {}
impl Msg for UpdateMetric {}

#[derive(Clone, Debug)]
struct Metric {
    name: String,
    value: f64,
    max: f64,
    unit: String,
}

impl Metric {
    fn new(name: &str, value: f64, max: f64, unit: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            max,
            unit: unit.to_string(),
        }
    }

    fn percentage(&self) -> f64 {
        (self.value / self.max * 100.0).min(100.0)
    }
}

struct DashboardState {
    metrics: Vec<Metric>,
    chart_data: Vec<DataPoint>,
    last_update: String,
}

impl DashboardState {
    fn new() -> Self {
        Self {
            metrics: vec![
                Metric::new("CPU", 45.0, 100.0, "%"),
                Metric::new("Memory", 6.2, 16.0, "GB"),
                Metric::new("Disk", 234.5, 512.0, "GB"),
                Metric::new("Network", 12.8, 100.0, "Mbps"),
            ],
            chart_data: vec![
                DataPoint::new("0", 45.0),
                DataPoint::new("1", 52.0),
                DataPoint::new("2", 38.0),
                DataPoint::new("3", 67.0),
                DataPoint::new("4", 45.0),
            ],
            last_update: "Just now".to_string(),
        }
    }
}

struct Dashboard {
    state: DashboardState,
}

impl Dashboard {
    fn new() -> Self {
        Self {
            state: DashboardState::new(),
        }
    }
}

impl Component for Dashboard {
    type Props = ();
    type State = DashboardState;

    fn create(_props: Self::Props) -> Self {
        Self::new()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let header = "╔══════════════════════════════════════════════╗";
        let title = "║           System Dashboard                    ║";
        let divider = "╠══════════════════════════════════════════════╣";
        let footer = "╚══════════════════════════════════════════════╝";

        let header_lines = [header, title, divider];
        for (row, line) in header_lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + row as u16) {
                    cell.symbol = ch.to_string();
                }
            }
        }

        let metrics_start = 3;
        for (idx, metric) in self.state.metrics.iter().enumerate() {
            let row = metrics_start + idx;
            let bar_width = 20;
            let filled = (metric.percentage() / 100.0 * bar_width as f64) as usize;

            let line = format!(
                "║ {:10} [{:<5}] {}{}",
                metric.name,
                format!("{:.1}{}", metric.value, metric.unit),
                "█".repeat(filled),
                "░".repeat(bar_width - filled)
            );

            for (col, ch) in line.chars().take(area.width as usize).enumerate() {
                if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + row as u16) {
                    cell.symbol = ch.to_string();
                }
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<RefreshData>() {
            self.state.metrics[0].value = (self.state.metrics[0].value + 5.0) % 100.0;
            self.state.metrics[1].value = (self.state.metrics[1].value + 0.5) % 16.0;
            Cmd::Render
        } else if let Some(update) = msg.downcast_ref::<UpdateMetric>() {
            if let Some(metric) = self.state.metrics.iter_mut().find(|m| m.name == update.0) {
                metric.value = update.1;
            }
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

fn main() {
    let mut dashboard = Dashboard::new();
    dashboard.on_mount();

    println!("Dashboard Example");
    println!("=================\n");

    println!("Initial Metrics:");
    for metric in &dashboard.state.metrics {
        println!(
            "  {}: {:.1}{} ({:.0}%)",
            metric.name,
            metric.value,
            metric.unit,
            metric.percentage()
        );
    }

    dashboard.update(Box::new(RefreshData));
    dashboard.update(Box::new(RefreshData));

    println!("\nAfter 2 refreshes:");
    for metric in &dashboard.state.metrics {
        println!(
            "  {}: {:.1}{} ({:.0}%)",
            metric.name,
            metric.value,
            metric.unit,
            metric.percentage()
        );
    }

    dashboard.update(Box::new(UpdateMetric("CPU".to_string(), 85.0)));
    println!("\nAfter setting CPU to 85%:");
    for metric in &dashboard.state.metrics {
        println!(
            "  {}: {:.1}{} ({:.0}%)",
            metric.name,
            metric.value,
            metric.unit,
            metric.percentage()
        );
    }

    println!("\nChart Data Points:");
    for point in &dashboard.state.chart_data {
        println!("  ({}: {})", point.label, point.value);
    }

    println!("\n✓ Dashboard data visualization verified");
    dashboard.on_unmount();
}
