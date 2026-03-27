# Code Component

Syntax-highlighted code blocks.

## Variants

### Rust Code

**Code:**

```rust
let code = Code::new(r#"fn main() {
    println!("Hello, World!");
}"#)
.language(Language::Rust);
```

**Render:**

```
fn main() {
    println!("Hello, World!");
}
```

### With Line Numbers

**Code:**

```rust
let code = Code::new(source)
    .language(Language::Rust)
    .line_numbers(true);
```

**Render:**

```
 1 │ fn main() {
 2 │     println!("Hello, World!");
 3 │ }
 4 │ 
```

### Highlighted Line

**Code:**

```rust
let code = Code::new(source)
    .language(Language::Rust)
    .highlight_line(2)
    .highlight_style(Style::default().bg(Color::Yellow));
```

**Render:**

```
 1 │ fn main() {
 2 │   println!("Hello, World!");  <-- highlighted
 3 │ }
```

## Supported Languages

| Language | Enum Value |
|----------|------------|
| Rust | `Language::Rust` |
| JavaScript | `Language::JavaScript` |
| TypeScript | `Language::TypeScript` |
| Python | `Language::Python` |
| Go | `Language::Go` |
| C | `Language::C` |
| C++ | `Language::Cpp` |
| Java | `Language::Java` |
| JSON | `Language::Json` |
| TOML | `Language::Toml` |
| YAML | `Language::Yaml` |
| HTML | `Language::Html` |
| CSS | `Language::Css` |
| Shell | `Language::Shell` |
| SQL | `Language::Sql` |
| Markdown | `Language::Markdown` |

## CodeTheme

```rust
let theme = CodeTheme::dark();   // Dark background
let theme = CodeTheme::light();  // Light background
let theme = CodeTheme::mono();   // Monochrome

let code = Code::new(source)
    .theme(theme);
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `source` | `String` | "" | Code text |
| `language` | `Language` | Text | Language for highlighting |
| `theme` | `CodeTheme` | dark | Color theme |
| `line_numbers` | `bool` | false | Show line numbers |
| `highlight_line` | `Option<usize>` | None | Line to highlight |
| `start_line` | `usize` | 1 | First line number |

## Token Access

Get highlighted tokens:

```rust
let tokens = Code::new(source).tokens();
for token in tokens {
    match token.kind {
        TokenKind::Keyword => { /* ... */ }
        TokenKind::String => { /* ... */ }
        TokenKind::Number => { /* ... */ }
        TokenKind::Comment => { /* ... */ }
        TokenKind::Function => { /* ... */ }
        _ => {}
    }
}
```

## Example

```rust
use ctui_components::{Code, Language, CodeTheme};

fn render(&self, area: Rect, buf: &mut Buffer) {
    let source = r#"
    use ctui_core::{Buffer, Component, Rect};
    
    struct MyComponent;
    
    impl Component for MyComponent {
        fn render(&self, area: Rect, buf: &mut Buffer) {
            // Render logic
        }
    }
    "#;
    
    let code = Code::new(source)
        .language(Language::Rust)
        .line_numbers(true)
        .theme(CodeTheme::dark());
    
    code.render(area, buf);
}
```

## See Also

- [Markdown](markdown.md) - Markdown with code blocks
- [Diff](diff.md) - Side-by-side diff viewer
- [Editor](editor.md) - Text editor
