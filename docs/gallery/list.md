# List Component

A scrollable list of items with selection support.

## Variants

### Basic List

**Code:**

```rust
let items = vec![
    ListItem::new("First"),
    ListItem::new("Second"),
    ListItem::new("Third"),
];
let list = List::new(items);
```

**Render:**

```
First          
Second         
Third          
```

### With Selection

**Code:**

```rust
let list = List::new(items)
    .select(Some(1));
```

**Render:**

```
Item A         
Item B         
Item C         
```

