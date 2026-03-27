# Gauge Components

Semi-circular and linear gauges for displaying values.

## Gauge (Semi-circular)

### Empty Gauge (0%)

**Code:**

```rust
let gauge = Gauge::new().value(0.0);
```

**Render:**

```
               
  ░░░░░░░░░░░  
 ░░░░░ ░ ░░░░░ 
 ░░░░  ░  ░░░░ 
 ░░░░░0% ░░░░░ 
               
               
               
```

### Half Gauge (50%)

**Code:**

```rust
let gauge = Gauge::new()
    .value(50.0)
    .max(100.0);
```

**Render:**

```
               
  ░░█████████  
 ░░░░█ █ █████ 
 ░░░░  █  ████ 
 ░░░░░50%░░░░░ 
               
               
               
```

### Full Gauge (100%)

**Code:**

```rust
let gauge = Gauge::new().value(100.0).max(100.0);
```

**Render:**

```
               
  ░░█████████  
 ░░░░█ █ █████ 
 ░░░░  █  ████ 
 ░░░░100%█████ 
               
               
               
```

## LinearGauge

### Linear Gauge with Percentage

**Code:**

```rust
let gauge = LinearGauge::new()
    .value(60.0)
    .max(100.0)
    .show_percent(true);
```

**Render:**

```
████████████░░░░░░░░
        60%         
```

