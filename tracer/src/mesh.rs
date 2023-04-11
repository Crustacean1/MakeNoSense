use std::mem;

use glad_gl::gl;
use std::fmt::Debug;

use crate::vertex::VertexBuffer;

pub enum MeshType {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
}

impl MeshType {
    fn to_gl(&self) -> u32 {
        match self {
            Self::Points => gl::POINTS,
            Self::Lines => gl::LINES,
            Self::LineStrip => gl::LINE_STRIP,
            Self::Triangles => gl::TRIANGLES,
            Self::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}

pub struct Mesh<T>
where
    T: VertexBuffer,
{
    vao: u32,
    ebo: u32,
    vbo: u32,

    vertices: Vec<T>,
    indices: Vec<u32>,
    mesh_type: MeshType,
}

impl<T> Mesh<T>
where
    T: VertexBuffer + Debug,
{
    pub fn build(vertices: Vec<T>, indices: Vec<u32>, mesh_type: MeshType) -> Self {
        let (mut vbo, mut ebo, mut vao) = (0, 0, 0);

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<T>() * vertices.len()) as isize,
                mem::transmute(vertices.get_unchecked(0)),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mem::size_of::<u32>() * indices.len()) as isize,
                mem::transmute(indices.get_unchecked(0)),
                gl::STATIC_DRAW,
            );

            T::declare_layout();

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Mesh {
            vao,
            vbo,
            ebo,
            vertices,
            indices,
            mesh_type,
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                self.mesh_type.to_gl(),
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }
}
