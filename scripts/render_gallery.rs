//! cTUI Visual Component Gallery Generator

use std::fs;
use std::path::Path;

use ctui_components::table::{Column, Row};
use ctui_components::text::Alignment as TextAlignment;
use ctui_components::{
    Alignment, BarSparkline, Block, BorderType, Borders, Chart, DataPoint, Gauge, Input,
    LinearGauge, List, ListItem, Orientation, Paragraph, ProgressBar, Select, SelectItem, Slider,
    Sparkline, Spinner, SpinnerStyle, Tab, Table, Tabs, Text, Tree, TreeNode, Widget, WidgetExt,
};
use ctui_core::style::Style;
use ctui_core::{Buffer, Component, Rect};

fn render_widget_to_string<W: Widget>(widget: &W, width: u16, height: u16) -> String {
    let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
    widget.render(Rect::new(0, 0, width, height), &mut buf);
    buffer_to_string(&buf)
}

fn render_component_to_string<C: Component>(component: &C, width: u16, height: u16) -> String {
    let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
    component.render(Rect::new(0, 0, width, height), &mut buf);
    buffer_to_string(&buf)
}

fn buffer_to_string(buf: &Buffer) -> String {
    let mut output = String::new();
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            output.push_str(&buf[(x, y)].symbol);
        }
        if y < buf.area.height - 1 {
            output.push('\n');
        }
    }
    output
}

fn generate_block_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Block Component\n\n");
    content.push_str("A container widget with borders, padding, and optional titles.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Plain Border\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let block = Block::new().borders(Borders::ALL);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let block = Block::new().borders(Borders::ALL);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&block, 15, 5));
    content.push_str("\n```\n\n");

    content.push_str("### Rounded Border\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let block = Block::new()\n");
    content.push_str("    .borders(Borders::ALL)\n");
    content.push_str("    .border_type(BorderType::Rounded);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&block, 15, 5));
    content.push_str("\n```\n\n");

    content.push_str("### Double Border\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let block = Block::new()\n");
    content.push_str("    .borders(Borders::ALL)\n");
    content.push_str("    .border_type(BorderType::Double);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Double);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&block, 15, 5));
    content.push_str("\n```\n\n");

    content.push_str("### With Title\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let block = Block::new()\n");
    content.push_str("    .borders(Borders::ALL)\n");
    content.push_str("    .title(\"Title\");\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let block = Block::new().borders(Borders::ALL).title("Title");
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&block, 20, 5));
    content.push_str("\n```\n\n");

    content.push_str("### Centered Title\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let block = Block::new()\n");
    content.push_str("    .borders(Borders::ALL)\n");
    content.push_str("    .title_with_alignment(\"Title\", Alignment::Center);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let block = Block::new()
        .borders(Borders::ALL)
        .title_with_alignment("Title", Alignment::Center);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&block, 20, 5));
    content.push_str("\n```\n\n");

    content.push_str("### Top and Bottom Borders Only\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let block = Block::new().borders(Borders::TOP | Borders::BOTTOM);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let block = Block::new().borders(Borders::TOP | Borders::BOTTOM);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&block, 15, 5));
    content.push_str("\n```\n\n");

    content
}

fn generate_paragraph_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Paragraph Component\n\n");
    content
        .push_str("A multi-line text rendering component with alignment and wrapping support.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Single Line\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let paragraph = Paragraph::new(\"Hello, World!\");\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let p = Paragraph::new("Hello, World!");
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&p, 20, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Multi-line Text\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let text = \"Line one\\nLine two\\nLine three\";\n");
    content.push_str("let paragraph = Paragraph::new(text);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let p = Paragraph::new("Line one\nLine two\nLine three");
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&p, 15, 3));
    content.push_str("\n```\n\n");

    content.push_str("### Text Wrapping\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let text = \"This is a long line that should wrap to fit the width\";\n");
    content.push_str("let paragraph = Paragraph::new(text);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let p = Paragraph::new("This is a long line that should wrap to fit the width");
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&p, 15, 5));
    content.push_str("\n```\n\n");

    content.push_str("### Left Aligned\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let paragraph = Paragraph::new(\"Left\")\n");
    content.push_str("    .alignment(TextAlignment::Left);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let p = Paragraph::new("Left").alignment(ctui_components::text::Alignment::Left);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&p, 15, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Center Aligned\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let paragraph = Paragraph::new(\"Center\")\n");
    content.push_str("    .alignment(TextAlignment::Center);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let p = Paragraph::new("Center").alignment(ctui_components::text::Alignment::Center);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&p, 15, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Right Aligned\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let paragraph = Paragraph::new(\"Right\")\n");
    content.push_str("    .alignment(TextAlignment::Right);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let p = Paragraph::new("Right").alignment(ctui_components::text::Alignment::Right);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&p, 15, 1));
    content.push_str("\n```\n\n");

    content
}

fn generate_list_gallery() -> String {
    let mut content = String::new();
    content.push_str("# List Component\n\n");
    content.push_str("A scrollable list of items with selection support.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Basic List\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let items = vec![\n");
    content.push_str("    ListItem::new(\"First\"),\n");
    content.push_str("    ListItem::new(\"Second\"),\n");
    content.push_str("    ListItem::new(\"Third\"),\n");
    content.push_str("];\n");
    content.push_str("let list = List::new(items);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let items = vec![
        ListItem::new("First"),
        ListItem::new("Second"),
        ListItem::new("Third"),
    ];
    let list = List::new(items);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&list, 15, 3));
    content.push_str("\n```\n\n");

    content.push_str("### With Selection\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let list = List::new(items)\n");
    content.push_str("    .select(Some(1));\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let items = vec![
        ListItem::new("Item A"),
        ListItem::new("Item B"),
        ListItem::new("Item C"),
    ];
    let list = List::new(items).select(Some(1));
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&list, 15, 3));
    content.push_str("\n```\n\n");

    content
}

fn generate_input_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Input Component\n\n");
    content.push_str("A single-line text input with cursor tracking and editing support.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Empty Input\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let input = Input::new();\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let input = Input::new();
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&input, 20, 1));
    content.push_str("\n```\n\n");

    content.push_str("### With Placeholder\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let input = Input::new()\n");
    content.push_str("    .placeholder(\"Enter your name...\");\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let input = Input::new().placeholder("Enter your name...");
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&input, 25, 1));
    content.push_str("\n```\n\n");

    content.push_str("### With Text\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let input = Input::new().value(\"Hello, World!\");\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let input = Input::new().value("Hello, World!");
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&input, 25, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Password Field\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let input = Input::new()\n");
    content.push_str("    .value(\"secret123\")\n");
    content.push_str("    .password(true);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let input = Input::new().value("secret123").password(true);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&input, 20, 1));
    content.push_str("\n```\n\n");

    content
}

fn generate_table_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Table Component\n\n");
    content.push_str("A tabular data display component with columns, rows, and selection.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Basic Table\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let table = Table::new()\n");
    content.push_str("    .add_column(Column::fixed(\"ID\", 5))\n");
    content.push_str("    .add_column(Column::fixed(\"Name\", 10))\n");
    content.push_str("    .add_row(Row::from_strings(vec![\"1\", \"Alice\"]))\n");
    content.push_str("    .add_row(Row::from_strings(vec![\"2\", \"Bob\"]));\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let table = Table::new()
        .add_column(Column::fixed("ID", 5))
        .add_column(Column::fixed("Name", 10))
        .add_row(Row::from_strings(vec!["1", "Alice"]))
        .add_row(Row::from_strings(vec!["2", "Bob"]));
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&table, 20, 5));
    content.push_str("\n```\n\n");

    content.push_str("### With Selection\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let table = Table::new()\n");
    content.push_str("    .add_column(Column::fixed(\"Item\", 10))\n");
    content.push_str("    .add_row(Row::from_strings(vec![\"First\"]))\n");
    content.push_str("    .add_row(Row::from_strings(vec![\"Second\"]))\n");
    content.push_str("    .with_selected(Some(0));\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let table = Table::new()
        .add_column(Column::fixed("Item", 10))
        .add_row(Row::from_strings(vec!["First"]))
        .add_row(Row::from_strings(vec!["Second"]))
        .with_selected(Some(0));
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&table, 12, 5));
    content.push_str("\n```\n\n");

    content
}

fn generate_progress_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Progress Components\n\n");
    content.push_str("Visual indicators for progress: `ProgressBar` and `Spinner`.\n\n");

    content.push_str("## ProgressBar\n\n");

    content.push_str("### 0% Progress\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let progress = ProgressBar::new().ratio(0.0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let progress = ProgressBar::new().ratio(0.0);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&progress, 20, 1));
    content.push_str("\n```\n\n");

    content.push_str("### 50% Progress\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let progress = ProgressBar::new().ratio(0.5);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let progress = ProgressBar::new().ratio(0.5);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&progress, 20, 1));
    content.push_str("\n```\n\n");

    content.push_str("### 100% Progress\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let progress = ProgressBar::new().ratio(1.0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let progress = ProgressBar::new().ratio(1.0);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&progress, 20, 1));
    content.push_str("\n```\n\n");

    content.push_str("### With Label\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let progress = ProgressBar::new()\n");
    content.push_str("    .ratio(0.6)\n");
    content.push_str("    .label(\"Loading...\");\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let progress = ProgressBar::new().ratio(0.6).label("Loading...");
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&progress, 20, 1));
    content.push_str("\n```\n\n");

    content.push_str("### With Percentage\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let progress = ProgressBar::new()\n");
    content.push_str("    .ratio(0.75)\n");
    content.push_str("    .show_percent(true);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let progress = ProgressBar::new().ratio(0.75).show_percent(true);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&progress, 20, 1));
    content.push_str("\n```\n\n");

    content.push_str("## Spinner\n\n");

    content.push_str("### Dots Spinner\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let spinner = Spinner::new()\n");
    content.push_str("    .spinner_style(SpinnerStyle::Dots)\n");
    content.push_str("    .frame(0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let spinner = Spinner::new().spinner_style(SpinnerStyle::Dots).frame(0);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&spinner, 10, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Bars Spinner\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let spinner = Spinner::new()\n");
    content.push_str("    .spinner_style(SpinnerStyle::Bars)\n");
    content.push_str("    .frame(0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let spinner = Spinner::new().spinner_style(SpinnerStyle::Bars).frame(0);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&spinner, 5, 1));
    content.push_str("\n```\n\n");

    content
}

fn generate_chart_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Chart Component\n\n");
    content.push_str("ASCII charts for data visualization.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Vertical Bar Chart\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let data = vec![\n");
    content.push_str("    DataPoint::new(\"A\", 30.0),\n");
    content.push_str("    DataPoint::new(\"B\", 60.0),\n");
    content.push_str("    DataPoint::new(\"C\", 90.0),\n");
    content.push_str("    DataPoint::new(\"D\", 45.0),\n");
    content.push_str("];\n");
    content.push_str("let chart = Chart::new().data(data);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let data = vec![
        DataPoint::new("A", 30.0),
        DataPoint::new("B", 60.0),
        DataPoint::new("C", 90.0),
        DataPoint::new("D", 45.0),
    ];
    let chart = Chart::new().data(data);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&chart, 20, 8));
    content.push_str("\n```\n\n");

    content.push_str("### Horizontal Bar Chart\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("use ctui_components::BarOrientation;\n");
    content.push_str("let chart = Chart::new()\n");
    content.push_str("    .data(data)\n");
    content.push_str("    .orientation(BarOrientation::Horizontal);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    use ctui_components::BarOrientation;
    let data = vec![
        DataPoint::new("ItemA", 30.0),
        DataPoint::new("ItemB", 60.0),
        DataPoint::new("ItemC", 90.0),
    ];
    let chart = Chart::new()
        .data(data)
        .orientation(BarOrientation::Horizontal);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&chart, 25, 5));
    content.push_str("\n```\n\n");

    content
}

fn generate_gauge_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Gauge Components\n\n");
    content.push_str("Semi-circular and linear gauges for displaying values.\n\n");

    content.push_str("## Gauge (Semi-circular)\n\n");

    content.push_str("### Empty Gauge (0%)\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let gauge = Gauge::new().value(0.0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let gauge = Gauge::new().value(0.0);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&gauge, 15, 8));
    content.push_str("\n```\n\n");

    content.push_str("### Half Gauge (50%)\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let gauge = Gauge::new()\n");
    content.push_str("    .value(50.0)\n");
    content.push_str("    .max(100.0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let gauge = Gauge::new().value(50.0).max(100.0);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&gauge, 15, 8));
    content.push_str("\n```\n\n");

    content.push_str("### Full Gauge (100%)\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let gauge = Gauge::new().value(100.0).max(100.0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let gauge = Gauge::new().value(100.0).max(100.0);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&gauge, 15, 8));
    content.push_str("\n```\n\n");

    content.push_str("## LinearGauge\n\n");

    content.push_str("### Linear Gauge with Percentage\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let gauge = LinearGauge::new()\n");
    content.push_str("    .value(60.0)\n");
    content.push_str("    .max(100.0)\n");
    content.push_str("    .show_percent(true);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let gauge = LinearGauge::new().value(60.0).max(100.0).show_percent(true);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&gauge, 20, 2));
    content.push_str("\n```\n\n");

    content
}

fn generate_sparkline_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Sparkline Components\n\n");
    content.push_str("Compact inline visualizations for data trends.\n\n");

    content.push_str("## Sparkline\n\n");

    content.push_str("### Basic Sparkline\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let sparkline = Sparkline::new()\n");
    content.push_str("    .data(vec![1.0, 2.0, 3.0, 4.0, 5.0]);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let sparkline = Sparkline::new().data(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&sparkline, 10, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Full Range Sparkline\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let sparkline = Sparkline::new()\n");
    content.push_str("    .data(vec![0.0, 25.0, 50.0, 75.0, 100.0]);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let sparkline = Sparkline::new().data(vec![0.0, 25.0, 50.0, 75.0, 100.0]);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&sparkline, 10, 1));
    content.push_str("\n```\n\n");

    content.push_str("## BarSparkline\n\n");

    content.push_str("### Bar Sparkline\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let sparkline = BarSparkline::new()\n");
    content.push_str("    .data(vec![10.0, 30.0, 20.0, 40.0, 15.0]);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let sparkline = BarSparkline::new().data(vec![10.0, 30.0, 20.0, 40.0, 15.0]);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&sparkline, 15, 5));
    content.push_str("\n```\n\n");

    content
}

fn generate_tree_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Tree Component\n\n");
    content.push_str("Hierarchical tree view with expandable nodes.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Single Node\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let tree = Tree::new().node(TreeNode::new(\"Root\"));\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let tree = Tree::new().node(TreeNode::new("Root"));
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&tree, 15, 5));
    content.push_str("\n```\n\n");

    content.push_str("### Nested Tree (Expanded)\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let tree = Tree::new().node(\n");
    content.push_str("    TreeNode::new(\"Root\")\n");
    content.push_str("        .expanded(true)\n");
    content.push_str("        .child(TreeNode::new(\"Child 1\"))\n");
    content.push_str("        .child(TreeNode::new(\"Child 2\"))\n");
    content.push_str("        .child(\n");
    content.push_str("            TreeNode::new(\"Child 3\")\n");
    content.push_str("                .expanded(true)\n");
    content.push_str("                .child(TreeNode::new(\"Grandchild\"))\n");
    content.push_str("        )\n");
    content.push_str(");\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let tree = Tree::new().node(
        TreeNode::new("Root")
            .expanded(true)
            .child(TreeNode::new("Child 1"))
            .child(TreeNode::new("Child 2"))
            .child(
                TreeNode::new("Child 3")
                    .expanded(true)
                    .child(TreeNode::new("Grandchild")),
            ),
    );
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&tree, 20, 5));
    content.push_str("\n```\n\n");

    content
}

fn generate_select_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Select Components\n\n");
    content.push_str("Dropdown selection and combo box widgets.\n\n");

    content.push_str("## Select (Closed)\n\n");
    content.push_str("### Closed State\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let select = Select::new()\n");
    content.push_str("    .items(vec![\n");
    content.push_str("        SelectItem::new(\"Option 1\"),\n");
    content.push_str("        SelectItem::new(\"Option 2\"),\n");
    content.push_str("    ])\n");
    content.push_str("    .selected(Some(0));\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let select = Select::new()
        .items(vec![
            SelectItem::new("Option 1"),
            SelectItem::new("Option 2"),
        ])
        .selected(Some(0));
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&select, 15, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Open State\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let select = Select::new()\n");
    content.push_str("    .items(vec![...])\n");
    content.push_str("    .open(true)\n");
    content.push_str("    .highlighted(1);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let select = Select::new()
        .items(vec![
            SelectItem::new("Apple"),
            SelectItem::new("Banana"),
            SelectItem::new("Cherry"),
        ])
        .open(true)
        .highlighted(1);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&select, 15, 5));
    content.push_str("\n```\n\n");

    content.push_str("### With Placeholder\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let select = Select::new()\n");
    content.push_str("    .items(vec![...])\n");
    content.push_str("    .placeholder(\"Choose...\");\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let select = Select::new()
        .items(vec![SelectItem::new("Item")])
        .placeholder("Choose...");
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&select, 15, 1));
    content.push_str("\n```\n\n");

    content
}

fn generate_slider_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Slider Component\n\n");
    content.push_str("A slider input for selecting values within a range.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Horizontal Slider\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let slider = Slider::new()\n");
    content.push_str("    .value(50.0)\n");
    content.push_str("    .min(0.0)\n");
    content.push_str("    .max(100.0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let slider = Slider::new().value(50.0).min(0.0).max(100.0);
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&slider, 20, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Vertical Slider\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let slider = Slider::new()\n");
    content.push_str("    .value(50.0)\n");
    content.push_str("    .vertical();\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let slider = Slider::new().value(50.0).vertical();
    content.push_str("```\n");
    content.push_str(&render_widget_to_string(&slider, 3, 10));
    content.push_str("\n```\n\n");

    content
}

fn generate_tabs_gallery() -> String {
    let mut content = String::new();
    content.push_str("# Tabs Component\n\n");
    content.push_str("Tabbed navigation component.\n\n");

    content.push_str("## Variants\n\n");

    content.push_str("### Basic Tabs\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let tabs = Tabs::titles([\"Tab 1\", \"Tab 2\", \"Tab 3\"])\n");
    content.push_str("    .with_selected(0);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let tabs = Tabs::titles(["Tab 1", "Tab 2", "Tab 3"]).with_selected(0);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&tabs, 25, 1));
    content.push_str("\n```\n\n");

    content.push_str("### Second Tab Selected\n\n");
    content.push_str("**Code:**\n\n");
    content.push_str("```rust\n");
    content.push_str("let tabs = Tabs::titles([\"Home\", \"Settings\", \"About\"])\n");
    content.push_str("    .with_selected(1);\n");
    content.push_str("```\n\n");
    content.push_str("**Render:**\n\n");
    let tabs = Tabs::titles(["Home", "Settings", "About"]).with_selected(1);
    content.push_str("```\n");
    content.push_str(&render_component_to_string(&tabs, 25, 1));
    content.push_str("\n```\n\n");

    content
}

fn generate_readme_index() -> String {
    let mut content = String::new();
    content.push_str("# cTUI Component Gallery\n\n");
    content.push_str("Visual reference for all cTUI components with ASCII renders.\n\n");

    content.push_str("## Components\n\n");
    content.push_str("| Component | Description |\n");
    content.push_str("|-----------|-------------|\n");
    content.push_str("| [Block](block.md) | Container with borders and titles |\n");
    content
        .push_str("| [Paragraph](paragraph.md) | Multi-line text with alignment and wrapping |\n");
    content.push_str("| [List](list.md) | Scrollable list with selection |\n");
    content.push_str("| [Input](input.md) | Single-line text input |\n");
    content.push_str("| [Table](table.md) | Tabular data display |\n");
    content.push_str("| [Progress](progress.md) | Progress bars and spinners |\n");
    content.push_str("| [Chart](chart.md) | ASCII bar and line charts |\n");
    content.push_str("| [Gauge](gauge.md) | Semi-circular and linear gauges |\n");
    content.push_str("| [Sparkline](sparkline.md) | Compact inline visualizations |\n");
    content.push_str("| [Tree](tree.md) | Hierarchical tree view |\n");
    content.push_str("| [Select](select.md) | Dropdown selection widgets |\n");
    content.push_str("| [Slider](slider.md) | Value slider input |\n");
    content.push_str("| [Tabs](tabs.md) | Tabbed navigation |\n\n");

    content.push_str("## Quick Example\n\n");
    content.push_str("```rust\n");
    content.push_str("use ctui_components::{Block, Paragraph, Borders, BorderType};\n");
    content.push_str("use ctui_core::{Buffer, Rect};\n\n");
    content.push_str("let block = Block::new()\n");
    content.push_str("    .borders(Borders::ALL)\n");
    content.push_str("    .border_type(BorderType::Rounded)\n");
    content.push_str("    .title(\"Example\");\n\n");
    content.push_str("let paragraph = Paragraph::new(\"Hello, World!\");\n");
    content.push_str("```\n\n");

    content.push_str("## Visual Preview\n\n");
    content.push_str("Here's a quick visual of the Block component:\n\n");
    content.push_str("```\n");
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Example");
    content.push_str(&render_component_to_string(&block, 25, 6));
    content.push_str("\n```\n");

    content
}

fn generate_all_components_gallery() -> String {
    let mut content = String::new();
    content.push_str("# cTUI Component Gallery - Complete Reference\n\n");
    content.push_str("This page shows ALL components with their rendered output.\n\n");
    content.push_str("---\n\n");

    // Block section
    content.push_str("## Block\n\n");
    content.push_str("Container widget with borders and titles.\n\n");
    content.push_str("```rust\n");
    content.push_str("let block = Block::new()\n");
    content.push_str("    .borders(Borders::ALL)\n");
    content.push_str("    .border_type(BorderType::Rounded)\n");
    content.push_str("    .title(\"Panel\");\n");
    content.push_str("```\n\n");
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Panel");
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&block, 20, 5));
    content.push_str("\n```\n\n---\n\n");

    // Paragraph section
    content.push_str("## Paragraph\n\n");
    content.push_str("Multi-line text rendering with alignment.\n\n");
    content.push_str("```rust\n");
    content.push_str(
        "let paragraph = Paragraph::new(\"Hello, cTUI!\\nMultiple lines supported.\");\n",
    );
    content.push_str("```\n\n");
    let p = Paragraph::new("Hello, cTUI!\nMultiple lines supported.");
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&p, 25, 2));
    content.push_str("\n```\n\n---\n\n");

    // List section
    content.push_str("## List\n\n");
    content.push_str("Scrollable list with selection.\n\n");
    content.push_str("```rust\n");
    content.push_str("let list = List::new(vec![\n");
    content.push_str("    ListItem::new(\"Item One\"),\n");
    content.push_str("    ListItem::new(\"Item Two\"),\n");
    content.push_str("    ListItem::new(\"Item Three\"),\n");
    content.push_str("]).select(Some(1));\n");
    content.push_str("```\n\n");
    let items = vec![
        ListItem::new("Item One"),
        ListItem::new("Item Two"),
        ListItem::new("Item Three"),
    ];
    let list = List::new(items).select(Some(1));
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&list, 15, 3));
    content.push_str("\n```\n\n---\n\n");

    // Table section
    content.push_str("## Table\n\n");
    content.push_str("Tabular data with columns and rows.\n\n");
    content.push_str("```rust\n");
    content.push_str("let table = Table::new()\n");
    content.push_str("    .add_column(Column::fixed(\"ID\", 4))\n");
    content.push_str("    .add_column(Column::fixed(\"Name\", 12))\n");
    content.push_str("    .add_row(Row::from_strings(vec![\"1\", \"Alice\"]))\n");
    content.push_str("    .add_row(Row::from_strings(vec![\"2\", \"Bob\"]));\n");
    content.push_str("```\n\n");
    let table = Table::new()
        .add_column(Column::fixed("ID", 4))
        .add_column(Column::fixed("Name", 12))
        .add_row(Row::from_strings(vec!["1", "Alice"]))
        .add_row(Row::from_strings(vec!["2", "Bob"]));
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&table, 20, 5));
    content.push_str("\n```\n\n---\n\n");

    // Input section
    content.push_str("## Input\n\n");
    content.push_str("Single-line text input.\n\n");
    content.push_str("```rust\n");
    content.push_str("let input = Input::new().value(\"Hello, World!\");\n");
    content.push_str("```\n\n");
    let input = Input::new().value("Hello, World!");
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&input, 25, 1));
    content.push_str("\n```\n\n---\n\n");

    // Progress section
    content.push_str("## ProgressBar\n\n");
    content.push_str("Progress bar with percentage.\n\n");
    content.push_str("```rust\n");
    content.push_str("let progress = ProgressBar::new().ratio(0.65).show_percent(true);\n");
    content.push_str("```\n\n");
    let progress = ProgressBar::new().ratio(0.65).show_percent(true);
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&progress, 20, 1));
    content.push_str("\n```\n\n---\n\n");

    // Spinner section
    content.push_str("## Spinner\n\n");
    content.push_str("Animated loading spinner.\n\n");
    content.push_str("```rust\n");
    content.push_str("let spinner = Spinner::new().spinner_style(SpinnerStyle::Dots).frame(0);\n");
    content.push_str("```\n\n");
    let spinner = Spinner::new().spinner_style(SpinnerStyle::Dots).frame(0);
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&spinner, 10, 1));
    content.push_str("\n```\n\n---\n\n");

    // Gauge section
    content.push_str("## Gauge\n\n");
    content.push_str("Semi-circular gauge.\n\n");
    content.push_str("```rust\n");
    content.push_str("let gauge = Gauge::new().value(75.0).max(100.0);\n");
    content.push_str("```\n\n");
    let gauge = Gauge::new().value(75.0).max(100.0);
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_widget_to_string(&gauge, 15, 8));
    content.push_str("\n```\n\n---\n\n");

    // Chart section
    content.push_str("## Chart\n\n");
    content.push_str("ASCII bar chart.\n\n");
    content.push_str("```rust\n");
    content.push_str("let chart = Chart::new().data(vec![\n");
    content.push_str("    DataPoint::new(\"Mon\", 40.0),\n");
    content.push_str("    DataPoint::new(\"Tue\", 60.0),\n");
    content.push_str("    DataPoint::new(\"Wed\", 80.0),\n");
    content.push_str("]);\n");
    content.push_str("```\n\n");
    let data = vec![
        DataPoint::new("Mon", 40.0),
        DataPoint::new("Tue", 60.0),
        DataPoint::new("Wed", 80.0),
    ];
    let chart = Chart::new().data(data);
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&chart, 20, 8));
    content.push_str("\n```\n\n---\n\n");

    // Tabs section
    content.push_str("## Tabs\n\n");
    content.push_str("Tabbed navigation.\n\n");
    content.push_str("```rust\n");
    content.push_str(
        "let tabs = Tabs::titles([\"Home\", \"Settings\", \"About\"]).with_selected(1);\n",
    );
    content.push_str("```\n\n");
    let tabs = Tabs::titles(["Home", "Settings", "About"]).with_selected(1);
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_component_to_string(&tabs, 25, 1));
    content.push_str("\n```\n\n---\n\n");

    // Slider section
    content.push_str("## Slider\n\n");
    content.push_str("Value slider input.\n\n");
    content.push_str("```rust\n");
    content.push_str("let slider = Slider::new().value(50.0).min(0.0).max(100.0);\n");
    content.push_str("```\n\n");
    let slider = Slider::new().value(50.0).min(0.0).max(100.0);
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_widget_to_string(&slider, 20, 1));
    content.push_str("\n```\n\n---\n\n");

    // Tree section
    content.push_str("## Tree\n\n");
    content.push_str("Hierarchical tree view.\n\n");
    content.push_str("```rust\n");
    content.push_str("let tree = Tree::new().node(\n");
    content.push_str("    TreeNode::new(\"Root\")\n");
    content.push_str("        .expanded(true)\n");
    content.push_str("        .child(TreeNode::new(\"Child 1\"))\n");
    content.push_str("        .child(TreeNode::new(\"Child 2\"))\n");
    content.push_str(");\n");
    content.push_str("```\n\n");
    let tree = Tree::new().node(
        TreeNode::new("Root")
            .expanded(true)
            .child(TreeNode::new("Child 1"))
            .child(TreeNode::new("Child 2")),
    );
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_widget_to_string(&tree, 15, 5));
    content.push_str("\n```\n\n---\n\n");

    // Sparkline section
    content.push_str("## Sparkline\n\n");
    content.push_str("Inline data trend visualization.\n\n");
    content.push_str("```rust\n");
    content.push_str("let sparkline = Sparkline::new().data(vec![1.0, 2.0, 3.0, 4.0, 5.0]);\n");
    content.push_str("```\n\n");
    let sparkline = Sparkline::new().data(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_widget_to_string(&sparkline, 10, 1));
    content.push_str("\n```\n\n---\n\n");

    // Select section
    content.push_str("## Select\n\n");
    content.push_str("Dropdown selection widget.\n\n");
    content.push_str("```rust\n");
    content.push_str("let select = Select::new()\n");
    content.push_str(
        "    .items(vec![SelectItem::new(\"Option A\"), SelectItem::new(\"Option B\")])\n",
    );
    content.push_str("    .selected(Some(0));\n");
    content.push_str("```\n\n");
    let select = Select::new()
        .items(vec![
            SelectItem::new("Option A"),
            SelectItem::new("Option B"),
        ])
        .selected(Some(0));
    content.push_str("**Render:**\n\n```\n");
    content.push_str(&render_widget_to_string(&select, 15, 1));
    content.push_str("\n```\n\n");

    content
}

fn main() {
    let gallery_dir = Path::new("docs/gallery");

    fs::create_dir_all(gallery_dir).expect("Failed to create gallery directory");

    let galleries: Vec<(&str, String)> = vec![
        ("block.md", generate_block_gallery()),
        ("paragraph.md", generate_paragraph_gallery()),
        ("list.md", generate_list_gallery()),
        ("input.md", generate_input_gallery()),
        ("table.md", generate_table_gallery()),
        ("progress.md", generate_progress_gallery()),
        ("chart.md", generate_chart_gallery()),
        ("gauge.md", generate_gauge_gallery()),
        ("sparkline.md", generate_sparkline_gallery()),
        ("tree.md", generate_tree_gallery()),
        ("select.md", generate_select_gallery()),
        ("slider.md", generate_slider_gallery()),
        ("tabs.md", generate_tabs_gallery()),
        ("README.md", generate_readme_index()),
        ("ALL_COMPONENTS.md", generate_all_components_gallery()),
    ];

    for (filename, content) in galleries {
        let path = gallery_dir.join(filename);
        fs::write(&path, content).expect(&format!("Failed to write {:?}", path));
        println!("Generated: {:?}", path);
    }

    println!("\nGallery generation complete!");
    println!("Files generated in: {}/", gallery_dir.display());
}
