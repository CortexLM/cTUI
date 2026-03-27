# Diff Component

Side-by-side diff viewer for comparing text.

## Variants

### Basic Diff

**Code:**

```rust
let diff = DiffViewer::new()
    .old_content(original)
    .new_content(modified);
```

**Render:**

```
Original          │ Modified
──────────────────┼─────────────────
Hello, World!     │ Hello, cTUI!   
This is a test.   │ This is a test. 
Extra line        │                 
                  │ New line added
```

### With Line Numbers

**Code:**

```rust
let diff = DiffViewer::new()
    .old_content(original)
    .new_content(modified)
    .line_numbers(true);
```

**Render:**

```
    1 │ Hello, World!     │     1 │ Hello, cTUI!
    2 │ This is a test.   │     2 │ This is a test.
    3 │ Extra line        │       │ 
      │                   │     3 │ New line added
```

### Unified Diff View

**Code:**

```rust
let diff = DiffViewer::new()
    .old_content(original)
    .new_content(modified)
    .mode(DiffMode::Unified);
```

**Render:**

```
--- Original
+++ Modified
@@ -1,3 +1,3 @@
-Hello, World!
+Hello, cTUI!
 This is a test.
-Extra line
+New line added
```

## Diff Algorithms

```rust
pub enum DiffAlgorithm {
    Myers,     // Fast, standard algorithm
    Patience,  // Better for moved blocks
    Histogram, // Better for similar changes
}
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `old_content` | `String` | "" | Original text |
| `new_content` | `String` | "" | Modified text |
| `algorithm` | `DiffAlgorithm` | Myers | Diff algorithm |
| `mode` | `DiffMode` | SideBySide | Display mode |
| `line_numbers` | `bool` | true | Show line numbers |
| `context_lines` | `usize` | 3 | Context around changes |

## DiffHunk

Access diff hunks:

```rust
let hunks = diff.hunks();
for hunk in hunks {
    match hunk {
        DiffHunk::Unchanged(lines) => { /* ... */ }
        DiffHunk::Added(lines) => { /* ... */ }
        DiffHunk::Removed(lines) => { /* ... */ }
    }
}
```

## DiffLine

```rust
pub struct DiffLine {
    pub old_line: Option<usize>,
    pub new_line: Option<usize>,
    pub content: String,
    pub kind: DiffLineKind,
}

pub enum DiffLineKind {
    Context,
    Added,
    Removed,
}
```

## Styling

```rust
let diff = DiffViewer::new()
    .old_content(original)
    .new_content(modified)
    .added_style(Style::default().fg(Color::Green))
    .removed_style(Style::default().fg(Color::Red));
```

## Example

```rust
use ctui_components::{DiffViewer, DiffAlgorithm, DiffMode};

fn render(&self, area: Rect, buf: &mut Buffer) {
    let original = r#"
    fn main() {
        println!("Hello");
    }
    "#;
    
    let modified = r#"
    fn main() {
        let name = "cTUI";
        println!("Hello, {}!", name);
    }
    "#;
    
    let diff = DiffViewer::new()
        .old_content(original)
        .new_content(modified)
        .mode(DiffMode::SideBySide)
        .algorithm(DiffAlgorithm::Myers)
        .line_numbers(true);
    
    diff.render(area, buf);
}
```

## See Also

- [Code](code.md) - Syntax-highlighted code
- [Editor](editor.md) - Text editor
