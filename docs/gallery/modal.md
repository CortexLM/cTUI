# Modal Component

Modal dialog overlay for alerts, confirmations, and forms.

## Variants

### Alert Modal

**Code:**

```rust
let modal = Modal::new()
    .title("Alert")
    .body("Operation completed successfully!")
    .button("OK");
```

**Render:**

```
         ╭ Alert ──────────────╮
         │                      │
         │ Operation completed  │
         │ successfully!        │
         │                      │
         │       [ OK ]         │
         ╰──────────────────────╯
```

### Confirmation Modal

**Code:**

```rust
let modal = Modal::new()
    .title("Confirm")
    .body("Are you sure you want to delete this item?")
    .buttons(vec![
        ModalButton::new("Cancel"),
        ModalButton::new("Delete").style(danger_style),
    ]);
```

**Render:**

```
         ╭ Confirm ─────────────╮
         │                      │
         │ Are you sure you     │
         │ want to delete?      │
         │                      │
         │ [Cancel]  [Delete]   │
         ╰──────────────────────╯
```

### Form Modal

**Code:**

```rust
let modal = Modal::new()
    .title("New Item")
    .content(Form::new()
        .field(FormField::new("name").label("Name"))
        .field(FormField::new("email").label("Email")))
    .buttons(vec![
        ModalButton::new("Cancel"),
        ModalButton::new("Save"),
    ]);
```

**Render:**

```
         ╭ New Item ────────────╮
         │ Name:  [________]    │
         │ Email: [________]    │
         │                      │
         │ [Cancel]  [Save]     │
         ╰──────────────────────╯
```

### Sizes

**Code:**

```rust
ModalSize::Small    // ~30% width
ModalSize::Medium   // ~50% width
ModalSize::Large    // ~70% width
ModalSize::Full     // Full screen
```

### Alignments

**Code:**

```rust
ModalAlignment::Top     // Top of screen
ModalAlignment::Center  // Center (default)
ModalAlignment::Bottom  // Bottom of screen
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `title` | `String` | "" | Modal title |
| `body` | `String` | "" | Body text |
| `content` | `Box<Widget>` | None | Custom content |
| `buttons` | `Vec<ModalButton>` | [] | Action buttons |
| `size` | `ModalSize` | Medium | Modal size |
| `alignment` | `ModalAlignment` | Center | Position |
| `backdrop` | `bool` | true | Show backdrop |
| `dismissible` | `bool` | true | Can be dismissed |

## ModalButton

```rust
let button = ModalButton::new("Label")
    .action(ModalAction::Submit)
    .style(Style::default().fg(Color::Green))
    .key_bind(KeyCode::Enter);
```

## ModalAction

```rust
pub enum ModalAction {
    Submit,    // Return result
    Close,     // Close without result
    Custom(String), // Custom action
}
```

## Events

| Event | Action |
|-------|--------|
| `Esc` | Close modal |
| `Tab` | Next button |
| `Enter` | Activate focused button |
| `MouseClick` | Click button |

## Example

```rust
use ctui_components::{Modal, ModalButton, ModalAction, ModalSize};

struct App {
    show_confirm: bool,
    pending_delete: Option<usize>,
}

impl Component for App {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render main content...
        
        if self.show_confirm {
            let modal = Modal::new()
                .title("Confirm Delete")
                .body("This action cannot be undone.")
                .buttons(vec![
                    ModalButton::new("Cancel")
                        .action(ModalAction::Close),
                    ModalButton::new("Delete")
                        .action(ModalAction::Submit),
                ])
                .size(ModalSize::Small);
            
            modal.render(area, buf);
        }
    }
}
```

## See Also

- [Form](form.md) - Form component
- [Tabs](tabs.md) - Tabbed navigation
