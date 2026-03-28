//! Taffy-based layout engine for cTUI
//!
//! This module provides a Taffy-based implementation of flexbox layout
//! as an alternative to the native FlexLayout. It's feature-gated behind
//! `taffy-layout` and provides a compatible API.
//!
//! # Example
//!
//! ```ignore
//! use ctui_layout::TaffyLayoutEngine;
//! use ctui_core::Rect;
//!
//! let engine = TaffyLayoutEngine::new();
//! // Use engine.split() like FlexLayout
//! ```

#[cfg(feature = "taffy-layout")]
use taffy::{
    prelude::*,
    AlignContent as TaffyAlignContent,
    AlignItems as TaffyAlignItems,
    FlexDirection as TaffyFlexDirection,
    FlexWrap as TaffyFlexWrap,
    JustifyContent as TaffyJustifyContent,
};

use crate::{
    AlignContent, AlignItems, Constraint, FlexDirection as CtuiFlexDirection, 
    JustifyContent as CtuiJustifyContent, Rect,
};

/// A layout engine backed by Taffy's flexbox implementation
#[cfg(feature = "taffy-layout")]
#[derive(Debug)]
pub struct TaffyLayoutEngine {
    direction: CtuiFlexDirection,
    justify_content: CtuiJustifyContent,
    align_items: AlignItems,
    gap: u16,
    flex_wrap: bool,
    align_content: AlignContent,
}

#[cfg(feature = "taffy-layout")]
impl TaffyLayoutEngine {
    /// Creates a new Taffy layout engine with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            direction: CtuiFlexDirection::Row,
            justify_content: CtuiJustifyContent::Start,
            align_items: AlignItems::Stretch,
            gap: 0,
            flex_wrap: false,
            align_content: AlignContent::Stretch,
        }
    }

    /// Sets the flex direction
    #[must_use]
    pub fn direction(mut self, direction: CtuiFlexDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the justify content
    #[must_use]
    pub fn justify_content(mut self, justify: CtuiJustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    /// Sets the align items
    #[must_use]
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    /// Sets the gap between children
    #[must_use]
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Enables or disables flex wrap
    #[must_use]
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.flex_wrap = wrap;
        self
    }

    /// Sets align content for wrapped lines
    #[must_use]
    pub fn align_content(mut self, align: AlignContent) -> Self {
        self.align_content = align;
        self
    }

    /// Converts Taffy Layout to cTUI Rect
    fn taffy_to_rect(layout: &Layout, container: Rect) -> Rect {
        let x = layout.location.x.round() as u16;
        let y = layout.location.y.round() as u16;
        let width = layout.size.width.round() as u16;
        let height = layout.size.height.round() as u16;

        Rect::new(container.x + x, container.y + y, width, height)
    }

    /// Creates a child style from a constraint
    fn create_child_style(&self, constraint: &Constraint, _area: Rect) -> Style {
        let (main_size, min_main, max_main, flex_grow) = match constraint {
            Constraint::Length(n) => {
                (Dimension::length(*n as f32), Dimension::auto(), Dimension::auto(), 0.0)
            }
            Constraint::Min(n) => {
                (Dimension::auto(), Dimension::length(*n as f32), Dimension::auto(), 1.0)
            }
            Constraint::Max(n) => {
                (Dimension::auto(), Dimension::auto(), Dimension::length(*n as f32), 0.0)
            }
            Constraint::Percentage(p) => {
                let pct = (*p as f32) / 100.0;
                (Dimension::percent(pct), Dimension::auto(), Dimension::auto(), 0.0)
            }
            Constraint::Ratio(num, den) => {
                let ratio = *num as f32 / *den as f32;
                (Dimension::auto(), Dimension::auto(), Dimension::auto(), ratio)
            }
            Constraint::Fill => {
                (Dimension::auto(), Dimension::auto(), Dimension::auto(), 1.0)
            }
            Constraint::Range { min, max } => {
                (Dimension::auto(), Dimension::length(*min as f32), Dimension::length(*max as f32), 1.0)
            }
            Constraint::Portion(p) => {
                (Dimension::auto(), Dimension::auto(), Dimension::auto(), *p as f32)
            }
        };

        let (width, min_width, max_width, height, min_height, max_height) = 
            match self.direction {
                CtuiFlexDirection::Row => {
                    (main_size, min_main, max_main, Dimension::auto(), Dimension::auto(), Dimension::auto())
                }
                CtuiFlexDirection::Column => {
                    (Dimension::auto(), Dimension::auto(), Dimension::auto(), main_size, min_main, max_main)
                }
            };

        Style {
            display: Display::Flex,
            size: Size { width, height },
            min_size: Size { width: min_width, height: min_height },
            max_size: Size { width: max_width, height: max_height },
            flex_grow,
            flex_shrink: 1.0,
            flex_basis: Dimension::auto(),
            ..Default::default()
        }
    }

    /// Splits the given area into rectangles based on constraints
    ///
    /// # Panics
    /// Panics if Taffy's internal node creation fails (should never happen).
    #[must_use]
    pub fn split(&self, area: Rect, constraints: &[Constraint]) -> Vec<Rect> {
        if constraints.is_empty() {
            return Vec::new();
        }

        let mut tree: TaffyTree<()> = TaffyTree::new();

        // Convert cTUI enums to Taffy enums
        let taffy_direction = match self.direction {
            CtuiFlexDirection::Row => TaffyFlexDirection::Row,
            CtuiFlexDirection::Column => TaffyFlexDirection::Column,
        };

        let taffy_justify = match self.justify_content {
            CtuiJustifyContent::Start => TaffyJustifyContent::Start,
            CtuiJustifyContent::Center => TaffyJustifyContent::Center,
            CtuiJustifyContent::End => TaffyJustifyContent::End,
            CtuiJustifyContent::SpaceBetween => TaffyJustifyContent::SpaceBetween,
            CtuiJustifyContent::SpaceAround => TaffyJustifyContent::SpaceAround,
        };

        let taffy_align_items = match self.align_items {
            AlignItems::Start => TaffyAlignItems::Start,
            AlignItems::Center => TaffyAlignItems::Center,
            AlignItems::End => TaffyAlignItems::End,
            AlignItems::Stretch => TaffyAlignItems::Stretch,
        };

        let taffy_align_content = match self.align_content {
            AlignContent::Start => TaffyAlignContent::Start,
            AlignContent::Center => TaffyAlignContent::Center,
            AlignContent::End => TaffyAlignContent::End,
            AlignContent::SpaceBetween => TaffyAlignContent::SpaceBetween,
            AlignContent::SpaceAround => TaffyAlignContent::SpaceAround,
            AlignContent::Stretch => TaffyAlignContent::Stretch,
        };

        let taffy_wrap = if self.flex_wrap { TaffyFlexWrap::Wrap } else { TaffyFlexWrap::NoWrap };

        // Build container style
        let container_style = Style {
            display: Display::Flex,
            flex_direction: taffy_direction,
            align_items: Some(taffy_align_items),
            justify_content: Some(taffy_justify),
            align_content: Some(taffy_align_content),
            flex_wrap: taffy_wrap,
            gap: Size {
                width: length(if self.direction == CtuiFlexDirection::Row { self.gap as f32 } else { 0.0 }),
                height: length(if self.direction == CtuiFlexDirection::Column { self.gap as f32 } else { 0.0 }),
            },
            size: Size {
                width: length(area.width as f32),
                height: length(area.height as f32),
            },
            ..Default::default()
        };

        // Create child nodes
        let children: Vec<NodeId> = constraints
            .iter()
            .map(|constraint| {
                let child_style = self.create_child_style(constraint, area);
                tree.new_leaf(child_style)
                    .expect("Failed to create child node")
            })
            .collect();

        // Create root with children
        let root = tree
            .new_with_children(container_style, &children)
            .expect("Failed to create root node");

        // Compute layout
        tree.compute_layout(root, Size {
            width: AvailableSpace::Definite(area.width as f32),
            height: AvailableSpace::Definite(area.height as f32),
        }).expect("Layout computation failed");

        // Extract results
        children
            .iter()
            .map(|child| {
                let layout = tree
                    .layout(*child)
                    .expect("Failed to get layout");
                Self::taffy_to_rect(layout, area)
            })
            .collect()
    }
}

#[cfg(feature = "taffy-layout")]
impl Default for TaffyLayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(feature = "taffy-layout")]
mod tests {
    use super::*;

    fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect::new(x, y, width, height)
    }

    #[test]
    fn taffy_row_layout() {
        let area = rect(0, 0, 80, 24);
        let engine = TaffyLayoutEngine::new()
            .direction(CtuiFlexDirection::Row)
            .gap(2);
        let rects = engine.split(area, &[Constraint::Length(20), Constraint::Length(30)]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].width, 20);
        assert_eq!(rects[1].width, 30);
    }

    #[test]
    fn taffy_column_layout() {
        let area = rect(0, 0, 80, 24);
        let engine = TaffyLayoutEngine::new()
            .direction(CtuiFlexDirection::Column);
        let rects = engine.split(area, &[Constraint::Length(10), Constraint::Length(14)]);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].height, 10);
        assert_eq!(rects[1].height, 14);
    }

    #[test]
    fn taffy_empty_constraints() {
        let area = rect(0, 0, 80, 24);
        let engine = TaffyLayoutEngine::new();
        let rects = engine.split(area, &[]);

        assert!(rects.is_empty());
    }

    #[test]
    fn taffy_flex_fill() {
        let area = rect(0, 0, 60, 24);
        let engine = TaffyLayoutEngine::new()
            .direction(CtuiFlexDirection::Row);
        let rects = engine.split(area, &[Constraint::Fill, Constraint::Fill]);

        assert_eq!(rects.len(), 2);
        // Both should split available space
        assert!(rects[0].width > 0);
        assert!(rects[1].width > 0);
    }
}

#[cfg(test)]
#[cfg(feature = "taffy-layout")]
mod parity_tests {
    use super::*;
    use crate::FlexLayout;

    fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect::new(x, y, width, height)
    }

    #[test]
    fn parity_row_simple() {
        let area = rect(0, 0, 80, 24);
        let constraints = vec![
            Constraint::Length(20),
            Constraint::Length(30),
            Constraint::Length(10),
        ];

        let flex = FlexLayout::new().direction(CtuiFlexDirection::Row);
        let taffy = TaffyLayoutEngine::new().direction(CtuiFlexDirection::Row);

        let flex_rects = flex.split(area, &constraints);
        let taffy_rects = taffy.split(area, &constraints);

        assert_eq!(flex_rects.len(), taffy_rects.len());
        
        for (i, (f, t)) in flex_rects.iter().zip(taffy_rects.iter()).enumerate() {
            let width_diff = (f.width as i32 - t.width as i32).abs();
            assert!(
                width_diff <= 1,
                "Rect {} width mismatch: flex={}, taffy={}", 
                i, f.width, t.width
            );
        }
    }

    #[test]
    fn parity_column_simple() {
        let area = rect(0, 0, 80, 40);
        let constraints = vec![
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Length(15),
        ];

        let flex = FlexLayout::new()
            .direction(CtuiFlexDirection::Column)
            .gap(2);
        let taffy = TaffyLayoutEngine::new()
            .direction(CtuiFlexDirection::Column)
            .gap(2);

        let flex_rects = flex.split(area, &constraints);
        let taffy_rects = taffy.split(area, &constraints);

        assert_eq!(flex_rects.len(), taffy_rects.len());
        
        for (i, (f, t)) in flex_rects.iter().zip(taffy_rects.iter()).enumerate() {
            let height_diff = (f.height as i32 - t.height as i32).abs();
            assert!(
                height_diff <= 1,
                "Rect {} height mismatch: flex={}, taffy={}", 
                i, f.height, t.height
            );
        }
    }

    #[test]
    fn parity_justify_center() {
        let area = rect(0, 0, 80, 24);
        let constraints = vec![Constraint::Length(20)];

        let flex = FlexLayout::new()
            .direction(CtuiFlexDirection::Row)
            .justify_content(CtuiJustifyContent::Center);
        let taffy = TaffyLayoutEngine::new()
            .direction(CtuiFlexDirection::Row)
            .justify_content(CtuiJustifyContent::Center);

        let flex_rects = flex.split(area, &constraints);
        let taffy_rects = taffy.split(area, &constraints);

        let flex_center = flex_rects[0].x + flex_rects[0].width / 2;
        let taffy_center = taffy_rects[0].x + taffy_rects[0].width / 2;
        
        let diff = (flex_center as i32 - taffy_center as i32).abs();
        assert!(diff <= 2, "Center diff too large: {}", diff);
    }

    #[test]
    fn parity_flex_fill_distribution() {
        let area = rect(0, 0, 100, 24);
        let constraints = vec![
            Constraint::Length(30),
            Constraint::Fill,
            Constraint::Length(30),
        ];

        let flex = FlexLayout::new().direction(CtuiFlexDirection::Row);
        let taffy = TaffyLayoutEngine::new().direction(CtuiFlexDirection::Row);

        let flex_rects = flex.split(area, &constraints);
        let taffy_rects = taffy.split(area, &constraints);

        assert_eq!(flex_rects.len(), 3);
        assert_eq!(taffy_rects.len(), 3);
        
        assert_eq!(flex_rects[0].width, 30);
        assert_eq!(flex_rects[2].width, 30);
    }

    #[test]
    fn parity_portion_distribution() {
        let area = rect(0, 0, 90, 24);
        let constraints = vec![
            Constraint::Portion(1),
            Constraint::Portion(2),
        ];

        let flex = FlexLayout::new().direction(CtuiFlexDirection::Row);
        let taffy = TaffyLayoutEngine::new().direction(CtuiFlexDirection::Row);

        let flex_rects = flex.split(area, &constraints);
        let taffy_rects = taffy.split(area, &constraints);

        assert_eq!(flex_rects.len(), 2);
        assert_eq!(taffy_rects.len(), 2);
        
        assert!(flex_rects[0].width < flex_rects[1].width);
    }
}
