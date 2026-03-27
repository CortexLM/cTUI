# ctui-components - AGENTS.md

## OVERVIEW

27 pre-built widgets. Highest complexity crate. Each widget in its own file with matching `{widget}.rs` pattern.

## WHERE TO LOOK

| Widget | File | Notes |
|--------|------|-------|
| Block | `block.rs` | Container with borders, padding, title |
| Paragraph | `paragraph.rs` | Multi-line text with alignment |
| List | `list.rs` | Scrollable list, selection modes |
| Table | `table.rs` | Tabular data, sorting |
| Input | `input.rs` | Single-line text editing |
| Editor | `editor.rs` | Multi-line textarea |
| Chart | `chart.rs` | ASCII charts |
| Form | `form.rs` | Form fields, validation |
| Modal | `modal.rs` | Dialog overlay |
| Tabs | `tabs.rs` | Tabbed navigation |
| ProgressBar | `progress.rs` | Progress, Spinner |
| Markdown | `markdown.rs` | Markdown rendering |
| Code | `code.rs` | Syntax highlighting |

## KEY TYPES

```rust
Widget trait              // Stateless rendering
WidgetExt                 // to_buffer(), render_to_string()
Block, Paragraph, List, Table, Input, Editor, Form, Modal, Tabs...
Props structs: {Widget}Props for each
```

## CONVENTIONS

- Each widget exports: `Widget`, `WidgetProps`
- Props use builder pattern with defaults
- Snapshot tests in `snapshots/` directory
- Complex widgets have associated State types
