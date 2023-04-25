use glium::{Frame, Program};

use crate::matrix::Matrix;

pub struct RenderingContext<'a> {
    shaders: &'a Vec<Program>,
    frame: &'a mut Frame,
    matrix_stack: Vec<Matrix>,
}

impl<'a> RenderingContext<'a> {
    pub fn new(shaders: &'a Vec<Program>, frame: &'a mut Frame) -> RenderingContext<'a> {
        RenderingContext {
            shaders,
            frame,
            matrix_stack: vec![Matrix::ident()],
        }
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

    pub fn shader_context(&mut self, i: usize) -> Option<(&Program, &mut Frame)> {
        Some((self.shaders.get(i)?, &mut self.frame))
    }
}
