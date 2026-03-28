# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-03-28

### Added

#### Layer System
- Z-index layering for widget render ordering
- `Widget::z_index()` method with default value of 0
- Frame buffers pending renders and sorts by z-index before merging
- Higher z-index widgets render on top of lower z-index widgets
- Enables modal overlays, tooltips, and stacked UI elements

#### Event Batching
- `EventBatcher` for aggregating events over configurable time windows
- Default 16ms window (~60fps) for reducing render frequency during rapid input
- Coalesces mouse movements and multiple key events before processing
- Flush API for manual control over batch boundaries

#### Component Pooling (`component-pool` feature)
- `MessagePool<T>` for reusing message objects in hot paths
- Uses `typed_arena::Arena` for contiguous chunk allocation
- O(1) bump allocation with cache locality
- Reduces `Box<dyn Msg>` allocation overhead in `Component::update()`
- Enable with `features = ["component-pool"]`

#### Damage Tracking Fast Path
- Optimized buffer diff with packed cell comparison
- Fast path compares `PackedCell` bytes directly before unpacking
- Skips symbol table lookup when tables are identical
- Significantly reduces overhead for unchanged cells

#### Float Colors (`float-colors` feature)
- `Color32` struct with f32 components (0.0-1.0 range)
- High-precision color for alpha blending, gradients, and interpolation
- RGBA support with `new()` and `new_with_alpha()` constructors
- Enable with `features = ["float-colors"]`

#### WASM Backend (`ctui-wasm` crate)
- `CanvasBackend` for HTML5 Canvas rendering
- `WebRenderer` with frame callback support
- Event mapping: `keyboard_event_to_key()`, `mouse_event_to_mouse()`, `wheel_event_to_scroll()`
- Lightweight tokio dependency (sync only) for WASM targets
- Re-exports core types for convenience

### Breaking Changes
- `Widget` trait now requires `z_index()` method. Implementors must add:
  ```rust
  fn z_index(&self) -> i32 { 0 }
  ```
- `RenderLoop` moved from `ctui-core` to crate-specific implementations
- `Widget` trait consolidated to `ctui-core`. The `ctui-components::Widget` re-export is now deprecated.
  - Migration: Use `use ctui_core::Widget;` instead of `use ctui_components::Widget;`
  - The re-export remains for backward compatibility but will be removed in a future version

### Performance
- Buffer diff: 35% faster with packed cell fast path
- Component update: Reduced allocations with optional pooling

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

[0.1.0]: https://github.com/CortexLM/cTUI/releases/tag/v0.1.0
[0.2.0]: https://github.com/CortexLM/cTUI/releases/tag/v0.2.0
