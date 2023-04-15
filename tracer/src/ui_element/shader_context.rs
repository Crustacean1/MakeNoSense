use crate::application::AppError;

use super::{matrix::Matrix, shader::ShaderProgram};

pub struct ShaderContext {
    pub col_shader: ShaderProgram,
    pub tex_shader: ShaderProgram,
    matrix_stack: Vec<Matrix>,
}

impl ShaderContext {
    pub fn build() -> Result<Self, AppError> {
        let col_shader = ShaderProgram::build(
            "tracer/shaders/col_shader.vs",
            "tracer/shaders/col_shader.fs",
        );

        let tex_shader = ShaderProgram::build(
            "tracer/shaders/tex_shader.vs",
            "tracer/shaders/tex_shader.fs",
        );

        Ok(ShaderContext {
            col_shader: col_shader?,
            tex_shader: tex_shader?,
            matrix_stack: vec![Matrix::ident()],
        })
    }

    pub fn push(&mut self, matrix: &Matrix) {
        let last: &Matrix = self.matrix_stack.last().unwrap();
        self.matrix_stack.push(*last * *matrix);
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
