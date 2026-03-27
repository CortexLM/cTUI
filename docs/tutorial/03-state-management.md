# Tutorial 03: State Management

Handle component state and messages.

## Goals

- Define messages
- Update state
- Trigger re-renders

## Message Pattern

Components communicate through messages:

```rust
use ctui_core::Msg;

// Define your messages
struct Increment;
struct Decrement;
struct Reset;

// Messages must implement Msg
impl Msg for Increment {}
impl Msg for Decrement {}
impl Msg for Reset {}
```

## Stateful Component

```rust
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

struct Counter {
    count: i32,
}

impl Component for Counter {
    type Props = i32;   // Initial count
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self { count: props }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("Count: {}", self.count);
        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<Increment>() {
            self.count += 1;
            Cmd::Render   // Request a re-render
        } else if msg.is::<Decrement>() {
            self.count -= 1;
            Cmd::Render
        } else if msg.is::<Reset>() {
            self.count = 0;
            Cmd::Render
        } else {
            Cmd::Noop     // Do nothing
        }
    }
}
```

## Cmd Return Value

The `Cmd` enum controls what happens after an update:

```rust
pub enum Cmd {
    Noop,    // Do nothing
    Render,  // Request a render
    Async(/* ... */),  // Start async operation
    Batch(/* ... */),  // Multiple commands
}
```

## Testing the Component

```rust
fn main() {
    let mut counter = Counter::create(0);
    counter.on_mount();

    // Simulate user actions
    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    
    println!("After 3 increments: {}", counter.count);

    counter.update(Box::new(Decrement));
    println!("After 1 decrement: {}", counter.count);

    counter.update(Box::new(Reset));
    println!("After reset: {}", counter.count);

    counter.on_unmount();
}
```

Output:

```
After 3 increments: 3
After 1 decrement: 2
After reset: 0
```

## Messages with Data

Pass data in messages:

```rust
struct SetValue(i32);
impl Msg for SetValue {}

struct AddTodo(String);
impl Msg for AddTodo {}

impl Counter {
    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if let Some(set_msg) = msg.downcast_ref::<SetValue>() {
            self.count = set_msg.0;
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}
```

## State Struct

For complex state, use a struct:

```rust
struct TodoState {
    items: Vec<TodoItem>,
    next_id: usize,
    input: String,
    filter: Filter,
}

#[derive(Clone)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

enum Filter {
    All,
    Active,
    Completed,
}

impl TodoState {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 0,
            input: String::new(),
            filter: Filter::All,
        }
    }

    fn add(&mut self, text: String) {
        let item = TodoItem {
            id: self.next_id,
            text,
            completed: false,
        };
        self.next_id += 1;
        self.items.push(item);
    }

    fn toggle(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = !item.completed;
        }
    }

    fn remove(&mut self, id: usize) {
        self.items.retain(|item| item.id != id);
    }
}
```

## Component with State

```rust
struct TodoApp {
    state: TodoState,
}

impl Component for TodoApp {
    type Props = ();
    type State = TodoState;

    fn create(_: Self::Props) -> Self {
        Self { state: TodoState::new() }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render todos, input, etc.
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if let Some(add_msg) = msg.downcast_ref::<AddTodo>() {
            self.state.add(add_msg.0.clone());
            Cmd::Render
        } else if let Some(toggle_msg) = msg.downcast_ref::<ToggleTodo>() {
            self.state.toggle(toggle_msg.0);
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}
```

## Derived State

Compute values from state:

```rust
impl TodoState {
    fn completed_count(&self) -> usize {
        self.items.iter().filter(|i| i.completed).count()
    }

    fn active_count(&self) -> usize {
        self.items.iter().filter(|i| !i.completed).count()
    }

    fn filtered_items(&self) -> Vec<&TodoItem> {
        match self.filter {
            Filter::All => self.items.iter().collect(),
            Filter::Active => self.items.iter().filter(|i| !i.completed).collect(),
            Filter::Completed => self.items.iter().filter(|i| i.completed).collect(),
        }
    }
}
```

## Exercise

1. Create a `Temperature` component that stores a value in Fahrenheit
2. Add messages to convert to Celsius
3. Add messages to increment/decrement by 1 degree

## Solution

```rust
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

struct ToCelsius;
struct ToFahrenheit;
struct TempUp;
struct TempDown;

impl Msg for ToCelsius {}
impl Msg for ToFahrenheit {}
impl Msg for TempUp {}
impl Msg for TempDown {}

struct Temperature {
    fahrenheit: f64,
    display_celsius: bool,
}

impl Temperature {
    fn celsius(&self) -> f64 {
        (self.fahrenheit - 32.0) * 5.0 / 9.0
    }

    fn display_value(&self) -> f64 {
        if self.display_celsius {
            self.celsius()
        } else {
            self.fahrenheit
        }
    }
}

impl Component for Temperature {
    type Props = f64;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            fahrenheit: props,
            display_celsius: false,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let unit = if self.display_celsius { "C" } else { "F" };
        let text = format!("Temperature: {:.1}°{}", self.display_value(), unit);
        
        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<ToCelsius>() {
            self.display_celsius = true;
            Cmd::Render
        } else if msg.is::<ToFahrenheit>() {
            self.display_celsius = false;
            Cmd::Render
        } else if msg.is::<TempUp>() {
            self.fahrenheit += 1.0;
            Cmd::Render
        } else if msg.is::<TempDown>() {
            self.fahrenheit -= 1.0;
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}
```

## Next Steps

Continue to [Tutorial 04: Layout System](04-layout.md) to learn about flexible layouts.

## See Also

- [State API](../api/core.md#state)
- [Todo Example](../examples/todo.md)
