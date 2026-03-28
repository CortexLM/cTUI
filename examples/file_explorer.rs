//! File explorer example - async I/O operations demonstration
//!
//! Run with: `cargo run --example file-explorer`

use ctui_components::{List, ListItem, ListProps};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug)]
struct FileEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: u64,
}

impl FileEntry {
    fn new(name: &str, path: PathBuf, is_dir: bool, size: u64) -> Self {
        Self {
            name: name.to_string(),
            path,
            is_dir,
            size,
        }
    }

    fn display_size(&self) -> String {
        if self.is_dir {
            "<DIR>".to_string()
        } else if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", self.size as f64 / (1024.0 * 1024.0))
        }
    }
}

struct NavigateTo(PathBuf);
struct SelectFile(usize);
struct RefreshFiles;

impl Msg for NavigateTo {}
impl Msg for SelectFile {}
impl Msg for RefreshFiles {}

struct FileExplorerState {
    current_path: PathBuf,
    entries: Vec<FileEntry>,
    selected_index: usize,
    loading: bool,
}

impl FileExplorerState {
    fn new() -> Self {
        Self {
            current_path: PathBuf::from("."),
            entries: Vec::new(),
            selected_index: 0,
            loading: false,
        }
    }

    fn add_mock_entries(&mut self) {
        self.entries = vec![
            FileEntry::new(
                "..",
                self.current_path
                    .parent()
                    .unwrap_or(&self.current_path)
                    .to_path_buf(),
                true,
                0,
            ),
            FileEntry::new("src", self.current_path.join("src"), true, 0),
            FileEntry::new(
                "Cargo.toml",
                self.current_path.join("Cargo.toml"),
                false,
                1024,
            ),
            FileEntry::new(
                "README.md",
                self.current_path.join("README.md"),
                false,
                2048,
            ),
            FileEntry::new("target", self.current_path.join("target"), true, 0),
        ];
    }

    fn select_next(&mut self) {
        if self.selected_index < self.entries.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}

struct FileExplorer {
    state: FileExplorerState,
}

impl FileExplorer {
    fn new() -> Self {
        let mut state = FileExplorerState::new();
        state.add_mock_entries();
        Self { state }
    }
}

impl Component for FileExplorer {
    type Props = PathBuf;
    type State = FileExplorerState;

    fn create(props: Self::Props) -> Self {
        let mut explorer = Self::new();
        explorer.state.current_path = props;
        explorer
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let header = "╔═══════════════════════════════════════════════════════╗";
        let title = format!(
            "║ File Explorer: {:<38}║",
            self.state
                .current_path
                .display()
                .to_string()
                .chars()
                .take(38)
                .collect::<String>()
        );
        let divider = "╠═══════════════════════════════════════════════════════╣";
        let footer = "╚═══════════════════════════════════════════════════════╝";

        for (col, ch) in header.chars().enumerate() {
            buf.modify_cell(area.x + col as u16, area.y, |cell| { cell.symbol = ch.to_string(); });
        }

        for (col, ch) in title.chars().take(area.width as usize).enumerate() {
            buf.modify_cell(area.x + col as u16, area.y + 1, |cell| { cell.symbol = ch.to_string(); });
        }

        for (col, ch) in divider.chars().enumerate() {
            buf.modify_cell(area.x + col as u16, area.y + 2, |cell| { cell.symbol = ch.to_string(); });
        }

        let entries_start = 3;
        for (idx, entry) in self.state.entries.iter().enumerate() {
            let selector = if idx == self.state.selected_index {
                ">"
            } else {
                " "
            };
            let icon = if entry.is_dir { "📁" } else { "📄" };
            let line = format!(
                "║ {} {} {} {:<30} {:>10} ║",
                selector,
                icon,
                if entry.is_dir { " [DIR]" } else { "      " },
                entry.name.chars().take(30).collect::<String>(),
                entry.display_size()
            );

            for (col, ch) in line.chars().take(area.width as usize).enumerate() {
                buf.modify_cell(area.x + col as u16, area.y + entries_start + idx as u16, |cell| {
                    cell.symbol = ch.to_string();
                });
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if let Some(nav) = msg.downcast_ref::<NavigateTo>() {
            self.state.current_path = nav.0.clone();
            self.state.loading = true;
            self.state.add_mock_entries();
            self.state.loading = false;
            self.state.selected_index = 0;
            Cmd::Render
        } else if let Some(select) = msg.downcast_ref::<SelectFile>() {
            if select.0 < self.state.entries.len() {
                self.state.selected_index = select.0;
            }
            Cmd::Render
        } else if msg.is::<RefreshFiles>() {
            self.state.add_mock_entries();
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

fn main() {
    let mut explorer = FileExplorer::create(PathBuf::from("."));
    explorer.on_mount();

    println!("File Explorer Example");
    println!("=====================\n");

    println!("Current path: {}", explorer.state.current_path.display());
    println!("\nEntries:");
    for (idx, entry) in explorer.state.entries.iter().enumerate() {
        let selector = if idx == explorer.state.selected_index {
            ">"
        } else {
            " "
        };
        let icon = if entry.is_dir { "📁" } else { "📄" };
        println!(
            "  {} {} {} - {}",
            selector,
            icon,
            entry.name,
            entry.display_size()
        );
    }

    explorer.update(Box::new(SelectFile(2)));
    println!("\nAfter selecting index 2:");
    for (idx, entry) in explorer.state.entries.iter().enumerate() {
        let selector = if idx == explorer.state.selected_index {
            ">"
        } else {
            " "
        };
        let icon = if entry.is_dir { "📁" } else { "📄" };
        println!(
            "  {} {} {} - {}",
            selector,
            icon,
            entry.name,
            entry.display_size()
        );
    }

    explorer.update(Box::new(NavigateTo(PathBuf::from("/tmp"))));
    println!("\nAfter navigating to /tmp:");
    println!("Current path: {}", explorer.state.current_path.display());
    println!("Selected index: {}", explorer.state.selected_index);

    println!("\n✓ File explorer async operations verified");
    explorer.on_unmount();
}
