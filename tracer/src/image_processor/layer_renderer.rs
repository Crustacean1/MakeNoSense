use glium::{
    index::PrimitiveType, uniform, Blend, BlendingFunction, Display, DrawParameters, IndexBuffer,
    LinearBlendingFactor, Surface, VertexBuffer,
};

use crate::{
    editor::{
        image_selection::LayerInfo, rendering_context::RenderingContext, ui_layer::UiLayer,
        vertex::VertexPC,
    },
    AppError,
};

pub struct LayerRenderer {
    area: Vec<u32>,
    area_index_buffer: IndexBuffer<u32>,
    outline: Vec<u32>,
    outline_index_buffer: IndexBuffer<u32>,
    layer_info: LayerInfo,
    version: usize,
}

impl LayerRenderer {
    pub fn build(display: &Display, layer_info: LayerInfo) -> Result<LayerRenderer, AppError> {
        let indices = [];
        Ok(LayerRenderer {
            area: vec![],
            outline: vec![],
            area_index_buffer: IndexBuffer::dynamic(
                display,
                PrimitiveType::TrianglesList,
                &indices,
            )?,

            outline_index_buffer: IndexBuffer::dynamic(display, PrimitiveType::LineLoop, &indices)?,
            layer_info,
            version: 0,
        })
    }

    pub fn render(&self, vertex_buffer: &VertexBuffer<VertexPC>, context: &mut RenderingContext) {
        let mut draw_parameters: DrawParameters = Default::default();

        draw_parameters.blend = Blend {
            color: BlendingFunction::Addition {
                source: LinearBlendingFactor::SourceAlpha,
                destination: LinearBlendingFactor::OneMinusSourceAlpha,
            },
            alpha: BlendingFunction::Addition {
                source: LinearBlendingFactor::SourceAlpha,
                destination: LinearBlendingFactor::OneMinusSourceAlpha,
            },
            constant_value: (1.0, 1.0, 1.0, 1.0),
        };

        let &matrix = context.get_matrix();

        if let Some((col_shader, frame)) = context.shader_context(0) {
            let uniforms = uniform! {
                world: matrix.data,
                ufCol: self.layer_info.color
            };

            frame
                .draw(
                    vertex_buffer,
                    &self.area_index_buffer,
                    col_shader,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();

            frame
                .draw(
                    vertex_buffer,
                    &self.outline_index_buffer,
                    col_shader,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }
    }

    pub fn reload(&mut self, display: &Display, layer: &UiLayer) -> Result<(), AppError> {
        self.try_reload_area(display, layer.triangles())?;
        self.version = layer.version();
        Ok(())
    }

    fn try_reload_area(
        &mut self,
        display: &Display,
        new_area: &[[u32; 3]],
    ) -> Result<(), AppError> {
        let new_buffer_size = Self::buffer_size(new_area.len() * 3);
        if new_buffer_size != self.area_index_buffer.len() {
            self.area = vec![0; new_buffer_size];
            self.load_area(new_area);
            self.area_index_buffer =
                IndexBuffer::dynamic(display, PrimitiveType::TrianglesList, &self.area)?;
        } else {
            self.load_area(new_area);
            self.area_index_buffer.write(&self.area);
        }
        Ok(())
    }

    fn load_area(&mut self, new_area: &[[u32; 3]]) {
        let mut i = 0;
        new_area.iter().flatten().for_each(|&index| {
            self.area[i] = index;
            i += 1;
        });
    }

    fn buffer_size(size: usize) -> usize {
        let mut i: usize = 1;
        while i < size {
            i <<= 1;
        }
        i
    }
}
