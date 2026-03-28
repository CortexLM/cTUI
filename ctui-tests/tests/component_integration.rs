//! Integration tests for component lifecycle

use ctui_core::backend::test::TestBackend;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Terminal};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct Increment;
struct Decrement;
struct Reset;

impl Msg for Increment {}
impl Msg for Decrement {}
impl Msg for Reset {}

struct CounterComponent {
    count: i32,
    mount_count: Arc<AtomicUsize>,
    unmount_count: Arc<AtomicUsize>,
}

struct CounterProps {
    initial: i32,
    mount_count: Arc<AtomicUsize>,
    unmount_count: Arc<AtomicUsize>,
}

impl Component for CounterComponent {
    type Props = CounterProps;
    type State = i32;

    fn create(props: Self::Props) -> Self {
        Self {
            count: props.initial,
            mount_count: props.mount_count,
            unmount_count: props.unmount_count,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("Count: {}", self.count);
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

    fn on_mount(&mut self) {
        self.mount_count.fetch_add(1, Ordering::SeqCst);
    }

    fn on_unmount(&mut self) {
        self.unmount_count.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn test_component_full_lifecycle() {
    let mount_count = Arc::new(AtomicUsize::new(0));
    let unmount_count = Arc::new(AtomicUsize::new(0));

    let props = CounterProps {
        initial: 0,
        mount_count: mount_count.clone(),
        unmount_count: unmount_count.clone(),
    };

    let mut counter = CounterComponent::create(props);
    assert_eq!(counter.count, 0);
    assert_eq!(mount_count.load(Ordering::SeqCst), 0);

    counter.on_mount();
    assert_eq!(mount_count.load(Ordering::SeqCst), 1);

    let cmd = counter.update(Box::new(Increment));
    assert_eq!(cmd, Cmd::Render);
    assert_eq!(counter.count, 1);

    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    assert_eq!(counter.count, 3);

    counter.update(Box::new(Decrement));
    assert_eq!(counter.count, 2);

    counter.update(Box::new(Reset));
    assert_eq!(counter.count, 0);

    counter.on_unmount();
    assert_eq!(unmount_count.load(Ordering::SeqCst), 1);
}

#[test]
fn test_component_render_to_buffer() {
    let mount_count = Arc::new(AtomicUsize::new(0));
    let unmount_count = Arc::new(AtomicUsize::new(0));

    let props = CounterProps {
        initial: 42,
        mount_count,
        unmount_count,
    };

    let counter = CounterComponent::create(props);
    let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));

    counter.render(Rect::new(0, 0, 20, 1), &mut buf);

    assert_eq!(buf.get(0, 0).unwrap().symbol, "C");
    assert_eq!(buf.get(1, 0).unwrap().symbol, "o");
    assert_eq!(buf.get(5, 0).unwrap().symbol, ":");
    assert_eq!(buf.get(7, 0).unwrap().symbol, "4");
}

#[test]
fn test_component_render_with_terminal() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mount_count = Arc::new(AtomicUsize::new(0));
    let unmount_count = Arc::new(AtomicUsize::new(0));

    let props = CounterProps {
        initial: 10,
        mount_count,
        unmount_count,
    };

    let counter = CounterComponent::create(props);

    terminal
        .draw(|f| {
            counter.render(Rect::new(0, 0, 20, 1), f.buffer_mut());
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(0, 0).unwrap().symbol, "C");
}

#[test]
fn test_cmd_render_trigger() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mount_count = Arc::new(AtomicUsize::new(0));
    let unmount_count = Arc::new(AtomicUsize::new(0));

    let props = CounterProps {
        initial: 0,
        mount_count,
        unmount_count,
    };

    let mut counter = CounterComponent::create(props);
    counter.on_mount();

    terminal
        .draw(|f| {
            counter.render(Rect::new(0, 0, 20, 1), f.buffer_mut());
        })
        .unwrap();

    let cmd = counter.update(Box::new(Increment));
    assert!(cmd.should_render());

    terminal
        .draw(|f| {
            counter.render(Rect::new(0, 0, 20, 1), f.buffer_mut());
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(7, 0).unwrap().symbol, "1");
}

#[test]
fn test_cmd_batch_operations() {
    let batch = Cmd::batch(vec![Cmd::Render, Cmd::Render, Cmd::Quit]);
    assert!(batch.should_render());
    assert!(batch.should_quit());

    let batch_no_quit = Cmd::batch(vec![Cmd::Render, Cmd::Noop]);
    assert!(batch_no_quit.should_render());
    assert!(!batch_no_quit.should_quit());
}

#[test]
fn test_cmd_navigation() {
    let cmd = Cmd::Navigate("home".to_string());
    assert!(!cmd.should_quit());
    assert!(!cmd.should_render());
}

#[test]
fn test_cmd_focus_management() {
    let focus_cmd = Cmd::RequestFocus;
    let yield_cmd = Cmd::YieldFocus;

    assert!(!focus_cmd.should_quit());
    assert!(!yield_cmd.should_quit());
    assert!(!focus_cmd.should_render());
    assert!(!yield_cmd.should_render());
}

#[test]
fn test_multiple_updates_same_component() {
    let mount_count = Arc::new(AtomicUsize::new(0));
    let unmount_count = Arc::new(AtomicUsize::new(0));

    let props = CounterProps {
        initial: 100,
        mount_count,
        unmount_count,
    };

    let mut counter = CounterComponent::create(props);
    counter.on_mount();

    for _ in 0..10 {
        counter.update(Box::new(Decrement));
    }

    assert_eq!(counter.count, 90);

    counter.on_unmount();
}

#[test]
fn test_state_persistence_across_renders() {
    let backend = TestBackend::new(30, 5);
    let mut terminal = Terminal::new(backend).unwrap();

    let mount_count = Arc::new(AtomicUsize::new(0));
    let unmount_count = Arc::new(AtomicUsize::new(0));

    let props = CounterProps {
        initial: 0,
        mount_count,
        unmount_count,
    };

    let mut counter = CounterComponent::create(props);
    counter.on_mount();

    for i in 1..=5 {
        counter.update(Box::new(Increment));

        terminal
            .draw(|f| {
                counter.render(Rect::new(0, 0, 30, 1), f.buffer_mut());
            })
            .unwrap();

        assert_eq!(counter.count, i);
    }
}

#[test]
fn test_message_type_routing() {
    let mount_count = Arc::new(AtomicUsize::new(0));
    let unmount_count = Arc::new(AtomicUsize::new(0));

    let props = CounterProps {
        initial: 50,
        mount_count,
        unmount_count,
    };

    let mut counter = CounterComponent::create(props);

    let msg: Box<dyn Msg> = Box::new(Increment);
    assert!(msg.is::<Increment>());
    assert!(!msg.is::<Decrement>());

    counter.update(msg);
    assert_eq!(counter.count, 51);
}

struct ToggleComponent {
    is_on: bool,
}

struct ToggleProps {}

impl Component for ToggleComponent {
    type Props = ToggleProps;
    type State = bool;

    fn create(_props: Self::Props) -> Self {
        Self { is_on: false }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = if self.is_on { "[ON]" } else { "[OFF]" };
        for (i, ch) in text.chars().take(area.width as usize).enumerate() {
            buf.modify_cell(area.x + i as u16, area.y, |cell| { cell.symbol = ch.to_string(); });
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<ToggleMsg>() {
            self.is_on = !self.is_on;
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

struct ToggleMsg;
impl Msg for ToggleMsg {}

#[test]
fn test_toggle_component_states() {
    let toggle = ToggleComponent::create(ToggleProps {});
    assert!(!toggle.is_on);

    let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
    toggle.render(Rect::new(0, 0, 6, 1), &mut buf);
    assert_eq!(buf.get(1, 0).unwrap().symbol, "O");
    assert_eq!(buf.get(2, 0).unwrap().symbol, "F");
    assert_eq!(buf.get(3, 0).unwrap().symbol, "F");
}

#[test]
fn test_toggle_component_interaction() {
    let mut toggle = ToggleComponent::create(ToggleProps {});
    assert!(!toggle.is_on);

    toggle.update(Box::new(ToggleMsg));
    assert!(toggle.is_on);

    toggle.update(Box::new(ToggleMsg));
    assert!(!toggle.is_on);
}
