use std::ops;

use crate::vector::Vector3;

#[derive(Clone, Copy)]
pub struct Matrix {
    pub data: [[f32; 3]; 3],
}

impl Matrix {
    pub fn ident() -> Self {
        let mut data = [[0.0; 3]; 3];
        (0..3).for_each(|i| data[i][i] = 1.0);

        Self { data }
    }

    pub fn translate(translation: Vector3) -> Self {
        let mut mat = Self::ident();
        mat.data[0][3] = translation.x;
        mat.data[1][3] = translation.y;
        mat
    }

    pub fn scale(scale: f32) -> Self {
        let mut mat = Self::ident();
        (0..3).for_each(|i| mat.data[i][i] = mat.data[i][i] * scale);
        mat
    }
}
impl From<Matrix> for [[f32; 3]; 3] {
    fn from(mat: Matrix) -> [[f32; 3]; 3] {
        mat.data
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [[0.0; 3]; 3];
        for (i, column) in result.iter_mut().enumerate() {
            for (j, cell) in column.iter_mut().enumerate() {
                *cell = (0..3).fold(0.0, |a, k| (a + self.data[i][k] * rhs.data[k][j]));
            }
        }
        Matrix { data: result }
    }
}

impl ops::Mul<Vector3> for Matrix {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let data = &self.data;
        Vector3 {
            x: data[0][0] * rhs.x + data[0][1] * rhs.y + data[0][2] * rhs.z,
            y: data[1][0] * rhs.x + data[1][1] * rhs.y + data[1][2] * rhs.z,
            z: data[2][0] * rhs.x + data[2][1] * rhs.y + data[2][2] * rhs.z,
        }
    }
}
