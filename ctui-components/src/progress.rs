//! Progress bar and spinner components for displaying progress status.
//!
//! This module provides visual indicators for progress:
//! - [`ProgressBar`] - Shows progress as a filled bar
//! - [`Spinner`] - Animated loading indicator

use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

/// Style for the progress bar track (background)
const TRACK_CHAR: char = ' ';
/// Character for filled portion
const FILLED_CHAR: char = '█';
/// Characters for partial filling
const PARTIAL_CHARS: &[char] = &[' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

/// A progress bar that shows completion progress.
///
/// # Example
///
/// ```
/// use ctui_components::ProgressBar;
/// use ctui_core::style::{Style, Color};
///
/// let progress = ProgressBar::new()
///     .ratio(0.75)
///     .label("Downloading...")
///     .style(Style::new().fg(Color::Green));
/// ```
#[derive(Clone, Debug)]
pub struct ProgressBar {
    /// Progress ratio (0.0 to 1.0)
    ratio: f64,
    /// Optional label displayed inside or above the bar
    label: Option<String>,
    /// Show percentage text
    show_percent: bool,
    /// Style for the filled portion
    style: Style,
    /// Style for the track (empty portion)
    track_style: Style,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            ratio: 0.0,
            label: None,
            show_percent: false,
            style: Style::default(),
            track_style: Style::default(),
        }
    }
}

impl ProgressBar {
    /// Creates a new progress bar with 0% progress
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the progress ratio (0.0 to 1.0)
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.ratio = ratio.clamp(0.0, 1.0);
        self
    }

    /// Sets a label to display with the progress bar
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Shows percentage text inside the bar
    pub fn show_percent(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }

    /// Sets the style for the filled portion
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style for the track (empty portion)
    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = style;
        self
    }

    /// Returns the current progress ratio
    pub fn get_ratio(&self) -> f64 {
        self.ratio
    }

    /// Sets the progress ratio
    pub fn set_ratio(&mut self, ratio: f64) {
        self.ratio = ratio.clamp(0.0, 1.0);
    }
}

/// Props for creating a ProgressBar
pub struct ProgressBarProps {
    pub ratio: f64,
    pub label: Option<String>,
    pub show_percent: bool,
    pub style: Style,
    pub track_style: Style,
}

impl ProgressBarProps {
    pub fn new(ratio: f64) -> Self {
        Self {
            ratio: ratio.clamp(0.0, 1.0),
            label: None,
            show_percent: false,
            style: Style::default(),
            track_style: Style::default(),
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn show_percent(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = style;
        self
    }
}

impl Component for ProgressBar {
    type Props = ProgressBarProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            ratio: props.ratio,
            label: props.label,
            show_percent: props.show_percent,
            style: props.style,
            track_style: props.track_style,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let width = area.width as usize;
        let filled_width = (self.ratio * width as f64).round() as usize;
        let partial_idx = ((self.ratio * width as f64).fract() * (PARTIAL_CHARS.len() - 1) as f64)
            .round() as usize;
        let partial_idx = partial_idx.min(PARTIAL_CHARS.len() - 1);

        // Build the percentage text if needed
        let percent_text = if self.show_percent {
            Some(format!("{:.0}%", self.ratio * 100.0))
        } else {
            None
        };

        // Calculate text position for centering
        let display_text = if let Some(ref label) = self.label {
            Some(if let Some(ref pct) = percent_text {
                format!("{} {}", label, pct)
            } else {
                label.clone()
            })
        } else {
            percent_text.clone()
        };

        // Render filled portion
        for x in 0..filled_width.min(width) {
            if let Some(cell) = buf.get_mut(area.x + x as u16, area.y) {
                cell.symbol = FILLED_CHAR.to_string();
                cell.set_style(self.style);
            }
        }

        // Render partial character
        if filled_width < width {
            if let Some(cell) = buf.get_mut(area.x + filled_width as u16, area.y) {
                cell.symbol = PARTIAL_CHARS[partial_idx].to_string();
                cell.set_style(self.style);
            }
        }

        // Render track (empty portion)
        for x in (filled_width + 1).min(width)..width {
            if let Some(cell) = buf.get_mut(area.x + x as u16, area.y) {
                cell.symbol = TRACK_CHAR.to_string();
                cell.set_style(self.track_style);
            }
        }

        // Render centered text if we have something to display
        if let Some(text) = display_text {
            let text_chars: Vec<char> = text.chars().collect();
            let text_width = text_chars.len();
            let text_start = if text_width < width {
                (width - text_width) / 2
            } else {
                0
            };

            for (i, ch) in text_chars.iter().enumerate() {
                let x_pos = text_start + i;
                if x_pos >= width {
                    break;
                }
                if let Some(cell) = buf.get_mut(area.x + x_pos as u16, area.y) {
                    cell.symbol = ch.to_string();
                    // Determine style based on position
                    let use_filled_style = x_pos < filled_width;
                    cell.set_style(if use_filled_style {
                        self.style
                    } else {
                        self.track_style
                    });
                }
            }
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

/// Spinner animation frame sets
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SpinnerStyle {
    /// Classic dots spinner: ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
    Dots,
    /// Simple bar spinner: |/-\
    Bars,
    ///ASCII arrow: <->
    Arrow,
    /// Custom spinner frames
    Custom,
}

impl SpinnerStyle {
    /// Returns the animation frames for this style
    pub fn frames(&self) -> &[char] {
        match self {
            SpinnerStyle::Dots => &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            SpinnerStyle::Bars => &['|', '/', '-', '\\'],
            SpinnerStyle::Arrow => &['<', '-', '>', '-'],
            SpinnerStyle::Custom => &['.'],
        }
    }
}

impl Default for SpinnerStyle {
    fn default() -> Self {
        SpinnerStyle::Dots
    }
}

/// An animated spinner for loading states.
///
/// # Example
///
/// ```
/// use ctui_components::{Spinner, SpinnerStyle};
///
/// let spinner = Spinner::new()
///     .spinner_style(SpinnerStyle::Bars)
///     .frame(2);
/// ```
#[derive(Clone, Debug)]
pub struct Spinner {
    /// Current frame index
    frame: usize,
    /// Spinner style (animation pattern)
    spinner_style: SpinnerStyle,
    /// Custom frames if using Custom style
    custom_frames: Vec<char>,
    /// Style for the spinner character
    style: Style,
    /// Optional label
    label: Option<String>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            frame: 0,
            spinner_style: SpinnerStyle::default(),
            custom_frames: vec!['.'],
            style: Style::default(),
            label: None,
        }
    }
}

impl Spinner {
    /// Creates a new spinner
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the spinner style
    pub fn spinner_style(mut self, style: SpinnerStyle) -> Self {
        self.spinner_style = style;
        self
    }

    /// Sets the current frame index
    pub fn frame(mut self, frame: usize) -> Self {
        let frames = self.get_frames();
        self.frame = frame % frames.len();
        self
    }

    /// Sets custom animation frames
    pub fn custom_frames(mut self, frames: impl Into<Vec<char>>) -> Self {
        let frames = frames.into();
        if !frames.is_empty() {
            self.custom_frames = frames;
            self.spinner_style = SpinnerStyle::Custom;
        }
        self
    }

    /// Sets the style for the spinner
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets an optional label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Advances to the next frame
    pub fn tick(&mut self) {
        let frames = self.get_frames();
        self.frame = (self.frame + 1) % frames.len();
    }

    /// Returns the current frame character
    pub fn current_char(&self) -> char {
        let frames = self.get_frames();
        frames[self.frame % frames.len()]
    }

    /// Returns the frames for the current style
    fn get_frames(&self) -> Vec<char> {
        match self.spinner_style {
            SpinnerStyle::Custom => self.custom_frames.clone(),
            _ => self.spinner_style.frames().to_vec(),
        }
    }
}

/// Props for creating a Spinner
pub struct SpinnerProps {
    pub spinner_style: SpinnerStyle,
    pub frame: usize,
    pub custom_frames: Option<Vec<char>>,
    pub style: Style,
    pub label: Option<String>,
}

impl SpinnerProps {
    pub fn new() -> Self {
        Self {
            spinner_style: SpinnerStyle::default(),
            frame: 0,
            custom_frames: None,
            style: Style::default(),
            label: None,
        }
    }

    pub fn spinner_style(mut self, style: SpinnerStyle) -> Self {
        self.spinner_style = style;
        self
    }

    pub fn frame(mut self, frame: usize) -> Self {
        self.frame = frame;
        self
    }

    pub fn custom_frames(mut self, frames: impl Into<Vec<char>>) -> Self {
        self.custom_frames = Some(frames.into());
        self.spinner_style = SpinnerStyle::Custom;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Default for SpinnerProps {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Spinner {
    type Props = SpinnerProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        let mut spinner = Self {
            frame: 0,
            spinner_style: props.spinner_style,
            custom_frames: props.custom_frames.unwrap_or_else(|| vec!['.']),
            style: props.style,
            label: props.label,
        };
        let frames = spinner.get_frames();
        spinner.frame = props.frame % frames.len();
        spinner
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let spinner_char = self.current_char();
        let label = self.label.as_deref().unwrap_or("");

        // Build the display string: "spinner label" or just "spinner"
        let display = if label.is_empty() {
            spinner_char.to_string()
        } else {
            format!("{} {}", spinner_char, label)
        };

        // Render the display string
        for (i, ch) in display.chars().enumerate() {
            if i >= area.width as usize {
                break;
            }
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
                cell.set_style(self.style);
            }
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        self.tick();
        Cmd::Render
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::style::Color;
    use ctui_core::Buffer;
    use insta::assert_snapshot;

    fn render_to_string(progress: &ProgressBar, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        progress.render(Rect::new(0, 0, width, height), &mut buf);

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

    fn render_spinner_to_string(spinner: &Spinner, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        spinner.render(Rect::new(0, 0, width, height), &mut buf);

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

    // ProgressBar Tests

    #[test]
    fn snapshot_progress_bar_zero() {
        let progress = ProgressBar::new().ratio(0.0);
        let result = render_to_string(&progress, 20, 1);
        assert_snapshot!("progress_bar_zero", result);
    }

    #[test]
    fn snapshot_progress_bar_half() {
        let progress = ProgressBar::new().ratio(0.5);
        let result = render_to_string(&progress, 20, 1);
        assert_snapshot!("progress_bar_half", result);
    }

    #[test]
    fn snapshot_progress_bar_full() {
        let progress = ProgressBar::new().ratio(1.0);
        let result = render_to_string(&progress, 20, 1);
        assert_snapshot!("progress_bar_full", result);
    }

    #[test]
    fn snapshot_progress_bar_with_label() {
        let progress = ProgressBar::new().ratio(0.6).label("Loading...");
        let result = render_to_string(&progress, 20, 1);
        assert_snapshot!("progress_bar_with_label", result);
    }

    #[test]
    fn snapshot_progress_bar_with_percent() {
        let progress = ProgressBar::new().ratio(0.75).show_percent(true);
        let result = render_to_string(&progress, 20, 1);
        assert_snapshot!("progress_bar_with_percent", result);
    }

    #[test]
    fn snapshot_progress_bar_with_label_and_percent() {
        let progress = ProgressBar::new()
            .ratio(0.8)
            .label("Download")
            .show_percent(true);
        let result = render_to_string(&progress, 30, 1);
        assert_snapshot!("progress_bar_with_label_and_percent", result);
    }

    #[test]
    fn test_progress_bar_default() {
        let progress = ProgressBar::new();
        assert_eq!(progress.get_ratio(), 0.0);
    }

    #[test]
    fn test_progress_bar_ratio_clamped() {
        let progress = ProgressBar::new().ratio(-0.5);
        assert_eq!(progress.get_ratio(), 0.0);

        let progress = ProgressBar::new().ratio(1.5);
        assert_eq!(progress.get_ratio(), 1.0);
    }

    #[test]
    fn test_progress_bar_set_ratio() {
        let mut progress = ProgressBar::new();
        progress.set_ratio(0.42);
        assert_eq!(progress.get_ratio(), 0.42);
    }

    #[test]
    fn test_progress_bar_props() {
        let props = ProgressBarProps::new(0.5).label("Test").show_percent(true);

        assert_eq!(props.ratio, 0.5);
        assert_eq!(props.label, Some("Test".to_string()));
        assert!(props.show_percent);

        let progress = ProgressBar::create(props);
        assert_eq!(progress.get_ratio(), 0.5);
    }

    // Spinner Tests

    #[test]
    fn snapshot_spinner_dots() {
        let spinner = Spinner::new().spinner_style(SpinnerStyle::Dots).frame(0);
        let result = render_spinner_to_string(&spinner, 15, 1);
        assert_snapshot!("spinner_dots", result);
    }

    #[test]
    fn snapshot_spinner_bars() {
        let spinner = Spinner::new().spinner_style(SpinnerStyle::Bars).frame(0);
        let result = render_spinner_to_string(&spinner, 15, 1);
        assert_snapshot!("spinner_bars", result);
    }

    #[test]
    fn snapshot_spinner_arrow() {
        let spinner = Spinner::new().spinner_style(SpinnerStyle::Arrow).frame(0);
        let result = render_spinner_to_string(&spinner, 15, 1);
        assert_snapshot!("spinner_arrow", result);
    }

    #[test]
    fn snapshot_spinner_with_label() {
        let spinner = Spinner::new()
            .spinner_style(SpinnerStyle::Dots)
            .label("Loading...")
            .frame(0);
        let result = render_spinner_to_string(&spinner, 20, 1);
        assert_snapshot!("spinner_with_label", result);
    }

    #[test]
    fn test_spinner_default() {
        let spinner = Spinner::new();
        assert_eq!(spinner.spinner_style, SpinnerStyle::Dots);
        assert_eq!(spinner.frame, 0);
    }

    #[test]
    fn test_spinner_tick() {
        let mut spinner = Spinner::new().spinner_style(SpinnerStyle::Bars);
        let first = spinner.current_char();
        spinner.tick();
        let second = spinner.current_char();
        assert_ne!(first, second);
    }

    #[test]
    fn test_spinner_frame_wrapping() {
        let spinner = Spinner::new().spinner_style(SpinnerStyle::Bars).frame(100);
        // Bars has 4 frames, so frame 100 should wrap to frame 0
        assert_eq!(spinner.frame, 0);
    }

    #[test]
    fn test_spinner_custom_frames() {
        let spinner = Spinner::new().custom_frames(vec!['a', 'b', 'c']).frame(0);
        assert_eq!(spinner.current_char(), 'a');
        assert_eq!(spinner.spinner_style, SpinnerStyle::Custom);
    }

    #[test]
    fn test_spinner_props() {
        let props = SpinnerProps::new()
            .spinner_style(SpinnerStyle::Bars)
            .frame(2)
            .label("Processing");

        let spinner = Spinner::create(props);
        assert_eq!(spinner.spinner_style, SpinnerStyle::Bars);
        assert_eq!(spinner.label, Some("Processing".to_string()));
    }

    #[test]
    fn test_spinner_frames() {
        let dots_frames = SpinnerStyle::Dots.frames();
        assert_eq!(dots_frames.len(), 10);

        let bars_frames = SpinnerStyle::Bars.frames();
        assert_eq!(bars_frames.len(), 4);

        let arrow_frames = SpinnerStyle::Arrow.frames();
        assert_eq!(arrow_frames.len(), 4);
    }

    #[test]
    fn test_component_render_empty_area() {
        let progress = ProgressBar::new().ratio(0.5);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        progress.render(Rect::new(0, 0, 0, 0), &mut buf);
        // Should not panic
    }

    #[test]
    fn test_spinner_render_empty_area() {
        let spinner = Spinner::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        spinner.render(Rect::new(0, 0, 0, 0), &mut buf);
        // Should not panic
    }
}
