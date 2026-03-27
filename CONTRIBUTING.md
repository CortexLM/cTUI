# Contributing to cTUI

Thanks for your interest in contributing to cTUI! This document outlines the guidelines for contributing to the project.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you agree to uphold this code. Please report any unacceptable behavior to the maintainers.

## AI Generated Content

We welcome high quality contributions, whether they are human generated or made with the assistance of AI tools. Please follow these guidelines:

- **Attribution**: Tell us about your use of AI tools. Don't make us guess whether you're using it.
- **Review**: Review every line of AI generated content for correctness and relevance.
- **Quality**: AI-generated content should meet the same quality standards as human-written content.
- **Quantity**: Avoid submitting large amounts of AI generated content in a single PR. Quality over quantity.
- **License**: Ensure that AI-generated content is compatible with cTUI's [MIT License](LICENSE-MIT).

> [!IMPORTANT]
> AI tools can assist in generating content, but you are responsible for the final quality and accuracy of the contributions.

## Development Setup

### Prerequisites

- Rust 1.75 or later (install via [rustup](https://rustup.rs/))
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/CortexLM/cTUI.git
cd cTUI

# Build the project
cargo build

# Run tests
cargo test --all

# Run clippy
cargo clippy --all-targets --all-features

# Format code
cargo fmt --all

# Run an example
cargo run --example counter
```

### Project Structure

```
cTUI/
├── ctui-core/        # Core rendering primitives
├── ctui-macros/      # Procedural macros
├── ctui-layout/      # Flexbox layout engine
├── ctui-components/  # Built-in widgets
├── ctui-animate/     # Animation system
├── ctui-theme/       # Theming and styling
├── ctui-cli/         # CLI tool and templates
├── ctui-tests/       # Integration tests
├── examples/         # Example applications
└── benches/          # Benchmarks
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed information about crate responsibilities.

## Reporting Issues

Before reporting an issue, please:

1. Search existing issues to avoid duplicates
2. Check if the issue exists in the latest version
3. Use the appropriate issue template

When reporting bugs, include:
- A minimal reproducible example
- Your environment (OS, terminal, Rust version)
- Expected vs actual behavior

## Pull Requests

### PR Guidelines

1. **Keep PRs small and focused** - One change per PR when possible
2. **Reference related issues** - Link any related issues in your PR description
3. **Write clear commit messages** - Follow Conventional Commits
4. **Add tests** - New features need tests
5. **Update documentation** - Public APIs must be documented

### PR Size Guidelines

- Ideal: Under 500 lines of changes
- Split large features into incremental PRs
- Separate refactoring from functional changes
- If a large PR is unavoidable, explain why

### Breaking Changes

We prioritize backwards compatibility:

- Prefer deprecation over removal
- Provide migration paths for breaking changes
- Document breaking changes in PR descriptions
- Wait at least two versions before removing deprecated items

## Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/). Format:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks

### Examples

```
feat(components): add Scrollable widget

fix(layout): prevent overflow in flex calculation

docs(readme): update installation instructions
```

## Testing Guidelines

### Running Tests

```bash
# Run all tests
cargo test --all

# Run tests for a specific crate
cargo test -p ctui-core

# Run a specific test
cargo test test_buffer_diff -- --nocapture
```

### Writing Tests

- Write unit tests in the same file as the code
- Write integration tests in the `tests/` directory
- Use doc tests for public APIs
- Test edge cases and error conditions

### Test Coverage

Good test coverage is important. Focus on:

- Public API behavior
- Edge cases
- Error handling
- Performance critical paths

## Documentation Guidelines

### Code Documentation

Every public API must be documented:

```rust
/// Brief description.
///
/// Detailed description with examples.
///
/// # Examples
///
/// ```
/// use ctui::Buffer;
/// let buf = Buffer::empty(Rect::new(0, 0, 10, 10));
/// ```
fn my_function() {}
```

### Style Guide

- First line is summary, second is blank, third onward is detail
- Max line length: 100 characters
- Use backticks for code items: `Buffer`, not Buffer
- Document parameters and return values

## Architecture Decisions

When making changes, consider:

1. Which crate should contain the changes
2. Whether changes affect the public API of `ctui-core`
3. How changes fit into the overall architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for crate responsibilities.

## Continuous Integration

CI runs on every pull request:

- Build on stable and MSRV
- Run all tests
- Run clippy
- Check formatting

Run locally before pushing:

```bash
cargo test --all && cargo clippy --all-targets --all-features && cargo fmt --all -- --check
```

## Getting Help

- Open a [Discussion](https://github.com/CortexLM/cTUI/discussions) for questions
- Join our community chat (coming soon)
- Check existing issues and documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
