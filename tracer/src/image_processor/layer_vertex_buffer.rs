use glium::{Display, VertexBuffer};

use crate::{editor::vertex::VertexPC, AppError};

pub struct LayerVertexBuffer {
    vertices: Vec<VertexPC>,
    vertex_buffer: VertexBuffer<VertexPC>,
}

impl LayerVertexBuffer {
    pub fn build(display: &Display) -> Result<Self, AppError> {
        let vertices = [];

        Ok(LayerVertexBuffer {
            vertices: vec![],
            vertex_buffer: VertexBuffer::dynamic(display, &vertices)?,
        })
    }

    pub fn reload(&mut self, display: &Display, vertices: &[(f32, f32)]) -> Result<(), AppError> {
        let new_buffer_size = Self::buffer_size(vertices.len());
        if new_buffer_size != self.vertex_buffer.len() {
            self.vertices = vec![
                VertexPC {
                    pos: [0.0, 0.0],
                    col: [0.0, 0.0, 0.0, 0.0],
                };
                new_buffer_size
            ];
            self.load_vertices(vertices);
            self.vertex_buffer = VertexBuffer::dynamic(display, &self.vertices)?;
        } else {
            self.load_vertices(vertices);
            self.vertex_buffer.write(&self.vertices);
        }
        Ok(())
    }

    pub fn vertex_buffer(&self) -> &VertexBuffer<VertexPC> {
        &self.vertex_buffer
    }

    fn load_vertices(&mut self, vertices: &[(f32, f32)]) {
        let mut i = 0;
        vertices.iter().for_each(|&vertex| {
            self.vertices[i] = VertexPC {
                pos: [vertex.0, vertex.1],
                col: [1.0, 1.0, 1.0, 1.0],
            };
            i += 1;
        });
    }

    fn buffer_size(size: usize) -> usize {
        let mut i: usize = 1;
        while i < size {
            i <<= 1;
        }
        i
    }
}
