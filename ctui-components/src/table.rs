//! Table component for rendering tabular data

use crate::text::Line;
use ctui_core::style::{Color, Modifier, Style};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};
use ctui_layout::Constraint;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortOrder {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn toggle(&self) -> Self {
        match self {
            SortOrder::None => SortOrder::Ascending,
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::None,
        }
    }

    pub fn is_sorted(&self) -> bool {
        !matches!(self, SortOrder::None)
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    header: Line,
    constraint: Constraint,
}

impl Column {
    pub fn new(header: impl Into<Line>, constraint: Constraint) -> Self {
        Self {
            header: header.into(),
            constraint,
        }
    }

    pub fn fixed(header: impl Into<Line>, width: u16) -> Self {
        Self::new(header, Constraint::Length(width))
    }

    pub fn fill(header: impl Into<Line>) -> Self {
        Self::new(header, Constraint::Fill)
    }

    pub fn percentage(header: impl Into<Line>, pct: u16) -> Self {
        Self::new(header, Constraint::Percentage(pct))
    }

    pub fn min(header: impl Into<Line>, min_width: u16) -> Self {
        Self::new(header, Constraint::Min(min_width))
    }

    pub fn header(&self) -> &Line {
        &self.header
    }

    pub fn constraint(&self) -> Constraint {
        self.constraint
    }
}

impl Default for Column {
    fn default() -> Self {
        Self {
            header: Line::default(),
            constraint: Constraint::Fill,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Cell {
    content: String,
    style: Option<Style>,
}

impl Cell {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: None,
        }
    }

    pub fn styled(content: impl Into<String>, style: Style) -> Self {
        Self {
            content: content.into(),
            style: Some(style),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn style(&self) -> Option<&Style> {
        self.style.as_ref()
    }

    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self.content.as_str())
    }
}

impl From<String> for Cell {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Row {
    cells: Vec<Cell>,
    style: Option<Style>,
}

impl Row {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_cells(cells: Vec<Cell>) -> Self {
        Self { cells, style: None }
    }

    pub fn from_strings<T: AsRef<str>>(strings: impl IntoIterator<Item = T>) -> Self {
        Self {
            cells: strings.into_iter().map(|s| Cell::new(s.as_ref())).collect(),
            style: None,
        }
    }

    pub fn data<D: Into<Cell>>(data: impl IntoIterator<Item = D>) -> Self {
        Self {
            cells: data.into_iter().map(|d| d.into()).collect(),
            style: None,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Cell> {
        self.cells.get(index)
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Row>,
    selected: Option<usize>,
    header_style: Style,
    selected_style: Style,
    row_style: Style,
    scroll: usize,
    show_header: bool,
    highlight_symbol: String,
    sort_column: Option<usize>,
    sort_order: SortOrder,
    column_widths: Vec<Option<u16>>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            selected: None,
            header_style: Style::new().add_modifier(Modifier::BOLD),
            selected_style: Style::new().bg(Color::Blue),
            row_style: Style::default(),
            scroll: 0,
            show_header: true,
            highlight_symbol: " ".to_string(),
            sort_column: None,
            sort_order: SortOrder::None,
            column_widths: Vec::new(),
        }
    }
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(columns: impl IntoIterator<Item = Column>) -> Self {
        Self {
            columns: columns.into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn set_columns(mut self, columns: impl IntoIterator<Item = Column>) -> Self {
        self.columns = columns.into_iter().collect();
        self
    }

    pub fn add_column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    pub fn set_rows(mut self, rows: impl IntoIterator<Item = Row>) -> Self {
        self.rows = rows.into_iter().collect();
        self
    }

    pub fn add_row(mut self, row: Row) -> Self {
        self.rows.push(row);
        self
    }

    pub fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    pub fn row_style(mut self, style: Style) -> Self {
        self.row_style = style;
        self
    }

    pub fn show_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    pub fn scroll(mut self, scroll: usize) -> Self {
        self.scroll = scroll;
        self
    }

    pub fn highlight_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.highlight_symbol = symbol.into();
        self
    }

    pub fn sort_by(mut self, column: usize, order: SortOrder) -> Self {
        if column < self.columns.len() {
            self.sort_column = Some(column);
            self.sort_order = order;
            self.apply_sort();
        }
        self
    }

    pub fn sort_column(mut self, column: usize) -> Self {
        if column < self.columns.len() {
            if self.sort_column == Some(column) {
                self.sort_order = self.sort_order.toggle();
            } else {
                self.sort_column = Some(column);
                self.sort_order = SortOrder::Ascending;
            }
            self.apply_sort();
        }
        self
    }

    pub fn get_sort(&self) -> (Option<usize>, SortOrder) {
        (self.sort_column, self.sort_order)
    }

    fn apply_sort(&mut self) {
        if let Some(col_idx) = self.sort_column {
            if self.sort_order.is_sorted() {
                let sort_idx = col_idx;
                self.rows.sort_by(|a, b| {
                    let a_val = a.get(sort_idx).map(|c| c.content()).unwrap_or("");
                    let b_val = b.get(sort_idx).map(|c| c.content()).unwrap_or("");
                    match self.sort_order {
                        SortOrder::Ascending => a_val.cmp(b_val),
                        SortOrder::Descending => b_val.cmp(a_val),
                        SortOrder::None => std::cmp::Ordering::Equal,
                    }
                });
            }
        }
    }

    pub fn clear_sort(mut self) -> Self {
        self.sort_column = None;
        self.sort_order = SortOrder::None;
        self
    }

    pub fn resize_column(mut self, column: usize, width: u16) -> Self {
        if column < self.columns.len() {
            if self.column_widths.len() <= column {
                self.column_widths.resize(column + 1, None);
            }
            self.column_widths[column] = Some(width);
        }
        self
    }

    pub fn auto_resize_columns(&mut self) {
        for (i, col) in self.columns.iter().enumerate() {
            let mut max_width = UnicodeWidthStr::width(col.header().content().as_str());
            for row in &self.rows {
                if let Some(cell) = row.get(i) {
                    let cell_width = cell.width();
                    max_width = max_width.max(cell_width);
                }
            }
            if self.column_widths.len() <= i {
                self.column_widths.resize(i + 1, None);
            }
            self.column_widths[i] = Some(max_width as u16 + 2);
        }
    }

    pub fn get_column_width(&self, column: usize) -> Option<u16> {
        self.column_widths.get(column).and_then(|w| *w)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn get_selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn get_rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn get_columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn select_next(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1).min(self.rows.len() - 1),
            None => 0,
        });
    }

    pub fn select_prev(&mut self) {
        self.selected = Some(match self.selected {
            Some(i) => i.saturating_sub(1),
            None => 0,
        });
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index.and_then(|i| if i < self.rows.len() { Some(i) } else { None });
    }

    fn calculate_column_widths(&self, available_width: u16) -> Vec<u16> {
        if self.columns.is_empty() {
            return vec![];
        }

        let mut widths = Vec::with_capacity(self.columns.len());
        let mut remaining_width = available_width as i32;
        let mut flexible_count = 0;

        for (i, col) in self.columns.iter().enumerate() {
            if let Some(custom_width) = self.column_widths.get(i).and_then(|w| *w) {
                let w = custom_width.min(available_width);
                widths.push(w);
                remaining_width -= w as i32;
                continue;
            }

            match col.constraint {
                Constraint::Length(len) => {
                    let w = len.min(available_width);
                    widths.push(w);
                    remaining_width -= w as i32;
                }
                Constraint::Percentage(pct) => {
                    let w = ((available_width as u32 * pct.min(100) as u32) / 100) as u16;
                    let w = w.max(1);
                    widths.push(w);
                    remaining_width -= w as i32;
                }
                _ => {
                    let min_w = col.constraint.min_size();
                    widths.push(min_w);
                    remaining_width -= min_w as i32;
                    flexible_count += 1;
                }
            }
        }

        if flexible_count == 0 {
            return widths;
        }

        let mut ratio_total: u32 = 0;
        for col in self.columns.iter() {
            if let Constraint::Ratio(num, _) = col.constraint {
                ratio_total += num;
            } else if matches!(col.constraint, Constraint::Min(_) | Constraint::Fill) {
                ratio_total += 1;
            }
        }

        if remaining_width > 0 && ratio_total > 0 {
            for (i, col) in self.columns.iter().enumerate() {
                let ratio = match col.constraint {
                    Constraint::Ratio(num, _) => num as i32,
                    Constraint::Min(_) | Constraint::Fill => 1,
                    _ => 0,
                };

                if ratio > 0 {
                    let extra = (remaining_width * ratio) / ratio_total as i32;
                    widths[i] = (widths[i] as i32 + extra) as u16;
                }
            }
        }

        widths
    }

    fn render_row(
        &self,
        row: &Row,
        y: u16,
        x_start: u16,
        widths: &[u16],
        is_selected: bool,
        buf: &mut Buffer,
    ) {
        let row_style = if is_selected {
            self.selected_style
        } else {
            row.style.unwrap_or(self.row_style)
        };

        let mut x = x_start;

        if is_selected && !self.highlight_symbol.is_empty() {
            let sym_width = UnicodeWidthStr::width(self.highlight_symbol.as_str());
            for (i, ch) in self.highlight_symbol.chars().enumerate() {
                buf.modify_cell(x + i as u16, y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(row_style);
                });
            }
            x += sym_width as u16;
        }

        for (i, width) in widths.iter().enumerate() {
            let cell = row.get(i);
            let cell_content = cell.map(|c| c.content()).unwrap_or("");
            let cell_style = cell.and_then(|c| c.style()).unwrap_or(&row_style);

            let truncated = Self::truncate_to_width(cell_content, *width as usize);

            for (ch_i, ch) in truncated.chars().enumerate() {
                let ch_x = x + ch_i as u16;
                if ch_x >= x_start + x + *width || ch_x >= buf.area.x + buf.area.width {
                    break;
                }
                buf.modify_cell(ch_x, y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(*cell_style);
                });
            }

            x += *width;
        }
    }

    fn truncate_to_width(s: &str, max_width: usize) -> String {
        let mut result = String::new();
        let mut width = 0;

        for ch in s.chars() {
            let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());
            if width + ch_width > max_width {
                break;
            }
            result.push(ch);
            width += ch_width;
        }

        result
    }
}

pub struct TableProps {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
    pub selected: Option<usize>,
    pub header_style: Style,
    pub selected_style: Style,
}

impl TableProps {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
            selected: None,
            header_style: Style::new().add_modifier(Modifier::BOLD),
            selected_style: Style::new().bg(Color::Blue),
        }
    }

    pub fn rows(mut self, rows: Vec<Row>) -> Self {
        self.rows = rows;
        self
    }

    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }
}

impl Component for Table {
    type Props = TableProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            columns: props.columns,
            rows: props.rows,
            selected: props.selected,
            header_style: props.header_style,
            selected_style: props.selected_style,
            row_style: Style::default(),
            scroll: 0,
            show_header: true,
            highlight_symbol: " ".to_string(),
            sort_column: None,
            sort_order: SortOrder::None,
            column_widths: Vec::new(),
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || self.columns.is_empty() {
            return;
        }

        let mut y = area.y;
        let widths = self.calculate_column_widths(area.width);

        if self.show_header && !self.columns.is_empty() {
            for (i, (col, width)) in self.columns.iter().zip(widths.iter()).enumerate() {
                let x = area.x + widths.iter().take(i).sum::<u16>();
                let header_content = col.header().content();

                let sort_indicator = if self.sort_column == Some(i) {
                    match self.sort_order {
                        SortOrder::Ascending => " ▲",
                        SortOrder::Descending => " ▼",
                        SortOrder::None => "",
                    }
                } else {
                    ""
                };

                let display = format!("{}{}", header_content, sort_indicator);
                let truncated = Self::truncate_to_width(&display, *width as usize);

                for (ch_i, ch) in truncated.chars().enumerate() {
                    let ch_x = x + ch_i as u16;
                    if ch_x >= area.x + area.width {
                        break;
                    }
                    buf.modify_cell(ch_x, y, |cell| {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.header_style);
                    });
                }
            }

            y += 1;
            if y < area.y + area.height {
                for x in area.x..area.x + area.width {
                    buf.modify_cell(x, y, |cell| {
                        cell.symbol = "─".to_string();
                        cell.set_style(self.header_style);
                    });
                }
            }
            y += 1;
        }

        let start_row = self.scroll;
        let visible_height = (area.y + area.height).saturating_sub(y) as usize;
        let end_row = (start_row + visible_height).min(self.rows.len());

        for row_idx in start_row..end_row {
            if y >= area.y + area.height {
                break;
            }

            let row = &self.rows[row_idx];
            let is_selected = self.selected == Some(row_idx);

            self.render_row(row, y, area.x, &widths, is_selected, buf);
            y += 1;
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::Buffer;
    use insta::assert_snapshot;

    fn render_to_string(table: &Table, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        table.render(Rect::new(0, 0, width, height), &mut buf);

        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = buf.get(x, y) { output.push_str(&cell.symbol); }
            }
            if y < height - 1 {
                output.push('\n');
            }
        }
        output
    }

    #[test]
    fn snapshot_table_empty() {
        let table = Table::new();
        let result = render_to_string(&table, 20, 5);
        assert_snapshot!("table_empty", result);
    }

    #[test]
    fn snapshot_table_header_only() {
        let table = Table::new()
            .add_column(Column::fixed("ID", 5))
            .add_column(Column::fixed("Name", 10))
            .add_column(Column::fixed("Age", 5));
        let result = render_to_string(&table, 20, 5);
        assert_snapshot!("table_header_only", result);
    }

    #[test]
    fn snapshot_table_with_rows() {
        let table = Table::new()
            .add_column(Column::fixed("ID", 5))
            .add_column(Column::fixed("Name", 10))
            .add_column(Column::fixed("Age", 5))
            .add_row(Row::from_strings(vec!["1", "Alice", "25"]))
            .add_row(Row::from_strings(vec!["2", "Bob", "30"]))
            .add_row(Row::from_strings(vec!["3", "Charlie", "35"]));
        let result = render_to_string(&table, 20, 5);
        assert_snapshot!("table_with_rows", result);
    }

    #[test]
    fn snapshot_table_no_header() {
        let table = Table::new()
            .show_header(false)
            .add_column(Column::fixed("A", 5))
            .add_column(Column::fixed("B", 5))
            .add_row(Row::from_strings(vec!["X1", "Y1"]))
            .add_row(Row::from_strings(vec!["X2", "Y2"]));
        let result = render_to_string(&table, 10, 2);
        assert_snapshot!("table_no_header", result);
    }

    #[test]
    fn snapshot_table_with_selection_first() {
        let table = Table::new()
            .add_column(Column::fixed("Item", 10))
            .add_row(Row::from_strings(vec!["First"]))
            .add_row(Row::from_strings(vec!["Second"]))
            .add_row(Row::from_strings(vec!["Third"]))
            .with_selected(Some(0))
            .selected_style(Style::new().bg(Color::Blue));
        let result = render_to_string(&table, 10, 5);
        assert_snapshot!("table_selection_first", result);
    }

    #[test]
    fn snapshot_table_with_selection_middle() {
        let table = Table::new()
            .add_column(Column::fixed("Item", 10))
            .add_row(Row::from_strings(vec!["First"]))
            .add_row(Row::from_strings(vec!["Second"]))
            .add_row(Row::from_strings(vec!["Third"]))
            .with_selected(Some(1))
            .selected_style(Style::new().bg(Color::Blue));
        let result = render_to_string(&table, 10, 5);
        assert_snapshot!("table_selection_middle", result);
    }

    #[test]
    fn snapshot_table_with_selection_last() {
        let table = Table::new()
            .add_column(Column::fixed("Item", 10))
            .add_row(Row::from_strings(vec!["First"]))
            .add_row(Row::from_strings(vec!["Second"]))
            .add_row(Row::from_strings(vec!["Third"]))
            .with_selected(Some(2))
            .selected_style(Style::new().bg(Color::Blue));
        let result = render_to_string(&table, 10, 6);
        assert_snapshot!("table_selection_last", result);
    }

    #[test]
    fn snapshot_table_scrolled() {
        let mut table = Table::new()
            .add_column(Column::fixed("Num", 5))
            .add_column(Column::fixed("Value", 10))
            .add_row(Row::from_strings(vec!["1", "One"]))
            .add_row(Row::from_strings(vec!["2", "Two"]))
            .add_row(Row::from_strings(vec!["3", "Three"]))
            .add_row(Row::from_strings(vec!["4", "Four"]))
            .add_row(Row::from_strings(vec!["5", "Five"]))
            .scroll(2);
        let result = render_to_string(&table, 15, 4);
        assert_snapshot!("table_scrolled", result);
    }

    #[test]
    fn snapshot_table_fill_column() {
        let table = Table::new()
            .add_column(Column::fixed("ID", 5))
            .add_column(Column::fill("Description"))
            .add_row(Row::from_strings(vec!["1", "A long description"]))
            .add_row(Row::from_strings(vec!["2", "Short"]));
        let result = render_to_string(&table, 25, 4);
        assert_snapshot!("table_fill_column", result);
    }

    #[test]
    fn snapshot_table_percentage_columns() {
        let table = Table::new()
            .add_column(Column::percentage("A", 30))
            .add_column(Column::percentage("B", 40))
            .add_column(Column::percentage("C", 30))
            .add_row(Row::from_strings(vec!["Left", "Middle", "Right"]));
        let result = render_to_string(&table, 20, 3);
        assert_snapshot!("table_percentage_columns", result);
    }

    #[test]
    fn test_column_new() {
        let col = Column::new("Name", Constraint::Length(10));
        assert_eq!(col.header().content(), "Name");
        assert_eq!(col.constraint(), Constraint::Length(10));
    }

    #[test]
    fn test_column_fixed() {
        let col = Column::fixed("ID", 8);
        assert_eq!(col.constraint(), Constraint::Length(8));
    }

    #[test]
    fn test_column_fill() {
        let col = Column::fill("Description");
        assert_eq!(col.constraint(), Constraint::Fill);
    }

    #[test]
    fn test_column_percentage() {
        let col = Column::percentage("Name", 50);
        assert_eq!(col.constraint(), Constraint::Percentage(50));
    }

    #[test]
    fn test_column_min() {
        let col = Column::min("Notes", 5);
        assert_eq!(col.constraint(), Constraint::Min(5));
    }

    #[test]
    fn test_cell_new() {
        let cell = Cell::new("Hello");
        assert_eq!(cell.content(), "Hello");
        assert!(cell.style().is_none());
    }

    #[test]
    fn test_cell_styled() {
        let style = Style::new().fg(Color::Red);
        let cell = Cell::styled("Test", style);
        assert_eq!(cell.content(), "Test");
        assert!(cell.style().is_some());
    }

    #[test]
    fn test_cell_width() {
        let cell = Cell::new("Hello");
        assert_eq!(cell.width(), 5);

        let wide_cell = Cell::new("你好");
        assert_eq!(wide_cell.width(), 4);
    }

    #[test]
    fn test_row_new() {
        let row = Row::new();
        assert!(row.is_empty());
        assert_eq!(row.len(), 0);
    }

    #[test]
    fn test_row_from_strings() {
        let row = Row::from_strings(vec!["A", "B", "C"]);
        assert_eq!(row.len(), 3);
        assert_eq!(row.get(0).unwrap().content(), "A");
        assert_eq!(row.get(1).unwrap().content(), "B");
        assert_eq!(row.get(2).unwrap().content(), "C");
    }

    #[test]
    fn test_row_from_cells() {
        let row = Row::from_cells(vec![Cell::new("X"), Cell::new("Y")]);
        assert_eq!(row.len(), 2);
    }

    #[test]
    fn test_row_style() {
        let style = Style::new().bg(Color::Blue);
        let row = Row::from_strings(vec!["A"]).style(style);
        assert!(row.style.is_some());
    }

    #[test]
    fn test_table_new() {
        let table = Table::new();
        assert!(table.is_empty());
        assert_eq!(table.get_selected(), None);
    }

    #[test]
    fn test_table_with_columns() {
        let table = Table::with_columns(vec![Column::fixed("ID", 5), Column::fill("Name")]);
        assert_eq!(table.get_columns().len(), 2);
    }

    #[test]
    fn test_table_rows() {
        let table = Table::new()
            .add_row(Row::from_strings(vec!["1", "Alice"]))
            .add_row(Row::from_strings(vec!["2", "Bob"]));
        assert_eq!(table.len(), 2);
        assert!(!table.is_empty());
    }

    #[test]
    fn test_table_select() {
        let mut table = Table::new()
            .add_row(Row::from_strings(vec!["A"]))
            .add_row(Row::from_strings(vec!["B"]))
            .add_row(Row::from_strings(vec!["C"]));

        table.select_next();
        assert_eq!(table.get_selected(), Some(0));

        table.select_next();
        assert_eq!(table.get_selected(), Some(1));

        table.select_next();
        table.select_next();
        assert_eq!(table.get_selected(), Some(2));

        table.select_prev();
        assert_eq!(table.get_selected(), Some(1));
    }

    #[test]
    fn test_table_select_empty() {
        let mut table = Table::new();
        table.select_next();
        assert_eq!(table.get_selected(), None);
    }

    #[test]
    fn test_calculate_column_widths_fixed() {
        let table = Table::new()
            .add_column(Column::fixed("A", 5))
            .add_column(Column::fixed("B", 10))
            .add_column(Column::fixed("C", 3));

        let widths = table.calculate_column_widths(100);
        assert_eq!(widths, vec![5, 10, 3]);
    }

    #[test]
    fn test_calculate_column_widths_percentage() {
        let table = Table::new()
            .add_column(Column::percentage("A", 25))
            .add_column(Column::percentage("B", 50))
            .add_column(Column::percentage("C", 25));

        let widths = table.calculate_column_widths(100);
        assert_eq!(widths, vec![25, 50, 25]);
    }

    #[test]
    fn test_calculate_column_widths_mixed() {
        let table = Table::new()
            .add_column(Column::fixed("ID", 5))
            .add_column(Column::fill("Name"))
            .add_column(Column::fixed("Age", 3));

        let widths = table.calculate_column_widths(20);
        assert_eq!(widths[0], 5);
        assert_eq!(widths[1], 12);
        assert_eq!(widths[2], 3);
    }

    #[test]
    fn test_calculate_column_widths_empty() {
        let table = Table::new();
        let widths = table.calculate_column_widths(100);
        assert!(widths.is_empty());
    }

    #[test]
    fn test_truncate_to_width() {
        assert_eq!(Table::truncate_to_width("Hello", 10), "Hello");
        assert_eq!(Table::truncate_to_width("Hello World", 5), "Hello");
        assert_eq!(Table::truncate_to_width("你好世界", 3), "你");
        assert_eq!(Table::truncate_to_width("你好世界", 4), "你好");
    }

    #[test]
    fn test_render_basic() {
        let table = Table::new()
            .show_header(false)
            .add_column(Column::fixed("A", 5))
            .add_column(Column::fixed("B", 5))
            .add_row(Row::from_strings(vec!["Hello", "World"]));

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        table.render(Rect::new(0, 0, 10, 1), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "H");
        assert_eq!(buf.get(1, 0).unwrap().symbol, "e");
        assert_eq!(buf.get(5, 0).unwrap().symbol, "W");
    }

    #[test]
    fn test_render_with_header() {
        let table = Table::new()
            .add_column(Column::fixed("Col1", 5))
            .add_column(Column::fixed("Col2", 5))
            .add_row(Row::from_strings(vec!["A", "B"]));

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 3));
        table.render(Rect::new(0, 0, 10, 3), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "C");
        assert_eq!(buf.get(5, 0).unwrap().symbol, "C");

        assert_eq!(buf.get(0, 1).unwrap().symbol, "─");

        assert_eq!(buf.get(0, 2).unwrap().symbol, "A");
        assert_eq!(buf.get(5, 2).unwrap().symbol, "B");
    }

    #[test]
    fn test_render_with_selection() {
        let table = Table::new()
            .show_header(false)
            .add_column(Column::fixed("A", 5))
            .add_column(Column::fixed("B", 5))
            .add_row(Row::from_strings(vec!["Row1", "Data"]))
            .add_row(Row::from_strings(vec!["Row2", "Data"]))
            .with_selected(Some(0))
            .selected_style(Style::new().bg(Color::Blue));

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 2));
        table.render(Rect::new(0, 0, 10, 2), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().bg, Color::Blue);
        assert_eq!(buf.get(0, 1).unwrap().bg, Color::Reset);
    }

    #[test]
    fn test_render_scroll() {
        let table = Table::new()
            .show_header(false)
            .add_column(Column::fixed("A", 5))
            .add_row(Row::from_strings(vec!["Row1"]))
            .add_row(Row::from_strings(vec!["Row2"]))
            .add_row(Row::from_strings(vec!["Row3"]))
            .scroll(1);

        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
        table.render(Rect::new(0, 0, 5, 1), &mut buf);

        assert!(buf.get(0, 0).unwrap().symbol.starts_with('R'));
    }

    #[test]
    fn test_cell_from_string() {
        let cell: Cell = "Hello".into();
        assert_eq!(cell.content(), "Hello");
    }

    #[test]
    fn test_row_data() {
        let row: Row = Row::data(vec![Cell::new("A"), Cell::new("B")]);
        assert_eq!(row.len(), 2);
    }

    #[test]
    fn test_sort_order_toggle() {
        let order = SortOrder::None;
        assert_eq!(order.toggle(), SortOrder::Ascending);
        assert_eq!(SortOrder::Ascending.toggle(), SortOrder::Descending);
        assert_eq!(SortOrder::Descending.toggle(), SortOrder::None);
    }

    #[test]
    fn test_sort_order_is_sorted() {
        assert!(!SortOrder::None.is_sorted());
        assert!(SortOrder::Ascending.is_sorted());
        assert!(SortOrder::Descending.is_sorted());
    }

    #[test]
    fn test_table_sort() {
        let table = Table::new()
            .add_column(Column::fixed("Name", 10))
            .add_row(Row::from_strings(vec!["Charlie"]))
            .add_row(Row::from_strings(vec!["Alice"]))
            .add_row(Row::from_strings(vec!["Bob"]))
            .sort_by(0, SortOrder::Ascending);

        let rows = table.get_rows();
        assert_eq!(rows[0].get(0).unwrap().content(), "Alice");
        assert_eq!(rows[1].get(0).unwrap().content(), "Bob");
        assert_eq!(rows[2].get(0).unwrap().content(), "Charlie");
    }

    #[test]
    fn test_table_sort_descending() {
        let table = Table::new()
            .add_column(Column::fixed("Value", 10))
            .add_row(Row::from_strings(vec!["10"]))
            .add_row(Row::from_strings(vec!["30"]))
            .add_row(Row::from_strings(vec!["20"]))
            .sort_by(0, SortOrder::Descending);

        let rows = table.get_rows();
        assert_eq!(rows[0].get(0).unwrap().content(), "30");
        assert_eq!(rows[1].get(0).unwrap().content(), "20");
        assert_eq!(rows[2].get(0).unwrap().content(), "10");
    }

    #[test]
    fn test_table_sort_toggle() {
        let mut table = Table::new()
            .add_column(Column::fixed("Name", 10))
            .add_row(Row::from_strings(vec!["Charlie"]))
            .add_row(Row::from_strings(vec!["Alice"]))
            .add_row(Row::from_strings(vec!["Bob"]));

        table = table.sort_column(0);
        assert_eq!(table.get_sort().1, SortOrder::Ascending);
        let rows = table.get_rows();
        assert_eq!(rows[0].get(0).unwrap().content(), "Alice");

        table = table.sort_column(0);
        assert_eq!(table.get_sort().1, SortOrder::Descending);
        let rows = table.get_rows();
        assert_eq!(rows[0].get(0).unwrap().content(), "Charlie");

        table = table.sort_column(0);
        assert_eq!(table.get_sort().1, SortOrder::None);
    }

    #[test]
    fn test_table_clear_sort() {
        let table = Table::new()
            .add_column(Column::fixed("Name", 10))
            .add_row(Row::from_strings(vec!["Z"]))
            .add_row(Row::from_strings(vec!["A"]))
            .sort_by(0, SortOrder::Ascending)
            .clear_sort();

        assert!(table.get_sort().0.is_none());
        assert_eq!(table.get_sort().1, SortOrder::None);
    }

    #[test]
    fn test_table_resize_column() {
        let table = Table::new()
            .add_column(Column::fixed("A", 5))
            .add_column(Column::fixed("B", 5))
            .resize_column(0, 10);

        assert_eq!(table.get_column_width(0), Some(10));
        assert_eq!(table.get_column_width(1), None);
    }

    #[test]
    fn test_table_auto_resize_columns() {
        let mut table = Table::new()
            .add_column(Column::fill("Short"))
            .add_column(Column::fill("MediumHeader"))
            .add_row(Row::from_strings(vec!["Data1", "VeryLongDataContent"]));

        table.auto_resize_columns();

        assert!(table.get_column_width(0).unwrap() >= 5);
        assert!(table.get_column_width(1).unwrap() >= 20);
    }

    #[test]
    fn snapshot_table_with_sort() {
        let table = Table::new()
            .add_column(Column::fixed("Name", 10))
            .add_column(Column::fixed("Value", 8))
            .add_row(Row::from_strings(vec!["Charlie", "30"]))
            .add_row(Row::from_strings(vec!["Alice", "10"]))
            .add_row(Row::from_strings(vec!["Bob", "20"]))
            .sort_by(0, SortOrder::Ascending);
        let result = render_to_string(&table, 18, 6);
        assert_snapshot!("table_with_sort", result);
    }
}
