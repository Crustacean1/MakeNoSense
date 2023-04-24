use glium::{index::PrimitiveType, uniforms::Uniforms, Program, Surface};

use crate::AppError;

use super::{
    bounded_rect::BoundingRect,
    vertex::{MeshGenerator, VertexPC, VertexPT},
};

pub struct Mesh<T: Copy + glium::Vertex> {
    vertices: glium::VertexBuffer<T>,
    indices: glium::IndexBuffer<u32>,
}

impl<T: Copy + glium::Vertex> Mesh<T> {
    pub fn build_quad(
        display: &glium::Display,
        bounding_box: BoundingRect,
    ) -> Result<Mesh<VertexPT>, AppError> {
        let (vertices, indices) =
            VertexPT::quad(bounding_box.width * 0.5, bounding_box.height * 0.5);

        Ok(Mesh {
            vertices: glium::VertexBuffer::new(display, vertices.as_slice())?,
            indices: glium::IndexBuffer::new(
                display,
                PrimitiveType::TrianglesList,
                indices.as_slice(),
            )?,
        })
    }

    pub fn build_ring(
        display: &glium::Display,
        inner: f32,
        outer: f32,
        resolution: u32,
    ) -> Result<Mesh<VertexPC>, AppError> {
        let (vertices, indices) = VertexPC::ring(inner, outer, resolution);

        Ok(Mesh {
            vertices: glium::VertexBuffer::new(display, vertices.as_slice())?,
            indices: glium::IndexBuffer::new(
                display,
                PrimitiveType::TrianglesList,
                indices.as_slice(),
            )?,
        })
    }

    pub fn build_polygon(display: &glium::Display) -> Result<Mesh<VertexPC>, AppError> {
        let (vertices, indices) = ([], []);
        Ok(Mesh {
            vertices: glium::VertexBuffer::new(display, &vertices)?,
            indices: glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices)?,
        })
    }

    pub fn update_vertices(
        &mut self,
        display: &glium::Display,
        vertices: &[T],
    ) -> Result<(), AppError> {
        self.vertices = glium::VertexBuffer::new(display, &vertices)?;
        Ok(())
    }

    pub fn render<Q: Uniforms>(&self, frame: &mut glium::Frame, uniforms: Q, program: &Program) {
        frame
            .draw(
                &self.vertices,
                &self.indices,
                program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
