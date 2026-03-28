use std::cmp::min;

use ctui_core::Rect;

use crate::Constraint;

/// The direction items are laid out in the flex container
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FlexDirection {
    /// Items are laid out horizontally (left to right)
    #[default]
    Row,
    /// Items are laid out vertically (top to bottom)
    Column,
}

/// How items are distributed along the main axis
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum JustifyContent {
    /// Items are packed toward the start of the line
    #[default]
    Start,
    /// Items are centered along the line
    Center,
    /// Items are packed toward the end of the line
    End,
    /// Items are evenly distributed with first item at start, last at end
    SpaceBetween,
    /// Items are evenly distributed with equal space around each item
    SpaceAround,
}

/// How items are aligned on the cross axis
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AlignItems {
    /// Items are aligned to the start of the cross axis
    #[default]
    Start,
    /// Items are centered on the cross axis
    Center,
    /// Items are aligned to the end of the cross axis
    End,
    /// Items stretch to fill the container on the cross axis
    Stretch,
}

/// How multiple lines are aligned when flex_wrap is enabled
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AlignContent {
    /// Lines are packed toward the start of the cross axis
    #[default]
    Start,
    /// Lines are centered on the cross axis
    Center,
    /// Lines are packed toward the end of the cross axis
    End,
    /// Lines evenly distributed with first at start, last at end
    SpaceBetween,
    /// Lines evenly distributed with equal space around each
    SpaceAround,
    /// Lines stretch to fill the container
    Stretch,
}

/// Margin around an element (supports negative values)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Margin {
    /// Top margin (can be negative)
    pub top: i16,
    /// Right margin (can be negative)
    pub right: i16,
    /// Bottom margin (can be negative)
    pub bottom: i16,
    /// Left margin (can be negative)
    pub left: i16,
}

impl Margin {
    /// Creates a new margin with equal values on all sides
    #[must_use]
    pub const fn uniform(value: i16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Creates a margin with horizontal and vertical values
    #[must_use]
    pub const fn symmetric(vertical: i16, horizontal: i16) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }

    /// Creates a margin with all four sides specified
    #[must_use]
    pub const fn new(top: i16, right: i16, bottom: i16, left: i16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Creates a zero margin
    #[must_use]
    pub const fn zero() -> Self {
        Self::uniform(0)
    }

    /// Returns the total horizontal margin (right + left)
    #[must_use]
    pub const fn horizontal(&self) -> i16 {
        self.left.saturating_add(self.right)
    }

    /// Returns the total vertical margin (top + bottom)
    #[must_use]
    pub const fn vertical(&self) -> i16 {
        self.top.saturating_add(self.bottom)
    }
}

/// A child element in a flex layout with advanced flexbox properties
#[derive(Clone, Debug, PartialEq)]
pub struct FlexChild {
    /// Size constraint for this child
    pub constraint: Constraint,
    /// Flex grow factor (how much to grow relative to siblings)
    pub flex_grow: u32,
    /// Flex shrink factor (how much to shrink when space is limited)
    pub flex_shrink: u32,
    /// Base size before flex adjustments (acts like CSS flex-basis)
    pub flex_basis: Option<u16>,
    /// Order for visual reordering (lower values appear first)
    pub order: i32,
    /// Margin around this child
    pub margin: Margin,
}

impl Default for FlexChild {
    fn default() -> Self {
        Self {
            constraint: Constraint::Fill,
            flex_grow: 0,
            flex_shrink: 1,
            flex_basis: None,
            order: 0,
            margin: Margin::zero(),
        }
    }
}

impl FlexChild {
    /// Creates a new flex child with the given constraint
    #[must_use]
    pub fn new(constraint: Constraint) -> Self {
        Self {
            constraint,
            ..Self::default()
        }
    }

    /// Creates a flex child that fills available space
    #[must_use]
    pub fn fill() -> Self {
        Self::new(Constraint::Fill)
    }

    /// Creates a flex child with a fixed size
    #[must_use]
    pub fn fixed(size: u16) -> Self {
        Self::new(Constraint::Length(size))
    }

    /// Sets the flex grow factor
    #[must_use]
    pub const fn grow(mut self, factor: u32) -> Self {
        self.flex_grow = factor;
        self
    }

    /// Sets the flex shrink factor
    #[must_use]
    pub const fn shrink(mut self, factor: u32) -> Self {
        self.flex_shrink = factor;
        self
    }

    /// Sets the flex basis
    #[must_use]
    pub const fn basis(mut self, size: u16) -> Self {
        self.flex_basis = Some(size);
        self
    }

    /// Sets the order for visual reordering
    #[must_use]
    pub const fn order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Sets the margin
    #[must_use]
    pub const fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    /// Creates from a constraint
    #[must_use]
    pub fn from_constraint(constraint: Constraint) -> Self {
        Self::new(constraint)
    }
}

impl From<Constraint> for FlexChild {
    fn from(constraint: Constraint) -> Self {
        Self::new(constraint)
    }
}

/// A flexible layout container with configurable direction, alignment, and spacing
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlexLayout {
    /// Direction of the layout (row or column)
    pub direction: FlexDirection,
    /// How to distribute items on the main axis
    pub justify_content: JustifyContent,
    /// How to align items on the cross axis
    pub align_items: AlignItems,
    /// Gap between children in cells
    pub gap: u16,
    /// Whether items wrap to multiple lines when they exceed container size
    pub flex_wrap: bool,
    /// How to align multiple lines when wrapping is enabled
    pub align_content: AlignContent,
}

impl FlexLayout {
    /// Creates a new flex layout with default settings
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the direction of the layout
    #[must_use]
    pub const fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets how items are distributed on the main axis
    #[must_use]
    pub const fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    /// Sets how items are aligned on the cross axis
    #[must_use]
    pub const fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    /// Sets the gap between children
    #[must_use]
    pub const fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Enables or disables flex wrapping
    #[must_use]
    pub const fn wrap(mut self, wrap: bool) -> Self {
        self.flex_wrap = wrap;
        self
    }

    /// Sets how multiple lines are aligned when wrapping is enabled
    #[must_use]
    pub const fn align_content(mut self, align: AlignContent) -> Self {
        self.align_content = align;
        self
    }

    /// Creates a Taffy-backed layout engine (requires `taffy-layout` feature)
    ///
    /// This method returns a `TaffyLayoutEngine` configured with the same
    /// settings as this FlexLayout. Taffy provides a battle-tested flexbox
    /// implementation that may differ slightly from cTUI's native algorithm.
    #[cfg(feature = "taffy-layout")]
    #[must_use]
    pub fn with_taffy(self) -> crate::TaffyLayoutEngine {
        crate::TaffyLayoutEngine::new()
            .direction(self.direction)
            .justify_content(self.justify_content)
            .align_items(self.align_items)
            .gap(self.gap)
            .wrap(self.flex_wrap)
            .align_content(self.align_content)
    }

    /// Splits the given area into rectangles using the Taffy engine
    #[cfg(feature = "taffy-layout")]
    #[must_use]
    pub fn split_with_taffy(&self, area: Rect, constraints: &[Constraint]) -> Vec<Rect> {
        (*self).with_taffy().split(area, constraints)
    }

    /// Splits the given area into rectangles based on constraints
    ///
    /// # Arguments
    /// * `area` - The rectangular area to split
    /// * `constraints` - Size constraints for each child
    ///
    /// # Returns
    /// A vector of Rects, one for each constraint
    #[must_use]
    pub fn split(&self, area: Rect, constraints: &[Constraint]) -> Vec<Rect> {
        if constraints.is_empty() {
            return Vec::new();
        }

        let total_gap = self.gap as u32 * constraints.len().saturating_sub(1) as u32;

        let (available_main, available_cross) = match self.direction {
            FlexDirection::Row => (area.width as u32, area.height as u32),
            FlexDirection::Column => (area.height as u32, area.width as u32),
        };

        let available_for_content = available_main.saturating_sub(total_gap);
        let mut sizes = self.calculate_sizes(constraints, available_for_content);
        Self::clamp_to_available(&mut sizes, available_for_content);
        let positions = self.calculate_positions(&sizes, available_main);

        let cross_alignment = self.calculate_cross_alignment(available_cross);

        constraints
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let main_pos = positions[i];
                let main_size = sizes[i];
                self.make_rect(area, main_pos, main_size, &cross_alignment, available_cross)
            })
            .collect()
    }

    /// Splits the given area into rectangles based on FlexChild objects
    ///
    /// This method supports advanced flexbox features like ordering,
    /// flex grow/shrink, flex basis, and margins.
    #[must_use]
    pub fn split_with_children(&self, area: Rect, children: &[FlexChild]) -> Vec<Rect> {
        if children.is_empty() {
            return Vec::new();
        }

        let mut indexed_children: Vec<(usize, &FlexChild)> = children.iter().enumerate().collect();
        indexed_children.sort_by_key(|(_, child)| child.order);

        let sorted_indices: Vec<usize> = indexed_children.iter().map(|(i, _)| *i).collect();

        let constraints: Vec<Constraint> = indexed_children
            .iter()
            .map(|(_, child)| child.constraint)
            .collect();

        let rects = self.split(area, &constraints);

        let mut result = vec![Rect::default(); children.len()];
        for (sorted_idx, &original_idx) in sorted_indices.iter().enumerate() {
            let rect = rects[sorted_idx];
            let child = &children[original_idx];
            result[original_idx] = Self::apply_margin(rect, &child.margin, area);
        }
        result
    }

    /// Splits with wrapping support, returning a Vec<Vec<Rect>> where each inner Vec is a line
    #[must_use]
    pub fn split_wrapped(&self, area: Rect, constraints: &[Constraint]) -> Vec<Vec<Rect>> {
        if !self.flex_wrap || constraints.is_empty() {
            let rects = self.split(area, constraints);
            if rects.is_empty() {
                return Vec::new();
            }
            return vec![rects];
        }

        let available_main = match self.direction {
            FlexDirection::Row => area.width,
            FlexDirection::Column => area.height,
        };

        let mut lines: Vec<Vec<Rect>> = Vec::new();
        let mut current_line_constraints: Vec<Constraint> = Vec::new();
        let mut current_line_size: u32 = 0;
        let gap = self.gap as u32;

        for constraint in constraints {
            let item_size = constraint.apply(available_main) as u32;
            let needed = if current_line_constraints.is_empty() {
                item_size
            } else {
                gap + item_size
            };

            if current_line_size + needed > available_main as u32
                && !current_line_constraints.is_empty()
            {
                let line_rects =
                    self.split_single_line(area, &current_line_constraints, lines.len());
                lines.push(line_rects);
                current_line_constraints.clear();
                current_line_size = 0;
            }

            current_line_constraints.push(*constraint);
            current_line_size += needed;
        }

        if !current_line_constraints.is_empty() {
            let line_rects = self.split_single_line(area, &current_line_constraints, lines.len());
            lines.push(line_rects);
        }

        self.apply_wrap_alignment(area, &mut lines);

        lines
    }

    fn split_single_line(
        &self,
        area: Rect,
        constraints: &[Constraint],
        line_index: usize,
    ) -> Vec<Rect> {
        let line_height = match self.direction {
            FlexDirection::Row => area.height,
            FlexDirection::Column => area.width,
        } / (self.estimate_line_count(area, constraints.len()) as u16).max(1);

        let line_area = match self.direction {
            FlexDirection::Row => Rect::new(
                area.x,
                area.y + (line_index as u16 * line_height),
                area.width,
                line_height,
            ),
            FlexDirection::Column => Rect::new(
                area.x + (line_index as u16 * line_height),
                area.y,
                line_height,
                area.height,
            ),
        };

        self.split(line_area, constraints)
    }

    fn estimate_line_count(&self, area: Rect, item_count: usize) -> usize {
        if !self.flex_wrap {
            return 1;
        }

        match self.direction {
            FlexDirection::Row => {
                let avg_width = 20u16;
                let items_per_line =
                    ((area.width as usize) / (avg_width as usize + self.gap as usize)).max(1);
                (item_count + items_per_line - 1) / items_per_line
            }
            FlexDirection::Column => {
                let avg_height = 5u16;
                let items_per_line =
                    ((area.height as usize) / (avg_height as usize + self.gap as usize)).max(1);
                (item_count + items_per_line - 1) / items_per_line
            }
        }
    }

    fn apply_wrap_alignment(&self, area: Rect, lines: &mut [Vec<Rect>]) {
        if lines.is_empty() {
            return;
        }

        let available_cross = match self.direction {
            FlexDirection::Row => area.height as u32,
            FlexDirection::Column => area.width as u32,
        };

        let total_content: u32 = lines
            .iter()
            .map(|line| match self.direction {
                FlexDirection::Row => line.first().map(|r| r.height as u32).unwrap_or(0),
                FlexDirection::Column => line.first().map(|r| r.width as u32).unwrap_or(0),
            })
            .sum();

        let offset = match self.align_content {
            AlignContent::Start => 0,
            AlignContent::Center => (available_cross.saturating_sub(total_content) / 2) as u16,
            AlignContent::End => available_cross.saturating_sub(total_content) as u16,
            AlignContent::SpaceBetween | AlignContent::SpaceAround | AlignContent::Stretch => 0,
        };

        let mut current_offset = offset;
        for line in lines.iter_mut() {
            for rect in line.iter_mut() {
                match self.direction {
                    FlexDirection::Row => {
                        rect.y = area.y + current_offset;
                    }
                    FlexDirection::Column => {
                        rect.x = area.x + current_offset;
                    }
                }
            }
            if let Some(first) = line.first() {
                current_offset += match self.direction {
                    FlexDirection::Row => first.height,
                    FlexDirection::Column => first.width,
                };
            }
        }
    }

    fn apply_margin(rect: Rect, margin: &Margin, _container: Rect) -> Rect {
        let x_offset = if margin.left >= 0 {
            margin.left as u16
        } else {
            0
        };

        let y_offset = if margin.top >= 0 {
            margin.top as u16
        } else {
            0
        };

        let width_adjustment = margin.left.abs() as i32 + margin.right.abs() as i32;
        let height_adjustment = margin.top.abs() as i32 + margin.bottom.abs() as i32;

        Rect::new(
            rect.x.saturating_add(x_offset),
            rect.y.saturating_add(y_offset),
            (rect.width as i32).saturating_sub(width_adjustment).max(1) as u16,
            (rect.height as i32)
                .saturating_sub(height_adjustment)
                .max(1) as u16,
        )
    }

    fn make_rect(
        &self,
        area: Rect,
        main_pos: u16,
        main_size: u16,
        cross_alignment: &CrossAlignment,
        available_cross: u32,
    ) -> Rect {
        let cross_size = cross_alignment.size(available_cross) as u16;
        let cross_offset = cross_alignment.offset(cross_size as u32, available_cross);

        match self.direction {
            FlexDirection::Row => {
                let x = area.x.saturating_add(main_pos.min(area.width));
                let width = main_size.min(area.width.saturating_sub(main_pos));
                Rect {
                    x,
                    y: area.y + cross_offset,
                    width,
                    height: cross_size,
                }
            }
            FlexDirection::Column => {
                let y = area.y.saturating_add(main_pos.min(area.height));
                let height = main_size.min(area.height.saturating_sub(main_pos));
                Rect {
                    x: area.x + cross_offset,
                    y,
                    width: cross_size,
                    height,
                }
            }
        }
    }

    fn calculate_sizes(&self, constraints: &[Constraint], available: u32) -> Vec<u16> {
        let n = constraints.len();
        let mut sizes = vec![0u16; n];

        let mut fixed_total = 0u32;
        let mut flex_count = 0u32;
        let mut ratio_total = 0u32;
        let mut portion_total = 0u32;

        for (i, constraint) in constraints.iter().enumerate() {
            match constraint {
                Constraint::Length(n) => {
                    sizes[i] = *n;
                    fixed_total += *n as u32;
                }
                Constraint::Min(n) => {
                    sizes[i] = *n;
                    fixed_total += *n as u32;
                    flex_count += 1;
                }
                Constraint::Max(n) => {
                    sizes[i] = *n;
                    fixed_total += *n as u32;
                    flex_count += 1;
                }
                Constraint::Percentage(p) => {
                    let size = ((available as u64 * *p as u64 / 100) as u16).max(0);
                    sizes[i] = size;
                    fixed_total += size as u32;
                }
                Constraint::Ratio(num, _den) => {
                    ratio_total += *num;
                    flex_count += 1;
                }
                Constraint::Fill => {
                    flex_count += 1;
                }
                Constraint::Range { min, max: _ } => {
                    sizes[i] = *min;
                    fixed_total += *min as u32;
                    flex_count += 1;
                }
                Constraint::Portion(p) => {
                    portion_total += *p;
                    flex_count += 1;
                }
            }
        }

        let remaining = available.saturating_sub(fixed_total);
        let flex_items: Vec<_> = constraints
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                matches!(
                    c,
                    Constraint::Min(_)
                        | Constraint::Max(_)
                        | Constraint::Ratio(_, _)
                        | Constraint::Fill
                        | Constraint::Range { .. }
                        | Constraint::Portion(_)
                )
            })
            .collect();

        if !flex_items.is_empty() && remaining > 0 {
            let effective_flex_count = if portion_total > 0 {
                portion_total
            } else {
                flex_count.max(1)
            };
            let base_share = remaining / effective_flex_count.max(1);

            for (i, constraint) in flex_items {
                let add = match constraint {
                    Constraint::Max(max) => {
                        let current = sizes[i] as u32;
                        min(base_share, (*max as u32).saturating_sub(current))
                    }
                    Constraint::Ratio(num, _) if ratio_total > 0 => {
                        ((remaining as u64 * *num as u64 / ratio_total as u64) as u32)
                            .min(remaining)
                    }
                    Constraint::Range { min: _, max } => {
                        let current = sizes[i] as u32;
                        let max_allowed = *max as u32;
                        if current >= max_allowed {
                            0
                        } else {
                            base_share.min(max_allowed.saturating_sub(current))
                        }
                    }
                    Constraint::Portion(p) if portion_total > 0 => {
                        ((remaining as u64 * *p as u64 / portion_total as u64) as u32)
                            .min(remaining)
                    }
                    Constraint::Portion(_) => base_share,
                    Constraint::Fill => base_share,
                    _ => base_share,
                };
                sizes[i] = sizes[i].saturating_add(add as u16);
            }
        }

        sizes
    }

    fn clamp_to_available(sizes: &mut [u16], available: u32) {
        let total: u32 = sizes.iter().map(|&s| s as u32).sum();
        if total <= available {
            return;
        }

        let len = sizes.len();
        let scale = available as f64 / total as f64;
        let mut used = 0u32;
        for (i, size) in sizes.iter_mut().enumerate() {
            if i == len - 1 {
                *size = (available.saturating_sub(used)) as u16;
            } else {
                let scaled = (*size as f64 * scale) as u16;
                *size = scaled;
                used += scaled as u32;
            }
        }
    }

    fn calculate_positions(&self, sizes: &[u16], available: u32) -> Vec<u16> {
        let n = sizes.len();
        if n == 0 {
            return Vec::new();
        }

        let total_content_size: u32 = sizes.iter().map(|&s| s as u32).sum();
        let total_gaps = self.gap as u32 * n.saturating_sub(1) as u32;
        let total_size_with_gaps = total_content_size.saturating_add(total_gaps);

        let mut positions = vec![0u16; n];

        match self.justify_content {
            JustifyContent::Start => {
                let mut pos = 0u16;
                for (i, &size) in sizes.iter().enumerate() {
                    positions[i] = pos;
                    pos = pos.saturating_add(size).saturating_add(self.gap);
                }
            }
            JustifyContent::Center => {
                let offset = available.saturating_sub(total_size_with_gaps) / 2;
                let mut pos = offset.min(u16::MAX as u32) as u16;
                for (i, &size) in sizes.iter().enumerate() {
                    positions[i] = pos;
                    pos = pos.saturating_add(size).saturating_add(self.gap);
                }
            }
            JustifyContent::End => {
                let offset = available.saturating_sub(total_size_with_gaps);
                let mut pos = offset.min(u16::MAX as u32) as u16;
                for (i, &size) in sizes.iter().enumerate() {
                    positions[i] = pos;
                    pos = pos.saturating_add(size).saturating_add(self.gap);
                }
            }
            JustifyContent::SpaceBetween => {
                let extra_space = available.saturating_sub(total_content_size);
                let spacing = if n > 1 {
                    extra_space / (n - 1) as u32
                } else {
                    0
                };

                let mut pos = 0u16;
                for (i, &size) in sizes.iter().enumerate() {
                    positions[i] = pos;
                    pos = pos.saturating_add(size);
                    if i < n - 1 {
                        pos = pos.saturating_add(spacing.min(u16::MAX as u32) as u16);
                    }
                }
            }
            JustifyContent::SpaceAround => {
                let extra_space = available.saturating_sub(total_content_size);
                let spacing = extra_space / n.max(1) as u32;

                let mut pos = (spacing / 2).min(u16::MAX as u32) as u16;
                for (i, &size) in sizes.iter().enumerate() {
                    positions[i] = pos;
                    pos = pos
                        .saturating_add(size)
                        .saturating_add(spacing.min(u16::MAX as u32) as u16);
                }
            }
        }

        positions
    }

    fn calculate_cross_alignment(&self, _available: u32) -> CrossAlignment {
        match self.align_items {
            AlignItems::Start => CrossAlignment::Start,
            AlignItems::Center => CrossAlignment::Center,
            AlignItems::End => CrossAlignment::End,
            AlignItems::Stretch => CrossAlignment::Stretch,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum CrossAlignment {
    Start,
    Center,
    End,
    Stretch,
}

impl CrossAlignment {
    fn offset(self, size: u32, available: u32) -> u16 {
        match self {
            Self::Start | Self::Stretch => 0,
            Self::Center => available.saturating_sub(size) / 2,
            Self::End => available.saturating_sub(size),
        }
        .min(u16::MAX as u32) as u16
    }

    fn size(self, available: u32) -> u32 {
        match self {
            Self::Stretch => available,
            _ => available,
        }
    }
}

/// Entry point for creating layouts
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Layout {
    inner: FlexLayout,
}

impl Layout {
    /// Creates a new flex layout builder
    #[must_use]
    pub fn flex() -> FlexLayout {
        FlexLayout::new()
    }

    /// Creates a default layout
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a row layout (horizontal)
    #[must_use]
    pub fn row() -> FlexLayout {
        FlexLayout::new().direction(FlexDirection::Row)
    }

    /// Creates a column layout (vertical)
    #[must_use]
    pub fn column() -> FlexLayout {
        FlexLayout::new().direction(FlexDirection::Column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect::new(x, y, width, height)
    }

    #[test]
    fn flex_direction_default_row() {
        let layout = Layout::flex();
        assert_eq!(layout.direction, FlexDirection::Row);
    }

    #[test]
    fn row_layout_splits_horizontally() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(30)]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0], rect(0, 0, 20, 24));
        assert_eq!(rects[1], rect(20, 0, 30, 24));
    }

    #[test]
    fn column_layout_splits_vertically() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::column();
        let rects = layout.split(area, &[Constraint::Length(10), Constraint::Length(14)]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0], rect(0, 0, 80, 10));
        assert_eq!(rects[1], rect(0, 10, 80, 14));
    }

    #[test]
    fn row_with_gap() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().gap(2);
        let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(20)]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0], rect(0, 0, 20, 24));
        assert_eq!(rects[1], rect(22, 0, 20, 24));
    }

    #[test]
    fn justify_content_center_row() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().justify_content(JustifyContent::Center);
        let rects = layout.split(area, &[Constraint::Length(20)]);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].x, 30);
        assert_eq!(rects[0].width, 20);
    }

    #[test]
    fn justify_content_end_row() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().justify_content(JustifyContent::End);
        let rects = layout.split(area, &[Constraint::Length(20)]);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].x, 60);
    }

    #[test]
    fn justify_content_space_between() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().justify_content(JustifyContent::SpaceBetween);
        let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(20)]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].x, 0);
        assert_eq!(rects[1].x, 60);
    }

    #[test]
    fn justify_content_space_around() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().justify_content(JustifyContent::SpaceAround);
        let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(20)]);

        assert_eq!(rects.len(), 2);
        assert!(rects[0].x > 0);
        assert!(rects[1].x > rects[0].x + rects[0].width);
    }

    #[test]
    fn align_items_center_row() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().align_items(AlignItems::Center);
        let rects = layout.split(area, &[Constraint::Length(20)]);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].y, 0);
        assert_eq!(rects[0].height, 24);
    }

    #[test]
    fn align_items_start_row() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().align_items(AlignItems::Start);
        let rects = layout.split(area, &[Constraint::Length(20)]);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].y, 0);
        assert_eq!(rects[0].height, 24);
    }

    #[test]
    fn align_items_stretch_row() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().align_items(AlignItems::Stretch);
        let rects = layout.split(area, &[Constraint::Length(20)]);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].y, 0);
        assert_eq!(rects[0].height, 24);
    }

    #[test]
    fn align_items_center_row_simple() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row().align_items(AlignItems::Center);
        let rects = layout.split(area, &[Constraint::Length(20)]);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].height, 24);
    }

    #[test]
    fn align_items_end_column() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::column().align_items(AlignItems::End);
        let rects = layout.split(area, &[Constraint::Length(10)]);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].width, 80);
    }

    #[test]
    fn min_constraint_grows_to_fill() {
        let area = rect(0, 0, 60, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Min(10), Constraint::Min(10)]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].width + rects[1].width, 60);
    }

    #[test]
    fn max_constraint_limits_size() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Max(20)]);

        assert_eq!(rects.len(), 1);
        assert!(rects[0].width <= 20);
    }

    #[test]
    fn percentage_constraint() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Percentage(50)]);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].width, 50);
    }

    #[test]
    fn fill_constraint() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Length(20), Constraint::Fill]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].width, 20);
        assert!(rects[1].width > 0);
    }

    #[test]
    fn empty_constraints_returns_empty_vec() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[]);

        assert!(rects.is_empty());
    }

    #[test]
    fn single_child_fills_cross_axis_by_default() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Length(20)]);

        assert_eq!(rects[0].height, 24);
    }

    #[test]
    fn offset_origin_is_preserved() {
        let area = rect(10, 5, 80, 24);
        let layout = Layout::row().gap(2);
        let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(30)]);

        assert_eq!(rects[0].x, 10);
        assert_eq!(rects[0].y, 5);
        assert_eq!(rects[1].x, 32);
        assert_eq!(rects[1].y, 5);
    }

    #[test]
    fn ratio_constraint_distributes_proportionally() {
        let area = rect(0, 0, 90, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);

        assert_eq!(rects.len(), 2);
        assert!(rects[0].width < rects[1].width);
    }

    #[test]
    fn mixed_constraints() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();
        let rects = layout.split(
            area,
            &[
                Constraint::Length(30),
                Constraint::Percentage(20),
                Constraint::Min(10),
            ],
        );

        assert_eq!(rects.len(), 3);
        assert_eq!(rects[0].width, 30);
        assert_eq!(rects[1].width, 20);
        assert!(rects[2].width > 0);
    }

    #[test]
    fn column_justify_center() {
        let area = rect(0, 0, 80, 40);
        let layout = Layout::column().justify_content(JustifyContent::Center);
        let rects = layout.split(area, &[Constraint::Length(10), Constraint::Length(10)]);

        assert_eq!(rects.len(), 2);
        assert!(rects[0].y > 0);
    }

    #[test]
    fn column_gap() {
        let area = rect(0, 0, 80, 40);
        let layout = Layout::column().gap(4);
        let rects = layout.split(area, &[Constraint::Length(10), Constraint::Length(10)]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[1].y, rects[0].y + rects[0].height + 4);
    }

    #[test]
    fn margin_uniform() {
        let m = Margin::uniform(5);
        assert_eq!(m.top, 5);
        assert_eq!(m.right, 5);
        assert_eq!(m.bottom, 5);
        assert_eq!(m.left, 5);
    }

    #[test]
    fn margin_symmetric() {
        let m = Margin::symmetric(10, 20);
        assert_eq!(m.top, 10);
        assert_eq!(m.bottom, 10);
        assert_eq!(m.left, 20);
        assert_eq!(m.right, 20);
    }

    #[test]
    fn margin_horizontal_vertical() {
        let m = Margin::new(1, 2, 3, 4);
        assert_eq!(m.horizontal(), 6);
        assert_eq!(m.vertical(), 4);
    }

    #[test]
    fn flex_child_new() {
        let child = FlexChild::new(Constraint::Length(20));
        assert_eq!(child.constraint, Constraint::Length(20));
        assert_eq!(child.flex_grow, 0);
        assert_eq!(child.flex_shrink, 1);
        assert_eq!(child.order, 0);
    }

    #[test]
    fn flex_child_builder() {
        let child = FlexChild::fill()
            .grow(2)
            .shrink(0)
            .order(-1)
            .basis(50)
            .margin(Margin::uniform(5));

        assert_eq!(child.flex_grow, 2);
        assert_eq!(child.flex_shrink, 0);
        assert_eq!(child.order, -1);
        assert_eq!(child.flex_basis, Some(50));
        assert_eq!(child.margin, Margin::uniform(5));
    }

    #[test]
    fn split_with_children_orders_correctly() {
        let area = rect(0, 0, 60, 24);
        let layout = Layout::row();

        let children = vec![
            FlexChild::fixed(20).order(2),
            FlexChild::fixed(20).order(0),
            FlexChild::fixed(20).order(1),
        ];

        let rects = layout.split_with_children(area, &children);
        assert_eq!(rects.len(), 3);
    }

    #[test]
    fn align_content_variants() {
        assert_eq!(AlignContent::default(), AlignContent::Start);
        assert_ne!(AlignContent::Start, AlignContent::Center);
    }

    #[test]
    fn flex_layout_wrap() {
        let layout = Layout::row().wrap(true).align_content(AlignContent::Center);
        assert!(layout.flex_wrap);
        assert_eq!(layout.align_content, AlignContent::Center);
    }

    #[test]
    fn split_wrapped_returns_lines() {
        let area = rect(0, 0, 20, 24);
        let layout = Layout::row().wrap(true);

        let constraints = vec![
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        let lines = layout.split_wrapped(area, &constraints);
        assert!(!lines.is_empty());
    }

    #[test]
    fn portion_constraint_in_layout() {
        let area = rect(0, 0, 90, 24);
        let layout = Layout::row();

        let rects = layout.split(area, &[Constraint::Portion(1), Constraint::Portion(2)]);
        assert_eq!(rects.len(), 2);
    }

    #[test]
    fn range_constraint_in_layout() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();

        let rects = layout.split(
            area,
            &[Constraint::Range { min: 10, max: 30 }, Constraint::Fill],
        );
        assert_eq!(rects.len(), 2);
        assert!(rects[0].width >= 10);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::collection::vec;
    use proptest::prelude::*;

    fn arbitrary_constraint() -> impl Strategy<Value = Constraint> {
        prop_oneof![
            (0u16..100).prop_map(Constraint::Length),
            (0u16..100).prop_map(Constraint::Min),
            (0u16..100).prop_map(Constraint::Max),
            (0u16..100).prop_map(Constraint::Percentage),
            ((1u32..10), (1u32..10)).prop_map(|(n, d)| Constraint::Ratio(n, d)),
            Just(Constraint::Fill),
            ((0u16..50), (0u16..100)).prop_map(|(min, max)| Constraint::Range {
                min,
                max: min.max(max)
            }),
            (1u32..10).prop_map(Constraint::Portion),
        ]
    }

    fn arbitrary_direction() -> impl Strategy<Value = FlexDirection> {
        prop_oneof![Just(FlexDirection::Row), Just(FlexDirection::Column),]
    }

    fn arbitrary_justify() -> impl Strategy<Value = JustifyContent> {
        prop_oneof![
            Just(JustifyContent::Start),
            Just(JustifyContent::Center),
            Just(JustifyContent::End),
            Just(JustifyContent::SpaceBetween),
            Just(JustifyContent::SpaceAround),
        ]
    }

    fn arbitrary_align() -> impl Strategy<Value = AlignItems> {
        prop_oneof![
            Just(AlignItems::Start),
            Just(AlignItems::Center),
            Just(AlignItems::End),
            Just(AlignItems::Stretch),
        ]
    }

    proptest! {
        #[test]
        fn layout_never_panics(
            x in 0u16..100u16,
            y in 0u16..100u16,
            width in 1u16..200u16,
            height in 1u16..200u16,
            child_count in 0usize..20usize,
            gap in 0u16..20u16,
            direction in arbitrary_direction(),
            justify in arbitrary_justify(),
            align in arbitrary_align(),
            constraints in vec(arbitrary_constraint(), 0..20),
        ) {
            let area = Rect::new(x, y, width, height);
            let layout = FlexLayout {
                direction,
                justify_content: justify,
                align_items: align,
                gap,
                flex_wrap: false,
                align_content: AlignContent::default(),
            };

            let constraints_to_use: Vec<Constraint> = if constraints.is_empty() && child_count > 0 {
                (0..child_count.min(20)).map(|_| Constraint::Fill).collect()
            } else if constraints.len() > child_count && child_count > 0 {
                constraints.into_iter().take(child_count).collect()
            } else {
                constraints
            };

            let result = std::panic::catch_unwind(|| layout.split(area, &constraints_to_use));
            prop_assert!(result.is_ok(), "Layout panicked with inputs: area={:?}, gap={}", area, gap);
        }

        #[test]
        fn children_fit_within_container(
            x in 0u16..50u16,
            y in 0u16..50u16,
            width in 1u16..200u16,
            height in 1u16..200u16,
            child_count in 1usize..20usize,
            gap in 0u16..20u16,
            direction in arbitrary_direction(),
            constraints in vec(arbitrary_constraint(), 1..20),
        ) {
            let area = Rect::new(x, y, width, height);
            let layout = FlexLayout {
                direction,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                gap,
                flex_wrap: false,
                align_content: AlignContent::default(),
            };

            let constraints_to_use: Vec<Constraint> =
                constraints.into_iter().take(child_count).collect();
            let rects = layout.split(area, &constraints_to_use);

            for (i, rect) in rects.iter().enumerate() {
                prop_assert!(rect.x >= area.x, "Rect {} x ({}) < container x ({})", i, rect.x, area.x);
                prop_assert!(rect.y >= area.y, "Rect {} y ({}) < container y ({})", i, rect.y, area.y);
                prop_assert!(rect.x <= area.x + area.width, "Rect {} x ({}) exceeds container boundary", i, rect.x);
                prop_assert!(rect.y <= area.y + area.height, "Rect {} y ({}) exceeds container boundary", i, rect.y);
            }
        }

        #[test]
        fn row_no_horizontal_overlaps(
            x in 0u16..50u16,
            y in 0u16..50u16,
            width in 10u16..200u16,
            height in 1u16..200u16,
            child_count in 2usize..15usize,
            gap in 0u16..15u16,
            constraints in vec(arbitrary_constraint(), 2..15),
        ) {
            let area = Rect::new(x, y, width, height);
            let layout = Layout::row().gap(gap);
            let constraints_to_use: Vec<Constraint> = constraints.into_iter().take(child_count).collect();
            let rects = layout.split(area, &constraints_to_use);

            for i in 0..rects.len().saturating_sub(1) {
                let end_i = rects[i].x.saturating_add(rects[i].width);
                let start_next = rects[i + 1].x;
                prop_assert!(
                    end_i <= start_next || start_next.saturating_sub(end_i) <= gap,
                    "Rects {} and {} overlap: end={} start_next={} gap={}", i, i + 1, end_i, start_next, gap
                );
            }
        }

        #[test]
        fn column_no_vertical_overlaps(
            x in 0u16..50u16,
            y in 0u16..50u16,
            width in 1u16..200u16,
            height in 10u16..200u16,
            child_count in 2usize..15usize,
            gap in 0u16..15u16,
            constraints in vec(arbitrary_constraint(), 2..15),
        ) {
            let area = Rect::new(x, y, width, height);
            let layout = Layout::column().gap(gap);
            let constraints_to_use: Vec<Constraint> = constraints.into_iter().take(child_count).collect();
            let rects = layout.split(area, &constraints_to_use);

            for i in 0..rects.len().saturating_sub(1) {
                let end_i = rects[i].y.saturating_add(rects[i].height);
                let start_next = rects[i + 1].y;
                prop_assert!(
                    end_i <= start_next || start_next.saturating_sub(end_i) <= gap,
                    "Rects {} and {} overlap: end={} start_next={} gap={}", i, i + 1, end_i, start_next, gap
                );
            }
        }

        #[test]
        fn row_total_width_within_bounds(
            x in 0u16..50u16,
            y in 0u16..50u16,
            width in 10u16..200u16,
            height in 1u16..200u16,
            child_count in 1usize..10usize,
            gap in 0u16..5u16,
            constraints in vec(arbitrary_constraint(), 1..10),
        ) {
            let area = Rect::new(x, y, width, height);
            let layout = Layout::row().gap(gap);
            let constraints_to_use: Vec<Constraint> = constraints.into_iter().take(child_count).collect();
            let rects = layout.split(area, &constraints_to_use);

            let total_width: u32 = rects.iter().map(|r| r.width as u32).sum();
            prop_assert!(
                total_width <= width as u32,
                "Total width {} exceeds container {}", total_width, width
            );
        }

        #[test]
        fn column_total_height_within_bounds(
            x in 0u16..50u16,
            y in 0u16..50u16,
            width in 1u16..200u16,
            height in 10u16..200u16,
            child_count in 1usize..10usize,
            gap in 0u16..5u16,
            constraints in vec(arbitrary_constraint(), 1..10),
        ) {
            let area = Rect::new(x, y, width, height);
            let layout = Layout::column().gap(gap);
            let constraints_to_use: Vec<Constraint> = constraints.into_iter().take(child_count).collect();
            let rects = layout.split(area, &constraints_to_use);

            let total_height: u32 = rects.iter().map(|r| r.height as u32).sum();
            prop_assert!(
                total_height <= height as u32,
                "Total height {} exceeds container {}", total_height, height
            );
        }

        #[test]
        fn output_count_matches_constraints(
            width in 1u16..200u16,
            height in 1u16..200u16,
            child_count in 0usize..20usize,
            direction in arbitrary_direction(),
        ) {
            let area = Rect::new(0, 0, width, height);
            let layout = FlexLayout {
                direction,
                justify_content: JustifyContent::default(),
                align_items: AlignItems::default(),
                gap: 0,
                flex_wrap: false,
                align_content: AlignContent::default(),
            };
            let constraints: Vec<Constraint> = (0..child_count).map(|_| Constraint::Fill).collect();
            let rects = layout.split(area, &constraints);
            prop_assert_eq!(rects.len(), child_count);
        }

        #[test]
        fn handles_single_pixel_container(
            x in 0u16..10u16,
            y in 0u16..10u16,
            child_count in 1usize..10usize,
            gap in 0u16..5u16,
        ) {
            let area = Rect::new(x, y, 1, 1);
            let layout = Layout::row().gap(gap);
            let constraints: Vec<Constraint> = (0..child_count).map(|_| Constraint::Fill).collect();
            let rects = layout.split(area, &constraints);
            prop_assert_eq!(rects.len(), child_count);
            for rect in &rects {
                prop_assert!(rect.width <= 1);
            }
        }

        #[test]
        fn high_percentage_constraints(
            width in 10u16..200u16,
            height in 10u16..200u16,
            percentage in 100u16..500u16,
            child_count in 1usize..10usize,
        ) {
            let area = Rect::new(0, 0, width, height);
            let layout = Layout::row();
            let constraints: Vec<Constraint> = (0..child_count).map(|_| Constraint::Percentage(percentage)).collect();
            let result = std::panic::catch_unwind(|| layout.split(area, &constraints));
            prop_assert!(result.is_ok());
            if let Ok(rects) = result {
                for rect in &rects {
                    prop_assert!(rect.width <= width);
                }
            }
        }

        #[test]
        fn handles_zero_length_constraints(
            width in 10u16..200u16,
            height in 1u16..100u16,
            child_count in 1usize..10usize,
        ) {
            let area = Rect::new(0, 0, width, height);
            let layout = Layout::row();
            let constraints: Vec<Constraint> = (0..child_count)
                .map(|i| if i == 0 { Constraint::Length(0) } else { Constraint::Length(10) })
                .collect();
            let rects = layout.split(area, &constraints);
            prop_assert_eq!(rects.len(), constraints.len());
            for rect in &rects {
                prop_assert!(rect.width <= width);
            }
        }
    }
}

#[cfg(test)]
mod error_boundary_tests {
    use super::*;

    fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect::new(x, y, width, height)
    }

    // === Zero-sized area tests ===

    #[test]
    fn zero_width_area_returns_valid_rects() {
        let area = rect(0, 0, 0, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Length(10), Constraint::Fill]);
        assert_eq!(rects.len(), 2);
        for r in &rects {
            assert!(r.width == 0 || r.width <= area.width);
        }
    }

    #[test]
    fn zero_height_area_returns_valid_rects() {
        let area = rect(0, 0, 80, 0);
        let layout = Layout::column();
        let rects = layout.split(area, &[Constraint::Length(10), Constraint::Fill]);
        assert_eq!(rects.len(), 2);
        for r in &rects {
            assert!(r.height == 0 || r.height <= area.height);
        }
    }

    #[test]
    fn zero_area_both_dimensions() {
        let area = rect(0, 0, 0, 0);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Length(10), Constraint::Fill]);
        assert_eq!(rects.len(), 2);
    }

    #[test]
    fn single_pixel_area() {
        let area = rect(0, 0, 1, 1);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Length(10), Constraint::Length(10)]);
        assert_eq!(rects.len(), 2);
        for r in &rects {
            assert!(r.width <= 1);
        }
    }

    // === Gap overflow tests ===

    #[test]
    fn gap_larger_than_area() {
        let area = rect(0, 0, 5, 24);
        let layout = Layout::row().gap(100);
        let rects = layout.split(area, &[Constraint::Length(10), Constraint::Fill]);
        assert_eq!(rects.len(), 2);
    }

    #[test]
    fn max_u16_gap_no_overflow() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row().gap(u16::MAX);
        let rects = layout.split(area, &[Constraint::Fill]);
        assert_eq!(rects.len(), 1);
    }

    // === Many constraints tests ===

    #[test]
    fn many_constraints_with_tiny_area() {
        let area = rect(0, 0, 10, 1);
        let layout = Layout::row();
        let constraints: Vec<Constraint> = (0..100).map(|_| Constraint::Fill).collect();
        let rects = layout.split(area, &constraints);
        assert_eq!(rects.len(), 100);
        for r in &rects {
            assert!(r.width <= area.width);
        }
    }

    // === Constraint edge cases ===

    #[test]
    fn ratio_zero_denominator_handled_gracefully() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Ratio(1, 0)]);
        assert_eq!(rects.len(), 1);
    }

    #[test]
    fn portion_zero_handled_gracefully() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Portion(0), Constraint::Portion(2)]);
        assert_eq!(rects.len(), 2);
    }

    #[test]
    fn percentage_over_100_clamped() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Percentage(200)]);
        assert_eq!(rects.len(), 1);
        assert!(rects[0].width <= 100);
    }

    #[test]
    fn range_min_greater_than_max() {
        let area = rect(0, 0, 100, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Range { min: 50, max: 10 }]);
        assert_eq!(rects.len(), 1);
    }

    // === Offset area tests ===

    #[test]
    fn large_offset_area() {
        let area = rect(1000, 500, 80, 24);
        let layout = Layout::row();
        let rects = layout.split(area, &[Constraint::Length(20), Constraint::Fill]);
        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].x, 1000);
    }

    // === Justify with overflow ===

    #[test]
    fn justify_center_with_overflow_content() {
        let area = rect(0, 0, 10, 24);
        let layout = Layout::row().justify_content(JustifyContent::Center);
        let rects = layout.split(area, &[Constraint::Length(20), Constraint::Length(20)]);
        assert_eq!(rects.len(), 2);
    }

    // === Flex wrap edge cases ===

    #[test]
    fn flex_wrap_with_single_item() {
        let area = rect(0, 0, 20, 24);
        let layout = Layout::row().wrap(true);
        let lines = layout.split_wrapped(area, &[Constraint::Length(10)]);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].len(), 1);
    }

    #[test]
    fn flex_wrap_zero_area() {
        let area = rect(0, 0, 0, 0);
        let layout = Layout::row().wrap(true);
        let lines = layout.split_wrapped(area, &[Constraint::Length(10), Constraint::Length(20)]);
        assert!(!lines.is_empty() || lines.is_empty());
    }

    // === Split with children edge cases ===

    #[test]
    fn split_with_children_empty() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row();
        let rects = layout.split_with_children(area, &[]);
        assert!(rects.is_empty());
    }

    #[test]
    fn negative_margin_handling() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row();
        let children = vec![
            FlexChild::fixed(20).margin(Margin::new(-5, -5, -5, -5)),
        ];
        let rects = layout.split_with_children(area, &children);
        assert_eq!(rects.len(), 1);
    }

    #[test]
    fn extreme_margin_values() {
        let area = rect(0, 0, 80, 24);
        let layout = Layout::row();
        let children = vec![
            FlexChild::fixed(20).margin(Margin::uniform(i16::MAX)),
        ];
        let rects = layout.split_with_children(area, &children);
        assert_eq!(rects.len(), 1);
    }

    // === Stress test ===

    #[test]
    fn layout_stress_test() {
        let area = rect(0, 0, 1, 1);
        let layout = Layout::row()
            .gap(u16::MAX)
            .justify_content(JustifyContent::SpaceBetween);

        let constraints = vec![
            Constraint::Length(0),
            Constraint::Min(0),
            Constraint::Max(0),
            Constraint::Percentage(0),
            Constraint::Ratio(0, 1),
            Constraint::Fill,
            Constraint::Range { min: 0, max: 0 },
            Constraint::Portion(0),
        ];

        let rects = layout.split(area, &constraints);
        assert_eq!(rects.len(), 8);
    }
}
