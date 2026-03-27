# Checkbox Component

Checkbox input for boolean selections.

## Variants

### Unchecked

**Code:**

```rust
let checkbox = Checkbox::new("Enable notifications");
```

**Render:**

```
[ ] Enable notifications
```

### Checked

**Code:**

```rust
let checkbox = Checkbox::new("Enable notifications")
    .checked(true);
```

**Render:**

```
[x] Enable notifications
```

### With Style

**Code:**

```rust
let checkbox = Checkbox::new("Remember me")
    .checked(true)
    .style(Style::default().fg(Color::Cyan));
```

**Render:**

```
[x] Remember me
```

## CheckboxGroup

Group multiple checkboxes together.

### Multiple Options

**Code:**

```rust
let group = CheckboxGroup::new()
    .item("Option A", true)
    .item("Option B", false)
    .item("Option C", true);
```

**Render:**

```
[x] Option A
[ ] Option B
[x] Option C
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `checked` | `bool` | false | Current state |
| `label` | `String` | "" | Label text |
| `style` | `Style` | default | Label style |
| `check_style` | `Style` | default | Checkbox marker style |

## Events

The checkbox responds to:

| Event | Action |
|-------|--------|
| `Enter` | Toggle state |
| `Space` | Toggle state |
| `MouseClick` | Toggle state |

## Example

```rust
use ctui_components::{Checkbox, CheckboxGroup};

fn render(&self, area: Rect, buf: &mut Buffer) {
    let group = CheckboxGroup::new()
        .item("Dark mode", self.dark_mode)
        .item("Notifications", self.notifications)
        .item("Auto-save", self.auto_save);
    
    group.render(area, buf);
}
```

## See Also

- [Radio](radio.md) - Radio button groups
- [Select](select.md) - Dropdown selection
- [Form](form.md) - Form with validation
