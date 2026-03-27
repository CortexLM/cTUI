# Tutorial 07: Events

Handle user input and system events.

## Goals

- Handle keyboard events
- Handle mouse events
- Process resize events

## Event Types

```rust
use ctui_core::Event;

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(ResizeEvent),
    FocusGained,
    FocusLost,
}
```

## Keyboard Events

### KeyEvent Structure

```rust
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

pub enum KeyCode {
    Char(char),     // Regular character
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
    F(u8),          // F1-F12
    Null,
}

pub struct KeyModifiers: u8 {
    const SHIFT = 0b0001;
    const CONTROL = 0b0010;
    const ALT = 0b0100;
    const SUPER = 0b1000;
}
```

### Handling Key Events

```rust
use ctui_core::{Component, Event, KeyCode, KeyEvent, KeyModifiers};

impl Component for MyComponent {
    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match event {
            Event::Key(key) => self.handle_key(key),
            _ => None,
        }
    }
}

impl MyComponent {
    fn handle_key(&mut self, key: &KeyEvent) -> Option<Box<dyn Msg>> {
        match key.code {
            // Quit on 'q'
            KeyCode::Char('q') => Some(Box::new(Quit)),

            // Navigate with arrows
            KeyCode::Up => Some(Box::new(NavigateUp)),
            KeyCode::Down => Some(Box::new(NavigateDown)),
            KeyCode::Left => Some(Box::new(NavigateLeft)),
            KeyCode::Right => Some(Box::new(NavigateRight)),

            // Select on Enter
            KeyCode::Enter => Some(Box::new(Select)),

            // Escape to go back
            KeyCode::Esc => Some(Box::new(GoBack)),

            _ => None,
        }
    }
}
```

### Modifier Keys

```rust
impl MyComponent {
    fn handle_key(&mut self, key: &KeyEvent) -> Option<Box<dyn Msg>> {
        // Check for Ctrl modifier
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') => Some(Box::new(Quit)),
                KeyCode::Char('s') => Some(Box::new(Save)),
                KeyCode::Char('z') => Some(Box::new(Undo)),
                _ => None,
            }
        } else if key.modifiers.contains(KeyModifiers::ALT) {
            match key.code {
                KeyCode::Char('h') => Some(Box::new(ShowHelp)),
                _ => None,
            }
        } else {
            // No modifier
            match key.code {
                KeyCode::Char('j') => Some(Box::new(NavigateDown)),
                KeyCode::Char('k') => Some(Box::new(NavigateUp)),
                _ => None,
            }
        }
    }
}
```

## Mouse Events

### MouseEvent Structure

```rust
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}

pub enum MouseEventKind {
    Down(MouseButton),   // Mouse button pressed
    Up(MouseButton),     // Mouse button released
    Drag(MouseButton),   // Mouse dragged
    Moved,               // Mouse moved
    ScrollDown,          // Scroll wheel down
    ScrollUp,            // Scroll wheel up
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
}
```

### Handling Mouse Events

```rust
impl Component for MyComponent {
    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match event {
            Event::Mouse(mouse) => self.handle_mouse(mouse),
            _ => None,
        }
    }
}

impl MyComponent {
    fn handle_mouse(&mut self, mouse: &MouseEvent) -> Option<Box<dyn Msg>> {
        match mouse.kind {
            // Click
            MouseEventKind::Down(MouseButton::Left) => {
                if self.hit_test(mouse.column, mouse.row) {
                    Some(Box::new(Click {
                        x: mouse.column,
                        y: mouse.row,
                    }))
                } else {
                    None
                }
            }

            // Scroll
            MouseEventKind::ScrollUp => {
                Some(Box::new(ScrollUp))
            }
            MouseEventKind::ScrollDown => {
                Some(Box::new(ScrollDown))
            }

            // Hover
            MouseEventKind::Moved => {
                if self.hit_test(mouse.column, mouse.row) {
                    Some(Box::new(Hover {
                        x: mouse.column,
                        y: mouse.row,
                    }))
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    fn hit_test(&self, x: u16, y: u16) -> bool {
        self.area.contains(Position { x, y })
    }
}
```

## Resize Events

```rust
pub struct ResizeEvent {
    pub width: u16,
    pub height: u16,
}

impl Component for MyComponent {
    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match event {
            Event::Resize(size) => {
                self.resize(size.width, size.height);
                Some(Box::new(Resized))
            }
            _ => None,
        }
    }
}
```

## Focus Management

Track which component is focused:

```rust
struct FocusManager {
    focused: usize,
    components: Vec<Box<dyn Component>>,
}

impl FocusManager {
    fn next(&mut self) {
        self.focused = (self.focused + 1) % self.components.len();
    }

    fn prev(&mut self) {
        self.focused = if self.focused == 0 {
            self.components.len() - 1
        } else {
            self.focused - 1
        };
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Box<dyn Msg>> {
        match key.code {
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.prev();
                } else {
                    self.next();
                }
                Some(Box::new(FocusChanged))
            }
            _ => self.components[self.focused].handle_event(&Event::Key(*key)),
        }
    }
}
```

## Input State

Track input text:

```rust
struct TextInput {
    value: String,
    cursor: usize,
}

impl TextInput {
    fn handle_key(&mut self, key: &KeyEvent) -> Option<Box<dyn Msg>> {
        match key.code {
            // Insert character
            KeyCode::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
                Some(Box::new(InputChanged))
            }

            // Backspace
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                    Some(Box::new(InputChanged))
                } else {
                    None
                }
            }

            // Delete
            KeyCode::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor);
                    Some(Box::new(InputChanged))
                } else {
                    None
                }
            }

            // Move cursor
            KeyCode::Left => {
                self.cursor = self.cursor.saturating_sub(1);
                Some(Box::new(CursorMoved))
            }
            KeyCode::Right => {
                self.cursor = (self.cursor + 1).min(self.value.len());
                Some(Box::new(CursorMoved))
            }

            // Home/End
            KeyCode::Home => {
                self.cursor = 0;
                Some(Box::new(CursorMoved))
            }
            KeyCode::End => {
                self.cursor = self.value.len();
                Some(Box::new(CursorMoved))
            }

            _ => None,
        }
    }
}
```

## Event Bubbling

Events bubble up through components:

```rust
impl Container {
    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        // Try children first
        for child in &mut self.children {
            if let Some(msg) = child.handle_event(event) {
                return Some(msg);
            }
        }

        // Then handle at container level
        match event {
            Event::Key(key) => self.handle_key(key),
            _ => None,
        }
    }
}
```

## Complete Example

```rust
use ctui_core::{Buffer, Cmd, Component, Event, Msg, Rect, KeyCode, KeyEvent, MouseEvent, MouseEventKind, MouseButton};

struct InteractiveList {
    items: Vec<String>,
    selected: usize,
    hovered: Option<usize>,
    area: Rect,
}

struct Select(usize);
impl Msg for Select {}

impl Component for InteractiveList {
    type Props = Vec<String>;
    type State = ();

    fn create(items: Self::Props) -> Self {
        Self {
            items,
            selected: 0,
            hovered: None,
            area: Rect::default(),
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.area = area;

        for (i, item) in self.items.iter().enumerate() {
            let y = area.y + i as u16;
            let style = if i == self.selected {
                "> "
            } else if self.hovered == Some(i) {
                "* "
            } else {
                "  "
            };

            let text = format!("{}{}", style, item);
            for (j, ch) in text.chars().enumerate() {
                if let Some(cell) = buf.get_mut(area.x + j as u16, y) {
                    cell.symbol = ch.to_string();
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match event {
            Event::Key(key) => self.handle_key(key),
            Event::Mouse(mouse) => self.handle_mouse(mouse),
            _ => None,
        }
    }
}

impl InteractiveList {
    fn handle_key(&mut self, key: &KeyEvent) -> Option<Box<dyn Msg>> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                Some(Box::new(Select(self.selected)))
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected < self.items.len() - 1 {
                    self.selected += 1;
                }
                Some(Box::new(Select(self.selected)))
            }
            KeyCode::Enter => {
                Some(Box::new(Select(self.selected)))
            }
            _ => None,
        }
    }

    fn handle_mouse(&mut self, mouse: &MouseEvent) -> Option<Box<dyn Msg>> {
        let item_index = (mouse.row - self.area.y) as usize;

        match mouse.kind {
            MouseEventKind::Moved => {
                if item_index < self.items.len() {
                    self.hovered = Some(item_index);
                    Some(Box::new(Hover(item_index)))
                } else {
                    self.hovered = None;
                    None
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                if item_index < self.items.len() {
                    self.selected = item_index;
                    Some(Box::new(Select(item_index)))
                } else {
                    None
                }
            }
            MouseEventKind::ScrollUp => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                Some(Box::new(Select(self.selected)))
            }
            MouseEventKind::ScrollDown => {
                if self.selected < self.items.len() - 1 {
                    self.selected += 1;
                }
                Some(Box::new(Select(self.selected)))
            }
            _ => None,
        }
    }
}
```

## Exercise

1. Create a text input that handles typing
2. Add copy/paste with Ctrl+C/Ctrl+V
3. Create a list that handles both keyboard and mouse

## Summary

You have completed the cTUI tutorial! You now know how to:

1. Set up a cTUI project
2. Create components
3. Manage state
4. Use layouts
5. Apply styling
6. Add animations
7. Handle events

## Next Steps

- Build a complete application using the [Examples](../examples/README.md)
- Explore the [Component Gallery](../gallery/README.md)
- Read the [API Reference](../api/README.md)

## See Also

- [Event API](../api/core.md#events)
- [Input Component](../gallery/input.md)
- [Todo Example](../examples/todo.md)
