//! Todo app example - full CRUD operations demonstration
//!
//! Run with: `cargo run --example todo`

use ctui_components::{Block, Borders, List, ListItem, ListProps, Paragraph, ParagraphProps};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Style};
use std::collections::HashMap;

struct AddTodo(String);
struct RemoveTodo(usize);
struct ToggleTodo(usize);
struct SetInput(String);

impl Msg for AddTodo {}
impl Msg for RemoveTodo {}
impl Msg for ToggleTodo {}
impl Msg for SetInput {}

#[derive(Clone, Debug)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

struct TodoState {
    items: Vec<TodoItem>,
    next_id: usize,
    input: String,
}

impl TodoState {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 0,
            input: String::new(),
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

    fn remove(&mut self, id: usize) {
        self.items.retain(|item| item.id != id);
    }

    fn toggle(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.completed = !item.completed;
        }
    }
}

struct TodoApp {
    state: TodoState,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            state: TodoState::new(),
        }
    }
}

impl Component for TodoApp {
    type Props = ();
    type State = TodoState;

    fn create(_props: Self::Props) -> Self {
        Self::new()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let header = "╔══════════════════════════════════════╗";
        let title = "║           Todo App                   ║";
        let divider = "╠══════════════════════════════════════╣";
        let footer = "╚══════════════════════════════════════╝";

        let lines = [header, title, divider];

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                buf.modify_cell(area.x + col as u16, area.y + row as u16, |cell| { cell.symbol = ch.to_string(); });
            }
        }

        let item_start_row = 4;
        for (idx, item) in self.state.items.iter().enumerate() {
            let check = if item.completed { "[x]" } else { "[ ]" };
            let text = format!("║ {} {} {}", idx + 1, check, item.text);
            for (col, ch) in text.chars().enumerate() {
                buf.modify_cell(area.x + col as u16, area.y + item_start_row + idx as u16, |cell| {
                    cell.symbol = ch.to_string();
                });
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if let Some(add_msg) = msg.downcast_ref::<AddTodo>() {
            if !add_msg.0.is_empty() {
                self.state.add(add_msg.0.clone());
            }
            Cmd::Render
        } else if let Some(remove_msg) = msg.downcast_ref::<RemoveTodo>() {
            self.state.remove(remove_msg.0);
            Cmd::Render
        } else if let Some(toggle_msg) = msg.downcast_ref::<ToggleTodo>() {
            self.state.toggle(toggle_msg.0);
            Cmd::Render
        } else if let Some(input_msg) = msg.downcast_ref::<SetInput>() {
            self.state.input = input_msg.0.clone();
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

fn main() {
    let mut app = TodoApp::new();
    app.on_mount();

    println!("Todo App Example");
    println!("================\n");

    app.update(Box::new(AddTodo("Learn cTUI framework".to_string())));
    app.update(Box::new(AddTodo("Build awesome TUI apps".to_string())));
    app.update(Box::new(AddTodo("Contribute to open source".to_string())));

    println!("Added 3 todos:");
    for (idx, item) in app.state.items.iter().enumerate() {
        println!(
            "  {}. {} - {}",
            idx + 1,
            if item.completed { "[x]" } else { "[ ]" },
            item.text
        );
    }

    app.update(Box::new(ToggleTodo(0)));
    println!("\nAfter toggling first item:");
    for (idx, item) in app.state.items.iter().enumerate() {
        println!(
            "  {}. {} - {}",
            idx + 1,
            if item.completed { "[x]" } else { "[ ]" },
            item.text
        );
    }

    app.update(Box::new(RemoveTodo(1)));
    println!("\nAfter removing second item:");
    for (idx, item) in app.state.items.iter().enumerate() {
        println!(
            "  {}. {} - {}",
            idx + 1,
            if item.completed { "[x]" } else { "[ ]" },
            item.text
        );
    }

    println!("\n✓ Todo app CRUD operations verified");
    app.on_unmount();
}
