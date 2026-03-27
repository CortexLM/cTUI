//! Geometry primitives for terminal layout
//!
//! This module provides basic geometric types for position and size calculations.

/// A position in the terminal (column, row)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Position {
    /// X position (column)
    pub x: u16,
    /// Y position (row)
    pub y: u16,
}

impl Position {
    /// Creates a new position
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Creates a position at the origin (0, 0)
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl From<(u16, u16)> for Position {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

impl From<Position> for (u16, u16) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

/// A size in the terminal (width, height)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Size {
    /// Width
    pub width: u16,
    /// Height
    pub height: u16,
}

impl Size {
    /// Creates a new size
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Creates a zero size
    pub fn zero() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    /// Returns the area (width * height)
    pub fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Returns true if this size is zero
    pub fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Self { width, height }
    }
}

impl From<Size> for (u16, u16) {
    fn from(size: Size) -> Self {
        (size.width, size.height)
    }
}

/// A rectangular area in the terminal
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rect {
    /// X position (column)
    pub x: u16,
    /// Y position (row)
    pub y: u16,
    /// Width
    pub width: u16,
    /// Height
    pub height: u16,
}

impl Rect {
    /// Creates a new rectangle with the given position and size
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a rectangle from position and size
    pub fn from_position_size(position: Position, size: Size) -> Self {
        Self {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Creates a zero-area rectangle at the origin
    pub fn zero() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    /// Returns the area of the rectangle
    pub fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Returns true if the rectangle has zero area
    pub fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Returns the position (top-left corner)
    pub fn position(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }

    /// Returns the size
    pub fn size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    /// Returns the right edge x coordinate
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the bottom edge y coordinate
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Returns the center position
    pub fn center(&self) -> Position {
        Position {
            x: self.x.saturating_add(self.width / 2),
            y: self.y.saturating_add(self.height / 2),
        }
    }

    /// Returns true if this rectangle contains the given point
    pub fn contains(&self, point: Position) -> bool {
        point.x >= self.x && point.y >= self.y && point.x < self.right() && point.y < self.bottom()
    }

    /// Returns true if this rectangle contains the given point (inclusive of edges)
    pub fn contains_inclusive(&self, point: Position) -> bool {
        point.x >= self.x
            && point.y >= self.y
            && point.x <= self.right()
            && point.y <= self.bottom()
    }

    /// Returns the intersection of this rectangle with another
    ///
    /// Returns a zero-area rectangle if the rectangles don't overlap
    pub fn intersection(&self, other: &Rect) -> Rect {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        if right <= x || bottom <= y {
            Rect::zero()
        } else {
            Rect::new(x, y, right - x, bottom - y)
        }
    }

    /// Returns the smallest rectangle that contains both rectangles
    pub fn union(&self, other: &Rect) -> Rect {
        if self.is_zero() {
            return *other;
        }
        if other.is_zero() {
            return *self;
        }

        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());

        Rect::new(x, y, right - x, bottom - y)
    }

    /// Clamp the rectangle's size to not exceed the given maximum dimensions
    pub fn clamp(&self, max_width: u16, max_height: u16) -> Rect {
        Rect::new(
            self.x,
            self.y,
            self.width.min(max_width),
            self.height.min(max_height),
        )
    }

    /// Clamp the rectangle to fit within the given bounds
    pub fn clamp_to_bounds(&self, bounds: &Rect) -> Rect {
        let x = self.x.max(bounds.x);
        let y = self.y.max(bounds.y);
        let max_right = bounds.x.saturating_add(bounds.width);
        let max_bottom = bounds.y.saturating_add(bounds.height);
        let right = self.right().min(max_right);
        let bottom = self.bottom().min(max_bottom);

        if right <= x || bottom <= y {
            Rect::zero()
        } else {
            Rect::new(x, y, right - x, bottom - y)
        }
    }

    /// Returns an iterator over all positions within this rectangle
    pub fn positions(&self) -> RectPositions {
        RectPositions {
            rect: *self,
            current_x: self.x,
            current_y: self.y,
        }
    }
}

impl From<(u16, u16, u16, u16)> for Rect {
    fn from((x, y, width, height): (u16, u16, u16, u16)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<Rect> for (u16, u16, u16, u16) {
    fn from(rect: Rect) -> Self {
        (rect.x, rect.y, rect.width, rect.height)
    }
}

impl From<(Position, Size)> for Rect {
    fn from((position, size): (Position, Size)) -> Self {
        Self::from_position_size(position, size)
    }
}

impl From<Rect> for (Position, Size) {
    fn from(rect: Rect) -> Self {
        (rect.position(), rect.size())
    }
}

/// Iterator over all positions within a rectangle
pub struct RectPositions {
    rect: Rect,
    current_x: u16,
    current_y: u16,
}

impl Iterator for RectPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.rect.bottom() {
            return None;
        }

        let pos = Position {
            x: self.current_x,
            y: self.current_y,
        };

        self.current_x = self.current_x.saturating_add(1);
        if self.current_x >= self.rect.right() {
            self.current_x = self.rect.x;
            self.current_y = self.current_y.saturating_add(1);
        }

        Some(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_area() {
        let rect = Rect::new(0, 0, 10, 20);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_rect_area_zero() {
        let rect = Rect::new(0, 0, 0, 10);
        assert_eq!(rect.area(), 0);
        assert!(rect.is_zero());
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(5, 5, 10, 10);

        // Inside
        assert!(rect.contains(Position::new(5, 5)));
        assert!(rect.contains(Position::new(10, 10)));
        assert!(rect.contains(Position::new(14, 14)));

        // Outside
        assert!(!rect.contains(Position::new(4, 5)));
        assert!(!rect.contains(Position::new(5, 4)));
        assert!(!rect.contains(Position::new(15, 5))); // right edge (exclusive)
        assert!(!rect.contains(Position::new(5, 15))); // bottom edge (exclusive)
    }

    #[test]
    fn test_rect_contains_inclusive() {
        let rect = Rect::new(5, 5, 10, 10);

        // Right and bottom edges are inclusive
        assert!(rect.contains_inclusive(Position::new(15, 5)));
        assert!(rect.contains_inclusive(Position::new(5, 15)));
    }

    #[test]
    fn test_rect_intersection() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);

        let intersection = a.intersection(&b);
        assert_eq!(intersection, Rect::new(5, 5, 5, 5));
    }

    #[test]
    fn test_rect_intersection_no_overlap() {
        let a = Rect::new(0, 0, 5, 5);
        let b = Rect::new(10, 10, 5, 5);

        let intersection = a.intersection(&b);
        assert!(intersection.is_zero());
    }

    #[test]
    fn test_rect_intersection_edge_touching() {
        let a = Rect::new(0, 0, 5, 5);
        let b = Rect::new(5, 0, 5, 5); // Touches at edge

        // Edges touching but not overlapping should return zero
        let intersection = a.intersection(&b);
        assert!(intersection.is_zero());
    }

    #[test]
    fn test_rect_union() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);

        let union = a.union(&b);
        assert_eq!(union, Rect::new(0, 0, 15, 15));
    }

    #[test]
    fn test_rect_union_with_zero() {
        let a = Rect::new(5, 5, 10, 10);
        let zero = Rect::zero();

        assert_eq!(a.union(&zero), a);
        assert_eq!(zero.union(&a), a);
    }

    #[test]
    fn test_rect_clamp_size() {
        let rect = Rect::new(0, 0, 100, 100);
        let clamped = rect.clamp(50, 30);

        assert_eq!(clamped, Rect::new(0, 0, 50, 30));
    }

    #[test]
    fn test_rect_clamp_no_change() {
        let rect = Rect::new(0, 0, 10, 10);
        let clamped = rect.clamp(100, 100);

        assert_eq!(clamped, rect);
    }

    #[test]
    fn test_rect_clamp_to_bounds() {
        let rect = Rect::new(0, 0, 100, 100);
        let bounds = Rect::new(50, 50, 20, 20);

        let clamped = rect.clamp_to_bounds(&bounds);
        assert_eq!(clamped, Rect::new(50, 50, 20, 20));
    }

    #[test]
    fn test_position_from_tuple() {
        let pos: Position = (10, 20).into();
        assert_eq!(pos, Position::new(10, 20));

        let tuple: (u16, u16) = pos.into();
        assert_eq!(tuple, (10, 20));
    }

    #[test]
    fn test_size_from_tuple() {
        let size: Size = (100, 50).into();
        assert_eq!(size, Size::new(100, 50));

        let tuple: (u16, u16) = size.into();
        assert_eq!(tuple, (100, 50));
    }

    #[test]
    fn test_rect_from_tuple() {
        let rect: Rect = (5, 10, 20, 30).into();
        assert_eq!(rect, Rect::new(5, 10, 20, 30));

        let tuple: (u16, u16, u16, u16) = rect.into();
        assert_eq!(tuple, (5, 10, 20, 30));
    }

    #[test]
    fn test_rect_from_position_size() {
        let pos = Position::new(5, 10);
        let size = Size::new(20, 30);
        let rect: Rect = (pos, size).into();

        assert_eq!(rect, Rect::new(5, 10, 20, 30));
    }

    #[test]
    fn test_rect_positions() {
        let rect = Rect::new(0, 0, 2, 2);
        let positions: Vec<Position> = rect.positions().collect();

        assert_eq!(
            positions,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
            ]
        );
    }

    #[test]
    fn test_rect_positions_empty() {
        let rect = Rect::zero();
        let positions: Vec<Position> = rect.positions().collect();
        assert!(positions.is_empty());
    }

    #[test]
    fn test_center() {
        let rect = Rect::new(0, 0, 10, 10);
        assert_eq!(rect.center(), Position::new(5, 5));

        let rect = Rect::new(0, 0, 9, 9);
        assert_eq!(rect.center(), Position::new(4, 4));
    }

    #[test]
    fn test_edge_overflow() {
        // Test that overflow is handled correctly
        let rect = Rect::new(u16::MAX - 5, u16::MAX - 5, 10, 10);

        // right() and bottom() use saturating_add
        assert_eq!(rect.right(), u16::MAX);
        assert_eq!(rect.bottom(), u16::MAX);
    }

    #[test]
    fn test_size_area() {
        let size = Size::new(10, 20);
        assert_eq!(size.area(), 200);
    }

    #[test]
    fn test_size_zero() {
        let size = Size::zero();
        assert!(size.is_zero());
        assert_eq!(size.area(), 0);
    }
}
