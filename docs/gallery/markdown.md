# Markdown Component

Render Markdown content with styling.

## Variants

### Basic Markdown

**Code:**

```rust
let markdown = Markdown::new(r#"
# Heading 1
## Heading 2

This is **bold** and *italic* text.

- List item 1
- List item 2

1. Numbered item
2. Another item

> Blockquote

`inline code`

```
code block
```
"#);
```

**Render:**

```
Heading 1
════════
Heading 2
──────────

This is bold and italic text.

• List item 1
• List item 2

1. Numbered item
2. Another item

  Blockquote

inline code

┌─────────┐
│code block│
└─────────┘
```

### Styled Markdown

**Code:**

```rust
let theme = MarkdownTheme::dark();
let markdown = Markdown::new(content)
    .theme(theme);
```

## Supported Elements

| Element | Syntax | Render |
|---------|--------|--------|
| Heading | `# H1` | Underlined |
| Bold | `**text**` | Bold modifier |
| Italic | `*text*` | Italic modifier |
| Code | `` `code` `` | Highlighted |
| Code Block | `` ``` `` | Block with border |
| Link | `[text](url)` | Colored, underlined |
| List | `- item` | Bullet points |
| Ordered | `1. item` | Numbers |
| Blockquote | `> quote` | Indented |
| Horizontal | `---` | Separator |

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `content` | `String` | "" | Markdown text |
| `theme` | `MarkdownTheme` | default | Colors and styles |
| `max_width` | `Option<u16>` | None | Soft line wrap |

## MarkdownTheme

```rust
let theme = MarkdownTheme {
    heading_style: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    bold_style: Style::default().add_modifier(Modifier::BOLD),
    italic_style: Style::default().add_modifier(Modifier::ITALIC),
    code_style: Style::default().fg(Color::Yellow),
    link_style: Style::default().fg(Color::Blue).add_modifier(Modifier::UNDERLINED),
    quote_style: Style::default().fg(Color::Gray),
};
```

## Example

```rust
use ctui_components::Markdown;

fn render(&self, area: Rect, buf: &mut Buffer) {
    let readme = r#"
    # Project Name
    
    Brief description of the project.
    
    ## Features
    
    - Feature 1
    - Feature 2
    
    ## Installation
    
    ```bash
    cargo add ctui
    ```
    "#;
    
    let markdown = Markdown::new(readme);
    markdown.render(area, buf);
}
```

## Inline Parsing

Parse markdown without rendering:

```rust
use ctui_components::{parse_markdown, MarkdownNode};

let nodes = parse_markdown(markdown_text);
for node in nodes {
    match node {
        MarkdownNode::Heading(level, text) => { /* ... */ }
        MarkdownNode::Paragraph(inlines) => { /* ... */ }
        MarkdownNode::List(items) => { /* ... */ }
        MarkdownNode::CodeBlock(language, code) => { /* ... */ }
        _ => {}
    }
}
```

## See Also

- [Paragraph](paragraph.md) - Plain text display
- [Code](code.md) - Syntax-highlighted code
