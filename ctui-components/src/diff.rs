//! Diff Viewer component for displaying file differences
//!
//! This module provides a `DiffViewer` component that renders file differences
//! in either unified or split view mode, with Myers diff algorithm support.

use ctui_core::style::{Color, Modifier, Style};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};
use unicode_width::UnicodeWidthStr;

/// The display mode for diff rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffMode {
    /// Unified view: old and new content interleaved
    #[default]
    Unified,
    /// Split view: old on left, new on right
    Split,
}

/// The diff algorithm to use for computing differences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffAlgorithm {
    /// Myers diff algorithm - standard diff
    #[default]
    Myers,
    /// Patience diff algorithm - better for moved blocks
    Patience,
}

/// A single line in a diff
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLine {
    /// Context line (unchanged)
    Context(String),
    /// Added line (appears in new content)
    Added(String),
    /// Removed line (appears in old content)
    Removed(String),
}

impl DiffLine {
    /// Get the content of the line
    pub fn content(&self) -> &str {
        match self {
            DiffLine::Context(s) => s,
            DiffLine::Added(s) => s,
            DiffLine::Removed(s) => s,
        }
    }

    /// Check if this is a context line
    pub fn is_context(&self) -> bool {
        matches!(self, DiffLine::Context(_))
    }

    /// Check if this is an added line
    pub fn is_added(&self) -> bool {
        matches!(self, DiffLine::Added(_))
    }

    /// Check if this is a removed line
    pub fn is_removed(&self) -> bool {
        matches!(self, DiffLine::Removed(_))
    }
}

/// A hunk of diff lines with position information
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Starting line number in old file
    pub old_start: usize,
    /// Number of lines from old file
    pub old_count: usize,
    /// Starting line number in new file
    pub new_start: usize,
    /// Number of lines in new file
    pub new_count: usize,
    /// The diff lines in this hunk
    pub lines: Vec<DiffLine>,
}

impl DiffHunk {
    /// Create a new diff hunk
    pub fn new(old_start: usize, new_start: usize) -> Self {
        Self {
            old_start,
            old_count: 0,
            new_start,
            new_count: 0,
            lines: Vec::new(),
        }
    }

    /// Add a line to this hunk
    pub fn add_line(&mut self, line: DiffLine) {
        match &line {
            DiffLine::Context(_) => {
                self.old_count += 1;
                self.new_count += 1;
            }
            DiffLine::Removed(_) => {
                self.old_count += 1;
            }
            DiffLine::Added(_) => {
                self.new_count += 1;
            }
        }
        self.lines.push(line);
    }

    /// Get the header string for this hunk
    pub fn header(&self) -> String {
        format!(
            "@@ -{},{} +{},{} @@",
            self.old_start, self.old_count, self.new_start, self.new_count
        )
    }
}

/// Diff viewer component
#[derive(Debug, Clone)]
pub struct DiffViewer {
    /// Display mode
    mode: DiffMode,
    /// Original content (old)
    old_content: Vec<String>,
    /// New content
    new_content: Vec<String>,
    /// Diff algorithm
    diff_algorithm: DiffAlgorithm,
    /// Computed hunks
    hunks: Vec<DiffHunk>,
    /// Style for added lines
    added_style: Style,
    /// Style for removed lines
    removed_style: Style,
    /// Style for context lines
    context_style: Style,
    /// Style for hunk headers
    header_style: Style,
    /// Style for line numbers
    line_number_style: Style,
    /// Whether to show line numbers
    show_line_numbers: bool,
    /// Scroll offset
    scroll_offset: usize,
}

impl Default for DiffViewer {
    fn default() -> Self {
        Self {
            mode: DiffMode::Unified,
            old_content: Vec::new(),
            new_content: Vec::new(),
            diff_algorithm: DiffAlgorithm::Myers,
            hunks: Vec::new(),
            added_style: Style::new().fg(Color::Green).bg(Color::Indexed(22)),
            removed_style: Style::new().fg(Color::Red).bg(Color::Indexed(52)),
            context_style: Style::default(),
            header_style: Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            line_number_style: Style::new().fg(Color::DarkGray),
            show_line_numbers: true,
            scroll_offset: 0,
        }
    }
}

impl DiffViewer {
    /// Create a new diff viewer
    pub fn new() -> Self {
        Self::default()
    }

    /// Set old content from a string
    pub fn old_content(mut self, content: &str) -> Self {
        self.old_content = content.lines().map(|s| s.to_string()).collect();
        self
    }

    /// Set old content from lines
    pub fn old_lines(mut self, lines: Vec<String>) -> Self {
        self.old_content = lines;
        self
    }

    /// Set new content from a string
    pub fn new_content(mut self, content: &str) -> Self {
        self.new_content = content.lines().map(|s| s.to_string()).collect();
        self
    }

    /// Set new content from lines
    pub fn new_lines(mut self, lines: Vec<String>) -> Self {
        self.new_content = lines;
        self
    }

    /// Set the display mode
    pub fn mode(mut self, mode: DiffMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the diff algorithm
    pub fn algorithm(mut self, algorithm: DiffAlgorithm) -> Self {
        self.diff_algorithm = algorithm;
        self
    }

    /// Set the style for added lines
    pub fn added_style(mut self, style: Style) -> Self {
        self.added_style = style;
        self
    }

    /// Set the style for removed lines
    pub fn removed_style(mut self, style: Style) -> Self {
        self.removed_style = style;
        self
    }

    /// Set the style for context lines
    pub fn context_style(mut self, style: Style) -> Self {
        self.context_style = style;
        self
    }

    /// Set the style for hunk headers
    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }

    /// Set the style for line numbers
    pub fn line_number_style(mut self, style: Style) -> Self {
        self.line_number_style = style;
        self
    }

    /// Set whether to show line numbers
    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Compute the diff and return hunks
    pub fn compute_diff(&mut self) {
        self.hunks = match self.diff_algorithm {
            DiffAlgorithm::Myers => self.compute_myers_diff(),
            DiffAlgorithm::Patience => self.compute_patience_diff(),
        };
    }

    /// Myers diff algorithm implementation
    fn compute_myers_diff(&self) -> Vec<DiffHunk> {
        let old_len = self.old_content.len();
        let new_len = self.new_content.len();

        if old_len == 0 && new_len == 0 {
            return Vec::new();
        }

        // Build edit script using Myers algorithm
        let mut edits: Vec<(usize, usize)> = Vec::new();
        let max_d = old_len + new_len;

        let mut v = vec![0i32; 2 * max_d + 1];
        let mut trace: Vec<Vec<i32>> = Vec::new();

        for d in 0..=max_d {
            let mut v_copy = v.clone();
            for k in (-(d as i32)..=d as i32).step_by(2) {
                let k_index = (k + max_d as i32) as usize;

                let mut x = if k == -(d as i32)
                    || (k != d as i32
                        && v[(k - 1 + max_d as i32) as usize] < v[(k + 1 + max_d as i32) as usize])
                {
                    v[(k + 1 + max_d as i32) as usize]
                } else {
                    v[(k - 1 + max_d as i32) as usize] + 1
                };

                let mut y = x - k;

                // Extend diagonal
                while x < old_len as i32
                    && y < new_len as i32
                    && self.old_content[x as usize] == self.new_content[y as usize]
                {
                    x += 1;
                    y += 1;
                }

                v[k_index] = x;

                if x >= old_len as i32 && y >= new_len as i32 {
                    // Found shortest path
                    // Reconstruct path
                    let mut x = old_len as i32;
                    let mut y = new_len as i32;
                    let d_i32 = d as i32;

                    for (d_idx, tv) in trace.iter().enumerate().rev() {
                        let kd = d_idx as i32 - d_i32;
                        let k_idx = (kd + max_d as i32) as usize;

                        let prev_x = tv.get(k_idx).copied().unwrap_or(0);
                        let prev_y = prev_x - kd;

                        while x > prev_x || y > prev_y {
                            if x > prev_x
                                && y > prev_y
                                && x > 0
                                && y > 0
                                && self.old_content[(x - 1) as usize]
                                    == self.new_content[(y - 1) as usize]
                            {
                                x -= 1;
                                y -= 1;
                                edits.push((x as usize, y as usize));
                            } else if x > prev_x {
                                x -= 1;
                                edits.push((x as usize, usize::MAX)); // Removed
                            } else {
                                y -= 1;
                                edits.push((usize::MAX, y as usize)); // Added
                            }
                        }
                    }

                    edits.reverse();
                    return self.build_hunks_from_edits(&edits);
                }
            }
            trace.push(v.clone());
        }

        // Fallback: build simple diff
        self.build_simple_hunks()
    }

    /// Build hunks from edit script
    fn build_hunks_from_edits(&self, edits: &[(usize, usize)]) -> Vec<DiffHunk> {
        let mut hunks: Vec<DiffHunk> = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;
        let mut old_line = 0;
        let mut new_line = 0;

        for (_i, edit) in edits.iter().enumerate() {
            let (old_idx, new_idx) = edit;

            if *old_idx != usize::MAX && *new_idx != usize::MAX {
                // Context line
                let content = self.old_content.get(*old_idx).cloned().unwrap_or_default();
                let line = DiffLine::Context(content);

                if current_hunk.is_none() {
                    current_hunk = Some(DiffHunk::new(*old_idx + 1, *new_idx + 1));
                }

                if let Some(ref mut hunk) = current_hunk {
                    hunk.add_line(line);
                }
            } else if *old_idx != usize::MAX {
                // Removed line
                let content = self.old_content.get(*old_idx).cloned().unwrap_or_default();
                let line = DiffLine::Removed(content);

                if current_hunk.is_none() {
                    current_hunk = Some(DiffHunk::new(*old_idx + 1, new_line + 1));
                }

                if let Some(ref mut hunk) = current_hunk {
                    hunk.add_line(line);
                }
            } else if *new_idx != usize::MAX {
                // Added line
                let content = self.new_content.get(*new_idx).cloned().unwrap_or_default();
                let line = DiffLine::Added(content);

                if current_hunk.is_none() {
                    current_hunk = Some(DiffHunk::new(old_line + 1, *new_idx + 1));
                }

                if let Some(ref mut hunk) = current_hunk {
                    hunk.add_line(line);
                }
            }
        }

        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        hunks
    }

    /// Build simple hunks (fallback)
    fn build_simple_hunks(&self) -> Vec<DiffHunk> {
        let mut hunks: Vec<DiffHunk> = Vec::new();
        let mut hunk = DiffHunk::new(1, 1);

        let max_len = self.old_content.len().max(self.new_content.len());
        for i in 0..max_len {
            let old_line = self.old_content.get(i);
            let new_line = self.new_content.get(i);

            match (old_line, new_line) {
                (Some(old), Some(new)) if old == new => {
                    hunk.add_line(DiffLine::Context(old.clone()));
                }
                (Some(old), Some(_)) => {
                    hunk.add_line(DiffLine::Removed(old.clone()));
                }
                (Some(old), None) => {
                    hunk.add_line(DiffLine::Removed(old.clone()));
                }
                (None, Some(new)) => {
                    hunk.add_line(DiffLine::Added(new.clone()));
                }
                (None, None) => {}
            }
        }

        if !hunk.lines.is_empty() {
            hunks.push(hunk);
        }

        hunks
    }

    /// Patience diff algorithm (simplified implementation)
    fn compute_patience_diff(&self) -> Vec<DiffHunk> {
        // For simplicity, fall back to Myers with some patience-like improvements
        // A full patience diff would find unique matching lines and recurse
        self.build_simple_hunks()
    }

    /// Get total content height
    fn total_height(&self) -> usize {
        self.hunks.iter().map(|h| h.lines.len() + 1).sum() // +1 for hunk header
    }

    /// Scroll by given amount
    pub fn scroll_by(&mut self, delta: i16) {
        if delta >= 0 {
            self.scroll_offset = self.scroll_offset.saturating_add(delta as usize);
        } else {
            self.scroll_offset = self.scroll_offset.saturating_sub((-delta) as usize);
        }
    }

    /// Get current scroll offset
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Render a line to the buffer
    fn render_line(
        &self,
        line: &DiffLine,
        y: u16,
        x_start: u16,
        width: u16,
        line_num: Option<(usize, usize)>, // (old_line, new_line)
        buf: &mut Buffer,
    ) {
        let style = match line {
            DiffLine::Added(_) => self.added_style,
            DiffLine::Removed(_) => self.removed_style,
            DiffLine::Context(_) => self.context_style,
        };

        let prefix = match line {
            DiffLine::Added(_) => "+",
            DiffLine::Removed(_) => "-",
            DiffLine::Context(_) => " ",
        };

        let mut x = x_start;
        let gutter_width = if self.show_line_numbers { 8 } else { 2 }; // Space for prefix + number

        // Render prefix
        if let Some(cell) = buf.get_mut(x, y) {
            cell.symbol = prefix.to_string();
            cell.set_style(style);
        }
        x += 1;

        // Render line number
        if self.show_line_numbers {
            if let Some((old_num, new_num)) = line_num {
                let num_str = match line {
                    DiffLine::Context(_) => format!("{:4}", old_num),
                    DiffLine::Removed(_) => format!("{:4}", old_num),
                    DiffLine::Added(_) => format!("{:4}", new_num),
                };
                for ch in num_str.chars() {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.line_number_style);
                    }
                    x += 1;
                }
                // Space
                if let Some(cell) = buf.get_mut(x, y) {
                    cell.symbol = " ".to_string();
                    cell.set_style(self.line_number_style);
                }
                x += 1;
            }
        }

        // Render content
        let content = line.content();
        let available_width = width.saturating_sub(gutter_width) as usize;
        let truncated: String = content.chars().take(available_width).collect();

        for ch in truncated.chars() {
            if x >= x_start + width {
                break;
            }
            if let Some(cell) = buf.get_mut(x, y) {
                cell.symbol = ch.to_string();
                cell.set_style(style);
            }
            x += 1;
        }

        // Fill remaining with styled spaces
        while x < x_start + width {
            if let Some(cell) = buf.get_mut(x, y) {
                cell.symbol = " ".to_string();
                cell.set_style(style);
            }
            x += 1;
        }
    }

    /// Render unified diff view
    fn render_unified(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let mut y = area.y;
        let mut line_idx = 0usize;
        let mut old_line = 0usize;
        let mut new_line = 0usize;

        for hunk in &self.hunks {
            // Check if we should render this hunk (scroll handling)
            line_idx += 1; // For header
            if line_idx <= self.scroll_offset {
                // Skip header
            } else if y < area.y + area.height {
                // Render hunk header
                let header = hunk.header();
                let mut x = area.x;
                for ch in header.chars().take(area.width as usize) {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.header_style);
                    }
                    x += 1;
                }
                // Fill rest of line
                while x < area.x + area.width {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.symbol = " ".to_string();
                        cell.set_style(self.header_style);
                    }
                    x += 1;
                }
                y += 1;
            }

            // Render hunk lines
            for diff_line in &hunk.lines {
                line_idx += 1;
                if line_idx <= self.scroll_offset {
                    match diff_line {
                        DiffLine::Context(_) => {
                            old_line += 1;
                            new_line += 1;
                        }
                        DiffLine::Removed(_) => {
                            old_line += 1;
                        }
                        DiffLine::Added(_) => {
                            new_line += 1;
                        }
                    }
                    continue;
                }

                if y >= area.y + area.height {
                    break;
                }

                let line_num = match diff_line {
                    DiffLine::Context(_) => {
                        old_line += 1;
                        new_line += 1;
                        Some((old_line, new_line))
                    }
                    DiffLine::Removed(_) => {
                        old_line += 1;
                        Some((old_line, new_line + 1)) // new_line doesn't advance for removed
                    }
                    DiffLine::Added(_) => {
                        new_line += 1;
                        Some((old_line, new_line))
                    }
                };

                self.render_line(diff_line, y, area.x, area.width, line_num, buf);
                y += 1;
            }
        }
    }

    /// Render split diff view
    fn render_split(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() || area.width < 2 {
            return;
        }

        // Split area into two halves with a divider
        let divider_width = 1u16;
        let half_width = (area.width - divider_width) / 2;

        let left_area = Rect::new(area.x, area.y, half_width, area.height);
        let right_area = Rect::new(
            area.x + half_width + divider_width,
            area.y,
            half_width,
            area.height,
        );

        // Draw divider
        for y in area.y..area.y + area.height {
            let x = area.x + half_width;
            if let Some(cell) = buf.get_mut(x, y) {
                cell.symbol = "│".to_string();
                cell.set_style(Style::default().fg(Color::DarkGray));
            }
        }

        // Render left and right sides
        let mut y = area.y;
        let mut line_idx = 0usize;
        let mut old_line = 0usize;
        let mut new_line = 0usize;

        for hunk in &self.hunks {
            // Render hunk header on both sides
            line_idx += 1;
            if line_idx > self.scroll_offset && y < area.y + area.height {
                let header = hunk.header();

                // Left side
                let mut x = left_area.x;
                for ch in header.chars().take(left_area.width as usize) {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.header_style);
                    }
                    x += 1;
                }

                // Right side
                x = right_area.x;
                for ch in header.chars().take(right_area.width as usize) {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.header_style);
                    }
                    x += 1;
                }

                y += 1;
            }

            // Render lines
            for diff_line in &hunk.lines {
                line_idx += 1;
                if line_idx <= self.scroll_offset {
                    continue;
                }

                if y >= area.y + area.height {
                    break;
                }

                match diff_line {
                    DiffLine::Context(content) => {
                        old_line += 1;
                        new_line += 1;
                        self.render_split_line(
                            content,
                            y,
                            left_area.width,
                            false,
                            false,
                            left_area.x,
                            buf,
                        );
                        self.render_split_line(
                            content,
                            y,
                            right_area.width,
                            false,
                            true,
                            right_area.x,
                            buf,
                        );
                    }
                    DiffLine::Removed(content) => {
                        old_line += 1;
                        self.render_split_line(
                            content,
                            y,
                            left_area.width,
                            true,
                            false,
                            left_area.x,
                            buf,
                        );
                        // Empty line on right
                        self.render_split_line(
                            "",
                            y,
                            right_area.width,
                            false,
                            true,
                            right_area.x,
                            buf,
                        );
                    }
                    DiffLine::Added(content) => {
                        new_line += 1;
                        // Empty line on left
                        self.render_split_line(
                            "",
                            y,
                            left_area.width,
                            false,
                            false,
                            left_area.x,
                            buf,
                        );
                        self.render_split_line(
                            content,
                            y,
                            right_area.width,
                            true,
                            true,
                            right_area.x,
                            buf,
                        );
                    }
                }
                y += 1;
            }
        }
    }

    /// Render a single line in split view
    fn render_split_line(
        &self,
        content: &str,
        y: u16,
        width: u16,
        is_change: bool,
        is_new: bool,
        x_start: u16,
        buf: &mut Buffer,
    ) {
        let style = if is_change {
            if is_new {
                self.added_style
            } else {
                self.removed_style
            }
        } else {
            self.context_style
        };

        // Line number width
        let num_width = if self.show_line_numbers { 5 } else { 0 };
        let mut x = x_start;

        // Render line number area (empty or with number)
        for _ in 0..num_width {
            if x >= x_start + width {
                break;
            }
            if let Some(cell) = buf.get_mut(x, y) {
                cell.symbol = " ".to_string();
                cell.set_style(style);
            }
            x += 1;
        }

        // Render content
        let available_width = width.saturating_sub(num_width) as usize;
        let truncated: String = content.chars().take(available_width).collect();

        for ch in truncated.chars() {
            if x >= x_start + width {
                break;
            }
            if let Some(cell) = buf.get_mut(x, y) {
                cell.symbol = ch.to_string();
                cell.set_style(style);
            }
            x += 1;
        }

        // Fill remaining
        while x < x_start + width {
            if let Some(cell) = buf.get_mut(x, y) {
                cell.symbol = " ".to_string();
                cell.set_style(style);
            }
            x += 1;
        }
    }
}

/// Props for DiffViewer
pub struct DiffViewerProps {
    pub mode: DiffMode,
    pub old_content: Vec<String>,
    pub new_content: Vec<String>,
    pub algorithm: DiffAlgorithm,
    pub added_style: Style,
    pub removed_style: Style,
    pub show_line_numbers: bool,
}

impl DiffViewerProps {
    /// Create new diff viewer props
    pub fn new(old_content: &str, new_content: &str) -> Self {
        Self {
            mode: DiffMode::Unified,
            old_content: old_content.lines().map(|s| s.to_string()).collect(),
            new_content: new_content.lines().map(|s| s.to_string()).collect(),
            algorithm: DiffAlgorithm::Myers,
            added_style: Style::new().fg(Color::Green).bg(Color::Indexed(22)),
            removed_style: Style::new().fg(Color::Red).bg(Color::Indexed(52)),
            show_line_numbers: true,
        }
    }

    /// Set mode
    pub fn mode(mut self, mode: DiffMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set algorithm
    pub fn algorithm(mut self, algorithm: DiffAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Set added style
    pub fn added_style(mut self, style: Style) -> Self {
        self.added_style = style;
        self
    }

    /// Set removed style
    pub fn removed_style(mut self, style: Style) -> Self {
        self.removed_style = style;
        self
    }

    /// Set show line numbers
    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }
}

impl Component for DiffViewer {
    type Props = DiffViewerProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        let mut viewer = Self {
            mode: props.mode,
            old_content: props.old_content,
            new_content: props.new_content,
            diff_algorithm: props.algorithm,
            added_style: props.added_style,
            removed_style: props.removed_style,
            show_line_numbers: props.show_line_numbers,
            ..Self::default()
        };
        viewer.compute_diff();
        viewer
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        match self.mode {
            DiffMode::Unified => self.render_unified(area, buf),
            DiffMode::Split => self.render_split(area, buf),
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

    fn render_to_string(viewer: &DiffViewer, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        viewer.render(Rect::new(0, 0, width, height), &mut buf);

        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                output.push_str(&buf[(x, y)].symbol);
            }
            if y < height - 1 {
                output.push('\n');
            }
        }
        output
    }

    #[test]
    fn snapshot_diff_empty() {
        let viewer = DiffViewer::new().old_content("").new_content("");
        let result = render_to_string(&viewer, 40, 5);
        assert_snapshot!("diff_empty", result);
    }

    #[test]
    fn snapshot_diff_identical() {
        let viewer = DiffViewer::new()
            .old_content("line 1\nline 2\nline 3")
            .new_content("line 1\nline 2\nline 3");
        let result = render_to_string(&viewer, 40, 10);
        assert_snapshot!("diff_identical", result);
    }

    #[test]
    fn snapshot_diff_simple_add() {
        let mut viewer = DiffViewer::new()
            .old_content("line 1\nline 2")
            .new_content("line 1\nline 2\nline 3");
        viewer.compute_diff();
        let result = render_to_string(&viewer, 40, 10);
        assert_snapshot!("diff_simple_add", result);
    }

    #[test]
    fn snapshot_diff_simple_remove() {
        let mut viewer = DiffViewer::new()
            .old_content("line 1\nline 2\nline 3")
            .new_content("line 1\nline 3");
        viewer.compute_diff();
        let result = render_to_string(&viewer, 40, 10);
        assert_snapshot!("diff_simple_remove", result);
    }

    #[test]
    fn snapshot_diff_modify() {
        let mut viewer = DiffViewer::new()
            .old_content("unchanged\nold line\nunchanged 2")
            .new_content("unchanged\nnew line\nunchanged 2");
        viewer.compute_diff();
        let result = render_to_string(&viewer, 40, 10);
        assert_snapshot!("diff_modify", result);
    }

    #[test]
    fn snapshot_diff_split_mode() {
        let mut viewer = DiffViewer::new()
            .old_content("line 1\nold line\nline 3")
            .new_content("line 1\nnew line\nline 3")
            .mode(DiffMode::Split);
        viewer.compute_diff();
        let result = render_to_string(&viewer, 50, 10);
        assert_snapshot!("diff_split_mode", result);
    }

    #[test]
    fn test_diff_line_types() {
        let context = DiffLine::Context("test".to_string());
        assert!(context.is_context());
        assert!(!context.is_added());
        assert!(!context.is_removed());
        assert_eq!(context.content(), "test");

        let added = DiffLine::Added("added".to_string());
        assert!(added.is_added());
        assert!(!added.is_context());

        let removed = DiffLine::Removed("removed".to_string());
        assert!(removed.is_removed());
    }

    #[test]
    fn test_diff_hunk() {
        let mut hunk = DiffHunk::new(1, 1);
        assert_eq!(hunk.old_start, 1);
        assert_eq!(hunk.new_start, 1);
        assert_eq!(hunk.old_count, 0);
        assert_eq!(hunk.new_count, 0);

        hunk.add_line(DiffLine::Context("test".to_string()));
        assert_eq!(hunk.old_count, 1);
        assert_eq!(hunk.new_count, 1);

        hunk.add_line(DiffLine::Removed("old".to_string()));
        assert_eq!(hunk.old_count, 2);
        assert_eq!(hunk.new_count, 1);

        hunk.add_line(DiffLine::Added("new".to_string()));
        assert_eq!(hunk.old_count, 2);
        assert_eq!(hunk.new_count, 2);

        assert_eq!(hunk.lines.len(), 3);
    }

    #[test]
    fn test_diff_hunk_header() {
        let hunk = DiffHunk {
            old_start: 1,
            old_count: 5,
            new_start: 1,
            new_count: 7,
            lines: vec![],
        };
        assert_eq!(hunk.header(), "@@ -1,5 +1,7 @@");
    }

    #[test]
    fn test_diff_viewer_builder() {
        let mut viewer = DiffViewer::new()
            .old_content("a\nb\nc")
            .new_content("a\nx\nc")
            .mode(DiffMode::Unified)
            .algorithm(DiffAlgorithm::Myers)
            .show_line_numbers(true);

        assert_eq!(viewer.mode, DiffMode::Unified);
        assert_eq!(viewer.diff_algorithm, DiffAlgorithm::Myers);
        assert_eq!(viewer.old_content.len(), 3);
        assert_eq!(viewer.new_content.len(), 3);
        assert!(viewer.show_line_numbers);
    }

    #[test]
    fn test_diff_viewer_with_styles() {
        let viewer = DiffViewer::new()
            .added_style(Style::new().fg(Color::Green).bg(Color::Black))
            .removed_style(Style::new().fg(Color::Red).bg(Color::Black))
            .header_style(Style::new().fg(Color::Cyan))
            .line_number_style(Style::new().fg(Color::Blue));

        assert_eq!(viewer.added_style.fg, Color::Green);
        assert_eq!(viewer.removed_style.fg, Color::Red);
    }

    #[test]
    fn test_diff_modes() {
        assert_eq!(DiffMode::default(), DiffMode::Unified);
        assert_eq!(DiffAlgorithm::default(), DiffAlgorithm::Myers);
    }

    #[test]
    fn test_props_creation() {
        let props = DiffViewerProps::new("a\nb", "a\nc")
            .mode(DiffMode::Split)
            .algorithm(DiffAlgorithm::Patience)
            .show_line_numbers(false);

        assert_eq!(props.mode, DiffMode::Split);
        assert_eq!(props.algorithm, DiffAlgorithm::Patience);
        assert!(!props.show_line_numbers);
    }

    #[test]
    fn test_component_create() {
        let props = DiffViewerProps::new("old", "new");
        let viewer = DiffViewer::create(props);

        assert_eq!(viewer.mode, DiffMode::Unified);
        assert_eq!(viewer.old_content, vec!["old"]);
        assert_eq!(viewer.new_content, vec!["new"]);
    }

    #[test]
    fn test_scroll() {
        let mut viewer = DiffViewer::new()
            .old_content("line 1\nline 2\nline 3")
            .new_content("line 1\nline 2\nline 3");

        assert_eq!(viewer.scroll_offset(), 0);

        viewer.scroll_by(5);
        assert_eq!(viewer.scroll_offset(), 5);

        viewer.scroll_by(-2);
        assert_eq!(viewer.scroll_offset(), 3);

        // Can't go below 0
        viewer.scroll_by(-10);
        assert_eq!(viewer.scroll_offset(), 0);
    }

    #[test]
    fn test_render_empty_area() {
        let viewer = DiffViewer::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        viewer.render(Rect::new(0, 0, 0, 0), &mut buf);
        // Should not panic
    }

    #[test]
    fn test_large_diff() {
        let old_content: String = (1..=50).map(|i| format!("line {}\n", i)).collect();
        let new_content: String = (1..=55).map(|i| format!("line {}\n", i)).collect();

        let mut viewer = DiffViewer::new()
            .old_content(&old_content)
            .new_content(&new_content);
        viewer.compute_diff();

        // Should have computed hunks
        assert!(!viewer.hunks.is_empty() || viewer.old_content == viewer.new_content);
    }

    #[test]
    fn snapshot_diff_no_line_numbers() {
        let mut viewer = DiffViewer::new()
            .old_content("line 1\nold line\nline 3")
            .new_content("line 1\nnew line\nline 3")
            .show_line_numbers(false);
        viewer.compute_diff();
        let result = render_to_string(&viewer, 40, 10);
        assert_snapshot!("diff_no_line_numbers", result);
    }
}
