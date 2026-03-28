//! Tree widget for hierarchical data visualization.
//!
//! This module provides a [`Tree`] widget for displaying hierarchical
//! data structures with expandable/collapsible nodes. Ideal for file
//! explorers, navigation menus, and nested data visualization.
//!
//! # Features
//!
//! - Nested nodes with unlimited depth
//! - Expand/collapse toggle for each node
//! - Keyboard navigation support
//! - Customizable indentation and guides
//! - Selection highlighting
//!
//! # Example
//!
//! ```ignore
//! use ctui_components::{Tree, TreeNode};
//!
//! let tree = Tree::new().node(
//!     TreeNode::new("Root")
//!         .expanded(true)
//!         .child(TreeNode::new("Child 1"))
//!         .child(TreeNode::new("Child 2"))
//! );
//! ```

use crate::text::Line;
use crate::Widget;
use ctui_core::style::Style;
use ctui_core::{Buffer, Rect};

/// A single node in a [`Tree`] hierarchy.
///
/// Each node contains text content and can have child nodes.
/// Nodes can be expanded or collapsed to show/hide their children.
///
/// # Example
///
/// ```ignore
/// let child = TreeNode::new("Child Item");
/// let parent = TreeNode::new("Parent")
///     .expanded(true)
///     .child(child);
/// ```
#[derive(Clone, Debug)]
pub struct TreeNode {
    content: Line,
    children: Vec<TreeNode>,
    expanded: bool,
}

impl TreeNode {
    /// Creates a new tree node with the given content.
    pub fn new(content: impl Into<Line>) -> Self {
        Self {
            content: content.into(),
            children: Vec::new(),
            expanded: false,
        }
    }

    /// Adds a child node to this node.
    ///
    /// Returns self for chaining multiple children.
    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    /// Sets all children at once.
    ///
    /// Replaces any existing children.
    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    /// Sets whether this node is expanded.
    ///
    /// When expanded, children are visible.
    /// When collapsed, children are hidden.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Toggles the expanded state of this node.
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }

    /// Returns true if this node has no children.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns true if this node has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Returns a reference to the node's content.
    pub fn content(&self) -> &Line {
        &self.content
    }

    /// Returns a reference to the children slice.
    pub fn children_ref(&self) -> &[TreeNode] {
        &self.children
    }

    /// Returns a mutable reference to the children vector.
    pub fn children_mut(&mut self) -> &mut Vec<TreeNode> {
        &mut self.children
    }
}

/// A widget for displaying hierarchical tree structures.
///
/// The `Tree` widget renders a collection of [`TreeNode`] items,
/// with support for expanding/collapsing branches and selection.
///
/// # Layout
///
/// The tree uses indentation to show hierarchy levels.
/// Expanded parent nodes show their children below them.
/// Collapsed nodes hide their children.
///
/// # Example
///
/// ```ignore
/// let tree = Tree::new()
///     .indent(2)
///     .show_guides(true)
///     .node(TreeNode::new("Root")
///         .expanded(true)
///         .child(TreeNode::new("Branch A"))
///         .child(TreeNode::new("Branch B")));
/// ```
#[derive(Clone, Debug, Default)]
pub struct Tree {
    nodes: Vec<TreeNode>,
    style: Style,
    highlight_style: Style,
    selected: Option<usize>,
    indent: u16,
    show_guides: bool,
}

impl Tree {
    /// Creates a new empty tree widget.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a root-level node to the tree.
    pub fn node(mut self, node: TreeNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// Sets all root-level nodes at once.
    pub fn nodes(mut self, nodes: Vec<TreeNode>) -> Self {
        self.nodes = nodes;
        self
    }

    /// Sets the base style for the tree.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style for selected/highlighted nodes.
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Sets the indentation width in cells for each level.
    ///
    /// Default is typically 2 spaces per level.
    pub fn indent(mut self, indent: u16) -> Self {
        self.indent = indent;
        self
    }

    /// Sets whether to show guide lines connecting nodes.
    pub fn show_guides(mut self, show: bool) -> Self {
        self.show_guides = show;
        self
    }

    /// Sets the selected node by flat index.
    ///
    /// The flat index counts all visible nodes in depth-first order.
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    /// Returns the currently selected node index, if any.
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Toggles the expanded state of the selected node.
    pub fn toggle_selected(&mut self) {
        if let Some(idx) = self.selected {
            self.toggle_node_at_index(idx);
        }
    }

    /// Internal helper to toggle a node by its flat index.
    fn toggle_node_at_index(&mut self, target_idx: usize) {
        fn toggle_recursive(nodes: &mut [TreeNode], target: usize, current: &mut usize) -> bool {
            for node in nodes.iter_mut() {
                if *current == target {
                    node.toggle();
                    return true;
                }
                *current += 1;
                if node.expanded {
                    if toggle_recursive(&mut node.children, target, current) {
                        return true;
                    }
                }
            }
            false
        }

        let mut current_idx = 0;
        toggle_recursive(&mut self.nodes, target_idx, &mut current_idx);
    }

    /// Flattens the visible tree nodes into a linear list.
    ///
    /// Returns (flat_index, node_reference, depth) tuples.
    fn flatten_nodes<'a>(
        nodes: &'a [TreeNode],
        result: &mut Vec<(usize, &'a TreeNode, u16)>,
        depth: u16,
    ) {
        for node in nodes {
            result.push((result.len(), node, depth));
            if node.expanded {
                Self::flatten_nodes(&node.children, result, depth + 1);
            }
        }
    }

    /// Gets a mutable reference to a node by flat index.
    fn get_node_at_index(&mut self, target_idx: Option<usize>) -> Option<&mut TreeNode> {
        let target_idx = target_idx?;

        fn find_recursive<'a>(
            nodes: &'a mut [TreeNode],
            target: usize,
            current: &mut usize,
        ) -> Option<&'a mut TreeNode> {
            for node in nodes.iter_mut() {
                if *current == target {
                    return Some(node);
                }
                *current += 1;
                if node.expanded {
                    if let Some(found) = find_recursive(&mut node.children, target, current) {
                        return Some(found);
                    }
                }
            }
            None
        }

        let mut current_idx = 0;
        find_recursive(&mut self.nodes, target_idx, &mut current_idx)
    }

    /// Converts a selection to a flat index.
    fn find_node_index(&self, selection: Option<usize>) -> Option<usize> {
        selection
    }

    /// Renders a single node at the given position.
    fn render_node(
        &self,
        node: &TreeNode,
        y: u16,
        depth: u16,
        area: Rect,
        buf: &mut Buffer,
        is_selected: bool,
    ) {
        let indent_x = area.x + depth * self.indent;
        let style = if is_selected {
            self.highlight_style
        } else {
            self.style
        };

        let prefix = if node.has_children() {
            if node.expanded {
                "▼ "
            } else {
                "▶ "
            }
        } else {
            "  "
        };

        let prefix_chars: Vec<char> = prefix.chars().collect();
        for (i, ch) in prefix_chars.iter().enumerate() {
            let x = indent_x + i as u16;
            if x < area.x + area.width {
                buf.modify_cell(x, y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(style);
                });
            }
        }

        let content_x = indent_x + 3;
        let styled_chars = node.content.styled_chars();
        for (i, (ch, char_style)) in styled_chars.iter().enumerate() {
            let x = content_x + i as u16;
            if x >= area.x + area.width {
                break;
            }
            buf.modify_cell(x, y, |cell| {
                cell.symbol = ch.to_string();
                cell.set_style(*char_style);
            });
        }
    }
}

impl Widget for Tree {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let mut flattened: Vec<(usize, &TreeNode, u16)> = Vec::new();
        Self::flatten_nodes(&self.nodes, &mut flattened, 0);

        for (i, (idx, node, depth)) in flattened.iter().enumerate() {
            let y = area.y + i as u16;
            if y >= area.y + area.height {
                break;
            }

            let is_selected = self.selected == Some(*idx);
            self.render_node(node, y, *depth, area, buf, is_selected);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WidgetExt;
    use insta::assert_snapshot;

    #[test]
    fn test_tree_empty() {
        let tree = Tree::new();
        let result = tree.render_to_string(15, 5);
        assert_snapshot!("tree_empty", result);
    }

    #[test]
    fn test_tree_single_node() {
        let tree = Tree::new().node(TreeNode::new("Root"));
        let result = tree.render_to_string(15, 5);
        assert_snapshot!("tree_single", result);
    }

    #[test]
    fn test_tree_nested() {
        let tree = Tree::new().node(
            TreeNode::new("Root")
                .expanded(true)
                .child(TreeNode::new("Child 1"))
                .child(TreeNode::new("Child 2"))
                .child(
                    TreeNode::new("Child 3")
                        .expanded(true)
                        .child(TreeNode::new("Grandchild")),
                ),
        );
        let result = tree.render_to_string(20, 8);
        assert_snapshot!("tree_nested", result);
    }

    #[test]
    fn test_tree_collapsed() {
        let tree = Tree::new().node(
            TreeNode::new("Root")
                .expanded(false)
                .child(TreeNode::new("Hidden Child 1"))
                .child(TreeNode::new("Hidden Child 2")),
        );
        let result = tree.render_to_string(20, 5);
        assert_snapshot!("tree_collapsed", result);
    }
}
