# Link Component

Clickable link for navigation and actions.

## Variants

### Basic Link

**Code:**

```rust
let link = Link::new("Click here")
    .url("https://example.com");
```

**Render:**

```
Click here
^^^^^^^^^
(underlined)
```

### Styled Link

**Code:**

```rust
let link = Link::new("Documentation")
    .url("https://docs.example.com")
    .style(Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::UNDERLINED));
```

**Render:**

```
Documentation
(Cyan, underlined)
```

### Action Link

**Code:**

```rust
let link = Link::new("Open Settings")
    .on_click(|| {
        // Handle click
        println!("Clicked!");
    });
```

### With Icon

**Code:**

```rust
let link = Link::new(" GitHub")
    .url("https://github.com")
    .icon("🔗");
```

**Render:**

```
🔗 GitHub
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `text` | `String` | "" | Link text |
| `url` | `String` | "" | Target URL |
| `style` | `Style` | blue + underline | Link style |
| `hover_style` | `Style` | cyan + underline | Hover style |
| `icon` | `String` | "" | Optional icon |
| `on_click` | `Option<fn()>` | None | Click handler |

## LinkProps

```rust
pub struct LinkProps {
    pub text: String,
    pub url: Option<String>,
    pub on_click: Option<Box<dyn Fn()>>,
    pub style: Style,
    pub hover_style: Style,
}
```

## Events

| Event | Action |
|-------|--------|
| `Enter` | Trigger click/open URL |
| `MouseClick` | Trigger click/open URL |
| `MouseHover` | Apply hover style |

## Example

```rust
use ctui_components::Link;

struct Menu {
    links: Vec<Link>,
}

impl Menu {
    fn new() -> Self {
        Self {
            links: vec![
                Link::new("Home")
                    .url("/"),
                Link::new("Documentation")
                    .url("/docs"),
                Link::new("GitHub")
                    .url("https://github.com/example/app"),
            ],
        }
    }
}

impl Component for Menu {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::flex()
            .direction(FlexDirection::Column)
            .gap(1);
        
        let rects = layout.split(area, &vec![
            Constraint::Length(1);
            self.links.len()
        ]);
        
        for (link, rect) in self.links.iter().zip(rects.iter()) {
            link.render(*rect, buf);
        }
    }
}
```

## External Links

When a URL is set and no `on_click` handler, clicking the link attempts to open it:

```rust
Link::new("Open GitHub")
    .url("https://github.com");
// Click opens: https://github.com
```

## Internal Navigation

For in-app navigation:

```rust
Link::new("Go to Settings")
    .on_click(|| {
        // Navigate to settings
        navigator::go("/settings");
    });
```

## Accessibility

Links should have:

1. Clear, descriptive text
2. Visible focus state
3. Underline or distinct color
4. Hover feedback

```rust
Link::new("Read the documentation")
    .url("/docs")
    .style(Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::UNDERLINED))
    .hover_style(Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::UNDERLINED));
```

## See Also

- [Button](#) - Action buttons
- [Tabs](tabs.md) - Tab navigation
