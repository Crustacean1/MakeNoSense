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
        mat.data[2][0] = translation.x;
        mat.data[2][1] = translation.y;
        mat
    }

    pub fn scale(scale: (f32, f32)) -> Self {
        let mut mat = Self::ident();
        mat.data[0][0] *= scale.0;
        mat.data[1][1] *= scale.1;
        mat
    }

    pub fn st_inverse(&self) -> Matrix {
        let scale = Vector3::new(
            1.0 / self.data[0][0],
            1.0 / self.data[1][1],
            1.0 / self.data[2][2],
        );

        let translation = Vector3::new(self.data[2][0], self.data[2][1], self.data[2][2]);

        let mut inverse = Matrix::ident();
        inverse.data[0][0] = scale.x;
        inverse.data[1][1] = scale.y;
        inverse.data[2][2] = scale.z;

        inverse.data[2][0] = -translation.x * scale.x;
        inverse.data[2][1] = -translation.y * scale.y;

        inverse.data[2][2] = -translation.z * scale.z;

        inverse
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
                *cell = (0..3).fold(0.0, |a, k| (a + rhs.data[i][k] * self.data[k][j]));
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
            x: data[0][0] * rhs.x + data[1][0] * rhs.y + data[2][0] * rhs.z,
            y: data[0][1] * rhs.x + data[1][1] * rhs.y + data[2][1] * rhs.z,
            z: data[0][2] * rhs.x + data[1][2] * rhs.y + data[2][2] * rhs.z,
        }
    }
}
