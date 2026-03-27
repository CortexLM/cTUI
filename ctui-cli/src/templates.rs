use crate::{TemplateContext, TemplateType};

/// # Errors
/// Returns an error if formatting fails.
pub fn render_cargo_toml(ctx: &TemplateContext) -> anyhow::Result<String> {
    let ctui_deps = r#"ctui-core = { path = "../ctui-core" }
ctui-components = { path = "../ctui-components" }"#;

    Ok(format!(
        r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
{ctui_deps}
crossterm = "0.28"

[[bin]]
name = "{project_name}"
path = "src/main.rs"
"#,
        project_name = ctx.project_name,
        ctui_deps = ctui_deps
    ))
}

/// # Errors
/// Returns an error if formatting fails.
pub fn render_main_rs(ctx: &TemplateContext) -> anyhow::Result<String> {
    let content = match ctx.template_type {
        TemplateType::Basic => render_basic_main(ctx),
        TemplateType::Counter => render_counter_main(ctx),
        TemplateType::TodoApp => render_todo_main(ctx),
    };
    Ok(content)
}

fn render_basic_main(ctx: &TemplateContext) -> String {
    format!(
        r#"
use ctui_core::{{Buffer, Cmd, Component, Msg, Rect}};

struct HelloWorld;

impl Component for HelloWorld {{
    type Props = ();
    type State = ();

    fn create(_props: Self::Props) -> Self {{
        Self
    }}

    fn render(&self, area: Rect, buf: &mut Buffer) {{
        let text = "Hello, {project_name}!";
        for (i, ch) in text.chars().take(area.width as usize).enumerate() {{
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {{
                cell.symbol = ch.to_string();
            }}
        }}
    }}

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {{
        Cmd::Noop
    }}
}}

fn main() {{
    println!("{project_name} - cTUI Application");
    println!("================================");
    
    let app = HelloWorld;
    app.on_mount();
    
    let area = Rect::new(0, 0, 30, 1);
    let mut buf = Buffer::empty(area);
    app.render(area, &mut buf);
    
    for col in 0..area.width {{
        if let Some(cell) = buf.get(col, 0) {{
            print!("{{}}", cell.symbol);
        }}
    }}
    println!();
    
    app.on_unmount();
}}
"#,
        project_name = ctx.project_name
    )
}

fn render_counter_main(ctx: &TemplateContext) -> String {
    format!(
        r#"
use ctui_core::{{Buffer, Cmd, Component, Msg, Rect}};

struct Increment;
struct Decrement;

impl Msg for Increment {{}}
impl Msg for Decrement {{}}

struct Counter {{
    count: i32,
}}

impl Counter {{
    fn new(initial: i32) -> Self {{
        Self {{ count: initial }}
    }}
}}

impl Component for Counter {{
    type Props = i32;
    type State = ();

    fn create(props: Self::Props) -> Self {{
        Self::new(props)
    }}

    fn render(&self, area: Rect, buf: &mut Buffer) {{
        let text = format!("Counter: {{}}", self.count);
        for (i, ch) in text.chars().take(area.width as usize).enumerate() {{
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {{
                cell.symbol = ch.to_string();
            }}
        }}
    }}

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {{
        if msg.is::<Increment>() {{
            self.count += 1;
            Cmd::Render
        }} else if msg.is::<Decrement>() {{
            self.count -= 1;
            Cmd::Render
        }} else {{
            Cmd::Noop
        }}
    }}
}}

fn main() {{
    println!("{project_name} - Counter Example");
    println!("========================");
    
    let mut counter = Counter::new(0);
    counter.on_mount();
    
    println!("Initial count: {{}}", counter.count);
    
    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    println!("After 3 increments: {{}}", counter.count);
    
    counter.update(Box::new(Decrement));
    println!("After 1 decrement: {{}}", counter.count);
    
    counter.on_unmount();
    println!("Counter unmounted");
}}
"#,
        project_name = ctx.project_name
    )
}

#[allow(clippy::too_many_lines)]
fn render_todo_main(ctx: &TemplateContext) -> String {
    format!(
        r#"
use ctui_core::{{Buffer, Cmd, Component, Msg, Rect}};
use std::collections::HashMap;

struct AddTodo(String);
struct RemoveTodo(usize);
struct ToggleTodo(usize);

impl Msg for AddTodo {{}}
impl Msg for RemoveTodo {{}}
impl Msg for ToggleTodo {{}}

#[derive(Clone, Debug)]
struct TodoItem {{
    id: usize,
    text: String,
    completed: bool,
}}

struct TodoState {{
    items: Vec<TodoItem>,
    next_id: usize,
}}

impl TodoState {{
    fn new() -> Self {{
        Self {{
            items: Vec::new(),
            next_id: 0,
        }}
    }}

    fn add(&mut self, text: String) {{
        let item = TodoItem {{
            id: self.next_id,
            text,
            completed: false,
        }};
        self.next_id += 1;
        self.items.push(item);
    }}

    fn remove(&mut self, id: usize) {{
        self.items.retain(|item| item.id != id);
    }}

    fn toggle(&mut self, id: usize) {{
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {{
            item.completed = !item.completed;
        }}
    }}
}}

struct TodoApp {{
    state: TodoState,
}}

impl Component for TodoApp {{
    type Props = ();
    type State = TodoState;

    fn create(_props: Self::Props) -> Self {{
        Self {{
            state: TodoState::new(),
        }}
    }}

    fn render(&self, area: Rect, buf: &mut Buffer) {{
        let header = "{{:_<width$}}";
        let header = format!(header, "", width = area.width as usize);
        for (col, ch) in header.chars().enumerate() {{
            if let Some(cell) = buf.get_mut(area.x + col as u16, area.y) {{
                cell.symbol = ch.to_string();
            }}
        }}
    }}

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {{
        if let Some(add_msg) = msg.downcast_ref::<AddTodo>() {{
            if !add_msg.0.is_empty() {{
                self.state.add(add_msg.0.clone());
            }}
            Cmd::Render
        }} else if let Some(remove_msg) = msg.downcast_ref::<RemoveTodo>() {{
            self.state.remove(remove_msg.0);
            Cmd::Render
        }} else if let Some(toggle_msg) = msg.downcast_ref::<ToggleTodo>() {{
            self.state.toggle(toggle_msg.0);
            Cmd::Render
        }} else {{
            Cmd::Noop
        }}
    }}
}}

fn main() {{
    println!("{project_name} - Todo App");
    println!("===================");
    
    let mut app = TodoApp::create(());
    app.on_mount();
    
    app.update(Box::new(AddTodo("Learn cTUI".to_string())));
    app.update(Box::new(AddTodo("Build apps".to_string())));
    app.update(Box::new(AddTodo("Ship to prod".to_string())));
    
    println!("Todo items:");
    for (idx, item) in app.state.items.iter().enumerate() {{
        let check = if item.completed {{ "[x]" }} else {{ "[ ]" }};
        println!("  {{}}. {{}} {{}}", idx + 1, check, item.text);
    }}
    
    app.update(Box::new(ToggleTodo(0)));
    println!("\nAfter toggling first item:");
    for (idx, item) in app.state.items.iter().enumerate() {{
        let check = if item.completed {{ "[x]" }} else {{ "[ ]" }};
        println!("  {{}}. {{}} {{}}", idx + 1, check, item.text);
    }}
    
    app.on_unmount();
}}
"#,
        project_name = ctx.project_name
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TemplateContext, TemplateType};

    #[test]
    fn test_cargo_toml_has_project_name() {
        let ctx = TemplateContext {
            project_name: "test-app",
            template_type: TemplateType::Basic,
        };
        let content = render_cargo_toml(&ctx).unwrap();
        assert!(content.contains("test-app"));
    }

    #[test]
    fn test_main_rs_basic() {
        let ctx = TemplateContext {
            project_name: "basic-app",
            template_type: TemplateType::Basic,
        };
        let content = render_main_rs(&ctx).unwrap();
        assert!(content.contains("basic-app"));
        assert!(content.contains("Hello"));
    }

    #[test]
    fn test_main_rs_counter() {
        let ctx = TemplateContext {
            project_name: "counter-app",
            template_type: TemplateType::Counter,
        };
        let content = render_main_rs(&ctx).unwrap();
        assert!(content.contains("Counter"));
        assert!(content.contains("Increment"));
        assert!(content.contains("Decrement"));
    }

    #[test]
    fn test_main_rs_todo() {
        let ctx = TemplateContext {
            project_name: "todo-app",
            template_type: TemplateType::TodoApp,
        };
        let content = render_main_rs(&ctx).unwrap();
        assert!(content.contains("TodoApp"));
        assert!(content.contains("AddTodo"));
    }
}
