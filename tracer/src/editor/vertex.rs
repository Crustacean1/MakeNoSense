use glad_gl::gl;
use glium::implement_vertex;
use std::{self, mem};
use vertex_buffer_macro_derive::{Vertex, VertexAttribute};

const PI: f32 = 3.1415926535;

pub struct IndexBuffer<const T: usize> {
    pub indices: Vec<[u32; T]>,
}

pub struct VertexBuffer<T> {
    pub vertices: Vec<T>,
}

pub trait MeshGenerator {
    type Vertex;
    fn quad(width: f32, height: f32) -> (Vec<Self::Vertex>, Vec<u32>);
    fn ring(inner: f32, outer: f32, res: u32) -> (Vec<Self::Vertex>, Vec<u32>);
}

trait VertexAttribute {
    fn get_size() -> usize;
    fn get_field_count() -> usize;
}

pub enum MeshType {
    Triangles,
    Lines,
    LineStrip,
    Points,
}

fn quad_ind() -> Vec<u32> {
    vec![0, 1, 3, 2, 3, 0]
}

fn ring_ind(res: u32) -> Vec<u32> {
    (0..res)
        .map(|i| {
            [
                (i * 2 + 0) % (res * 2),
                (i * 2 + 1) % (res * 2),
                (i * 2 + 2) % (res * 2),
                (i * 2 + 1) % (res * 2),
                (i * 2 + 2) % (res * 2),
                (i * 2 + 3) % (res * 2),
            ]
        })
        .flatten()
        .collect()
}

impl<T> VertexBuffer<T> {
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
#[derive(Copy, Clone)]
pub struct VertexPC {
    pub pos: [f32; 2],
    pub col: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexPT {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

implement_vertex!(VertexPT, pos, tex);

fn quad_pos(width: f32, height: f32, i: u32) -> [f32; 2] {
    [
        if i / 2 == 0 { -width } else { width },
        if i % 2 == 0 { -height } else { height },
    ]
}

fn ring_pos(i: u32, inner: f32, outer: f32, res: u32) -> [f32; 2] {
    let angle = 2.0 * PI * ((i / 2) as f32 / (res as f32));
    let radius = if i % 2 == 0 { inner } else { outer };
    [angle.cos() * radius, angle.sin() * radius]
}

fn quad_tex(i: u32) -> [f32; 2] {
    [
        if i / 2 == 0 { 0.0 } else { 1.0 },
        if i % 2 == 0 { 1.0 } else { 0.0 },
    ]
}

fn ring_tex(i: u32, res: u32) -> [f32; 2] {
    let normalized_angle = (i / 2) as f32 / (res as f32);
    if i % 2 == 0 {
        [normalized_angle, 0.0]
    } else {
        [normalized_angle, 1.0]
    }
}

impl MeshGenerator for VertexPC {
    type Vertex = VertexPC;
    fn quad(width: f32, height: f32) -> (Vec<Self::Vertex>, Vec<u32>) {
        let vertices = (0..4)
            .map(|i| VertexPC {
                pos: quad_pos(width, height, i),
                col: [1.0, 1.0, 1.0, 1.0],
            })
            .collect();

        (vertices, quad_ind())
    }

    fn ring(inner: f32, outer: f32, res: u32) -> (Vec<VertexPC>, Vec<u32>) {
        let vertices = (0..res * 2)
            .map(|i| VertexPC {
                pos: ring_pos(i, inner, outer, res),
                col: [1.0, 1.0, 1.0, 1.0],
            })
            .collect();

        (vertices, ring_ind(res))
    }
}

impl MeshGenerator for VertexPT {
    type Vertex = VertexPT;
    fn quad(width: f32, height: f32) -> (Vec<VertexPT>, Vec<u32>) {
        let vertices = (0..4)
            .map(|i| VertexPT {
                pos: quad_pos(width, height, i),
                tex: quad_tex(i),
            })
            .collect();

        (vertices, quad_ind())
    }

    fn ring(inner: f32, outer: f32, res: u32) -> (Vec<VertexPT>, Vec<u32>) {
        let vertices = (0..res * 2)
            .map(|i| VertexPT {
                pos: ring_pos(i, inner, outer, res),
                tex: ring_tex(i, res),
            })
            .collect();

        (vertices, ring_ind(res))
    }
}
