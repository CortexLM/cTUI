//! cTUI Components - Built-in UI components
//!
//! This crate provides pre-built UI components for the cTUI framework.
//!
//! # Components
//!
//! - [`Block`] - Container with borders, padding, and optional title
//! - [`Input`] - Single-line text input with cursor and editing
//! - [`List`] - Scrollable list with selection and keyboard navigation
//! - [`ListItem`] - Individual item in a list
//! - [`Paragraph`] - Multi-line text rendering with alignment and wrapping
//! - [`Table`] - Tabular data rendering with columns and rows
//! - [`Text`] - Multi-line text content type
//! - [`Line`] - Single line of styled text
//! - [`Alignment`] - Text alignment options
//! - [`ProgressBar`] - Progress bar with percentage display
//! - [`Spinner`] - Animated loading spinner
//! - [`Chart`] - ASCII chart for data visualization
//! - [`Form`] - Form with fields and validation
//! - [`Form`] - Form field with validation support
//! - [`Scrollable`] - Scrollable region with scrollbars
//! - [`Tabs`] - Tabbed navigation component
//! - [`Modal`] - Modal/dialog overlay component
//!
//! # Widget Trait
//!
//! The [`Widget`] trait provides a simple interface for stateless rendering.
//! Unlike the full `Component` trait, `Widget` focuses solely on rendering
//! without state management or message handling.

// Suppress pedantic lints
#![allow(missing_docs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::use_self)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::type_complexity)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::double_must_use)]
#![allow(clippy::float_cmp)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::unused_self)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::unnecessary_min_or_max)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::if_not_else)]
#![allow(clippy::while_let_on_iterator)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::manual_strip)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::write_with_newline)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::unused_peekable)]
#![allow(clippy::unused_enumerate_index)]
#![allow(clippy::single_match)]
#![allow(clippy::single_match_else)]
#![allow(clippy::redundant_clone)]
#![allow(clippy::needless_collect)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::iter_without_into_iter)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::explicit_counter_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::branches_sharing_code)]
use ctui_core::{Buffer, Rect};

/// A trait for stateless widgets that can render themselves.
///
/// `Widget` is a simplified rendering interface compared to `Component`.
/// It's useful for widgets that don't need to manage internal state or
/// handle messages - they just need to render to a buffer.
///
/// # Example
///
/// ```
/// use ctui_components::Widget;
/// use ctui_core::{Buffer, Rect};
///
/// struct HelloWorld;
///
/// impl Widget for HelloWorld {
///     fn render(&self, area: Rect, buf: &mut Buffer) {
///         buf.modify_cell(area.x, area.y, |cell| {
///             cell.symbol = "Hello, World!".to_string();
///         });
///     }
/// }
/// ```
pub trait Widget {
    /// Renders the widget to the given buffer area.
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangular area allocated for this widget
    /// * `buf` - The buffer to render into
    fn render(&self, area: Rect, buf: &mut Buffer);
}

/// Extension trait for rendering widgets.
pub trait WidgetExt: Widget + Sized {
    /// Renders this widget into a new buffer and returns it.
    fn to_buffer(&self, width: u16, height: u16) -> Buffer {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        self.render(area, &mut buf);
        buf
    }

    /// Renders this widget to a string for testing.
    fn render_to_string(&self, width: u16, height: u16) -> String {
        let buf = self.to_buffer(width, height);
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
}

impl<W: Widget> WidgetExt for W {}

pub mod block;
pub mod borders;
pub mod canvas;
pub mod chart;
pub mod checkbox;
pub mod code;
pub mod diff;
pub mod editor;
pub mod form;
pub mod gauge;
pub mod input;
pub mod link;
pub mod list;
pub mod markdown;
pub mod modal;
pub mod mouse_trail;
pub mod padding;
pub mod paragraph;
pub mod progress;
pub mod radio;
pub mod scrollable;
pub mod select;
pub mod slider;
pub mod sparkline;
pub mod tab_controller;
pub mod table;
pub mod tabs;
pub mod text;
pub mod tree;

pub use block::{Alignment, Block, PositionedTitle, Title, TitlePosition};
pub use borders::{BorderType, Borders};
pub use canvas::{Canvas, Point, Shape};
pub use chart::{BarOrientation, Chart, ChartProps, ChartType, DataPoint};
pub use checkbox::{Checkbox, CheckboxGroup};
pub use code::{Code, CodeProps, CodeTheme, DiffMarker, Language, Token, TokenKind};
pub use diff::{DiffAlgorithm, DiffHunk, DiffLine, DiffMode, DiffViewer, DiffViewerProps};
pub use editor::{Editor, EditorProps, EditorState, Selection, Textarea};
pub use form::{FieldType, Form, FormField, FormProps};
pub use gauge::{Gauge, LinearGauge};
pub use input::{Input, InputProps, InputState};
pub use link::{Link, LinkProps};
pub use list::{List, ListItem, ListProps, SelectionMode};
pub use markdown::{parse_markdown, Inline, Markdown, MarkdownNode, MarkdownTheme};
pub use modal::{Modal, ModalAction, ModalAlignment, ModalButton, ModalProps, ModalSize};
pub use mouse_trail::{MouseTrail, MouseTrailBuilder};
pub use padding::Padding;
pub use paragraph::{Paragraph, ParagraphProps};
pub use progress::{ProgressBar, ProgressBarProps, Spinner, SpinnerProps, SpinnerStyle};
pub use radio::{RadioGroup, RadioItem};
pub use scrollable::{Scrollable, ScrollableProps, ScrollbarVisibility};
pub use select::{ComboBox, Select, SelectItem};
pub use slider::{Orientation, Slider};
pub use sparkline::{BarSparkline, Sparkline};
pub use tab_controller::{TabContent, TabController, TabControllerConfig, TabEntry, TabLifecycle};
pub use table::{Cell, Column, Row, SortOrder, Table, TableProps};
pub use tabs::{Tab, TabAlignment, Tabs, TabsProps};
pub use text::{Alignment as TextAlignment, Line, Span, Text};
pub use tree::{Tree, TreeNode};
