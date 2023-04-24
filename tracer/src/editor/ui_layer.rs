use crate::{triangulator::triangulate, AppError};

use super::image_selection::LayerInfo;

pub struct UiLayer {
    indices: Vec<u32>,
    triangles: Vec<[u32; 3]>,
    layer_info: LayerInfo,
}

impl UiLayer {
    pub fn new(layer_info: LayerInfo) -> Result<Self, AppError> {
        Ok(UiLayer {
            triangles: vec![],
            indices: vec![],
            layer_info,
        })
    }

    pub fn add_point(&mut self, point: u32) {
        self.indices.push(point);
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
}
