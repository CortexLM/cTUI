//! Counter example - minimal stateful component demonstration
//!
//! Run with: `cargo run --example counter`

use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

struct Increment;
struct Decrement;
struct Reset;

impl Msg for Increment {}
impl Msg for Decrement {}
impl Msg for Reset {}

struct Counter {
    count: i32,
}

impl Counter {
    fn new(initial: i32) -> Self {
        Self { count: initial }
    }
}

impl Component for Counter {
    type Props = i32;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self::new(props)
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("Counter: {}", self.count);
        for (i, ch) in text.chars().take(area.width as usize).enumerate() {
            buf.modify_cell(area.x + i as u16, area.y, |cell| { cell.symbol = ch.to_string(); });
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<Increment>() {
            self.count += 1;
            Cmd::Render
        } else if msg.is::<Decrement>() {
            self.count -= 1;
            Cmd::Render
        } else if msg.is::<Reset>() {
            self.count = 0;
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

fn main() {
    let mut counter = Counter::new(0);
    counter.on_mount();

    println!("Counter Example");
    println!("===============\n");
    println!("Initial count: {}", counter.count);

    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    println!("After 3 increments: {}", counter.count);

    counter.update(Box::new(Decrement));
    println!("After 1 decrement: {}", counter.count);

    counter.update(Box::new(Reset));
    println!("After reset: {}", counter.count);

    println!("\n✓ Component pattern verified");
    counter.on_unmount();
}
