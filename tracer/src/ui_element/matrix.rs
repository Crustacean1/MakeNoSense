use std::ops;

use crate::vec::Vec2;

#[derive(Clone, Copy)]
pub struct Matrix {
    pub data: [[f32; 4]; 4],
}

impl Matrix {
    pub fn ident() -> Self {
        let mut data = [[0.0; 4]; 4];
        (0..4).for_each(|i| data[i][i] = 1.0);

        Self { data }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        let mut mat = Self::ident();
        mat.data[3][0] = x;
        mat.data[3][1] = y;
        mat.data[3][2] = z;
        mat
    }

    pub fn scale(scale: f32) -> Self {
        let mut mat = Self::ident();
        (0..3).for_each(|i| mat.data[i][i] = mat.data[i][i] * scale);
        mat
    }
}
impl From<Matrix> for [[f32; 4]; 4] {
    fn from(mat: Matrix) -> [[f32; 4]; 4] {
        mat.data
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [[0.0; 4]; 4];
        for (i, column) in result.iter_mut().enumerate() {
            for (j, cell) in column.iter_mut().enumerate() {
                *cell = (0..4).fold(0.0, |a, k| (a + self.data[i][k] * rhs.data[k][j]));
            }
        }
        Matrix { data: result }
    }
}

impl ops::Mul<Vec2> for Matrix {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        let data = &self.data;
        Vec2::new((
            rhs.x * data[0][0] + rhs.y * data[1][0] + data[3][0],
            rhs.x * data[0][1] + rhs.y * data[1][1] + data[3][1],
        ))
    }
}
