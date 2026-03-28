//! Gauge widgets for displaying progress and measurements in terminal UIs.
//!
//! This module provides widgets for visualizing progress, percentages, and
//! numerical values. Gauges come in two variants: circular (arc) gauges and
//! linear progress bars.
//!
//! # Widgets
//!
//! - [`Gauge`]: A circular/arc gauge for displaying percentages
//! - [`LinearGauge`]: A horizontal progress bar
//!
//! # Example
//!
//! \`\`\`rust
//! use ctui_components::{Gauge, LinearGauge, Widget};
//! use ctui_core::style::{Style, Color};
//!
//! // Circular gauge for CPU usage
//! let gauge = Gauge::new()
//!     .value(65.0)
//!     .max(100.0)
//!     .label("CPU")
//!     .gauge_style(Style::new().fg(Color::Green));
//!
//! // Linear progress bar for downloads
//! let progress = LinearGauge::new()
//!     .value(750.0)
//!     .max(1000.0)
//!     .show_percent(true)
//!     .filled_style(Style::new().fg(Color::Blue));
//! \`\`\`

use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};
use std::f64::consts::PI;

/// A circular/arc gauge widget for displaying progress.
///
/// Renders a semi-circular gauge filled based on the value ratio.
/// Useful for displaying CPU usage, battery level, temperature, or any
/// other percentage-based metric.
///
/// # Example
///
/// \`\`\`rust
/// use ctui_components::Gauge;
/// use ctui_core::style::{Style, Color};
///
/// let gauge = Gauge::new()
///     .value(75.0)
///     .max(100.0)
///     .label("Memory")
///     .gauge_style(Style::new().fg(Color::Cyan));
/// \`\`\`
#[derive(Clone, Debug)]
pub struct Gauge {
    /// Current value of the gauge.
    value: f64,
    /// Maximum value (value/max gives the percentage).
    max: f64,
    /// Optional label displayed in the center.
    label: Option<String>,
    /// Style for unfilled portions.
    style: Style,
    /// Style for filled portions.
    gauge_style: Style,
    /// Starting angle of the arc (radians).
    start_angle: f64,
    /// Ending angle of the arc (radians).
    end_angle: f64,
}

impl Default for Gauge {
    fn default() -> Self {
        Self {
            value: 0.0,
            max: 100.0,
            label: None,
            style: Style::default(),
            gauge_style: Style::default(),
            start_angle: -PI * 0.75,
            end_angle: PI * 0.75,
        }
    }
}

impl Gauge {
    /// Creates a new gauge with 0% filled.
    ///
    /// Default settings:
    /// - Arc from -135° to +135° (semi-circular)
    /// - 100 max value
    /// - No label (shows percentage instead)
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Gauge;
    ///
    /// let gauge = Gauge::new();
    /// assert_eq!(gauge.percent(), 0.0);
    /// \`\`\`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the current value of the gauge.
    ///
    /// The value is clamped between 0 and \`max\`.
    ///
    /// # Arguments
    ///
    /// * \`value\` - The current value.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Gauge;
    ///
    /// let gauge = Gauge::new().value(50.0);
    /// assert_eq!(gauge.percent(), 0.5);
    /// \`\`\`
    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(0.0, self.max);
        self
    }

    /// Sets the maximum value (100% point).
    ///
    /// # Arguments
    ///
    /// * \`max\` - The maximum value (must be >= 0).
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Gauge;
    ///
    /// let gauge = Gauge::new().max(200.0).value(100.0);
    /// assert_eq!(gauge.percent(), 0.5);
    /// \`\`\`
    pub fn max(mut self, max: f64) -> Self {
        self.max = max.max(0.0);
        self
    }

    /// Sets the label displayed in the gauge center.
    ///
    /// If no label is set, the percentage is displayed instead.
    ///
    /// # Arguments
    ///
    /// * \`label\` - The label text to display.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Gauge;
    ///
    /// let gauge = Gauge::new().value(42.0).label("°C");
    /// \`\`\`
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the style for unfilled portions of the gauge.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The [`Style`] for empty areas.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Gauge;
    /// use ctui_core::style::{Style, Color};
    ///
    /// let gauge = Gauge::new().style(Style::new().fg(Color::DarkGray));
    /// \`\`\`
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style for filled portions of the gauge.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The [`Style`] for filled areas.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Gauge;
    /// use ctui_core::style::{Style, Color};
    ///
    /// let gauge = Gauge::new()
    ///     .value(100.0)
    ///     .gauge_style(Style::new().fg(Color::Green));
    /// \`\`\`
    pub fn gauge_style(mut self, style: Style) -> Self {
        self.gauge_style = style;
        self
    }

    /// Sets the angle range for the gauge arc.
    ///
    /// Angles are specified in radians. Default is -135° to +135°,
    /// creating a semi-circular gauge opening upward.
    ///
    /// # Arguments
    ///
    /// * \`start\` - Starting angle in radians.
    /// * \`end\` - Ending angle in radians.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Gauge;
    /// use std::f64::consts::PI;
    ///
    /// // Full circle gauge
    /// let gauge = Gauge::new().angle_range(0.0, 2.0 * PI);
    /// \`\`\`
    pub fn angle_range(mut self, start: f64, end: f64) -> Self {
        self.start_angle = start;
        self.end_angle = end;
        self
    }

    /// Returns the current value as a percentage (0.0 to 1.0).
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::Gauge;
    ///
    /// let gauge = Gauge::new().value(25.0).max(100.0);
    /// assert_eq!(gauge.percent(), 0.25);
    /// \`\`\`
    pub fn percent(&self) -> f64 {
        if self.max == 0.0 {
            0.0
        } else {
            self.value / self.max
        }
    }

    /// Checks if a point is within an arc segment.
    ///
    /// Used internally to determine which cells to fill.
    fn in_arc(x: f64, y: f64, outer_r: f64, inner_r: f64, start: f64, end: f64) -> bool {
        let dist_sq = x * x + y * y;
        let outer_r_sq = outer_r * outer_r;
        let inner_r_sq = inner_r * inner_r;

        if dist_sq > outer_r_sq || dist_sq < inner_r_sq {
            return false;
        }

        let angle = y.atan2(x);
        let angle = if angle < 0.0 { angle + 2.0 * PI } else { angle };

        let start = if start < 0.0 { start + 2.0 * PI } else { start };
        let end = if end < 0.0 { end + 2.0 * PI } else { end };

        if start <= end {
            angle >= start && angle <= end
        } else {
            angle >= start || angle <= end
        }
    }

    /// Checks if an angle falls within the filled portion of the gauge.
    fn angle_in_range(&self, angle: f64, percent: f64) -> bool {
        let range = self.end_angle - self.start_angle;
        let filled_range = range * percent;
        let filled_end = self.start_angle + filled_range;

        let normalized_angle = if angle < 0.0 { angle + 2.0 * PI } else { angle };
        let normalized_start = if self.start_angle < 0.0 {
            self.start_angle + 2.0 * PI
        } else {
            self.start_angle
        };
        let normalized_end = if filled_end < 0.0 {
            filled_end + 2.0 * PI
        } else {
            filled_end
        };

        if normalized_start <= normalized_end {
            normalized_angle >= normalized_start && normalized_angle <= normalized_end
        } else {
            normalized_angle >= normalized_start || normalized_angle <= normalized_end
        }
    }
}

impl Widget for Gauge {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let center_x = area.x + area.width / 2;
        let center_y = area.y + area.height / 2;

        let size = area.width.min(area.height * 2).min(20) as usize;
        let outer_r = size as f64 / 2.0;
        let inner_r = outer_r * 0.6;

        let percent = self.percent();

        for y_offset in 0..=size / 2 {
            for x_offset in 0..=size {
                let dx = (x_offset as f64 - outer_r) / 2.0;
                let dy = y_offset as f64 - outer_r / 2.0;

                let dist = (dx * dx * 4.0 + dy * dy).sqrt();

                if dist <= outer_r && dist >= inner_r * 0.8 {
                    let angle = (dy).atan2(dx * 2.0);

                    let is_filled = self.angle_in_range(angle, percent);
                    let style = if is_filled {
                        self.gauge_style
                    } else {
                        self.style
                    };

                    let buf_x = center_x.saturating_sub((outer_r - x_offset as f64) as u16);
                    let buf_y = center_y.saturating_sub((outer_r / 2.0 - y_offset as f64) as u16);

                    if buf_x >= area.x
                        && buf_x < area.x + area.width
                        && buf_y >= area.y
                        && buf_y < area.y + area.height
                    {
                        buf.modify_cell(buf_x, buf_y, |cell| {
                            cell.symbol = if is_filled { "█" } else { "░" }.to_string();
                            cell.set_style(style);
                        });
                    }

                    let mirror_x = center_x + (x_offset as f64 - outer_r) as u16;
                    if mirror_x < area.x + area.width {
                        buf.modify_cell(mirror_x, buf_y, |cell| {
                            cell.symbol = if is_filled { "█" } else { "░" }.to_string();
                            cell.set_style(style);
                        });
                    }
                }
            }
        }

        let pct_text = format!("{:.0}%", percent * 100.0);
        let label_text = self.label.as_deref().unwrap_or(&pct_text);
        let label_x = center_x.saturating_sub(label_text.len() as u16 / 2);
        let label_y = center_y;

        for (i, ch) in label_text.chars().enumerate() {
            let x = label_x + i as u16;
            if x < area.x + area.width {
                buf.modify_cell(x, label_y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(self.style);
                });
            }
        }
    }
}

/// A linear (horizontal) progress bar widget.
///
/// Renders a horizontal bar filled from left to right based on the value ratio.
/// Useful for download progress, loading states, or any sequential progress.
///
/// # Example
///
/// \`\`\`rust
/// use ctui_components::LinearGauge;
/// use ctui_core::style::{Style, Color};
///
/// let progress = LinearGauge::new()
///     .value(750.0)
///     .max(1000.0)
///     .show_percent(true)
///     .filled_style(Style::new().fg(Color::Blue));
/// \`\`\`
#[derive(Clone, Debug, Default)]
pub struct LinearGauge {
    /// Current value.
    value: f64,
    /// Maximum value.
    max: f64,
    /// Optional label.
    label: Option<String>,
    /// Style for unfilled portion.
    style: Style,
    /// Style for filled portion.
    filled_style: Style,
    /// Whether to show percentage label.
    show_percent: bool,
}

impl LinearGauge {
    /// Creates a new linear gauge at 0%.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::LinearGauge;
    ///
    /// let gauge = LinearGauge::new();
    /// assert_eq!(gauge.percent(), 0.0);
    /// \`\`\`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the current value.
    ///
    /// # Arguments
    ///
    /// * \`value\` - The current value.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::LinearGauge;
    ///
    /// let gauge = LinearGauge::new().value(50.0);
    /// \`\`\`
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    /// Sets the maximum value (100% point).
    ///
    /// # Arguments
    ///
    /// * \`max\` - The maximum value.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::LinearGauge;
    ///
    /// let gauge = LinearGauge::new().max(200.0).value(100.0);
    /// assert_eq!(gauge.percent(), 0.5);
    /// \`\`\`
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Sets the label displayed below the bar.
    ///
    /// Only displayed if \`show_percent\` is \`false\` and area has height > 1.
    ///
    /// # Arguments
    ///
    /// * \`label\` - The label text.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::LinearGauge;
    ///
    /// let gauge = LinearGauge::new().label("Downloading...");
    /// \`\`\`
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the style for the unfilled portion.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The [`Style`] for empty areas.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::LinearGauge;
    /// use ctui_core::style::{Style, Color};
    ///
    /// let gauge = LinearGauge::new().style(Style::new().fg(Color::DarkGray));
    /// \`\`\`
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style for the filled portion.
    ///
    /// # Arguments
    ///
    /// * \`style\` - The [`Style`] for filled areas.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::LinearGauge;
    /// use ctui_core::style::{Style, Color};
    ///
    /// let gauge = LinearGauge::new()
    ///     .filled_style(Style::new().fg(Color::Green));
    /// \`\`\`
    pub fn filled_style(mut self, style: Style) -> Self {
        self.filled_style = style;
        self
    }

    /// Whether to display the percentage below the bar.
    ///
    /// When \`true\`, shows "XX%" instead of the custom label.
    ///
    /// # Arguments
    ///
    /// * \`show\` - \`true\` to show percentage, \`false\` for label.
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::LinearGauge;
    ///
    /// let gauge = LinearGauge::new()
    ///     .value(75.0)
    ///     .show_percent(true);  // Shows "75%"
    /// \`\`\`
    pub fn show_percent(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }

    /// Returns the current value as a percentage (0.0 to 1.0).
    ///
    /// # Example
    ///
    /// \`\`\`rust
    /// use ctui_components::LinearGauge;
    ///
    /// let gauge = LinearGauge::new().value(30.0).max(100.0);
    /// assert_eq!(gauge.percent(), 0.3);
    /// \`\`\`
    pub fn percent(&self) -> f64 {
        if self.max == 0.0 {
            0.0
        } else {
            self.value / self.max
        }
    }
}

impl Widget for LinearGauge {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let percent = self.percent().clamp(0.0, 1.0);
        let filled_width = ((area.width as f64) * percent) as u16;

        for x in 0..area.width {
            let buf_x = area.x + x;
            buf.modify_cell(buf_x, area.y, |cell| {
                if x < filled_width {
                cell.symbol = "█".to_string();
                cell.set_style(self.filled_style);
                } else {
                cell.symbol = "░".to_string();
                cell.set_style(self.style);
                }
            });
        }

        let label = if self.show_percent {
            format!("{:.0}%", percent * 100.0)
        } else if let Some(ref l) = self.label {
            l.clone()
        } else {
            String::new()
        };

        if !label.is_empty() && area.height > 1 {
            let label_x = area.x + (area.width.saturating_sub(label.len() as u16)) / 2;
            let label_y = area.y + 1;

            for (i, ch) in label.chars().enumerate() {
                let x_pos = label_x + i as u16;
                if x_pos < area.x + area.width {
                    buf.modify_cell(x_pos, label_y, |cell| {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.style);
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WidgetExt;
    use ctui_core::style::Color;
    use insta::assert_snapshot;

    #[test]
    fn test_gauge_empty() {
        let gauge = Gauge::new().value(0.0);
        let result = gauge.render_to_string(15, 8);
        assert_snapshot!("gauge_empty", result);
    }

    #[test]
    fn test_gauge_half() {
        let gauge = Gauge::new()
            .value(50.0)
            .max(100.0)
            .gauge_style(Style::new().fg(Color::Green));
        let result = gauge.render_to_string(15, 8);
        assert_snapshot!("gauge_half", result);
    }

    #[test]
    fn test_gauge_full() {
        let gauge = Gauge::new().value(100.0).max(100.0);
        let result = gauge.render_to_string(15, 8);
        assert_snapshot!("gauge_full", result);
    }

    #[test]
    fn test_gauge_with_label() {
        let gauge = Gauge::new().value(75.0).max(100.0).label("Speed");
        let result = gauge.render_to_string(15, 8);
        assert_snapshot!("gauge_with_label", result);
    }

    #[test]
    fn test_linear_gauge() {
        let gauge = LinearGauge::new().value(60.0).max(100.0).show_percent(true);
        let result = gauge.render_to_string(20, 2);
        assert_snapshot!("linear_gauge", result);
    }
}
