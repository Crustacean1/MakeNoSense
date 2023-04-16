use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn cross(self, rhs: Vec2) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn new((x, y): (f32, f32)) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn perp(&self) -> Vec2 {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul for Vec2 {
    type Output = f32;
    fn mul(self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
