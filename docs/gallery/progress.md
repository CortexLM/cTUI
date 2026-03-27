# Progress Components

Visual indicators for progress: `ProgressBar` and `Spinner`.

## ProgressBar

### 0% Progress

**Code:**

```rust
let progress = ProgressBar::new().ratio(0.0);
```

**Render:**

```
                    
```

### 50% Progress

**Code:**

```rust
let progress = ProgressBar::new().ratio(0.5);
```

**Render:**

```
██████████          
```

### 100% Progress

**Code:**

```rust
let progress = ProgressBar::new().ratio(1.0);
```

**Render:**

```
████████████████████
```

### With Label

**Code:**

```rust
let progress = ProgressBar::new()
    .ratio(0.6)
    .label("Loading...");
```

**Render:**

```
█████Loading...     
```

### With Percentage

**Code:**

```rust
let progress = ProgressBar::new()
    .ratio(0.75)
    .show_percent(true);
```

**Render:**

```
████████75%████     
```

## Spinner

### Dots Spinner

**Code:**

```rust
let spinner = Spinner::new()
    .spinner_style(SpinnerStyle::Dots)
    .frame(0);
```

**Render:**

```
⠋         
```

### Bars Spinner

**Code:**

```rust
let spinner = Spinner::new()
    .spinner_style(SpinnerStyle::Bars)
    .frame(0);
```

**Render:**

```
|    
```

