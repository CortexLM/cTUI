# Tutorial 02: First Component

Build your first real component.

## Goals

- Implement the Component trait
- Understand render lifecycle
- Render a Block widget

## The Component Trait

```rust
pub trait Component {
    type Props;  // Configuration passed at creation
    type State;  // Shared state type

    fn create(props: Self::Props) -> Self;
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd { Cmd::Noop }
}
```

## Create a Component

```rust
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};
use ctui_components::{Block, Borders, BorderType};

// Your first component
struct MyPanel {
    title: String,
}

impl Component for MyPanel {
    type Props = String;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self { title: props }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(&self.title);
        
        block.render(area, buf);
    }
}
```

## Understanding Render

The `render` method is called whenever the component needs to be drawn:

```rust
fn render(&self, area: Rect, buf: &mut Buffer) {
    // area: The rectangle allocated for this component
    // buf:  The buffer to draw into
    
    // 1. Create widgets
    let block = Block::new().borders(Borders::ALL);
    
    // 2. Render widgets
    block.render(area, buf);
}
```

## Rendering Text

Add text inside the block:

```rust
use ctui_components::{Block, Paragraph, Text, Borders};

struct HelloWorld;

impl Component for HelloWorld {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Create a block with borders
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("My App");
        
        // Render block first (it defines the area)
        let inner = block.inner(area);
        block.render(area, buf);
        
        // Render content inside the block
        let text = Text::from("Hello, World!");
        let paragraph = Paragraph::new(text);
        paragraph.render(inner, buf);
    }
}
```

## Nested Components

Compose components together:

```rust
struct Dashboard {
    header: MyPanel,
    content: HelloWorld,
}

impl Dashboard {
    fn new() -> Self {
        Self {
            header: MyPanel::create("Header".into()),
            content: HelloWorld::create(()),
        }
    }
}

impl Component for Dashboard {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self::new()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Split area into header and content
        let header_area = Rect::new(area.x, area.y, area.width, 3);
        let content_area = Rect::new(area.x, area.y + 3, area.width, area.height - 3);
        
        // Render each part
        self.header.render(header_area, buf);
        self.content.render(content_area, buf);
    }
}
```

## Main Function

Put it all together:

```rust
fn main() {
    let component = MyPanel::create("Welcome".into());
    
    let area = Rect::new(0, 0, 40, 10);
    let mut buf = Buffer::empty(area);
    
    component.render(area, &mut buf);
    
    // Print buffer
    for y in 0..area.height {
        for x in 0..area.width {
            print!("{}", buf[(x, y)].symbol);
        }
        println!();
    }
}
```

Output:

```
╭ Welcome ────────────────────────────╮
│                                    │
│                                    │
│                                    │
│                                    │
│                                    │
│                                    │
│                                    │
│                                    │
╰────────────────────────────────────╯
```

## Lifecycle Hooks

Add lifecycle callbacks:

```rust
impl Component for MyPanel {
    // Called when component is created
    fn on_mount(&mut self) {
        println!("Component mounted!");
    }
    
    // Called when component is destroyed
    fn on_unmount(&mut self) {
        println!("Component unmounted!");
    }
}
```

## Exercise

1. Create a `Header` component that displays a centered title
2. Create a `Footer` component that displays keyboard shortcuts
3. Combine them in a `Layout` component

## Solution

```rust
use ctui_core::{Buffer, Component, Rect};
use ctui_components::{Block, Paragraph, Text, Borders, BorderType};

// Header component
struct Header {
    title: String,
}

impl Component for Header {
    type Props = String;
    type State = ();

    fn create(title: Self::Props) -> Self {
        Self { title }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .borders(Borders::BOTTOM)
            .title(&self.title);
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        // Center the title
        let title = Text::from(&self.title);
        let paragraph = Paragraph::new(title);
        paragraph.render(inner, buf);
    }
}

// Footer component  
struct Footer;

impl Component for Footer {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self { Self }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = "q: quit | h: help | ?: about";
        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }
}

// Layout component
struct Layout {
    header: Header,
    footer: Footer,
}

impl Component for Layout {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self {
            header: Header::create("My App".into()),
            footer: Footer::create(()),
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let header_area = Rect::new(area.x, area.y, area.width, 3);
        let footer_area = Rect::new(area.x, area.y + area.height - 1, area.width, 1);
        
        self.header.render(header_area, buf);
        self.footer.render(footer_area, buf);
    }
}
```

## Next Steps

Continue to [Tutorial 03: State Management](03-state-management.md) to learn about handling state.

## See Also

- [Component API](../api/core.md#component)
- [Block Component](../gallery/block.md)
- [Paragraph Component](../gallery/paragraph.md)
