#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Padding {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

impl Padding {
    pub fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn uniform(value: u16) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn horizontal(value: u16) -> Self {
        Self {
            left: value,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    pub fn vertical(value: u16) -> Self {
        Self {
            left: 0,
            right: 0,
            top: value,
            bottom: value,
        }
    }

    pub fn left(value: u16) -> Self {
        Self {
            left: value,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }

    pub fn right(value: u16) -> Self {
        Self {
            left: 0,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    pub fn top(value: u16) -> Self {
        Self {
            left: 0,
            right: 0,
            top: value,
            bottom: 0,
        }
    }

    pub fn bottom(value: u16) -> Self {
        Self {
            left: 0,
            right: 0,
            top: 0,
            bottom: value,
        }
    }
}
