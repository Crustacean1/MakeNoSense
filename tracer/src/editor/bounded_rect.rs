#[derive(Clone, Copy, Debug)]
pub struct BoundingRect {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
}

impl BoundingRect {
    pub fn from_quad((left, top): (f32, f32), (width, height): (f32, f32)) -> Self {
        BoundingRect {
            top,
            left,
            width,
            height,
        }
    }

    pub fn contains(&self, pos: (f32, f32)) -> bool {
        pos.0 > self.left
            && pos.1 > self.top
            && pos.0 - self.left < self.width
            && pos.1 - self.top < self.height
    }
}

