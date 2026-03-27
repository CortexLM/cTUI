# Select Components

Dropdown selection and combo box widgets.

## Select (Closed)

### Closed State

**Code:**

```rust
let select = Select::new()
    .items(vec![
        SelectItem::new("Option 1"),
        SelectItem::new("Option 2"),
    ])
    .selected(Some(0));
```

**Render:**

```
Option 1     ▼ 
```

### Open State

**Code:**

```rust
let select = Select::new()
    .items(vec![...])
    .open(true)
    .highlighted(1);
```

**Render:**

```
Select...    ▲ 
Apple          
               
               
               
```

### With Placeholder

**Code:**

```rust
let select = Select::new()
    .items(vec![...])
    .placeholder("Choose...");
```

**Render:**

```
Choose...    ▼ 
```

