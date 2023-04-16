use glad_gl::gl;

use std::{mem, ptr};

use super::vertex::{IndexBuffer, MeshType, Vertex, VertexBuffer};

pub struct Mesh<T, const N: usize>
where
    T: Vertex,
{
    vao: u32,
    ebo: u32,
    vbo: u32,

    pub v_buffer: VertexBuffer<T>,
    pub i_buffer: IndexBuffer<N>,
    mesh_type: MeshType,
}

impl<T, const N: usize> Mesh<T, N>
where
    T: Vertex,
{
    pub fn build(vertices: VertexBuffer<T>, indices: IndexBuffer<N>, mesh_type: MeshType) -> Self {
        let (vbo, ebo, vao) = Self::create();

        let mesh = Mesh {
            ebo,
            vbo,
            vao,
            v_buffer: vertices,
            i_buffer: indices,
            mesh_type,
        };

        mesh.load();
        T::declare_layout();

        mesh
    }

    pub fn load(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.v_buffer.size() as isize,
                mem::transmute(self.v_buffer.as_ptr()),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                self.i_buffer.size() as isize,
                mem::transmute(self.i_buffer.as_ptr()),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                self.mesh_type.into_gl(),
                self.i_buffer.index_count() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }

    fn create() -> (u32, u32, u32) {
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
        }
        (vbo, ebo, vao)
    }
}
