use glium::{index::PrimitiveType, Display, IndexBuffer, VertexBuffer};

use crate::{editor::vertex::VertexPC, AppError};

pub struct LayerRenderer {
    vertices_vec: Vec<VertexPC>,
    vertices: VertexBuffer<VertexPC>,
    indices_vec: Vec<u32>,
    indices: IndexBuffer<u32>,
}

impl LayerRenderer {
    pub fn build(display: &Display) -> Result<LayerRenderer, AppError> {
        let (vertices, indices) = ([], []);

        Ok(LayerRenderer {
            vertices: VertexBuffer::dynamic(display, &vertices)?,
            vertices_vec: vec![],
            indices: IndexBuffer::dynamic(display, PrimitiveType::TrianglesList, &indices)?,
            indices_vec: vec![],
        })
    }

    pub fn indices(&self) -> &IndexBuffer<u32> {
        &self.indices
    }

    pub fn vertices(&self) -> &VertexBuffer<VertexPC> {
        &self.vertices
    }

    pub fn reload_indices(&mut self, display: &Display, indices: &[u32]) -> Result<(), AppError> {
        self.indices = IndexBuffer::dynamic(display, PrimitiveType::TrianglesList, &indices)?;
        Ok(())
    }

    pub fn reload_vertices(
        &mut self,
        display: &Display,
        vertices: &[VertexPC],
    ) -> Result<(), AppError> {
        self.vertices = VertexBuffer::dynamic(display, &vertices)?;
        Ok(())
    }
}
