# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-03-27

### Added

#### Core Framework
- Initial workspace structure with modular crate organization
- Buffer and Cell types for screen representation
- Backend trait for terminal abstraction
- Terminal struct for managing terminal state
- Geometry primitives (`Rect`, `Position`, `Size`)
- Style types (`Color`, `Modifier`, `Style`)
- Event system (`KeyEvent`, `MouseEvent`, `ResizeEvent`)

#### Component System
- Component trait for declarative UI elements
- Props system for component configuration
- State management with dispatch pattern
- `#[component]` procedural macro for reducing boilerplate
- Render loop with frame statistics

#### Layout Engine
- Flexbox-inspired layout system
- Flex direction (row, column)
- Justify content alignment
- Align items alignment
- Gap between children
- Grid layout with rows/columns
- Absolute positioning with z-index
- Layout validation

#### Built-in Components
- `Block` - Container with borders, padding, and title
- `Paragraph` - Multi-line text rendering
- `Text` and `Line` - Text content types
- `Input` - Single-line text input with cursor
- `List` and `ListItem` - Scrollable list with selection
- `Table` with columns and rows
- `Tabs` - Tabbed navigation
- `Modal` - Dialog overlay component
- `Scrollable` - Scrollable region with scrollbars
- `ProgressBar` and `Spinner` - Progress indicators
- `Chart` and `Sparkline` - Data visualization
- `Checkbox` and `CheckboxGroup` - Checkbox components
- `RadioGroup` and `RadioItem` - Radio button components
- `Select` and `ComboBox` - Selection components
- `Slider` - Slider component
- `Form` and `FormField` - Form with validation
- `Editor` and `Textarea` - Text editing
- `Code` with syntax highlighting support
- `DiffViewer` - Diff visualization
- `Markdown` - Markdown rendering
- `Tree` and `TreeNode` - Tree view component
- `Link` - Clickable link component

#### Animation System
- Easing functions library
  - Linear
  - Quadratic (ease-in, ease-out, ease-in-out)
  - Cubic (ease-in, ease-out, ease-in-out)
  - Quartic (ease-in, ease-out, ease-in-out)
  - Quintic (ease-in, ease-out, ease-in-out)
  - Sinusoidal (ease-in, ease-out, ease-in-out)
  - Exponential (ease-in, ease-out, ease-in-out)
  - Circular (ease-in, ease-out, ease-in-out)
  - Elastic (ease-in, ease-out, ease-in-out)
  - Back (ease-in, ease-out, ease-in-out)
  - Bounce (ease-in, ease-out, ease-in-out)
- Keyframe animations with playback modes
  - Once
  - Loop
  - Alternate
  - Reverse
- Spring physics animations
- Animation sequences and groups
- Global animation manager
- Animated style and layout wrappers

#### Theme System
- Color types (Named, Indexed, RGB)
- Style properties (fg, bg, modifiers, padding, margin)
- Built-in themes
  - Dark
  - Light
  - Tokyo Night
  - Dracula
  - Catppuccin
  - Nord
  - Gruvbox
- Theme loader (TOML support)
- Theme transitions
- Accessibility validation
  - Contrast ratio checking
  - WCAG compliance
  - Color blindness simulation

#### CLI Tool
- Project generator with templates
  - Basic template
  - Counter template
  - Todo-app template
- Template rendering

#### Examples
- `counter` - Counter app with state
- `todo` - Todo application with CRUD
- `dashboard` - Real-time data visualization
- `file_explorer` - File browser with async I/O
- `animation` - Animation showcase

### Performance
- Zero-allocation diff algorithm
- Cell-based updates for minimal redraws
- Layout caching
- Event batching

### Platform Support
- Linux
- macOS
- Windows
- Crossterm backend

## [Unreleased]

### Planned
- WebAssembly support
- Plugin system
- Additional themes
- More built-in components
- Performance benchmarks
- Comprehensive documentation

[0.1.0]: https://github.com/CortexLM/cTUI/releases/tag/v0.1.0
[Unreleased]: https://github.com/CortexLM/cTUI/compare/v0.1.0...HEAD
