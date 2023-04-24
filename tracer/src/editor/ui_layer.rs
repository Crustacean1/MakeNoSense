use crate::{
    triangulator::{contains_triangle, triangulate, Vec2},
    AppError,
};

use super::image_selection::LayerInfo;

pub enum LayerStatus {
    New,
    Finished,
}

pub struct UiLayer {
    indices: Vec<u32>,
    triangles: Vec<[u32; 3]>,
    layer_info: LayerInfo,
    status: LayerStatus,
    version: usize,
    id: usize,
}

impl UiLayer {
    pub fn new(id: usize, layer_info: LayerInfo) -> Result<Self, AppError> {
        Ok(UiLayer {
            triangles: vec![],
            indices: vec![],
            layer_info,
            status: LayerStatus::New,
            version: 0,
            id,
        })
    }

    pub fn contains(&self, vertices: &Vec<(f32, f32)>, (x, y): (f32, f32)) -> bool {
        self.triangles.iter().any(|&triangle| {
            contains_triangle(
                (
                    (Vec2::new(
                        vertices[triangle[0] as usize].0,
                        vertices[triangle[0] as usize].1,
                    )),
                    (Vec2::new(
                        vertices[triangle[1] as usize].0,
                        vertices[triangle[1] as usize].1,
                    )),
                    (Vec2::new(
                        vertices[triangle[2] as usize].0,
                        vertices[triangle[2] as usize].1,
                    )),
                ),
                Vec2::new(x, y),
            )
        })
    }

    pub fn add_node(&mut self, point: u32) {
        if let Some((i, _)) = self
            .indices
            .iter()
            .enumerate()
            .find(|(_, index)| **index == point)
        {
            match i {
                0 => self.status = LayerStatus::Finished,
                _ => (),
            }
        } else {
            self.indices.push(point);
        }
    }

    pub fn update(&mut self, nodes: &Vec<(f32, f32)>) {
        triangulate(nodes, &self.indices, &mut self.triangles);
        self.version += 1;
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn triangles(&self) -> &Vec<[u32; 3]> {
        &self.triangles
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }

    pub fn layer_info(&self) -> &LayerInfo {
        &self.layer_info
    }

    pub fn is_completed(&self) -> bool {
        match self.status {
            LayerStatus::Finished => true,
            _ => false,
        }
    }

    pub fn version(&self) -> usize {
        self.version
    }
}
