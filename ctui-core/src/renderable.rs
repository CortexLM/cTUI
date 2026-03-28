//! Renderable hierarchy for managing parent-child relationships and z-index ordering.

use crate::buffer::Buffer;
use crate::geometry::{Position, Rect, Size};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;

/// Type alias for z-index values
pub type ZIndex = i32;

/// Default z-index for elements without explicit z-index
pub const DEFAULT_Z_INDEX: ZIndex = 0;

/// Trait for objects that can be rendered in a scene graph.
pub trait Renderable: Any {
    fn id(&self) -> &str;
    fn parent(&self) -> Option<Weak<RefCell<dyn Renderable>>>;
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Renderable>>>);
    fn children(&self) -> &[Box<dyn Renderable>];
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Renderable>>;
    fn z_index(&self) -> i32;
    fn visible(&self) -> bool;
    fn position(&self) -> Position;
    fn size(&self) -> Size;
    fn absolute(&self) -> bool {
        false
    }
    fn add(&mut self, child: Box<dyn Renderable>);
    fn remove(&mut self, id: &str) -> bool;
    fn get(&self, id: &str) -> Option<&dyn Renderable>;
    fn get_mut(&mut self, id: &str) -> Option<&mut dyn Renderable>;
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn hit_test(&self, x: u16, y: u16) -> Vec<&dyn Renderable>;
    fn bounds(&self, parent_offset: Option<Position>) -> Rect;
}

impl dyn Renderable {
    pub fn is<T: Renderable>(&self) -> bool {
        self.type_id() == std::any::TypeId::of::<T>()
    }

    pub fn downcast_ref<T: Renderable>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Renderable>(&mut self) -> Option<&mut T> {
        (self as &mut dyn Any).downcast_mut::<T>()
    }
}

/// A concrete renderable object with position, size, and z-index.
pub struct RenderObject {
    id: String,
    position: Position,
    size: Size,
    z_index: ZIndex,
    visible: bool,
    children: Vec<Box<dyn Renderable>>,
    parent: Option<Weak<RefCell<dyn Renderable>>>,
    absolute: bool,
    render_fn: Option<Box<dyn Fn(Rect, &mut Buffer)>>,
}

impl std::fmt::Debug for RenderObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderObject")
            .field("id", &self.id)
            .field("position", &self.position)
            .field("size", &self.size)
            .field("z_index", &self.z_index)
            .field("visible", &self.visible)
            .field("children_count", &self.children.len())
            .field("absolute", &self.absolute)
            .finish()
    }
}

impl RenderObject {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            position: Position::origin(),
            size: Size::zero(),
            z_index: DEFAULT_Z_INDEX,
            visible: true,
            children: Vec::new(),
            parent: None,
            absolute: false,
            render_fn: None,
        }
    }

    #[must_use]
    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    #[must_use]
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn with_z_index(mut self, z_index: ZIndex) -> Self {
        self.z_index = z_index;
        self
    }

    #[must_use]
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    #[must_use]
    pub fn with_absolute(mut self, absolute: bool) -> Self {
        self.absolute = absolute;
        self
    }

    pub fn with_render_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(Rect, &mut Buffer) + 'static,
    {
        self.render_fn = Some(Box::new(f));
        self
    }

    pub fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }

    pub fn set_z_index(&mut self, z_index: ZIndex) {
        self.z_index = z_index;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Renderable for RenderObject {
    fn id(&self) -> &str {
        &self.id
    }

    fn parent(&self) -> Option<Weak<RefCell<dyn Renderable>>> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Renderable>>>) {
        self.parent = parent;
    }

    fn children(&self) -> &[Box<dyn Renderable>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        &mut self.children
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn position(&self) -> Position {
        self.position
    }

    fn size(&self) -> Size {
        self.size
    }

    fn absolute(&self) -> bool {
        self.absolute
    }

    fn add(&mut self, child: Box<dyn Renderable>) {
        self.children.push(child);
    }

    fn remove(&mut self, id: &str) -> bool {
        let initial_len = self.children.len();
        self.children.retain(|child| child.id() != id);
        initial_len != self.children.len()
    }

    fn get(&self, id: &str) -> Option<&dyn Renderable> {
        if self.id == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.get(id) {
                return Some(found);
            }
        }
        None
    }

    fn get_mut(&mut self, id: &str) -> Option<&mut dyn Renderable> {
        if self.id == id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.get_mut(id) {
                return Some(found);
            }
        }
        None
    }

    fn bounds(&self, parent_offset: Option<Position>) -> Rect {
        let offset = if self.absolute || parent_offset.is_none() {
            Position::origin()
        } else {
            parent_offset.unwrap()
        };
        Rect::new(
            offset.x.saturating_add(self.position.x),
            offset.y.saturating_add(self.position.y),
            self.size.width,
            self.size.height,
        )
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }
        let absolute_pos = if self.absolute {
            Rect::new(
                self.position.x,
                self.position.y,
                self.size.width,
                self.size.height,
            )
        } else {
            Rect::new(
                area.x.saturating_add(self.position.x),
                area.y.saturating_add(self.position.y),
                self.size.width,
                self.size.height,
            )
        };
        if let Some(ref render_fn) = self.render_fn {
            render_fn(absolute_pos, buf);
        }
        let mut sorted_children: Vec<_> = self.children.iter().collect();
        sorted_children.sort_by_key(|c| c.z_index());
        for child in sorted_children {
            child.render(absolute_pos, buf);
        }
    }

    fn hit_test(&self, x: u16, y: u16) -> Vec<&dyn Renderable> {
        let mut results = Vec::new();
        self.hit_test_recursive(x, y, Position::origin(), &mut results);
        results.sort_by(|a, b| b.z_index().cmp(&a.z_index()));
        results
    }
}

impl RenderObject {
    fn hit_test_recursive<'a>(
        &'a self,
        x: u16,
        y: u16,
        parent_offset: Position,
        results: &mut Vec<&'a dyn Renderable>,
    ) {
        if !self.visible {
            return;
        }
        let offset = if self.absolute {
            Position::origin()
        } else {
            parent_offset
        };
        let bounds = Rect::new(
            offset.x.saturating_add(self.position.x),
            offset.y.saturating_add(self.position.y),
            self.size.width,
            self.size.height,
        );
        if x >= bounds.x
            && x < bounds.x.saturating_add(bounds.width)
            && y >= bounds.y
            && y < bounds.y.saturating_add(bounds.height)
        {
            results.push(self as &dyn Renderable);
        }
        for child in &self.children {
            child.hit_test_recursive(x, y, bounds.position(), results);
        }
    }
}

trait HitTestRecursive {
    fn hit_test_recursive<'a>(
        &'a self,
        x: u16,
        y: u16,
        parent_offset: Position,
        results: &mut Vec<&'a dyn Renderable>,
    );
}

impl HitTestRecursive for dyn Renderable {
    fn hit_test_recursive<'a>(
        &'a self,
        x: u16,
        y: u16,
        parent_offset: Position,
        results: &mut Vec<&'a dyn Renderable>,
    ) {
        if !self.visible() {
            return;
        }
        let offset = if self.absolute() {
            Position::origin()
        } else {
            parent_offset
        };
        let bounds = self.bounds(Some(offset));
        if x >= bounds.x
            && x < bounds.x.saturating_add(bounds.width)
            && y >= bounds.y
            && y < bounds.y.saturating_add(bounds.height)
        {
            results.push(self);
        }
        for child in self.children() {
            child.hit_test_recursive(x, y, bounds.position(), results);
        }
    }
}

/// A builder for creating renderable hierarchies.
pub struct RenderBuilder {
    id: String,
    position: Position,
    size: Size,
    z_index: ZIndex,
    visible: bool,
    absolute: bool,
    children: Vec<Box<dyn Renderable>>,
    render_fn: Option<Box<dyn Fn(Rect, &mut Buffer)>>,
}

impl RenderBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            position: Position::origin(),
            size: Size::zero(),
            z_index: DEFAULT_Z_INDEX,
            visible: true,
            absolute: false,
            children: Vec::new(),
            render_fn: None,
        }
    }

    #[must_use]
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn z_index(mut self, z_index: ZIndex) -> Self {
        self.z_index = z_index;
        self
    }

    #[must_use]
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    #[must_use]
    pub fn absolute(mut self, absolute: bool) -> Self {
        self.absolute = absolute;
        self
    }

    #[must_use]
    pub fn child(mut self, child: Box<dyn Renderable>) -> Self {
        self.children.push(child);
        self
    }

    pub fn render_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(Rect, &mut Buffer) + 'static,
    {
        self.render_fn = Some(Box::new(f));
        self
    }

    pub fn build(self) -> Box<dyn Renderable> {
        let mut obj = RenderObject::new(self.id)
            .with_position(self.position)
            .with_size(self.size)
            .with_z_index(self.z_index)
            .with_visible(self.visible)
            .with_absolute(self.absolute);
        for child in self.children {
            obj.add(child);
        }
        if let Some(render_fn) = self.render_fn {
            obj = obj.with_render_fn(render_fn);
        }
        Box::new(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_buffer() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 80, 24))
    }

    #[test]
    fn test_render_object_new() {
        let obj = RenderObject::new("test");
        assert_eq!(obj.id(), "test");
        assert_eq!(obj.z_index(), 0);
        assert!(obj.visible());
        assert!(!obj.absolute());
        assert_eq!(obj.position(), Position::origin());
        assert_eq!(obj.size(), Size::zero());
    }

    #[test]
    fn test_render_object_builder() {
        let obj = RenderObject::new("test")
            .with_position(Position::new(10, 20))
            .with_size(Size::new(30, 40))
            .with_z_index(5)
            .with_visible(false)
            .with_absolute(true);
        assert_eq!(obj.position(), Position::new(10, 20));
        assert_eq!(obj.size(), Size::new(30, 40));
        assert_eq!(obj.z_index(), 5);
        assert!(!obj.visible());
        assert!(obj.absolute());
    }

    #[test]
    fn test_render_object_mutable_setters() {
        let mut obj = RenderObject::new("test");
        obj.set_z_index(10);
        obj.set_visible(false);
        assert_eq!(obj.z_index(), 10);
        assert!(!obj.visible());
    }

    #[test]
    fn test_parent_child_relationship() {
        let mut parent = RenderObject::new("parent");
        let child = RenderObject::new("child");
        parent.add(Box::new(child));
        assert_eq!(parent.children().len(), 1);
        assert_eq!(parent.children()[0].id(), "child");
    }

    #[test]
    fn test_remove_child() {
        let mut parent = RenderObject::new("parent");
        parent.add(Box::new(RenderObject::new("child1")));
        parent.add(Box::new(RenderObject::new("child2")));
        assert_eq!(parent.children().len(), 2);
        let removed = parent.remove("child1");
        assert!(removed);
        assert_eq!(parent.children().len(), 1);
        assert_eq!(parent.children()[0].id(), "child2");
        let removed = parent.remove("nonexistent");
        assert!(!removed);
        assert_eq!(parent.children().len(), 1);
    }

    #[test]
    fn test_get_by_id() {
        let mut parent = RenderObject::new("parent");
        parent.add(Box::new(RenderObject::new("child1")));
        parent.add(Box::new(RenderObject::new("child2")));
        let found = parent.get("parent");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), "parent");
        let found = parent.get("child1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), "child1");
        let found = parent.get("nonexistent");
        assert!(found.is_none());
    }

    #[test]
    fn test_get_mut_by_id() {
        let mut parent = RenderObject::new("parent");
        parent.add(Box::new(RenderObject::new("child")));
        let child = parent.get_mut("child");
        assert!(child.is_some());
        if let Some(c) = child {
            assert_eq!(c.id(), "child");
        }
    }

    #[test]
    fn test_nested_hierarchy() {
        let mut grandparent = RenderObject::new("grandparent");
        let mut parent = RenderObject::new("parent");
        let child = RenderObject::new("child");
        parent.add(Box::new(child));
        grandparent.add(Box::new(parent));
        let found = grandparent.get("child");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), "child");
    }

    #[test]
    fn test_z_index_ordering_render() {
        let mut parent = RenderObject::new("parent").with_size(Size::new(80, 24));
        let child_z5 = RenderObject::new("z5")
            .with_z_index(5)
            .with_position(Position::new(0, 0))
            .with_size(Size::new(10, 1))
            .with_render_fn(|area, buf| {
                buf.modify_cell(area.x, area.y, |cell| {
                    cell.symbol = "5".to_string();
                });
            });
        let child_z1 = RenderObject::new("z1")
            .with_z_index(1)
            .with_position(Position::new(0, 0))
            .with_size(Size::new(10, 1))
            .with_render_fn(|area, buf| {
                buf.modify_cell(area.x, area.y, |cell| {
                    cell.symbol = "1".to_string();
                });
            });
        let child_z10 = RenderObject::new("z10")
            .with_z_index(10)
            .with_position(Position::new(0, 0))
            .with_size(Size::new(10, 1))
            .with_render_fn(|area, buf| {
                buf.modify_cell(area.x, area.y, |cell| {
                    cell.symbol = "A".to_string();
                });
            });
        parent.add(Box::new(child_z5));
        parent.add(Box::new(child_z1));
        parent.add(Box::new(child_z10));
        let mut buf = make_buffer();
        parent.render(Rect::new(0, 0, 80, 24), &mut buf);
        assert_eq!(buf.get(0, 0).unwrap().symbol, "A");
    }

    #[test]
    fn test_negative_z_index() {
        let mut parent = RenderObject::new("parent").with_size(Size::new(80, 24));
        let background = RenderObject::new("bg")
            .with_z_index(-1)
            .with_position(Position::new(0, 0))
            .with_size(Size::new(10, 1))
            .with_render_fn(|area, buf| {
                buf.modify_cell(area.x, area.y, |cell| {
                    cell.symbol = "B".to_string();
                });
            });
        let foreground = RenderObject::new("fg")
            .with_z_index(0)
            .with_position(Position::new(0, 0))
            .with_size(Size::new(10, 1))
            .with_render_fn(|area, buf| {
                buf.modify_cell(area.x, area.y, |cell| {
                    cell.symbol = "F".to_string();
                });
            });
        parent.add(Box::new(background));
        parent.add(Box::new(foreground));
        let mut buf = make_buffer();
        parent.render(Rect::new(0, 0, 80, 24), &mut buf);
        assert_eq!(buf.get(0, 0).unwrap().symbol, "F");
    }

    #[test]
    fn test_position_inheritance_relative() {
        let mut parent = RenderObject::new("parent")
            .with_position(Position::new(10, 5))
            .with_size(Size::new(80, 24));
        let child = RenderObject::new("child")
            .with_position(Position::new(2, 3))
            .with_size(Size::new(10, 5))
            .with_absolute(false)
            .with_render_fn(|area, _buf| {
                assert_eq!(area.x, 12);
                assert_eq!(area.y, 8);
            });
        parent.add(Box::new(child));
        let mut buf = make_buffer();
        parent.render(Rect::new(0, 0, 80, 24), &mut buf);
    }

    #[test]
    fn test_position_absolute() {
        let mut parent = RenderObject::new("parent")
            .with_position(Position::new(10, 5))
            .with_size(Size::new(80, 24));
        let child = RenderObject::new("child")
            .with_position(Position::new(20, 15))
            .with_size(Size::new(10, 5))
            .with_absolute(true)
            .with_render_fn(|area, _buf| {
                assert_eq!(area.x, 20);
                assert_eq!(area.y, 15);
            });
        parent.add(Box::new(child));
        let mut buf = make_buffer();
        parent.render(Rect::new(0, 0, 80, 24), &mut buf);
    }

    #[test]
    fn test_hit_test_single() {
        let parent = RenderObject::new("parent")
            .with_position(Position::new(10, 10))
            .with_size(Size::new(20, 10));
        let hits = parent.hit_test(15, 15);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id(), "parent");
        let hits = parent.hit_test(5, 5);
        assert!(hits.is_empty());
        let hits = parent.hit_test(10, 20);
        assert!(hits.is_empty());
    }

    #[test]
    fn test_hit_test_overlapping() {
        let mut parent = RenderObject::new("parent")
            .with_position(Position::new(0, 0))
            .with_size(Size::new(20, 20));
        let child1 = RenderObject::new("child1")
            .with_position(Position::new(5, 5))
            .with_size(Size::new(10, 10))
            .with_z_index(1);
        let child2 = RenderObject::new("child2")
            .with_position(Position::new(5, 5))
            .with_size(Size::new(10, 10))
            .with_z_index(5);
        parent.add(Box::new(child1));
        parent.add(Box::new(child2));
        let hits = parent.hit_test(10, 10);
        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].id(), "child2");
        assert_eq!(hits[1].id(), "child1");
        assert_eq!(hits[2].id(), "parent");
    }

    #[test]
    fn test_hit_test_nested() {
        let mut root = RenderObject::new("root")
            .with_position(Position::new(0, 0))
            .with_size(Size::new(100, 100));
        let mut level1 = RenderObject::new("level1")
            .with_position(Position::new(10, 10))
            .with_size(Size::new(50, 50))
            .with_z_index(10);
        let level2 = RenderObject::new("level2")
            .with_position(Position::new(5, 5))
            .with_size(Size::new(20, 20))
            .with_z_index(20);
        level1.add(Box::new(level2));
        root.add(Box::new(level1));
        let hits = root.hit_test(20, 20);
        assert_eq!(hits.len(), 3);
        assert!(hits.iter().any(|h| h.id() == "level2"));
        assert!(hits.iter().any(|h| h.id() == "level1"));
        assert!(hits.iter().any(|h| h.id() == "root"));
    }

    #[test]
    fn test_visibility() {
        let mut parent = RenderObject::new("parent")
            .with_size(Size::new(10, 1))
            .with_render_fn(|area, buf| {
                buf.modify_cell(area.x, area.y, |cell| {
                    cell.symbol = "P".to_string();
                });
            });
        let child = RenderObject::new("child")
            .with_position(Position::new(0, 0))
            .with_size(Size::new(10, 1))
            .with_visible(false)
            .with_render_fn(|area, buf| {
                buf.modify_cell(area.x, area.y, |cell| {
                    cell.symbol = "C".to_string();
                });
            });
        parent.add(Box::new(child));
        let mut buf = make_buffer();
        parent.render(Rect::new(0, 0, 80, 24), &mut buf);
        assert_eq!(buf.get(0, 0).unwrap().symbol, "P");
    }

    #[test]
    fn test_render_builder() {
        let root = RenderBuilder::new("root")
            .position(Position::new(0, 0))
            .size(Size::new(80, 24))
            .z_index(0)
            .child(
                RenderBuilder::new("child")
                    .position(Position::new(10, 5))
                    .size(Size::new(20, 10))
                    .z_index(1)
                    .build(),
            )
            .build();
        assert_eq!(root.id(), "root");
        assert_eq!(root.children().len(), 1);
        assert_eq!(root.children()[0].id(), "child");
    }

    #[test]
    fn test_bounds_calculation() {
        let parent = RenderObject::new("parent")
            .with_position(Position::new(10, 10))
            .with_size(Size::new(20, 20));
        let bounds = parent.bounds(None);
        assert_eq!(bounds, Rect::new(10, 10, 20, 20));
        let bounds = parent.bounds(Some(Position::new(5, 5)));
        assert_eq!(bounds, Rect::new(15, 15, 20, 20));
    }

    #[test]
    fn test_bounds_absolute_positioning() {
        let obj = RenderObject::new("obj")
            .with_position(Position::new(20, 30))
            .with_size(Size::new(10, 5))
            .with_absolute(true);
        let bounds = obj.bounds(Some(Position::new(100, 100)));
        assert_eq!(bounds, Rect::new(20, 30, 10, 5));
    }

    #[test]
    fn test_downcast() {
        let obj = RenderObject::new("test");
        let boxed: Box<dyn Renderable> = Box::new(obj);
        assert_eq!(boxed.id(), "test");
    }

    #[test]
    fn test_three_level_hierarchy() {
        let mut level0 = RenderObject::new("level0")
            .with_position(Position::new(0, 0))
            .with_size(Size::new(100, 100));
        let mut level1 = RenderObject::new("level1")
            .with_position(Position::new(10, 10))
            .with_size(Size::new(50, 50));
        let level2 = RenderObject::new("level2")
            .with_position(Position::new(5, 5))
            .with_size(Size::new(10, 10));
        level1.add(Box::new(level2));
        level0.add(Box::new(level1));
        assert!(level0.get("level0").is_some());
        assert!(level0.get("level1").is_some());
        assert!(level0.get("level2").is_some());
        let hits = level0.hit_test(20, 20);
        assert!(hits.iter().any(|h| h.id() == "level0"));
        assert!(hits.iter().any(|h| h.id() == "level1"));
        assert!(hits.iter().any(|h| h.id() == "level2"));
    }
}
