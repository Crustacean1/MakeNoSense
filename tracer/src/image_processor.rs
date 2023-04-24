use crate::editor::{image_selection::LayerInfo, ui_layer::UiLayer};
use rand::Rng;

use self::mask_builder::build_mask;

pub mod layer_renderer;
pub mod layer_vertex_buffer;
mod mask_builder;

#[derive(Debug)]
pub enum EditorEvent {
    NewLayer(usize),
    PointSelected(usize),
    NewPoint((f32, f32)),
    PointMoved(u32, (f32, f32)),
    Save(f32),
}

pub struct ImageProcessor {
    pub selected_layer_type: usize,
    pub selected_layer: Option<usize>,
    layer_types: Vec<LayerInfo>,
    layers: Vec<UiLayer>,
    total_layer_count: usize,
    nodes: Vec<(f32, f32)>,
    resolution: (u32, u32),
}

impl ImageProcessor {
    pub fn new(resolution: (u32, u32), layer_types: &[String]) -> Self {
        let layer_types = Self::generate_layer_types(layer_types);
        Self {
            layer_types,
            resolution,
            selected_layer_type: 0,
            selected_layer: None,
            layers: vec![],
            nodes: vec![],
            total_layer_count: 0,
        }
    }

    pub fn layer_types(&self) -> &Vec<LayerInfo> {
        &self.layer_types
    }

    pub fn layers(&self) -> &Vec<UiLayer> {
        &self.layers
    }

    pub fn nodes(&self) -> &Vec<(f32, f32)> {
        &self.nodes
    }

    pub fn starting_node(&self) -> Option<(&u32, [f32; 4])> {
        if let Some(layer) = self.selected_layer() {
            if !layer.is_completed() {
                Some((layer.indices().first()?, layer.layer_info().color))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn handle_event(&mut self, event: EditorEvent) {
        match event {
            EditorEvent::NewPoint(node) => {
                self.on_new_node(node);
            }
            EditorEvent::PointMoved(index, pos) => {
                self.on_move_node(index as usize, pos);
            }
            EditorEvent::PointSelected(index) => {
                self.on_select_node(index);
            }
            EditorEvent::Save(scale) => {
                let image = build_mask(self.resolution, &self.nodes, &self.layers, scale);
                image.save("./result.png");
            }
            _ => {}
        }
        self.update_layers();
    }

    fn update_layers(&mut self) {
        self.layers
            .iter_mut()
            .for_each(|layer| layer.update(&self.nodes));
    }

    fn generate_layer_types(types: &[String]) -> Vec<LayerInfo> {
        let mut rng = rand::thread_rng();
        types
            .iter()
            .map(|name| LayerInfo {
                layer_type: name.clone(),
                color: [rng.gen(), rng.gen(), rng.gen(), 0.5],
            })
            .collect()
    }

    fn on_select_node(&mut self, node_index: usize) -> Option<()> {
        if let Some(selected_layer) = self.selected_layer_mut() {
            if selected_layer.is_completed() {
                self.add_layer();
                self.selected_layer_mut()?.add_node(node_index as u32);
            } else {
                selected_layer.add_node(node_index as u32);
            }
        } else {
            self.add_layer();
            self.selected_layer_mut()?.add_node(node_index as u32);
        }
        Some(())
    }

    fn on_move_node(&mut self, node_index: usize, new_node: (f32, f32)) -> Option<()> {
        match self.nodes.get_mut(node_index) {
            Some(node) => {
                *node = new_node;
                Some(())
            }
            None => None,
        }
    }

    fn on_new_node(&mut self, node: (f32, f32)) -> Option<()> {
        match self.selected_layer() {
            Some(layer) => {
                if layer.is_completed() {
                    self.add_layer();
                }
            }
            None => {
                self.add_layer();
            }
        }
        self.nodes.push(node);
        let new_node = self.nodes.len() - 1;
        self.selected_layer_mut()?.add_node(new_node as u32);
        Some(())
    }

    fn add_layer(&mut self) {
        if let Some(layer_type) = self.layer_types.get(self.selected_layer_type) {
            if let Ok(new_layer) = UiLayer::new(self.total_layer_count, layer_type.clone()) {
                self.total_layer_count += 1;
                self.layers.push(new_layer);
                self.selected_layer = Some(self.layers.len() - 1);
            }
        }
    }

    pub fn selected_layer_mut(&mut self) -> Option<&mut UiLayer> {
        self.layers.get_mut(self.selected_layer?)
    }

    pub fn selected_layer(&self) -> Option<&UiLayer> {
        self.layers.get(self.selected_layer?)
    }
}
