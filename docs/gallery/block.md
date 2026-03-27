# Block Component

A container widget with borders, padding, and optional titles.

## Variants

### Plain Border

**Code:**

```rust
let block = Block::new().borders(Borders::ALL);
```

**Render:**

```
┌─────────────┐
│             │
│             │
│             │
└─────────────┘
```

### Rounded Border

**Code:**

```rust
let block = Block::new()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded);
```

**Render:**

```
╭─────────────╮
│             │
│             │
│             │
╰─────────────╯
```

### Double Border

**Code:**

```rust
let block = Block::new()
    .borders(Borders::ALL)
    .border_type(BorderType::Double);
```

**Render:**

```
╔═════════════╗
║             ║
║             ║
║             ║
╚═════════════╝
```

### With Title

**Code:**

```rust
let block = Block::new()
    .borders(Borders::ALL)
    .title("Title");
```

**Render:**

```
┌ Title ───────────┐
│                  │
│                  │
│                  │
└──────────────────┘
```

### Centered Title

**Code:**

```rust
let block = Block::new()
    .borders(Borders::ALL)
    .title_with_alignment("Title", Alignment::Center);
```

**Render:**

```
┌───── Title ──────┐
│                  │
│                  │
│                  │
└──────────────────┘
```

### Top and Bottom Borders Only

**Code:**

```rust
let block = Block::new().borders(Borders::TOP | Borders::BOTTOM);
```

**Render:**

```
───────────────
               
               
               
───────────────
```

