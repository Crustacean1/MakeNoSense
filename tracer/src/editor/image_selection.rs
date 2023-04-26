#[derive(Clone)]
pub struct LayerInfo {
    pub layer_type: String,
    pub color: [f32; 4],
    pub id: usize,
}

impl LayerInfo {
    pub fn id(&self) -> usize {
        self.id
    }
}
