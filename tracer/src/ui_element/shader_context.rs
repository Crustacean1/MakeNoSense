use crate::application::AppError;

use super::{matrix::Matrix, shader::ShaderProgram};

pub struct ShaderContext {
    pub col_shader: ShaderProgram,
    pub tex_shader: ShaderProgram,
    aspect_matrix: Matrix,
    matrix_stack: Vec<Matrix>,
}

impl ShaderContext {
    pub fn build((width, height): (u32, u32)) -> Result<Self, AppError> {
        let col_shader = ShaderProgram::build(
            "tracer/shaders/col_shader.vs",
            "tracer/shaders/col_shader.fs",
        );

        let tex_shader = ShaderProgram::build(
            "tracer/shaders/tex_shader.vs",
            "tracer/shaders/tex_shader.fs",
        );

        let (width, height) = (width as f32, height as f32);
        let (width_factor, height_factor) = (height / width, 1.0);

        let mut aspect_matrix = Matrix::ident();
        aspect_matrix.data[0][0] = width_factor;
        aspect_matrix.data[1][1] = height_factor;

        Ok(ShaderContext {
            col_shader: col_shader?,
            tex_shader: tex_shader?,
            matrix_stack: vec![aspect_matrix],
            aspect_matrix,
        })
    }

    pub fn push(&mut self, matrix: &Matrix) {
        let last: &Matrix = self.matrix_stack.last().unwrap();
        self.matrix_stack.push(*matrix * *last);
    }

    pub fn get_aspect_matrix(&self) -> &Matrix {
        &self.aspect_matrix
    }

    pub fn get_matrix(&self) -> &Matrix {
        self.matrix_stack
            .last()
            .expect("Matrix in shader context stack should never be empty")
    }

    pub fn pop(&mut self) {
        self.matrix_stack.pop();
    }
}
