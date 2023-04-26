use std::{cmp, ops};

#[derive(Clone, Copy, Debug)]

pub struct BoundingBox<
    T: cmp::PartialOrd + ops::Add<Output = T> + ops::Sub<Output = T> + Clone + Copy,
> {
    pub top: T,
    pub left: T,
    pub right: T,
    pub bottom: T,
}

impl<T: cmp::PartialOrd + ops::Add<Output = T> + ops::Sub<Output = T> + Clone + Copy>
    BoundingBox<T>
{
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn from_quad((left, top): (T, T), (width, height): (T, T)) -> Self {
        BoundingBox {
            top,
            left,
            bottom: top + height,
            right: left + width,
        }
    }

    pub fn contains(&self, pos: (T, T)) -> bool {
        pos.0 > self.left && pos.1 > self.top && pos.0 < self.right && pos.1 < self.bottom
    }

    pub fn width(&self) -> T {
        self.right - self.left
    }

    pub fn height(&self) -> T {
        self.bottom - self.top
    }
}
