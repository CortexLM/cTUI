<div align="center">

# cTUI

**A high-performance TUI framework for Rust with React-style declarative components**

[![Crate](https://img.shields.io/crates/v/ctui?logo=rust&style=flat-square&color=E05D44)][crates.io]
[![Repo](https://img.shields.io/badge/repo-CortexLM/cTUI-1370D3?style=flat-square&logo=github)][repo]
[![Docs](https://img.shields.io/badge/docs-ctui-1370D3?style=flat-square&logo=rust)][docs.rs]
[![License](https://img.shields.io/crates/l/ctui?style=flat-square&color=1370D3)][license]
[![CI](https://img.shields.io/github/actions/workflow/status/CortexLM/cTUI/ci.yml?style=flat-square&logo=github)][ci]

[cTUI Website] · [Docs] · [Examples] · [Changelog] · [Contributing] · [Report a Bug] · [Request a Feature]

</div>

cTUI (_see-too-eye_) is a Rust crate for building terminal user interfaces with a declarative, component-based approach. It brings modern frontend paradigms to the terminal, making it easy to create responsive, animated, and performant TUIs.

## Features

- **Layer System** - Z-index based layering for proper render ordering. Widgets can specify `z_index()` to control which elements appear on top.
- **Event Batching** - Efficiently batch and process terminal events. Reduces overhead by coalescing rapid input sequences.
- **Component Pooling** - Reuse component instances to minimize allocations and improve performance in frequently updated UIs.
- **WASM Backend** - Compile to WebAssembly for browser-based terminals. Run your TUI in the browser with full feature parity.
- **Float Colors** - `Color32` type for high-precision color manipulation (0.0-1.0 range). Enable with the `float-colors` feature.
- **React-style Components** - Declarative components with hooks (`use_state`, `use_effect`) for familiar state management.

## Quickstart

Add cTUI to your project:

\`\`\`shell
cargo add ctui
\`\`\`

Here's a minimal counter application:

\`\`\`rust
use ctui::{component, App, Component, Terminal};
use ctui::hooks::{use_state, use_effect};

#[component]
fn Counter() -> impl Component {
    let (count, set_count) = use_state(0);
    
    use_effect(move || {
        println!("Count: {}", count);
    }, [count]);
    
    Column::new()
        .child(Text::new(&format!("Count: {}", count)))
        .child(
            Row::new()
                .child(Button::new("-").on_click(move || set_count(count - 1)))
                .child(Button::new("+").on_click(move || set_count(count + 1)))
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = Terminal::new()?;
    let app = App::new(Counter {});
    terminal.run(app).await
}
\`\`\`

## Documentation

- [Docs] - Full API documentation on docs.rs
- [ARCHITECTURE.md] - Crate organization and design decisions
- [CHANGELOG.md] - Version history and release notes
- [Examples] - Complete example applications

## Templates

Get started quickly with project templates:

\`\`\`shell
cargo install ctui-cli
ctui new my-app --template counter
\`\`\`

Available templates:
- \`basic\` - Minimal hello world application
- \`counter\` - Counter app with state management
- \`todo-app\` - Full CRUD todo application

## Built with cTUI

[![Built with cTUI](https://img.shields.io/badge/Built_With_cTUI-000?style=flat-square)](https://github.com/CortexLM/cTUI)

Building something with cTUI? Add a badge to your README:

\`\`\`md
[![Built with cTUI](https://img.shields.io/badge/Built_With_cTUI-000?style=flat-square)](https://github.com/CortexLM/cTUI)
\`\`\`

## Alternatives

If cTUI doesn't fit your needs, consider these alternatives:

- [ratatui](https://crates.io/crates/ratatui) - Imperative TUI library with excellent widget ecosystem
- [cursive](https://crates.io/crates/cursive) - ncurses-based TUI library
- [iocraft](https://crates.io/crates/iocraft) - Declarative TUI library with React-like API

## Contributing

We welcome contributions! Whether you're fixing a bug, adding a feature, or improving documentation, your help makes cTUI better.

Please read the [Contributing Guidelines](CONTRIBUTING.md) before submitting a pull request. Key points:

- Follow the [Code of Conduct](CODE_OF_CONDUCT.md)
- Use [Conventional Commits](https://www.conventionalcommits.org/)
- Run `cargo test --all` before pushing
- AI-generated content must follow our [AI Content Policy](CONTRIBUTING.md#ai-generated-content)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

cTUI draws inspiration from several projects:

- [ratatui](https://github.com/ratatui-org/ratatui) for pushing the Rust TUI ecosystem forward
- [React](https://react.dev) for pioneering the declarative component model
- The Rust community for creating amazing tools and libraries

---

Built with care by the [CortexLM](https://github.com/CortexLM) team.

[cTUI Website]: https://github.com/CortexLM/cTUI
[Docs]: https://docs.rs/ctui
[Examples]: https://github.com/CortexLM/cTUI/tree/main/examples
[Changelog]: https://github.com/CortexLM/cTUI/blob/main/CHANGELOG.md
[Contributing]: https://github.com/CortexLM/cTUI/blob/main/CONTRIBUTING.md
[Report a Bug]: https://github.com/CortexLM/cTUI/issues/new?labels=bug&template=bug_report.md
[Request a Feature]: https://github.com/CortexLM/cTUI/issues/new?labels=enhancement&template=feature_request.md
[ARCHITECTURE.md]: https://github.com/CortexLM/cTUI/blob/main/ARCHITECTURE.md
[CHANGELOG.md]: https://github.com/CortexLM/cTUI/blob/main/CHANGELOG.md
[crates.io]: https://crates.io/crates/ctui
[repo]: https://github.com/CortexLM/cTUI
[docs.rs]: https://docs.rs/ctui
[license]: ./LICENSE-MIT
[ci]: https://github.com/CortexLM/cTUI/actions/workflows/ci.yml
