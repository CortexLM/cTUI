//! End-to-end integration tests based on example apps

use ctui_components::{Block, Borders, Line, Paragraph, Span, Text};
use ctui_core::backend::test::TestBackend;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Terminal, Widget};
use ctui_layout::{Constraint, Layout};

struct Increment;
struct Decrement;
struct Reset;

impl Msg for Increment {}
impl Msg for Decrement {}
impl Msg for Reset {}

struct CounterApp {
    count: i32,
}

struct CounterProps {
    initial: i32,
}

impl Component for CounterApp {
    type Props = CounterProps;
    type State = i32;

    fn create(props: Self::Props) -> Self {
        Self {
            count: props.initial,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("Counter: {}", self.count);
        for (i, ch) in text.chars().take(area.width as usize).enumerate() {
            buf.modify_cell(area.x + i as u16, area.y, |cell| { cell.symbol = ch.to_string(); });
        }

        let help_text = "Press +/- or r to reset";
        let start_x = area.x + 2;
        if area.height > 1 {
            for (i, ch) in help_text.chars().take(area.width as usize).enumerate() {
                buf.modify_cell(start_x + i as u16, area.y + 1, |cell| { cell.symbol = ch.to_string(); });
            }
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

#[test]
fn test_counter_full_lifecycle() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let props = CounterProps { initial: 0 };
    let mut counter = CounterApp::create(props);
    counter.on_mount();

    terminal
        .draw(|f| {
            counter.render(Rect::new(0, 0, 40, 2), f.buffer_mut());
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(0, 0).unwrap().symbol, "C");
    assert_eq!(backend.buffer().get(7, 0).unwrap().symbol, ":");
    assert_eq!(backend.buffer().get(9, 0).unwrap().symbol, "0");

    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));

    assert_eq!(counter.count, 3);

    terminal
        .draw(|f| {
            counter.render(Rect::new(0, 0, 40, 2), f.buffer_mut());
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(9, 0).unwrap().symbol, "3");

    counter.update(Box::new(Decrement));
    assert_eq!(counter.count, 2);

    counter.update(Box::new(Reset));
    assert_eq!(counter.count, 0);

    counter.on_unmount();
}

#[test]
fn test_counter_multiple_sessions() {
    let props1 = CounterProps { initial: 10 };
    let mut counter1 = CounterApp::create(props1);
    counter1.on_mount();

    let props2 = CounterProps { initial: 100 };
    let mut counter2 = CounterApp::create(props2);
    counter2.on_mount();

    counter1.update(Box::new(Increment));
    counter2.update(Box::new(Increment));

    assert_eq!(counter1.count, 11);
    assert_eq!(counter2.count, 101);

    counter1.on_unmount();
    counter2.on_unmount();
}

struct AddTodo(String);
struct RemoveTodo(usize);
struct ToggleTodo(usize);

impl Msg for AddTodo {}
impl Msg for RemoveTodo {}
impl Msg for ToggleTodo {}

#[derive(Clone)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

struct TodoApp {
    items: Vec<TodoItem>,
    next_id: usize,
}

struct TodoProps;

impl Component for TodoApp {
    type Props = TodoProps;
    type State = ();

    fn create(_props: Self::Props) -> Self {
        Self {
            items: Vec::new(),
            next_id: 0,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let title = "Todo List";
        for (i, ch) in title.chars().take(area.width as usize).enumerate() {
            buf.modify_cell(area.x + i as u16, area.y, |cell| { cell.symbol = ch.to_string(); });
        }

        for (idx, item) in self.items.iter().enumerate() {
            let y = area.y + 2 + idx as u16;
            if y >= area.y + area.height {
                break;
            }
            let check = if item.completed { "[x] " } else { "[ ] " };
            let text = format!("{}{}", check, item.text);
            for (i, ch) in text.chars().take(area.width as usize).enumerate() {
                buf.modify_cell(area.x + i as u16, y, |cell| { cell.symbol = ch.to_string(); });
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if let Some(add_msg) = msg.downcast_ref::<AddTodo>() {
            if !add_msg.0.is_empty() {
                let item = TodoItem {
                    id: self.next_id,
                    text: add_msg.0.clone(),
                    completed: false,
                };
                self.next_id += 1;
                self.items.push(item);
            }
            Cmd::Render
        } else if let Some(remove_msg) = msg.downcast_ref::<RemoveTodo>() {
            let id = remove_msg.0;
            self.items.retain(|item| item.id != id);
            Cmd::Render
        } else if let Some(toggle_msg) = msg.downcast_ref::<ToggleTodo>() {
            let id = toggle_msg.0;
            if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
                item.completed = !item.completed;
            }
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

#[test]
fn test_todo_full_lifecycle() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut todo = TodoApp::create(TodoProps);
    todo.on_mount();

    todo.update(Box::new(AddTodo("Learn cTUI".to_string())));
    todo.update(Box::new(AddTodo("Build apps".to_string())));
    todo.update(Box::new(AddTodo("Ship to prod".to_string())));

    assert_eq!(todo.items.len(), 3);
    assert_eq!(todo.items[0].text, "Learn cTUI");

    terminal
        .draw(|f| {
            todo.render(Rect::new(0, 0, 40, 10), f.buffer_mut());
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(0, 0).unwrap().symbol, "T");
    assert_eq!(backend.buffer().get(0, 2).unwrap().symbol, "[");
    assert_eq!(backend.buffer().get(1, 2).unwrap().symbol, " ");
    assert_eq!(backend.buffer().get(2, 2).unwrap().symbol, "]");
}

#[test]
fn test_todo_toggle_complete() {
    let mut todo = TodoApp::create(TodoProps);
    todo.on_mount();

    todo.update(Box::new(AddTodo("Task 1".to_string())));
    todo.update(Box::new(AddTodo("Task 2".to_string())));

    assert!(!todo.items[0].completed);
    assert!(!todo.items[1].completed);

    todo.update(Box::new(ToggleTodo(0)));
    assert!(todo.items[0].completed);
    assert!(!todo.items[1].completed);

    todo.update(Box::new(ToggleTodo(0)));
    assert!(!todo.items[0].completed);

    todo.on_unmount();
}

#[test]
fn test_todo_remove_items() {
    let mut todo = TodoApp::create(TodoProps);
    todo.on_mount();

    todo.update(Box::new(AddTodo("Task 1".to_string())));
    todo.update(Box::new(AddTodo("Task 2".to_string())));
    todo.update(Box::new(AddTodo("Task 3".to_string())));

    assert_eq!(todo.items.len(), 3);

    todo.update(Box::new(RemoveTodo(1)));
    assert_eq!(todo.items.len(), 2);
    assert_eq!(todo.items[0].text, "Task 1");
    assert_eq!(todo.items[1].text, "Task 3");

    todo.update(Box::new(RemoveTodo(0)));
    assert_eq!(todo.items.len(), 1);
    assert_eq!(todo.items[0].text, "Task 3");

    todo.on_unmount();
}

#[test]
fn test_todo_empty_list() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut todo = TodoApp::create(TodoProps);
    todo.on_mount();

    terminal
        .draw(|f| {
            todo.render(Rect::new(0, 0, 40, 10), f.buffer_mut());
        })
        .unwrap();

    assert_eq!(todo.items.len(), 0);

    todo.on_unmount();
}

#[derive(Clone, Copy)]
struct DashboardWidget {
    title: &'static str,
    value: u32,
}

impl Widget for DashboardWidget {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        for x in area.x..area.x.saturating_add(area.width) {
            buffer.modify_cell(x, area.y, |cell| { cell.symbol = "═".to_string(); });
            buffer.modify_cell(x, area.y + area.height.saturating_sub(1), |cell| { cell.symbol = "═".to_string(); });
        }
        for y in area.y..area.y.saturating_add(area.height) {
            buffer.modify_cell(area.x, y, |cell| { cell.symbol = "║".to_string(); });
            buffer.modify_cell(area.x + area.width.saturating_sub(1), y, |cell| { cell.symbol = "║".to_string(); });
        }

        let inner = Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        );
        let text = format!("{}: {}", self.title, self.value);
        for (i, ch) in text.chars().take(inner.width as usize).enumerate() {
            buffer.modify_cell(inner.x + i as u16, inner.y, |cell| { cell.symbol = ch.to_string(); });
        }
    }
}

#[test]
fn test_dashboard_layout() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let layout = Layout::row();
    let areas = layout.split(
        Rect::new(0, 0, 80, 24),
        &[
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ],
    );

    let widgets = [
        DashboardWidget {
            title: "Panel 1",
            value: 100,
        },
        DashboardWidget {
            title: "Panel 2",
            value: 200,
        },
        DashboardWidget {
            title: "Panel 3",
            value: 300,
        },
    ];

    terminal
        .draw(|f| {
            for (area, widget) in areas.iter().zip(widgets.iter()) {
                widget.render(*area, f.buffer_mut());
            }
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(1, 1).unwrap().symbol, "P");
}

#[test]
fn test_key_press_simulation() {
    use ctui_core::{Event, EventHandler, KeyCode, KeyEvent, KeyModifiers};

    let mut counter = CounterApp::create(CounterProps { initial: 0 });
    counter.on_mount();

    let key_event = KeyEvent {
        code: KeyCode::Char('+'),
        modifiers: KeyModifiers::new(),
    };

    if true {
        counter.update(Box::new(Increment));
    }

    assert_eq!(counter.count, 1);

    counter.on_unmount();
}

#[test]
fn test_render_resize_rerender() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let props = CounterProps { initial: 5 };
    let counter = CounterApp::create(props);

    terminal
        .draw(|f| {
            counter.render(Rect::new(0, 0, 40, 2), f.buffer_mut());
        })
        .unwrap();

    terminal
        .draw(|f| {
            counter.render(Rect::new(0, 0, 40, 2), f.buffer_mut());
        })
        .unwrap();

    terminal
        .draw(|f| {
            counter.render(Rect::new(0, 0, 40, 2), f.buffer_mut());
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(9, 0).unwrap().symbol, "5");
}

#[test]
fn test_batch_updates() {
    let mut counter = CounterApp::create(CounterProps { initial: 0 });
    counter.on_mount();

    let batch = Cmd::batch(vec![Cmd::Render, Cmd::Render, Cmd::Render]);

    if batch.should_render() {
        counter.update(Box::new(Increment));
        counter.update(Box::new(Increment));
        counter.update(Box::new(Increment));
    }

    assert_eq!(counter.count, 3);

    counter.on_unmount();
}
