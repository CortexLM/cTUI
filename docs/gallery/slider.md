# Slider Component

A slider input for selecting values within a range.

## Variants

### Horizontal Slider

**Code:**

```rust
let slider = Slider::new()
    .value(50.0)
    .min(0.0)
    .max(100.0);
```

**Render:**

```
██████████●░░░░░░░░░
```

### Vertical Slider

**Code:**

```rust
let slider = Slider::new()
    .value(50.0)
    .vertical();
```

**Render:**

```
░││
░││
░││
░││
●││
█││
█││
█││
█││
█││
```

