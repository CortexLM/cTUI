# Radio Component

Radio button group for single selection.

## Variants

### Basic Radio Group

**Code:**

```rust
let radio = RadioGroup::new()
    .item("Small")
    .item("Medium")
    .item("Large");
```

**Render:**

```
( ) Small
( ) Medium
( ) Large
```

### With Selection

**Code:**

```rust
let radio = RadioGroup::new()
    .item("Option A")
    .item("Option B")
    .item("Option C")
    .selected(1);
```

**Render:**

```
( ) Option A
(•) Option B
( ) Option C
```

### Horizontal Layout

**Code:**

```rust
let radio = RadioGroup::new()
    .items(vec![
        RadioItem::new("Red").value("r"),
        RadioItem::new("Green").value("g"),
        RadioItem::new("Blue").value("b"),
    ])
    .selected(0)
    .horizontal(true);
```

**Render:**

```
(•) Red  ( ) Green  ( ) Blue
```

## RadioItem

Individual radio item with value.

```rust
let item = RadioItem::new("Label")
    .value("value")
    .disabled(false);
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `items` | `Vec<RadioItem>` | [] | Available options |
| `selected` | `Option<usize>` | None | Selected index |
| `horizontal` | `bool` | false | Horizontal layout |
| `style` | `Style` | default | Text style |
| `highlight_style` | `Style` | default | Selected style |

## Events

| Event | Action |
|-------|--------|
| `Up` / `k` | Previous option |
| `Down` / `j` | Next option |
| `Enter` / `Space` | Select option |

## Example

```rust
use ctui_components::{RadioGroup, RadioItem};

struct SizePicker {
    selected: usize,
}

impl Component for SizePicker {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let radio = RadioGroup::new()
            .items(vec![
                RadioItem::new("Small (S)").value("s"),
                RadioItem::new("Medium (M)").value("m"),
                RadioItem::new("Large (L)").value("l"),
                RadioItem::new("Extra Large (XL)").value("xl"),
            ])
            .selected(self.selected);
        
        radio.render(area, buf);
    }
}
```

## See Also

- [Checkbox](checkbox.md) - Multiple selections
- [Select](select.md) - Dropdown selection
- [Form](form.md) - Form with validation
