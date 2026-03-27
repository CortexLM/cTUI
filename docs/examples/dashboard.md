# Dashboard Example

Real-time data visualization with charts, gauges, and tables.

## Overview

The dashboard demonstrates:

- Multiple layout regions
- Real-time data updates
- Charts and visualization
- Tables with data
- Progress indicators

## Render Output

```
╭ System Dashboard ──────────────────────────────────────────────╮
│                                                                 │
│  CPU Usage              Memory Usage         Disk I/O           │
│  ┌──────────────┐      ┌──────────────┐     ┌──────────────┐   │
│  │    ████████   │      │  ██████████ │     │ ██░░░░░░░░░░ │   │
│  │    ████████   │      │  ██████████ │     │ ██░░░░░░░░░░ │   │
│  │    ████████   │      │  ██████████ │     │ ██░░░░░░░░░░ │   │
│  │     45%       │      │     78%     │     │     23%     │   │
│  └──────────────┘      └──────────────┘     └──────────────┘   │
│                                                                 │
│  Recent Requests                                               │
│  ┌────────────────────────────────────────────────────────────┐│
│  │ Time    │ Endpoint      │ Status │ Latency │ Size        ││
│  │ 12:01:15│ /api/users    │   200  │   45ms  │    1.2KB   ││
│  │ 12:01:14│ /api/products │   200  │   32ms  │    4.5KB   ││
│  │ 12:01:13│ /api/orders   │   201  │  156ms  │    890B    ││
│  └────────────────────────────────────────────────────────────┘│
│                                                                 │
│  Throughput (req/s)                                             │
│      ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁                                         │
│                                                                 │
╰─────────────────────────────────────────────────────────────────╯
```

## Source Code

```rust
//! Dashboard example - real-time data visualization

use ctui_components::{
    Block, Borders, Chart, Gauge, Paragraph, 
    ProgressBar, Sparkline, Table, Row, Column
};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Style, Color};
use ctui_layout::{Layout, FlexDirection, Constraint};

struct Dashboard {
    cpu_usage: f64,
    memory_usage: f64,
    disk_io: f64,
    requests: Vec<RequestLog>,
    throughput: Vec<f64>,
}

#[derive(Clone)]
struct RequestLog {
    time: String,
    endpoint: String,
    status: u16,
    latency: u32,
    size: String,
}

impl Dashboard {
    fn new() -> Self {
        Self {
            cpu_usage: 45.0,
            memory_usage: 78.0,
            disk_io: 23.0,
            requests: Vec::new(),
            throughput: vec![10.0, 25.0, 50.0, 80.0, 100.0, 90.0, 70.0, 50.0, 30.0, 15.0],
        }
    }

    fn update_metrics(&mut self) {
        // Simulate metric updates
        self.cpu_usage = (self.cpu_usage + 1.0) % 100.0;
        self.memory_usage = (self.memory_usage + 0.5) % 100.0;
        self.disk_io = (self.disk_io + 2.0) % 100.0;
        
        // Add throughput data
        self.throughput.push((rand::random::<f64>() * 100.0));
        if self.throughput.len() > 20 {
            self.throughput.remove(0);
        }
    }
}

impl Component for Dashboard {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self::new()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Layout: Header / Stats / Table / Sparkline / Footer
        let layout = Layout::flex()
            .direction(FlexDirection::Column)
            .gap(1);

        let rects = layout.split(area, &[
            Constraint::Length(3),   // Header
            Constraint::Length(8),   // Gauges
            Constraint::Length(8),   // Table
            Constraint::Length(3),   // Sparkline
        ]);

        // Render header
        self.render_header(rects[0], buf);

        // Render gauges row
        self.render_gauges(rects[1], buf);

        // Render table
        self.render_table(rects[2], buf);

        // Render sparkline
        self.render_sparkline(rects[3], buf);
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("System Dashboard");
        
        block.render(area, buf);
        
        // Add current time
        let time = chrono::Local::now().format("%H:%M:%S").to_string();
        let text = format!("Last update: {}", time);
        // ... render text
    }

    fn render_gauges(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::flex()
            .direction(FlexDirection::Row)
            .gap(2);

        let rects = layout.split(area, &[
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]);

        // CPU Gauge
        Gauge::new()
            .value(self.cpu_usage)
            .max(100.0)
            .label("CPU")
            .render(rects[0], buf);

        // Memory Gauge  
        Gauge::new()
            .value(self.memory_usage)
            .max(100.0)
            .label("Memory")
            .render(rects[1], buf);

        // Disk I/O Gauge
        Gauge::new()
            .value(self.disk_io)
            .max(100.0)
            .label("Disk I/O")
            .render(rects[2], buf);
    }

    fn render_table(&self, area: Rect, buf: &mut Buffer) {
        let table = Table::new()
            .columns(vec![
                Column::new("Time").width(Constraint::Length(10)),
                Column::new("Endpoint").width(Constraint::Min(15)),
                Column::new("Status").width(Constraint::Length(8)),
                Column::new("Latency").width(Constraint::Length(10)),
                Column::new("Size").width(Constraint::Length(10)),
            ])
            .rows(self.requests.iter().map(|r| {
                Row::from_strings(vec![
                    r.time.clone(),
                    r.endpoint.clone(),
                    r.status.to_string(),
                    format!("{}ms", r.latency),
                    r.size.clone(),
                ])
            }))
            .header_style(Style::default().fg(Color::Cyan));

        table.render(area, buf);
    }

    fn render_sparkline(&self, area: Rect, buf: &mut Buffer) {
        let sparkline = Sparkline::new()
            .data(self.throughput.clone())
            .label("Throughput (req/s)");

        sparkline.render(area, buf);
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        self.update_metrics();
        Cmd::Render
    }
}

fn main() {
    let mut dashboard = Dashboard::new();
    dashboard.on_mount();

    // Add some sample requests
    dashboard.requests = vec![
        RequestLog { time: "12:01:15".into(), endpoint: "/api/users".into(), status: 200, latency: 45, size: "1.2KB".into() },
        RequestLog { time: "12:01:14".into(), endpoint: "/api/products".into(), status: 200, latency: 32, size: "4.5KB".into() },
        RequestLog { time: "12:01:13".into(), endpoint: "/api/orders".into(), status: 201, latency: 156, size: "890B".into() },
    ];

    println!("Dashboard Example");
    println!("=================\n");
    println!("CPU: {:.1}%", dashboard.cpu_usage);
    println!("Memory: {:.1}%", dashboard.memory_usage);
    println!("Disk I/O: {:.1}%", dashboard.disk_io);

    dashboard.on_unmount();
}
```

## Key Concepts

### Layout Composition

Split the dashboard into regions:

```rust
let rects = Layout::flex()
    .direction(FlexDirection::Column)
    .split(area, &[
        Constraint::Length(3),   // Header
        Constraint::Length(8),   // Charts
        Constraint::Min(10),     // Table
        Constraint::Length(3),   // Sparkline
    ]);
```

### Real-time Updates

Use a timer or async task:

```rust
// Timer-based updates
use tokio::time::{interval, Duration};

async fn run_dashboard() {
    let mut interval = interval(Duration::from_millis(100));
    
    loop {
        interval.tick().await;
        dashboard.update(Box::new(Tick));
    }
}
```

### Data Visualization

```rust
// Gauges
Gauge::new().value(75.0).max(100.0);

// Sparklines
Sparkline::new().data(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

// Charts
Chart::new()
    .data(vec![
        DataPoint::new("Mon", 40.0),
        DataPoint::new("Tue", 60.0),
    ]);
```

## Enhancements

### Add WebSocket Updates

```rust
use tokio_tungstenite::{connect_async, WebSocketStream};

async fn websocket_updates(tx: mpsc::Sender<Box<dyn Msg>>) {
    let (ws_stream, _) = connect_async("ws://localhost:8080/metrics").await.unwrap();
    
    while let Some(msg) = ws_stream.next().await {
        let data: MetricUpdate = parse_message(msg.unwrap());
        tx.send(Box::new(data)).await.unwrap();
    }
}
```

### Add Theme Support

```rust
use ctui_theme::Theme;

struct ThemedDashboard {
    theme: Theme,
    // ...
}

impl ThemedDashboard {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let style = Style::new().fg(self.theme.colors.primary);
        // Apply theme colors throughout
    }
}
```

## Run the Example

```bash
cargo run --example dashboard
```

## See Also

- [Todo App](todo.md) - CRUD application
- [Chart Component](../gallery/chart.md) - Chart documentation
- [Gauge Component](../gallery/gauge.md) - Gauge documentation
