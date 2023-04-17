use super::shader::ShaderProgram;
use crate::{matrix::Matrix, AppError};

pub struct ShaderContext {
    pub col_shader: glium::Program,
    pub tex_shader: glium::Program,
    matrix_stack: Vec<Matrix>,
}

impl ShaderContext {
    pub fn build(display: &glium::Display) -> Result<Self, AppError> {
        let col_shader = ShaderProgram::build(
            display,
            "tracer/shaders/col_shader.vs",
            "tracer/shaders/col_shader.fs",
        )?;

        let tex_shader = ShaderProgram::build(
            display,
            "tracer/shaders/tex_shader.vs",
            "tracer/shaders/tex_shader.fs",
        )?;

        Ok(ShaderContext {
            col_shader,
            tex_shader,
            matrix_stack: vec![],
        })
    }

    pub fn push(&mut self, matrix: &Matrix) {
        let last: &Matrix = self.matrix_stack.last().unwrap();
        self.matrix_stack.push(*matrix * *last);
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
