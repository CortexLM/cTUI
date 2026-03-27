# Sparkline Components

Compact inline visualizations for data trends.

## Sparkline

### Basic Sparkline

**Code:**

```rust
let sparkline = Sparkline::new()
    .data(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
```

**Render:**

```
     ▁▃▅▆█
```

### Full Range Sparkline

**Code:**

```rust
let sparkline = Sparkline::new()
    .data(vec![0.0, 25.0, 50.0, 75.0, 100.0]);
```

**Render:**

```
     ▁▃▅▆█
```

## BarSparkline

### Bar Sparkline

**Code:**

```rust
let sparkline = BarSparkline::new()
    .data(vec![10.0, 30.0, 20.0, 40.0, 15.0]);
```

**Render:**

```
         ███   
   ███   ███   
   █████████   
   ████████████
███████████████
```

