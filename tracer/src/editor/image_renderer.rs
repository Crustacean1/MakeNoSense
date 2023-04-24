use std::collections::HashMap;

use glium::{uniform, Display};
use image::DynamicImage;

use crate::{
    image_processor::{
        layer_renderer::LayerRenderer, layer_vertex_buffer::LayerVertexBuffer, ImageProcessor,
    },
    matrix::Matrix,
    vector::Vector3,
    AppError,
};

use super::{
    bounded_rect::BoundingRect,
    image::Image,
    mesh::Mesh,
    rendering_context::RenderingContext,
    ui_layer::UiLayer,
    vertex::{VertexPC, VertexPT},
    MouseEvent,
};

impl From<glium::index::BufferCreationError> for AppError {
    fn from(value: glium::index::BufferCreationError) -> Self {
        AppError {
            error_msg: format!("Failed to create index buffer: {}", value.to_string()),
        }
    }
}

impl From<glium::vertex::BufferCreationError> for AppError {
    fn from(value: glium::vertex::BufferCreationError) -> Self {
        AppError {
            error_msg: format!("Failed to create vertex buffer: {}", value.to_string()),
        }
    }
}

pub struct ImageRenderer {
    image: Image,
    canvas: Mesh<VertexPT>,
    node: Mesh<VertexPC>,
    bounding_box: BoundingRect,
    world_matrix: Matrix,
    display: Display,
    sensitivity: f32,
    selected_node: Option<u32>,
    layer_renderers: HashMap<usize, LayerRenderer>,
    layer_vertex_buffer: LayerVertexBuffer,
    image_scaling: f32,
}

impl ImageRenderer {
    pub fn new(
        image: &DynamicImage,
        display: &glium::Display,
        (width, height): (f32, f32),
    ) -> Result<ImageRenderer, AppError> {
        let display = display.clone();
        let image = Image::from_file(&display, image)?;
        let image_resolution = (image.width() as f32, image.height() as f32);

        let viewport = (width as f32, height as f32);
        let bounding_box = Self::create_scaled_quad(viewport, image_resolution);

        let canvas = Mesh::<VertexPT>::build_quad(&display, bounding_box)?;
        let node = Mesh::<VertexPC>::build_ring(&display, 0.0, 5.0, 10)?;

        let world_matrix = Matrix::ident();
        let image_scaling = (image.width() as f32 / bounding_box.width)
            .max(image.height() as f32 / bounding_box.height);

        Ok(ImageRenderer {
            canvas,
            node,
            image,
            world_matrix,
            sensitivity: 0.1,
            selected_node: None,
            bounding_box,
            layer_renderers: HashMap::new(),
            layer_vertex_buffer: LayerVertexBuffer::build(&display)?,
            display,
            image_scaling,
        })
    }

    pub fn image_scale(&self) -> f32 {
        self.image_scaling
    }

    pub fn on_mouse_event(
        &mut self,
        cursor: (f32, f32),
        event: MouseEvent,
        image_processor: &mut ImageProcessor,
    ) {
        match event {
            MouseEvent::Scroll(scroll) => self.scale_image(cursor, scroll),
            MouseEvent::LeftClick(action) => {
                let cursor = self.transform_cursor_pos(cursor);
                match action {
                    super::MouseEventAction::Released => {
                        self.on_mouse_press(cursor, image_processor)
                    }
                    super::MouseEventAction::Pressed => {}
                }
            }
            MouseEvent::Movement => self.on_movement(cursor, image_processor),
            _ => {}
        }
    }

    fn create_scaled_quad(viewport: (f32, f32), image: (f32, f32)) -> BoundingRect {
        let scaling = (viewport.0 / image.0, viewport.1 / image.1);
        let max_scaling = f32::min(scaling.0, scaling.1);

        let (width, height) = (image.0 * max_scaling * 0.5, image.1 * max_scaling * 0.5);

        BoundingRect {
            left: -width,
            top: -height,
            width: 2.0 * width,
            height: 2.0 * height,
        }
    }

    fn scale_image(&mut self, (x, y): (f32, f32), factor: f32) {
        let scale = self.world_matrix.data[0][0];
        let factor = 1.0 + factor * self.sensitivity;

        if (scale > 0.2 || factor > 1.0) && (scale < 50.0 || factor < 1.0) {
            let mat = Matrix::translate(Vector3::new(x, y, 0.0))
                * Matrix::scale((factor, factor))
                * Matrix::translate(Vector3::new(-x, -y, 0.0));
            self.world_matrix = mat * self.world_matrix;
        }
    }

    fn on_mouse_press(&mut self, pos: (f32, f32), image_processor: &mut ImageProcessor) {
        match self.selected_node {
            Some(selected_node) => {
                image_processor.handle_event(crate::image_processor::EditorEvent::PointSelected(
                    selected_node as usize,
                ));
            }
            None => {
                let pos = self.clamp_node(pos);
                image_processor.handle_event(crate::image_processor::EditorEvent::NewPoint(pos));
            }
        }
    }

    fn on_movement(&mut self, pos: (f32, f32), image_processor: &mut ImageProcessor) {
        let cursor = Vector3::new(pos.0, pos.1, 1.0);

        let selected_node = image_processor
            .nodes()
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let node = self.world_matrix * Vector3::new(node.0, node.1, 1.0);
                ((node - cursor).sqr_dst() as i32, i)
            })
            .filter(|(dst, _)| *dst < 200)
            .min();

        self.selected_node = match selected_node {
            Some((_, i)) => Some(i as u32),
            None => None,
        };
    }

    fn transform_cursor_pos(&self, pos: (f32, f32)) -> (f32, f32) {
        let cursor1 = Vector3::new(pos.0, pos.1, 1.0);

        let inverse = self.world_matrix.st_inverse();
        let cursor = inverse * cursor1;
        (cursor.x, cursor.y)
    }

    fn clamp_node(&self, (x, y): (f32, f32)) -> (f32, f32) {
        let (min_x, max_x) = (
            self.bounding_box.left,
            self.bounding_box.left + self.bounding_box.width,
        );
        let (min_y, max_y) = (
            self.bounding_box.top,
            self.bounding_box.top + self.bounding_box.height,
        );

        (x.max(min_x).min(max_x), y.max(min_y).min(max_y))
    }

    pub fn render(
        &mut self,
        image_processor: &ImageProcessor,
        context: &mut RenderingContext,
    ) -> Result<(), AppError> {
        let image_matrix = *context.get_matrix() * self.world_matrix;

        let uniforms = uniform! {
            texture0: self.image.texture(),
            world: image_matrix.data
        };

        if let Some((tex_shader, frame)) = context.shader_context(1) {
            self.canvas.render(frame, uniforms, tex_shader);
        }

        context.push(&self.world_matrix);
        self.render_layers(context, image_processor.nodes(), image_processor.layers())?;
        context.pop();
        self.render_nodes(image_processor, context);
        Ok(())
    }

    fn render_nodes(&self, image_processor: &ImageProcessor, context: &mut RenderingContext) {
        let base_matrix = *context.get_matrix();

        let points = image_processor.nodes();
        let starting_point = image_processor.starting_node();

        if let Some((col_shader, frame)) = context.shader_context(0) {
            points.iter().enumerate().for_each(|(i, point)| {
                let translation = self.world_matrix * Vector3::new(point.0, point.1, 1.0);
                let mut point_matrix = base_matrix * Matrix::translate(translation);

                if let Some(index) = self.selected_node {
                    if index as usize == i {
                        point_matrix = point_matrix * Matrix::scale((1.5, 1.5));
                    }
                }

                let mut col: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

                if let Some((index, color)) = starting_point {
                    if *index as usize == i {
                        col = color;
                    }
                }

                let uniforms = uniform! {
                    world: point_matrix.data,
                    ufCol: col
                };
                self.node.render(frame, uniforms, col_shader);
            })
        }
    }

    fn render_layers(
        &mut self,
        context: &mut RenderingContext,
        vertices: &[(f32, f32)],
        layers: &[UiLayer],
    ) -> Result<(), AppError> {
        self.layer_vertex_buffer.reload(&self.display, vertices);

        for layer in layers {
            if let Some(layer_renderer) = self.layer_renderers.get_mut(&layer.id()) {
                layer_renderer.reload(&self.display, layer)?;
            } else {
                self.layer_renderers.insert(
                    layer.id(),
                    LayerRenderer::build(&self.display, layer.layer_info().clone())?,
                );
            }
        }

        for layer in layers {
            if let Some(layer_renderer) = self.layer_renderers.get_mut(&layer.id()) {
                layer_renderer.render(self.layer_vertex_buffer.vertex_buffer(), context);
            }
        }

        Ok(())
    }
}
