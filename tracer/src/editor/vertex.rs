use glium::implement_vertex;
use std;

use super::bounded_rect::BoundingBox;
const PI: f32 = 3.1415926535;

pub trait MeshGenerator {
    type Vertex;
    fn quad(bounding_box: BoundingBox<f32>) -> (Vec<Self::Vertex>, Vec<u32>);
    fn ring(inner: f32, outer: f32, res: u32) -> (Vec<Self::Vertex>, Vec<u32>);
}

trait VertexAttribute {
    fn get_size() -> usize;
    fn get_field_count() -> usize;
}

fn quad_ind() -> Vec<u32> {
    vec![0, 1, 2, 2, 3, 0]
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
implement_vertex!(VertexPC, pos, col);

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

fn ring_col(i: u32) -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

fn modulus(a: f32, b: f32) -> f32 {
    let i: i32 = (a / b) as i32;
    a - b * i as f32
}

impl MeshGenerator for VertexPC {
    type Vertex = VertexPC;
    fn quad(bounding_box: BoundingBox<f32>) -> (Vec<Self::Vertex>, Vec<u32>) {
        let vertices = vec![
            VertexPC {
                pos: [bounding_box.left, bounding_box.top],
                col: [1.0, 1.0, 1.0, 1.0],
            },
            VertexPC {
                pos: [bounding_box.right, bounding_box.top],
                col: [1.0, 1.0, 1.0, 1.0],
            },
            VertexPC {
                pos: [bounding_box.right, bounding_box.bottom],
                col: [1.0, 1.0, 1.0, 1.0],
            },
            VertexPC {
                pos: [bounding_box.left, bounding_box.bottom],
                col: [1.0, 1.0, 1.0, 1.0],
            },
        ];

        (vertices, quad_ind())
    }

    fn ring(inner: f32, outer: f32, res: u32) -> (Vec<VertexPC>, Vec<u32>) {
        let vertices = (0..res * 2)
            .map(|i| VertexPC {
                pos: ring_pos(i, inner, outer, res),
                col: ring_col(i),
            })
            .collect();

        (vertices, ring_ind(res))
    }
}

impl MeshGenerator for VertexPT {
    type Vertex = VertexPT;
    fn quad(bounding_box: BoundingBox<f32>) -> (Vec<VertexPT>, Vec<u32>) {
        let vertices = vec![
            VertexPT {
                pos: [bounding_box.left, bounding_box.top],
                tex: [0.0, 1.0],
            },
            VertexPT {
                pos: [bounding_box.right, bounding_box.top],
                tex: [1.0, 1.0],
            },
            VertexPT {
                pos: [bounding_box.right, bounding_box.bottom],
                tex: [1.0, 0.0],
            },
            VertexPT {
                pos: [bounding_box.left, bounding_box.bottom],
                tex: [0.0, 0.0],
            },
        ];

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
