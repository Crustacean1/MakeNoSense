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
    triangle_indices: Vec<u32>,
    triangles: Vec<[u32; 3]>,
    layer_info: LayerInfo,
    status: LayerStatus,
    id: usize,
}

impl UiLayer {
    pub fn new(id: usize, layer_info: LayerInfo) -> Result<Self, AppError> {
        Ok(UiLayer {
            triangles: vec![],
            triangle_indices: vec![],
            indices: vec![],
            layer_info,
            status: LayerStatus::New,
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
        if let Some(i) = self.indices.iter().position(|index| *index == point) {
            if i == 0 {
                self.status = LayerStatus::Finished;
            } else if i == self.indices.len() - 1 {
                self.indices.remove(i);
            }
        } else {
            self.indices.push(point);
        }
    }

    pub fn update(&mut self, nodes: &Vec<(f32, f32)>) {
        triangulate(nodes, &self.indices, &mut self.triangles);
        self.update_triangle_indices();
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

    pub fn triangle_indices(&self) -> &Vec<u32> {
        &self.triangle_indices
    }

    fn update_triangle_indices(&mut self) {
        self.triangle_indices.clear();
        self.triangles.iter().for_each(|triangle| {
            self.triangle_indices.push(triangle[0]);
            self.triangle_indices.push(triangle[1]);
            self.triangle_indices.push(triangle[2]);
        })
    }
}
