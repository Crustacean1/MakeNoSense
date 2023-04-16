use glad_gl::gl;
use std::{self, mem};
use vertex_buffer_macro_derive::{Vertex, VertexAttribute};

const PI: f32 = 3.1415926535;

pub struct IndexBuffer<const T: usize> {
    pub indices: Vec<[u32; T]>,
}

pub struct VertexBuffer<T: Vertex> {
    pub vertices: Vec<T>,
}

pub trait Vertex {
    fn declare_layout() {}
}

pub trait MeshGenerator {
    type Vertex: Vertex;
    fn quad(width: f32, height: f32) -> (VertexBuffer<Self::Vertex>, IndexBuffer<3>);
    fn ring(inner: f32, outer: f32, res: u32) -> (VertexBuffer<Self::Vertex>, IndexBuffer<3>);
}

trait VertexAttribute {
    fn get_size() -> usize;
    fn get_field_count() -> usize;
}

#[derive(Copy, Clone, Debug, VertexAttribute)]
pub struct Position(pub f32, pub f32);

#[derive(Copy, Clone, Debug, VertexAttribute)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

#[derive(Copy, Clone, Debug, VertexAttribute)]
pub struct Texture(pub f32, pub f32);

pub enum MeshType {
    Triangles,
    Lines,
    LineStrip,
    Points,
}

impl MeshType {
    pub fn into_gl(&self) -> u32 {
        match self {
            Self::Triangles => gl::TRIANGLES,
            Self::Lines => gl::LINES,
            Self::Points => gl::POINTS,
            Self::LineStrip => gl::LINE_STRIP,
        }
    }
}

impl<const T: usize> IndexBuffer<T> {
    pub fn new(indices: Vec<[u32; T]>) -> Self {
        IndexBuffer::<T> { indices }
    }

    pub fn as_ptr(&self) -> &u32 {
        unsafe { mem::transmute(self.indices.as_ptr()) }
    }

    pub fn count(&self) -> usize {
        self.indices.len()
    }

    pub fn index_count(&self) -> usize {
        self.indices.len() * T
    }

    pub fn size(&self) -> usize {
        self.indices.len() * mem::size_of::<[u32; T]>()
    }

    pub fn add_point(&mut self, point: [u32; T]) {
        self.indices.push(point)
    }

    pub fn quad() -> IndexBuffer<3> {
        IndexBuffer {
            indices: vec![[0, 1, 3], [2, 3, 0]],
        }
    }

    pub fn ring(res: u32) -> IndexBuffer<3> {
        let mut indices1: Vec<_> = (0..res)
            .map(|i| [i * 2, (i * 2 + 1) % (res * 2), (i * 2 + 2) % (res * 2)])
            .collect();
        let mut indices2: Vec<_> = (0..res)
            .map(|i| {
                [
                    (i * 2 + 1) % (res * 2),
                    (i * 2 + 2) % (res * 2),
                    (i * 2 + 3) % (res * 2),
                ]
            })
            .collect();

        indices1.append(&mut indices2);

        IndexBuffer { indices: indices1 }
    }
}

impl<T: Vertex> VertexBuffer<T> {
    pub fn new(vertices: Vec<T>) -> Self {
        VertexBuffer { vertices }
    }

    pub fn size(&self) -> usize {
        self.vertices.len() * mem::size_of::<T>()
    }

    pub fn as_ptr(&self) -> &T {
        unsafe { mem::transmute(self.vertices.as_ptr()) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Vertex, Debug)]
pub struct VertexPC {
    pub pos: Position,
    pub col: Color,
}

#[repr(C)]
#[derive(Copy, Clone, Vertex, Debug)]
pub struct VertexPT {
    pub pos: Position,
    pub tex: Texture,
}

impl Position {
    fn quad(width: f32, height: f32, i: u32) -> Position {
        Position(
            if i / 2 == 0 { -width } else { width },
            if i % 2 == 0 { -height } else { height },
        )
    }

    fn ring(i: u32, inner: f32, outer: f32, res: u32) -> Position {
        let angle = 2.0 * PI * ((i / 2) as f32 / (res as f32));
        let radius = if i % 2 == 0 { inner } else { outer };
        Position(angle.cos() * radius, angle.sin() * radius)
    }
}

impl Texture {
    fn quad(i: u32) -> Texture {
        Texture(
            if i / 2 == 0 { 0.0 } else { 1.0 },
            if i % 2 == 0 { 1.0 } else { 0.0 },
        )
    }
    fn ring(i: u32, res: u32) -> Texture {
        let normalized_angle = (i / 2) as f32 / (res as f32);
        if i % 2 == 0 {
            Texture(normalized_angle, 0.0)
        } else {
            Texture(normalized_angle, 1.0)
        }
    }
}

impl MeshGenerator for VertexPC {
    type Vertex = VertexPC;
    fn quad(width: f32, height: f32) -> (VertexBuffer<Self::Vertex>, IndexBuffer<3>) {
        let vertices = (0..4)
            .map(|i| VertexPC {
                pos: Position::quad(width, height, i),
                col: Color(1.0, 1.0, 1.0, 1.0),
            })
            .collect();

        (VertexBuffer::new(vertices), IndexBuffer::<3>::quad())
    }

    fn ring(inner: f32, outer: f32, res: u32) -> (VertexBuffer<Self::Vertex>, IndexBuffer<3>) {
        let vertices = (0..res * 2)
            .map(|i| VertexPC {
                pos: Position::ring(i, inner, outer, res),
                col: Color(1.0, 1.0, 1.0, 1.0),
            })
            .collect();

        (VertexBuffer::new(vertices), IndexBuffer::<3>::ring(res))
    }
}

impl MeshGenerator for VertexPT {
    type Vertex = VertexPT;
    fn quad(width: f32, height: f32) -> (VertexBuffer<Self::Vertex>, IndexBuffer<3>) {
        let vertices = (0..4)
            .map(|i| VertexPT {
                pos: Position::quad(width, height, i),
                tex: Texture::quad(i),
            })
            .collect();

        (VertexBuffer::new(vertices), IndexBuffer::<3>::quad())
    }

    fn ring(inner: f32, outer: f32, res: u32) -> (VertexBuffer<Self::Vertex>, IndexBuffer<3>) {
        let vertices = (0..res * 2)
            .map(|i| VertexPT {
                pos: Position::ring(i, inner, outer, res),
                tex: Texture::ring(i, res),
            })
            .collect();

        (VertexBuffer::new(vertices), IndexBuffer::<3>::ring(res))
    }
}
