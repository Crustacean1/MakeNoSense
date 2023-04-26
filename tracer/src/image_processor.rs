use crate::editor::{image_selection::LayerInfo, ui_layer::UiLayer};

use self::{
    layer_image_exporter::build_mask, layer_json_exporter::save_coco, triangle::Segmentation,
};

mod layer_image_exporter;
mod triangle;

pub mod layer_json_exporter;
pub mod layer_renderer;

#[derive(Debug)]
pub enum EditorEvent {
    NewLayer(usize),
    PointSelected(usize),
    NewPoint((f32, f32)),
    Save,
}

pub struct ImageInfo {
    filename: String,
    resolution: (u32, u32),
}

pub struct ImageProcessor {
    pub selected_layer_type: usize,
    selected_layer_id: Option<usize>,
    layer_types: Vec<LayerInfo>,
    layers: Vec<UiLayer>,
    total_layer_count: usize,
    vertices: Vec<(f32, f32)>,
    nodes: Vec<usize>,
    image_info: ImageInfo,
}

impl ImageProcessor {
    pub fn new(filename: &str, resolution: (u32, u32), layer_types: &[(String, [f32; 4])]) -> Self {
        let layer_types = Self::generate_layer_types(layer_types);
        Self {
            layer_types,
            image_info: ImageInfo {
                filename: String::from(filename),
                resolution,
            },
            selected_layer_type: 0,
            selected_layer_id: None,
            layers: vec![],
            vertices: vec![],
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

    pub fn nodes(&self) -> &Vec<usize> {
        &self.nodes
    }

    pub fn vertices(&self) -> &Vec<(f32, f32)> {
        &self.vertices
    }

    pub fn handle_event(&mut self, event: EditorEvent) {
        match event {
            EditorEvent::NewPoint(node) => {
                self.on_new_node(node);
                self.update_selected_layer();
                self.prune_nodes();
            }
            EditorEvent::PointSelected(index) => {
                self.on_select_node(index);
                self.update_selected_layer();
                self.prune_nodes();
            }
            EditorEvent::Save => {
                let segmentations = self.create_segmentations();
                let base_name = Self::extract_base_filename(&self.image_info.filename);
                let base_name = base_name.unwrap();

                let mask_filename = format!("{}.mask.png", &base_name);

                match save_coco(
                    &self.image_info.filename,
                    self.image_info.resolution,
                    &segmentations,
                    &self.layer_types,
                ) {
                    Ok(_) => {
                        println!("Coco file saved");
                    }
                    Err(e) => {
                        println!("Failed to save Coco file: {}", e)
                    }
                }

                let image = build_mask(self.image_info.resolution, &segmentations);

                match image.save(mask_filename) {
                    Ok(_) => {
                        println!("Work saved");
                    }
                    Err(e) => {
                        println!("Failed to save file: {}", e.to_string());
                    }
                }
            }
            _ => {}
        }
    }

    fn extract_base_filename(filepath: &str) -> Option<String> {
        let path_segments = filepath.split("/");
        match path_segments.last() {
            Some(filename) => {
                let filename_segments = filename.split(".");
                let mut filename_parts: Vec<_> = filename_segments.collect();
                filename_parts.pop();
                Some(filename_parts.join("."))
            }
            _ => None,
        }
    }

    pub fn selected_layer_mut(&mut self) -> Option<&mut UiLayer> {
        match self.selected_layer_id {
            Some(selected_layer_id) => self
                .layers
                .iter_mut()
                .find(|layer| layer.id() == selected_layer_id),
            None => None,
        }
    }

    pub fn selected_layer(&self) -> Option<&UiLayer> {
        match self.selected_layer_id {
            Some(selected_layer_id) => self
                .layers
                .iter()
                .find(|layer| layer.id() == selected_layer_id),
            None => None,
        }
    }
    fn prune_nodes(&mut self) {
        self.nodes.clear();
        self.vertices.iter().enumerate().for_each(|(i, _)| {
            if self
                .layers
                .iter()
                .any(|layer| layer.indices().contains(&(i as u32)))
            {
                self.nodes.push(i);
            }
        })
    }

    fn update_selected_layer(&mut self) -> Option<()> {
        if let Some(selected_layer_id) = self.selected_layer_id {
            if let Some(layer) = self
                .layers
                .iter_mut()
                .find(|layer| layer.id() == selected_layer_id)
            {
                layer.update(&self.vertices);
                return Some(());
            }
        }
        None
    }

    fn generate_layer_types(types: &[(String, [f32; 4])]) -> Vec<LayerInfo> {
        types
            .iter()
            .enumerate()
            .map(|(i, (name, color))| LayerInfo {
                layer_type: name.clone(),
                color: *color,
                id: i + 1,
            })
            .collect()
    }

    fn on_select_node(&mut self, node_index: usize) -> Option<()> {
        if let Some(selected_layer) = self.selected_layer_mut() {
            selected_layer.add_node(node_index as u32);
            if selected_layer.is_completed() {
                self.selected_layer_id = None;
            }
        } else {
            self.new_layer();
            self.selected_layer_mut()?.add_node(node_index as u32);
        }
        Some(())
    }

    fn on_new_node(&mut self, node: (f32, f32)) -> Option<()> {
        match self.selected_layer() {
            Some(layer) => {
                if layer.is_completed() {
                    self.new_layer();
                }
            }
            None => {
                self.new_layer();
            }
        }
        self.vertices.push(node);
        let new_node = self.vertices.len() - 1;
        self.selected_layer_mut()?.add_node(new_node as u32);
        Some(())
    }

    fn new_layer(&mut self) {
        if let Some(layer_type) = self.layer_types.get(self.selected_layer_type) {
            if let Ok(new_layer) = UiLayer::new(self.total_layer_count, layer_type.clone()) {
                self.total_layer_count += 1;
                self.selected_layer_id = Some(new_layer.id());
                self.layers.push(new_layer);
            }
        }
    }

    fn create_segmentations(&self) -> Vec<Segmentation> {
        let (half_width, half_height) = self.image_info.resolution;
        let (half_width, half_height) = (half_width as f32 / 2.0, half_height as f32 / 2.0);

        let normalized_vertices: Vec<_> = self
            .vertices
            .iter()
            .map(|&v| (v.0 + half_width, v.1 + half_height))
            .collect();

        self.layers
            .iter()
            .map(|layer| Segmentation::from_layer(layer, &normalized_vertices))
            .collect()
    }
}
