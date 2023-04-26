use glium::{uniform, Display, VertexBuffer};
use image::DynamicImage;

use crate::{
    image_processor::{layer_renderer::render_layer, ImageProcessor},
    matrix::Matrix,
    vector::Vector3,
    AppError,
};

use super::{
    bounded_rect::BoundingBox,
    image::Image,
    mesh::Mesh,
    rendering_context::RenderingContext,
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
    starting_node: Mesh<VertexPC>,
    display: Display,
    bounding_box: BoundingBox<f32>,
    world_matrix: Matrix,
    selected_node: Option<u32>,
    sensitivity: f32,
}

impl ImageRenderer {
    pub fn new(
        image: &DynamicImage,
        display: &glium::Display,
        (width, height): (i32, i32),
    ) -> Result<ImageRenderer, AppError> {
        let display = display.clone();
        let image = Image::from_file(&display, image)?;

        let image_box = BoundingBox::<f32>::new(
            (-(image.width() as i32) / 2) as f32,
            (-(image.height() as i32) / 2) as f32,
            (image.width() as i32 / 2) as f32,
            (image.height() as i32 / 2) as f32,
        );

        let canvas = Mesh::<VertexPT>::build_quad(&display, image_box)?;
        let node = Mesh::<VertexPC>::build_ring(&display, 0.0, 5.0, 10)?;
        let starting_node = Mesh::<VertexPC>::build_ring(&display, 5.0, 10.0, 10)?;

        let image_scaling =
            (width as f32 / image_box.width()).min(height as f32 / image_box.height());

        let world_matrix = Matrix::scale((image_scaling, image_scaling));

        Ok(ImageRenderer {
            canvas,
            node,
            starting_node,
            image,
            world_matrix,
            sensitivity: 0.1,
            selected_node: None,
            bounding_box: image_box,
            display,
        })
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
            .map(|&node| {
                let vertex = image_processor.vertices()[node];
                let vertex = self.world_matrix * Vector3::new(vertex.0, vertex.1, 1.0);
                ((vertex - cursor).sqr_dst() as i32, node)
            })
            .filter(|(dst, _)| *dst < 100)
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
        (
            x.clamp(self.bounding_box.left, self.bounding_box.right),
            y.clamp(self.bounding_box.top, self.bounding_box.bottom),
        )
    }

    pub fn render(
        &mut self,
        image_processor: &ImageProcessor,
        context: &mut RenderingContext,
    ) -> Result<(), AppError> {
        self.render_image(context);
        context.push(&self.world_matrix);
        self.render_layers(image_processor, context)?;
        context.pop();
        self.render_nodes(image_processor, context);

        Ok(())
    }

    fn render_image(&self, context: &mut RenderingContext) {
        let base_matrix = *context.get_matrix() * self.world_matrix;
        let uniforms = uniform! {
            texture0: self.image.texture(),
            world: base_matrix.data
        };

        if let Some((tex_shader, frame)) = context.shader_context(1) {
            self.canvas.render(frame, uniforms, tex_shader);
        }
    }

    fn render_nodes(&self, image_processor: &ImageProcessor, context: &mut RenderingContext) {
        let world_matrix = *context.get_matrix();
        let base_matrix = self.world_matrix;

        let indices: &[u32] = match image_processor.selected_layer() {
            Some(layer) => layer.indices(),
            None => &[],
        };

        let starting_point = match image_processor.selected_layer() {
            Some(layer) => match layer.indices().first() {
                Some(&point) => point as usize,
                None => image_processor.vertices().len(),
            },
            None => image_processor.vertices().len(),
        };

        let selected_node = match self.selected_node {
            Some(node) => node as usize,
            None => image_processor.vertices().len(),
        };

        let layer_color = if let Some(layer) = image_processor.selected_layer() {
            layer.layer_info().color
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };

        if let Some((col_shader, frame)) = context.shader_context(0) {
            image_processor
                .nodes()
                .iter()
                .map(|&node| (node, image_processor.vertices()[node]))
                .for_each(|(node, vertex)| {
                    let translation = base_matrix * Vector3::new(vertex.0, vertex.1, 1.0);
                    let point_matrix = if node == selected_node {
                        world_matrix * Matrix::translate(translation) * Matrix::scale((1.5, 1.5))
                    } else {
                        world_matrix * Matrix::translate(translation)
                    };

                    let node_color = if indices.iter().any(|&i| i as usize == node) {
                        layer_color
                    } else {
                        [1.0, 1.0, 1.0, 1.0]
                    };

                    let uniforms = uniform! {
                        world: point_matrix.data,
                        ufCol: node_color
                    };

                    if node == starting_point {
                        self.starting_node.render(frame, uniforms, col_shader);
                    } else {
                        self.node.render(frame, uniforms, col_shader);
                    }
                });
        }
    }

    fn render_layers(
        &mut self,
        image_processor: &ImageProcessor,
        context: &mut RenderingContext,
    ) -> Result<(), AppError> {
        let vertices: Vec<_> = image_processor
            .vertices()
            .iter()
            .map(|&(x, y)| VertexPC {
                pos: [x, y],
                col: [1.0, 1.0, 1.0, 1.0],
            })
            .collect();
        let vertices = VertexBuffer::new(&self.display, &vertices);
        if let Ok(vertices) = vertices {
            for layer in image_processor.layers() {
                render_layer(&self.display, layer, &vertices, context)
            }
        }
        Ok(())
    }
}
