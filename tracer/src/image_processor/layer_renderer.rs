use glium::{
    index::PrimitiveType, uniform, Blend, BlendingFunction, Display, DrawParameters, IndexBuffer,
    LinearBlendingFactor, Surface, VertexBuffer,
};

use crate::editor::{rendering_context::RenderingContext, ui_layer::UiLayer, vertex::VertexPC};

pub fn render_layer(
    display: &Display,
    layer: &UiLayer,
    vertex_buffer: &VertexBuffer<VertexPC>,
    context: &mut RenderingContext,
) {
    let &matrix = context.get_matrix();
    let draw_parameters = draw_parameters();

    let area_index_buffer = IndexBuffer::new(
        display,
        PrimitiveType::TrianglesList,
        layer.triangle_indices(),
    );
    let outline_index_buffer = IndexBuffer::new(display, PrimitiveType::LineLoop, layer.indices());
    let mut color = layer.layer_info().color;

    if let Some((col_shader, frame)) = context.shader_context(0) {
        color[3] = 0.3;
        let uniforms = uniform! {
            world: matrix.data,
            ufCol: color
        };

        if let Ok(area_index_buffer) = area_index_buffer {
            frame
                .draw(
                    vertex_buffer,
                    &area_index_buffer,
                    col_shader,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }

        color[3] = 1.0;
        let uniforms = uniform! {
            world: matrix.data,
            ufCol: color
        };

        if let Ok(outline_index_buffer) = outline_index_buffer {
            frame
                .draw(
                    vertex_buffer,
                    &outline_index_buffer,
                    col_shader,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }
    }
}

fn draw_parameters<'a>() -> DrawParameters<'a> {
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
    draw_parameters.line_width = Some(4.0);

    draw_parameters
}
