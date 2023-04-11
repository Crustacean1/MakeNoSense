use vertex_buffer_macro_derive::{VertexBuffer, VertexAttribute};

trait VertexAttribute {
    fn get_size() -> usize;
    fn get_field_count() -> usize;
}

pub trait VertexBuffer {
    fn declare_layout() {}
}

pub trait MeshGenerator {
    type Vertex;
    fn quad(side: f32, color: Color) -> (Vec<Self::Vertex>, Vec<u32>);
}

#[derive(Copy, Clone, Debug, VertexAttribute)]
pub struct Position(pub f32, pub f32, pub f32);

#[derive(Copy, Clone, Debug, VertexAttribute)]
pub struct Color(pub f32, pub f32, pub f32);

#[derive(VertexBuffer, Debug)]
pub struct VertexPC {
    pub pos: Position,
    pub col: Color,
}

impl Position {
    fn quad(side: f32, i: u32) -> Position {
        Position(
            if i & 2 == 0 { -side } else { side },
            if (i + 1) & 2 == 0 { side } else { -side },
            1.0,
        )
    }
}

impl MeshGenerator for VertexPC {
    type Vertex = VertexPC;
    fn quad(side: f32, color: Color) -> (Vec<Self::Vertex>, Vec<u32>) {
        let vertices = (0..4)
            .map(|i| VertexPC {
                pos: Position::quad(side, i),
                col: color,
            })
            .collect();
        let indices = vec![0, 1, 2, 2, 3, 0];
        (vertices, indices)
    }
}
