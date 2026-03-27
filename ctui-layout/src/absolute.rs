use ctui_core::Rect;

/// Z-index for stacking order of absolutely positioned elements
pub type ZIndex = i32;

/// Default Z-index for elements without explicit z-index
pub const DEFAULT_Z_INDEX: ZIndex = 0;

/// An absolutely positioned element with offset and z-index
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbsoluteItem {
    /// Horizontal offset from the container's left edge
    pub x: i16,
    /// Vertical offset from the container's top edge
    pub y: i16,
    /// Width of the element
    pub width: u16,
    /// Height of the element
    pub height: u16,
    /// Z-index for stacking order (higher = on top)
    pub z_index: ZIndex,
}

impl AbsoluteItem {
    /// Creates a new absolutely positioned item
    #[must_use]
    pub const fn new(x: i16, y: i16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            z_index: DEFAULT_Z_INDEX,
        }
    }

    /// Sets the z-index for stacking order
    #[must_use]
    pub const fn z_index(mut self, z: ZIndex) -> Self {
        self.z_index = z;
        self
    }

    /// Creates an item at position (0, 0)
    #[must_use]
    pub const fn at_origin(width: u16, height: u16) -> Self {
        Self::new(0, 0, width, height)
    }

    /// Creates an item positioned from the top-right corner
    #[must_use]
    pub const fn top_right(
        container_width: u16,
        offset_x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Self {
        let x = (container_width as i32)
            .saturating_sub(offset_x as i32)
            .saturating_sub(width as i32);
        Self::new(x as i16, y, width, height)
    }

    /// Creates an item positioned from the bottom-right corner
    #[must_use]
    pub const fn bottom_right(
        container_width: u16,
        container_height: u16,
        offset_x: i16,
        offset_y: i16,
        width: u16,
        height: u16,
    ) -> Self {
        let x = (container_width as i32)
            .saturating_sub(offset_x as i32)
            .saturating_sub(width as i32);
        let y = (container_height as i32)
            .saturating_sub(offset_y as i32)
            .saturating_sub(height as i32);
        Self::new(x as i16, y as i16, width, height)
    }

    /// Creates an item centered in the container
    #[must_use]
    pub fn centered(container_width: u16, container_height: u16, width: u16, height: u16) -> Self {
        let x = ((container_width as i32).saturating_sub(width as i32) / 2) as i16;
        let y = ((container_height as i32).saturating_sub(height as i32) / 2) as i16;
        Self::new(x, y, width, height)
    }

    /// Converts to a Rect within the given container bounds
    #[must_use]
    pub fn to_rect(&self, container: Rect) -> Rect {
        let x = if self.x >= 0 {
            container.x.saturating_add(self.x as u16)
        } else {
            container.x.saturating_sub(self.x.unsigned_abs())
        };

        let y = if self.y >= 0 {
            container.y.saturating_add(self.y as u16)
        } else {
            container.y.saturating_sub(self.y.unsigned_abs())
        };

        Rect::new(x, y, self.width, self.height)
    }
}

/// A container for absolutely positioned elements
#[derive(Clone, Debug, Default)]
pub struct AbsoluteLayout {
    /// Items in this layout
    items: Vec<AbsoluteItem>,
}

impl AbsoluteLayout {
    /// Creates a new empty absolute layout
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an item to the layout
    #[must_use]
    pub fn add(mut self, item: AbsoluteItem) -> Self {
        self.items.push(item);
        self
    }

    /// Adds an item with a builder callback
    #[must_use]
    pub fn with_item<F>(mut self, f: F) -> Self
    where
        F: FnOnce(AbsoluteItem) -> AbsoluteItem,
    {
        self.items.push(f(AbsoluteItem::new(0, 0, 0, 0)));
        self
    }

    /// Returns the number of items in the layout
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the layout is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns items sorted by z-index (painter's order: lowest first)
    #[must_use]
    pub fn sorted_by_z_index(&self) -> Vec<&AbsoluteItem> {
        let mut items: Vec<_> = self.items.iter().collect();
        items.sort_by_key(|item| item.z_index);
        items
    }

    /// Splits the container area into rects for all items
    #[must_use]
    pub fn split(&self, container: Rect) -> Vec<Rect> {
        self.sorted_by_z_index()
            .iter()
            .map(|item| item.to_rect(container))
            .collect()
    }

    /// Returns items that overlap with a point
    #[must_use]
    pub fn items_at_point(&self, container: Rect, x: u16, y: u16) -> Vec<&AbsoluteItem> {
        self.items
            .iter()
            .filter(|item| {
                let rect = item.to_rect(container);
                x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
            })
            .collect()
    }

    /// Returns the topmost item at a point (highest z-index)
    #[must_use]
    pub fn topmost_at_point(&self, container: Rect, x: u16, y: u16) -> Option<&AbsoluteItem> {
        self.items_at_point(container, x, y)
            .into_iter()
            .max_by_key(|item| item.z_index)
    }

    /// Clears all items from the layout
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

/// Stacking context for managing z-index hierarchies
#[derive(Clone, Debug, Default)]
pub struct StackingContext {
    base_z_index: ZIndex,
}

impl StackingContext {
    /// Creates a new stacking context
    #[must_use]
    pub fn new(base_z_index: ZIndex) -> Self {
        Self { base_z_index }
    }

    /// Returns the effective z-index for a local z-index
    #[must_use]
    pub const fn effective_z_index(&self, local_z: ZIndex) -> ZIndex {
        self.base_z_index.saturating_add(local_z)
    }

    /// Creates a new context offset by a delta
    #[must_use]
    pub const fn offset(&self, delta: ZIndex) -> Self {
        Self {
            base_z_index: self.base_z_index.saturating_add(delta),
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
    fn absolute_item_new() {
        let item = AbsoluteItem::new(10, 20, 30, 40);
        assert_eq!(item.x, 10);
        assert_eq!(item.y, 20);
        assert_eq!(item.width, 30);
        assert_eq!(item.height, 40);
        assert_eq!(item.z_index, 0);
    }

    #[test]
    fn absolute_item_with_z_index() {
        let item = AbsoluteItem::new(10, 20, 30, 40).z_index(5);
        assert_eq!(item.z_index, 5);
    }

    #[test]
    fn absolute_item_to_rect_positive_offset() {
        let item = AbsoluteItem::new(5, 10, 20, 15);
        let container = rect(0, 0, 100, 100);
        let r = item.to_rect(container);
        assert_eq!(r, rect(5, 10, 20, 15));
    }

    #[test]
    fn absolute_item_to_rect_with_container_offset() {
        let item = AbsoluteItem::new(5, 10, 20, 15);
        let container = rect(50, 30, 100, 100);
        let r = item.to_rect(container);
        assert_eq!(r, rect(55, 40, 20, 15));
    }

    #[test]
    fn absolute_item_to_rect_negative_offset() {
        let item = AbsoluteItem::new(-5, -10, 20, 15);
        let container = rect(100, 100, 50, 50);
        let r = item.to_rect(container);
        assert_eq!(r.x, 95);
        assert_eq!(r.y, 90);
    }

    #[test]
    fn absolute_item_centered() {
        let item = AbsoluteItem::centered(100, 50, 40, 20);
        assert_eq!(item.x, 30);
        assert_eq!(item.y, 15);
    }

    #[test]
    fn absolute_layout_empty() {
        let layout = AbsoluteLayout::new();
        assert!(layout.is_empty());
        assert_eq!(layout.len(), 0);
    }

    #[test]
    fn absolute_layout_add() {
        let layout = AbsoluteLayout::new()
            .add(AbsoluteItem::new(0, 0, 10, 10))
            .add(AbsoluteItem::new(20, 20, 15, 15));
        assert_eq!(layout.len(), 2);
    }

    #[test]
    fn absolute_layout_split() {
        let layout = AbsoluteLayout::new()
            .add(AbsoluteItem::new(0, 0, 10, 10))
            .add(AbsoluteItem::new(20, 20, 15, 15));
        let container = rect(0, 0, 100, 100);
        let rects = layout.split(container);
        assert_eq!(rects.len(), 2);
    }

    #[test]
    fn absolute_layout_sorted_by_z_index() {
        let layout = AbsoluteLayout::new()
            .add(AbsoluteItem::new(0, 0, 10, 10).z_index(10))
            .add(AbsoluteItem::new(0, 0, 10, 10).z_index(5))
            .add(AbsoluteItem::new(0, 0, 10, 10).z_index(15));
        let sorted = layout.sorted_by_z_index();
        assert_eq!(sorted[0].z_index, 5);
        assert_eq!(sorted[1].z_index, 10);
        assert_eq!(sorted[2].z_index, 15);
    }

    #[test]
    fn absolute_layout_items_at_point() {
        let layout = AbsoluteLayout::new()
            .add(AbsoluteItem::new(0, 0, 10, 10))
            .add(AbsoluteItem::new(5, 5, 10, 10));
        let container = rect(0, 0, 100, 100);
        let items = layout.items_at_point(container, 7, 7);
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn absolute_layout_topmost_at_point() {
        let layout = AbsoluteLayout::new()
            .add(AbsoluteItem::new(0, 0, 10, 10).z_index(1))
            .add(AbsoluteItem::new(0, 0, 10, 10).z_index(5))
            .add(AbsoluteItem::new(0, 0, 10, 10).z_index(3));
        let container = rect(0, 0, 100, 100);
        let topmost = layout.topmost_at_point(container, 5, 5);
        assert!(topmost.is_some());
        assert_eq!(topmost.unwrap().z_index, 5);
    }

    #[test]
    fn stacking_context() {
        let ctx = StackingContext::new(100);
        assert_eq!(ctx.effective_z_index(10), 110);
    }

    #[test]
    fn stacking_context_offset() {
        let ctx = StackingContext::new(100);
        let offset_ctx = ctx.offset(50);
        assert_eq!(offset_ctx.base_z_index, 150);
    }
}
