use crate::{
    editor::{image_selection::LayerInfo, ui_layer::UiLayer},
    AppError,
};
use rand::Rng;

pub mod layer_renderer;

#[derive(Debug)]
pub enum EditorEvent {
    NewLayer(usize),
    LayerSelected(usize),
    PointSelected(usize),
    NewPoint((f32, f32)),
    PointMoved(u32, (f32, f32)),
}

pub struct ImageProcessor {
    pub selected_layer_type: usize,
    pub selected_layer: Option<usize>,
    layer_types: Vec<LayerInfo>,
    layers: Vec<UiLayer>,
    nodes: Vec<(f32, f32)>,
}

impl ImageProcessor {
    pub fn new(layer_types: &[String]) -> Self {
        let layer_types = Self::generate_layer_types(layer_types);
        Self {
            layer_types,
            selected_layer_type: 0,
            selected_layer: None,
            layers: vec![],
            nodes: vec![],
        }
    }

    pub fn get_selected_layer(&self) -> Option<&UiLayer> {
        self.layers.get(self.selected_layer?)
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

    pub fn handle_event(&mut self, event: EditorEvent) {
        println!("Handlin: {:?}", event);
        match event {
            EditorEvent::NewPoint(node) => {
                self.on_add_node(node);
            }
            EditorEvent::NewLayer(id) => {
                self.on_add_layer(id);
            }
            EditorEvent::PointMoved(index, pos) => {
                self.on_move_node(index as usize, pos);
            }
            EditorEvent::PointSelected(index) => {
                self.on_select_node(index);
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
        if let Some(layer) = self.layers.get(self.selected_layer?) {
            if node_index as u32 == *layer.indices().first()? {
                self.selected_layer = None;
            }
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

    fn on_add_node(&mut self, node: (f32, f32)) -> Option<()> {
        match self.selected_layer {
            Some(selected_layer) => {
                if let Some(layer) = self.layers.get_mut(selected_layer) {
                    self.nodes.push(node);
                    layer.add_point(self.nodes.len() as u32 - 1);
                }
                Some(())
            }
            None => {
                if let Some(layer_type) = self.layer_types.get(self.selected_layer_type) {
                    if let Ok(mut new_layer) = UiLayer::new(layer_type.clone()) {
                        self.nodes.push(node);
                        new_layer.add_point(self.nodes.len() as u32 - 1);
                        self.layers.push(new_layer);
                        self.selected_layer = Some(self.layers.len() - 1);
                    }
                }
                Some(())
            }
        }
    }

    fn on_add_layer(&mut self, type_id: usize) -> Option<()> {
        let layer_info = self.layer_types.get(type_id)?;
        if let Ok(layer_info) = UiLayer::new(layer_info.clone()) {
            self.layers.push(layer_info);
            self.selected_layer = Some(self.layers.len() - 1);
            Some(())
        } else {
            None
        }
    }
}
