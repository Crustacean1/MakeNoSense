use crate::{triangulator::triangulate, AppError};

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
}

impl UiLayer {
    pub fn new(layer_info: LayerInfo) -> Result<Self, AppError> {
        Ok(UiLayer {
            triangles: vec![],
            indices: vec![],
            layer_info,
            status: LayerStatus::New,
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
}
