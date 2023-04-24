use std::ops;

#[derive(Clone, Copy, Debug)]

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn cross(self, rhs: Vector3) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn perp(&self) -> Vector3 {
        Vector3 {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }

    pub fn scale(self, scale: f32) -> Vector3 {
        Vector3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }

    pub fn sqr_dst(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn from((x, y): (f32, f32)) -> Self {
        Vector3 { x, y, z: 0.0 }
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = f32;
    fn mul(self, rhs: Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
