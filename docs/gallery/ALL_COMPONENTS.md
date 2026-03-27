# cTUI Component Gallery - Complete Reference

This page shows ALL components with their rendered output.

---

## Block

Container widget with borders and titles.

```rust
let block = Block::new()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .title("Panel");
```

**Render:**

```
╭ Panel ───────────╮
│                  │
│                  │
│                  │
╰──────────────────╯
```

---

## Paragraph

Multi-line text rendering with alignment.

```rust
let paragraph = Paragraph::new("Hello, cTUI!\nMultiple lines supported.");
```

**Render:**

```
Hello, cTUI!             
Multiple lines supported.
```

---

## List

Scrollable list with selection.

```rust
let list = List::new(vec![
    ListItem::new("Item One"),
    ListItem::new("Item Two"),
    ListItem::new("Item Three"),
]).select(Some(1));
```

**Render:**

```
Item One       
Item Two       
Item Three     
```

---

## Table

Tabular data with columns and rows.

```rust
let table = Table::new()
    .add_column(Column::fixed("ID", 4))
    .add_column(Column::fixed("Name", 12))
    .add_row(Row::from_strings(vec!["1", "Alice"]))
    .add_row(Row::from_strings(vec!["2", "Bob"]));
```

**Render:**

```
ID  Name            
────────────────────
1   Alice           
2   Bob             
                    
```

---

## Input

Single-line text input.

```rust
let input = Input::new().value("Hello, World!");
```

**Render:**

```
Hello, World!            
```

---

## ProgressBar

Progress bar with percentage.

```rust
let progress = ProgressBar::new().ratio(0.65).show_percent(true);
```

**Render:**

```
████████65%██       
```

---

## Spinner

Animated loading spinner.

```rust
let spinner = Spinner::new().spinner_style(SpinnerStyle::Dots).frame(0);
```

**Render:**

```
⠋         
```

---

## Gauge

Semi-circular gauge.

```rust
let gauge = Gauge::new().value(75.0).max(100.0);
```

**Render:**

```
               
  ░░█████████  
 ░░░░█ █ █████ 
 ░░░░  █  ████ 
 ░░░░░75%█████ 
               
               
               
```

---

## Chart

ASCII bar chart.

```rust
let chart = Chart::new().data(vec![
    DataPoint::new("Mon", 40.0),
    DataPoint::new("Tue", 60.0),
    DataPoint::new("Wed", 80.0),
]);
```

**Render:**

```
            █████   
            █████   
            █████   
      █████ █████   
      █████ █████   
      █████ █████   
      █████ █████   
Mon   Tue   Wed     
```

---

## Tabs

Tabbed navigation.

```rust
let tabs = Tabs::titles(["Home", "Settings", "About"]).with_selected(1);
```

**Render:**

```
 Home │ Settings │ About 
```

---

## Slider

Value slider input.

```rust
let slider = Slider::new().value(50.0).min(0.0).max(100.0);
```

**Render:**

```
██████████●░░░░░░░░░
```

---

## Tree

Hierarchical tree view.

```rust
let tree = Tree::new().node(
    TreeNode::new("Root")
        .expanded(true)
        .child(TreeNode::new("Child 1"))
        .child(TreeNode::new("Child 2"))
);
```

**Render:**

```
▼  Root        
   Child 1     
   Child 2     
               
               
```

---

## Sparkline

Inline data trend visualization.

```rust
let sparkline = Sparkline::new().data(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
```

**Render:**

```
     ▁▃▅▆█
```

---

## Select

Dropdown selection widget.

```rust
let select = Select::new()
    .items(vec![SelectItem::new("Option A"), SelectItem::new("Option B")])
    .selected(Some(0));
```

**Render:**

```
Option A     ▼ 
```

