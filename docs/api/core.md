# API Reference - Core

Core primitives for the cTUI framework.

## Modules

- [Buffer](#buffer) - Screen buffer and cell management
- [Cell](#cell) - Individual cell in a buffer
- [Backend](#backend) - Terminal backend abstraction
- [Terminal](#terminal) - Terminal management
- [Component](#component) - Component trait definition
- [State](#state) - State management
- [Events](#events) - Input event handling
- [Geometry](#geometry) - Position and size types
- [Style](#style) - Colors and modifiers

---

## Buffer

The `Buffer` type represents the terminal screen content.

```rust
use ctui_core::{Buffer, Rect};

// Create an empty buffer (80x24)
let buffer = Buffer::empty(Rect::new(0, 0, 80, 24));

// Create a filled buffer
let buffer = Buffer::filled(Rect::new(0, 0, 40, 10), Cell::new("X"));

// Access cells
let cell = buffer[(x, y)];  // Index access
let cell = buffer.get(x, y).unwrap();  // Checked access

// Set cells
buffer[(x, y)] = Cell::new("A");

// Resize buffer (preserves existing content)
buffer.resize(Rect::new(0, 0, 100, 30));

// Clear buffer
buffer.clear();

// Fill with style
buffer.fill(Style::new().fg(Color::Red).bg(Color::Black));
```

### Methods

| Method | Description |
|--------|-------------|
| `empty(area)` | Create empty buffer |
| `filled(area, cell)` | Create filled buffer |
| `new(area)` | Alias for `empty` |
| `area()` | Get buffer area |
| `get(x, y)` | Get cell reference |
| `get_mut(x, y)` | Get mutable cell reference |
| `set(x, y, cell)` | Set cell value |
| `len()` | Number of cells |
| `is_empty()` | Check if empty |
| `iter()` | Iterate over cells |
| `iter_mut()` | Iterate mutably |
| `reset()` | Reset all cells |
| `resize(area)` | Resize buffer |
| `clear()` | Clear to default |
| `clear_with(cell)` | Clear with specific cell |
| `fill(style)` | Apply style to all cells |
| `diff(other)` | Compare with other buffer |
| `copy_from(other)` | Copy from another buffer |
| `row(y)` | Get row slice |
| `row_mut(y)` | Get mutable row slice |

### BufferDiff

```rust
let old_buffer = Buffer::empty(area);
let new_buffer = /* ... after rendering ... */;

// Get cells that changed
for (x, y, cell) in new_buffer.diff(&old_buffer) {
    // Only update changed cells
    backend.draw_cell(x, y, cell);
}
```

---

## Cell

An individual cell in the buffer.

```rust
use ctui_core::Cell;

let cell = Cell::new("A");
let cell = Cell::new("🎉");  // Emoji support
let cell = Cell::new("漢");  // CJK support

// With style
let cell = Cell::new("X")
    .fg(Color::Red)
    .bg(Color::Black)
    .modifier(Modifier::BOLD);
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `symbol` | `String` | Character(s) to display |
| `fg` | `Color` | Foreground color |
| `bg` | `Color` | Background color |
| `modifier` | `Modifier` | Text modifiers |

### Methods

| Method | Description |
|--------|-------------|
| `new(s)` | Create cell with symbol |
| `default()` | Create empty cell |
| `set_fg(color)` | Set foreground |
| `set_bg(color)` | Set background |
| `set_style(style)` | Apply style |
| `reset()` | Reset to default |
| `width()` | Display width (1 or 2 for wide chars) |

---

## Backend

Abstract interface for terminal backends.

```rust
use ctui_core::backend::Backend;
use ctui_core::backend::CrosstermBackend;

// Create crossterm backend
let backend = CrosstermBackend::new()?;
let mut terminal = Terminal::new(backend)?;
```

### Backend Trait

```rust
pub trait Backend {
    fn draw<'a, I>(&mut self, content: I) -> Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;

    fn clear(&mut self) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
    fn size(&self) -> Result<Rect>;
    fn cursor_pos(&self) -> Result<(u16, u16)>;
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()>;
    fn show_cursor(&mut self) -> Result<()>;
    fn hide_cursor(&mut self) -> Result<()>;
    fn scroll_up(&mut self, n: u16) -> Result<()>;
    fn scroll_down(&mut self, n: u16) -> Result<()>;
    fn set_title(&mut self, title: &str) -> Result<()>;
    fn enter_alternate_screen(&mut self) -> Result<()>;
    fn leave_alternate_screen(&mut self) -> Result<()>;
}
```

### TestBackend

For unit testing:

```rust
use ctui_core::backend::test::TestBackend;

let backend = TestBackend::new(80, 24);

// Assert on rendered content
backend.assert_buffer(&expected);
backend.assert_buffer_lines(["Line 1", "Line 2"]);

// Get string representation
let output = backend.to_string();
```

---

## Terminal

Manages the terminal state and rendering.

```rust
use ctui_core::{Terminal, TerminalOptions};

let mut terminal = Terminal::new(backend)?;

// Get terminal size
let size = terminal.size()?;

// Draw a frame
let frame = terminal.draw(|f| {
    // Render your UI
})?;

// Clear terminal
terminal.clear()?;

// Handle resize
terminal.resize(Rect::new(0, 0, 100, 30))?;
```

### Frame

```rust
terminal.draw(|frame| {
    let area = frame.size();

    // Render widgets
    frame.render_widget(widget, area);
});
```

### LayoutCache

```rust
// Terminal caches layout computations
let metrics = terminal.cache_metrics();
println!("Cache hits: {}", metrics.hits);
println!("Cache misses: {}", metrics.misses);
```

---

## Component

The core trait for UI components.

```rust
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

// Define messages
struct MyMessage;
impl Msg for MyMessage {}

// Define component
struct MyComponent {
    // Internal state
    counter: u32,
}

impl Component for MyComponent {
    type Props = ();      // Props passed at creation
    type State = ();      // Shared state type

    // Constructor
    fn create(props: Self::Props) -> Self {
        Self { counter: 0 }
    }

    // Rendering
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Draw to buffer
    }

    // Message handling
    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        // Handle messages
        Cmd::Noop
    }

    // Lifecycle
    fn on_mount(&mut self) { /* Called when mounted */ }
    fn on_unmount(&mut self) { /* Called when unmounted */ }
}
```

### Msg Trait

```rust
use ctui_core::Msg;

// Define custom messages
struct Clicked { x: u16, y: u16 }
struct ValueChanged(String);

impl Msg for Clicked {}
impl Msg for ValueChanged {}
```

### Cmd Enum

```rust
pub enum Cmd {
    /// No operation
    Noop,
    /// Request a render
    Render,
    /// Async command
    Async(BoxFuture<'static, Box<dyn Msg>>),
    /// Multiple commands
    Batch(Vec<Cmd>),
}
```

---

## State

State management for complex applications.

```rust
use ctui_core::{State, Store};

// Define state
struct AppState {
    todos: Vec<TodoItem>,
    filter: Filter,
}

// Create store
let store = Store::new(AppState::default());

// Dispatch actions
store.dispatch(Action::AddTodo("Learn cTUI".into()));

// Subscribe to changes
store.subscribe(|state| {
    // React to state changes
});
```

---

## Events

Input event handling.

```rust
use ctui_core::{Event, KeyEvent, KeyCode, KeyModifiers, MouseEvent, MouseButton};

// Event types
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(ResizeEvent),
    FocusGained,
    FocusLost,
}
```

### KeyEvent

```rust
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

pub enum KeyCode {
    Char(char),
    Enter,
    Backspace,
    Delete,
    Tab,
    Esc,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
    Null,
}

pub struct KeyModifiers: u8 {
    const SHIFT = 0b0001;
    const CONTROL = 0b0010;
    const ALT = 0b0100;
    const SUPER = 0b1000;
}
```

### MouseEvent

```rust
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}

pub enum MouseEventKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Moved,
    ScrollDown,
    ScrollUp,
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
}
```

### Usage

```rust
impl Component for MyComponent {
    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => Some(Box::new(Quit)),
                KeyCode::Up => Some(Box::new(NavigateUp)),
                _ => None,
            },
            Event::Mouse(mouse) => {
                Some(Box::new(MouseClick {
                    x: mouse.column,
                    y: mouse.row,
                }))
            }
            _ => None,
        }
    }
}
```

---

## Geometry

Position and size primitives.

```rust
use ctui_core::{Position, Rect, Size};

// Rectangle
let rect = Rect::new(x, y, width, height);
let rect = Rect::default();  // (0, 0, 0, 0)

// Position
let pos = Position { x: 10, y: 20 };

// Size
let size = Size { width: 80, height: 24 };

// Useful methods
rect.area()         // width * height
rect.contains(pos)  // Check if position is inside
rect.intersection(other)  // Get intersection
rect.union(other)   // Get union
```

### Rect

| Field | Type | Description |
|-------|------|-------------|
| `x` | `u16` | Left position |
| `y` | `u16` | Top position |
| `width` | `u16` | Width |
| `height` | `u16` | Height |

---

## Style

Colors and text modifiers.

```rust
use ctui_core::{Style, Color, Modifier};

let style = Style::new()
    .fg(Color::Cyan)
    .bg(Color::Rgb(30, 30, 40))
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::ITALIC);
```

### Color

```rust
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Indexed(u8),    // 256-color
    Rgb(u8, u8, u8), // True color
}
```

### Modifier

```rust
bitflags! {
    pub struct Modifier: u16 {
        const BOLD = 0b000000001;
        const DIM = 0b000000010;
        const ITALIC = 0b000000100;
        const UNDERLINED = 0b000001000;
        const SLOW_BLINK = 0b000010000;
        const RAPID_BLINK = 0b000100000;
        const REVERSED = 0b001000000;
        const HIDDEN = 0b010000000;
        const CROSSED_OUT = 0b100000000;
    }
}
```

### Methods

| Method | Description |
|--------|-------------|
| `new()` | Create default style |
| `fg(color)` | Set foreground |
| `bg(color)` | Set background |
| `add_modifier(mod)` | Add modifier |
| `remove_modifier(mod)` | Remove modifier |
| `patch(other)` | Merge with another style |
