# Tabs Component

Tabbed navigation component.

## Variants

### Basic Tabs

**Code:**

```rust
let tabs = Tabs::titles(["Tab 1", "Tab 2", "Tab 3"])
    .with_selected(0);
```

**Render:**

```
 Tab 1 │ Tab 2 │ Tab 3   
```

### Second Tab Selected

**Code:**

```rust
let tabs = Tabs::titles(["Home", "Settings", "About"])
    .with_selected(1);
```

**Render:**

```
 Home │ Settings │ About 
```

