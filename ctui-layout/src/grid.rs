use ctui_core::Rect;

use crate::Constraint;

/// Track sizing for grid rows and columns
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum GridTrack {
    /// Auto-sized based on content
    #[default]
    Auto,
    /// Fixed size in cells
    Fixed(u16),
    /// Flexible size that fills available space
    Flexible(u16),
}

impl GridTrack {
    /// Creates an auto-sized track
    #[must_use]
    pub const fn auto() -> Self {
        Self::Auto
    }

    /// Creates a fixed-size track
    #[must_use]
    pub const fn fixed(size: u16) -> Self {
        Self::Fixed(size)
    }

    /// Creates a flexible track with flex factor
    #[must_use]
    pub const fn flex(factor: u16) -> Self {
        Self::Flexible(factor)
    }

    /// Returns true if this track is flexible
    #[must_use]
    pub fn is_flexible(&self) -> bool {
        matches!(self, Self::Flexible(_))
    }

    /// Returns true if this track is fixed-size
    #[must_use]
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }

    /// Returns the flex factor for flexible tracks
    #[must_use]
    pub fn flex_factor(&self) -> u16 {
        match self {
            Self::Flexible(f) => *f,
            _ => 0,
        }
    }

    /// Returns the fixed size if this is a fixed track
    #[must_use]
    pub fn fixed_size(&self) -> Option<u16> {
        match self {
            Self::Fixed(s) => Some(*s),
            _ => None,
        }
    }
}

/// Alignment options for grid items within their cells
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum GridAlignment {
    /// Align to the start of the cell
    #[default]
    Start,
    /// Center within the cell
    Center,
    /// Align to the end of the cell
    End,
    /// Stretch to fill the cell
    Stretch,
}

/// A 2D grid layout with rows, columns, and gutters
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    /// Column track definitions
    pub columns: Vec<GridTrack>,
    /// Row track definitions
    pub rows: Vec<GridTrack>,
    /// Gap between columns
    pub column_gap: u16,
    /// Gap between rows
    pub row_gap: u16,
    /// Default horizontal alignment for items
    pub justify_items: GridAlignment,
    /// Default vertical alignment for items
    pub align_items: GridAlignment,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    /// Creates a new empty grid
    #[must_use]
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            column_gap: 0,
            row_gap: 0,
            justify_items: GridAlignment::default(),
            align_items: GridAlignment::default(),
        }
    }

    /// Creates a grid with the specified number of equal columns
    #[must_use]
    pub fn with_columns(count: usize) -> Self {
        Self::new().columns(vec![GridTrack::Flexible(1); count])
    }

    /// Creates a grid with the specified number of equal rows
    #[must_use]
    pub fn with_rows(count: usize) -> Self {
        Self::new().rows(vec![GridTrack::Flexible(1); count])
    }

    /// Sets the column tracks
    #[must_use]
    pub fn columns(mut self, columns: Vec<GridTrack>) -> Self {
        self.columns = columns;
        self
    }

    /// Sets the row tracks
    #[must_use]
    pub fn rows(mut self, rows: Vec<GridTrack>) -> Self {
        self.rows = rows;
        self
    }

    /// Sets the gap between columns
    #[must_use]
    pub const fn column_gap(mut self, gap: u16) -> Self {
        self.column_gap = gap;
        self
    }

    /// Sets the gap between rows
    #[must_use]
    pub const fn row_gap(mut self, gap: u16) -> Self {
        self.row_gap = gap;
        self
    }

    /// Sets both column and row gap
    #[must_use]
    pub const fn gap(mut self, gap: u16) -> Self {
        self.column_gap = gap;
        self.row_gap = gap;
        self
    }

    /// Sets horizontal alignment for items
    #[must_use]
    pub const fn justify_items(mut self, align: GridAlignment) -> Self {
        self.justify_items = align;
        self
    }

    /// Sets vertical alignment for items
    #[must_use]
    pub const fn align_items(mut self, align: GridAlignment) -> Self {
        self.align_items = align;
        self
    }

    /// Splits the area into grid cells
    #[must_use]
    pub fn split(&self, area: Rect) -> Vec<Vec<Rect>> {
        if self.columns.is_empty() || self.rows.is_empty() {
            return Vec::new();
        }

        let column_widths = self.calculate_column_widths(area.width);
        let row_heights = self.calculate_row_heights(area.height);

        let mut cells = Vec::with_capacity(self.rows.len());
        let mut y = area.y;

        for (row_idx, &row_height) in row_heights.iter().enumerate() {
            let mut row_cells = Vec::with_capacity(self.columns.len());
            let mut x = area.x;

            for (col_idx, &col_width) in column_widths.iter().enumerate() {
                let cell = Rect::new(x, y, col_width, row_height);
                row_cells.push(self.apply_alignment(cell, col_idx, row_idx));
                x = x.saturating_add(col_width).saturating_add(self.column_gap);
            }

            cells.push(row_cells);
            y = y.saturating_add(row_height).saturating_add(self.row_gap);
        }

        cells
    }

    /// Splits the area and returns cells as a flat vector (row-major order)
    #[must_use]
    pub fn split_flat(&self, area: Rect) -> Vec<Rect> {
        self.split(area).into_iter().flatten().collect()
    }

    /// Gets a specific cell by row and column index
    #[must_use]
    pub fn cell(&self, area: Rect, row: usize, col: usize) -> Option<Rect> {
        let cells = self.split(area);
        cells.get(row)?.get(col).copied()
    }

    fn calculate_column_widths(&self, available: u16) -> Vec<u16> {
        self.calculate_track_sizes(&self.columns, available, self.column_gap)
    }

    fn calculate_row_heights(&self, available: u16) -> Vec<u16> {
        self.calculate_track_sizes(&self.rows, available, self.row_gap)
    }

    fn calculate_track_sizes(&self, tracks: &[GridTrack], available: u16, gap: u16) -> Vec<u16> {
        let n = tracks.len();
        if n == 0 {
            return Vec::new();
        }

        let total_gap = gap as u32 * n.saturating_sub(1) as u32;
        let available_for_tracks = available as u32;
        let available_after_gap = available_for_tracks.saturating_sub(total_gap);

        let mut sizes = vec![0u16; n];
        let mut fixed_total = 0u32;
        let mut flex_total = 0u32;

        for (i, track) in tracks.iter().enumerate() {
            match track {
                GridTrack::Fixed(s) => {
                    sizes[i] = *s;
                    fixed_total += *s as u32;
                }
                GridTrack::Auto => {
                    sizes[i] = 1;
                    fixed_total += 1;
                }
                GridTrack::Flexible(f) => {
                    flex_total += *f as u32;
                }
            }
        }

        if flex_total > 0 {
            let remaining = available_after_gap.saturating_sub(fixed_total);
            if remaining > 0 {
                for (i, track) in tracks.iter().enumerate() {
                    if let GridTrack::Flexible(f) = track {
                        let flex_share = (remaining as u64 * *f as u64 / flex_total as u64) as u32;
                        sizes[i] = flex_share.min(u16::MAX as u32) as u16;
                    }
                }
            }
        }

        sizes
    }

    fn apply_alignment(&self, cell: Rect, _col: usize, _row: usize) -> Rect {
        let justify = self.justify_items;
        let align = self.align_items;

        let (x_offset, width) = match justify {
            GridAlignment::Start => (0, cell.width),
            GridAlignment::Center => (cell.width / 4, cell.width / 2.max(1)),
            GridAlignment::End => (cell.width / 2, cell.width / 2.max(1)),
            GridAlignment::Stretch => (0, cell.width),
        };

        let (y_offset, height) = match align {
            GridAlignment::Start => (0, cell.height),
            GridAlignment::Center => (cell.height / 4, cell.height / 2.max(1)),
            GridAlignment::End => (cell.height / 2, cell.height / 2.max(1)),
            GridAlignment::Stretch => (0, cell.height),
        };

        Rect::new(
            cell.x.saturating_add(x_offset),
            cell.y.saturating_add(y_offset),
            width.max(1),
            height.max(1),
        )
    }
}

/// A grid position for explicit placement of items
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GridPosition {
    /// Starting column (0-indexed)
    pub column: usize,
    /// Starting row (0-indexed)
    pub row: usize,
    /// Number of columns to span
    pub column_span: usize,
    /// Number of rows to span
    pub row_span: usize,
}

impl GridPosition {
    /// Creates a new grid position
    #[must_use]
    pub const fn new(column: usize, row: usize) -> Self {
        Self {
            column,
            row,
            column_span: 1,
            row_span: 1,
        }
    }

    /// Creates a position spanning multiple columns
    #[must_use]
    pub const fn with_column_span(mut self, span: usize) -> Self {
        self.column_span = span;
        self
    }

    /// Creates a position spanning multiple rows
    #[must_use]
    pub const fn with_row_span(mut self, span: usize) -> Self {
        self.row_span = span;
        self
    }
}

impl From<Constraint> for GridTrack {
    fn from(constraint: Constraint) -> Self {
        match constraint {
            Constraint::Length(n) => GridTrack::Fixed(n),
            Constraint::Min(n) => GridTrack::Fixed(n),
            Constraint::Max(n) => GridTrack::Fixed(n),
            Constraint::Percentage(_) => GridTrack::Auto,
            Constraint::Ratio(_, _) => GridTrack::Flexible(1),
            Constraint::Fill => GridTrack::Flexible(1),
            Constraint::Range { min, .. } => GridTrack::Fixed(min),
            Constraint::Portion(n) => GridTrack::Flexible(n as u16),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect::new(x, y, width, height)
    }

    #[test]
    fn grid_track_constructors() {
        assert_eq!(GridTrack::auto(), GridTrack::Auto);
        assert_eq!(GridTrack::fixed(20), GridTrack::Fixed(20));
        assert_eq!(GridTrack::flex(2), GridTrack::Flexible(2));
    }

    #[test]
    fn grid_track_is_flexible() {
        assert!(!GridTrack::Auto.is_flexible());
        assert!(!GridTrack::Fixed(10).is_flexible());
        assert!(GridTrack::Flexible(1).is_flexible());
    }

    #[test]
    fn empty_grid_returns_empty() {
        let grid = Grid::new();
        let area = rect(0, 0, 80, 24);
        assert!(grid.split(area).is_empty());
    }

    #[test]
    fn grid_with_columns() {
        let grid = Grid::with_columns(3);
        assert_eq!(grid.columns.len(), 3);
        for col in &grid.columns {
            assert!(matches!(col, GridTrack::Flexible(1)));
        }
    }

    #[test]
    fn grid_with_rows() {
        let grid = Grid::with_rows(4);
        assert_eq!(grid.rows.len(), 4);
        for row in &grid.rows {
            assert!(matches!(row, GridTrack::Flexible(1)));
        }
    }

    #[test]
    fn grid_splits_2x2() {
        let grid = Grid::new()
            .columns(vec![GridTrack::Flexible(1), GridTrack::Flexible(1)])
            .rows(vec![GridTrack::Flexible(1), GridTrack::Flexible(1)]);
        let area = rect(0, 0, 80, 24);
        let cells = grid.split(area);

        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0].len(), 2);
        assert_eq!(cells[1].len(), 2);
    }

    #[test]
    fn grid_fixed_columns() {
        let grid = Grid::new().columns(vec![
            GridTrack::Fixed(20),
            GridTrack::Fixed(30),
            GridTrack::Fixed(30),
        ]);
        let area = rect(0, 0, 80, 24);
        let cells = grid.split_flat(area);

        assert_eq!(cells.len(), 0);
    }

    #[test]
    fn grid_with_rows_and_fixed_columns() {
        let grid = Grid::new()
            .columns(vec![GridTrack::Fixed(20), GridTrack::Fixed(30)])
            .rows(vec![GridTrack::Fixed(10), GridTrack::Fixed(14)]);
        let area = rect(0, 0, 80, 24);
        let cells = grid.split(area);

        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0][0].width, 20);
        assert_eq!(cells[0][1].width, 30);
        assert_eq!(cells[0][0].height, 10);
        assert_eq!(cells[1][0].height, 14);
    }

    #[test]
    fn grid_with_gap() {
        let grid = Grid::new()
            .columns(vec![GridTrack::Flexible(1), GridTrack::Flexible(1)])
            .rows(vec![GridTrack::Flexible(1), GridTrack::Flexible(1)])
            .gap(2);
        let area = rect(0, 0, 84, 28);
        let cells = grid.split(area);

        assert_eq!(cells[0][1].x, cells[0][0].x + cells[0][0].width + 2);
        assert_eq!(cells[1][0].y, cells[0][0].y + cells[0][0].height + 2);
    }

    #[test]
    fn grid_position() {
        let pos = GridPosition::new(0, 0);
        assert_eq!(pos.column, 0);
        assert_eq!(pos.row, 0);
        assert_eq!(pos.column_span, 1);
        assert_eq!(pos.row_span, 1);

        let pos_span = GridPosition::new(0, 0).with_column_span(2).with_row_span(3);
        assert_eq!(pos_span.column_span, 2);
        assert_eq!(pos_span.row_span, 3);
    }

    #[test]
    fn grid_flexible_distribution() {
        let grid = Grid::new()
            .columns(vec![GridTrack::Flexible(1), GridTrack::Flexible(2)])
            .rows(vec![GridTrack::Flexible(1)]);
        let area = rect(0, 0, 90, 24);
        let cells = grid.split(area);

        assert_eq!(cells[0][0].width, 30);
        assert_eq!(cells[0][1].width, 60);
    }

    #[test]
    fn grid_justify_align_center() {
        let grid = Grid::new()
            .columns(vec![GridTrack::Flexible(1)])
            .rows(vec![GridTrack::Flexible(1)])
            .justify_items(GridAlignment::Center)
            .align_items(GridAlignment::Center);
        let area = rect(0, 0, 80, 24);
        let cells = grid.split(area);

        assert_eq!(cells[0][0].x, 20);
        assert_eq!(cells[0][0].y, 6);
    }

    #[test]
    fn grid_cell_method() {
        let grid = Grid::new()
            .columns(vec![GridTrack::Flexible(1), GridTrack::Flexible(1)])
            .rows(vec![GridTrack::Flexible(1), GridTrack::Flexible(1)]);
        let area = rect(0, 0, 80, 24);

        let cell = grid.cell(area, 0, 0);
        assert!(cell.is_some());

        let cell = grid.cell(area, 5, 5);
        assert!(cell.is_none());
    }

    #[test]
    fn constraint_to_grid_track() {
        assert_eq!(
            GridTrack::from(Constraint::Length(20)),
            GridTrack::Fixed(20)
        );
        assert_eq!(GridTrack::from(Constraint::Fill), GridTrack::Flexible(1));
        assert_eq!(
            GridTrack::from(Constraint::Portion(3)),
            GridTrack::Flexible(3)
        );
    }
}
