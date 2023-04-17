use crate::{matrix::Matrix, vector::Vector3, AppError};

use super::{
    bounded_rect::BoundingRect,
    image::Image,
    shader_context::ShaderContext,
    vertex::{MeshGenerator, VertexPT},
};

use glium::{index::PrimitiveType, uniform};
use glium::{glutin, Surface};

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

pub struct UiImageEditor {
    image: Image,
    quad: (glium::VertexBuffer<VertexPT>, glium::IndexBuffer<u32>),

    bounding_box: BoundingRect,
    world_matrix: Matrix,
    sensitivity: f32,
}

impl UiImageEditor {
    pub fn new(
        filename: &str,
        display: &glium::Display,
        (x, y): (f32, f32),
        (width, height): (f32, f32),
    ) -> Result<UiImageEditor, AppError> {
        let (vertices, indices) = VertexPT::quad(width, height);
        let bounding_box = BoundingRect::new((x, y), (width, height));
        let image = Image::from_file(filename)?;
        let image_resolution = (image.width() as f32, image.height() as f32);
        let quad = Self::create_scaled_quad(display, bounding_box, image_resolution)?;

        Ok(UiImageEditor {
            quad,
            image,
            world_matrix: Matrix::translate(Vector3::new(x, y, 0.0)),
            bounding_box,
            sensitivity: 0.1,
        })
    }

    fn create_scaled_quad(
        display: &glium::Display,
        bounding_box: BoundingRect,
        img_resolution: (f32, f32),
    ) -> Result<(glium::VertexBuffer<VertexPT>, glium::IndexBuffer<u32>), AppError> {
        let quad_size = (bounding_box.width, bounding_box.height);

        let PhysicalSize { width, height } = display.gl_window().window().inner_size();
        let axis_scaling = (
            width as f32 / img_resolution.0,
            height as f32 / img_resolution.1,
        );

        let maximal_scaling = f32::min(axis_scaling.0, axis_scaling.1);

        Ok(Self::create_quad(
            display,
            (
                quad_size.0 * img_resolution.0 * maximal_scaling,
                quad_size.1 * img_resolution.1 * maximal_scaling,
            ),
        )?)
    }

    fn create_quad(
        display: &glium::Display,
        (width, height): (f32, f32),
    ) -> Result<(glium::VertexBuffer<VertexPT>, glium::IndexBuffer<u32>), AppError> {
        let (vertices, indices) = VertexPT::quad(width, height);

        Ok((
            glium::VertexBuffer::new(display, vertices.as_slice())?,
            glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, indices.as_slice())?,
        ))
    }

    fn scale_image(&mut self, (x, y): (f32, f32), factor: f32) {
        let scale = self.world_matrix.data[0][0];
        let factor = 1.0 + factor * self.sensitivity;

        if (scale > 0.2 || factor > 1.0) && (scale < 50.0 || factor < 1.0) {
            let mat = Matrix::translate(Vector3::new(-x, -y, 0.0))
                * Matrix::scale(factor)
                * Matrix::translate(Vector3::new(x, y, 0.0));
            self.world_matrix = self.world_matrix * mat;
        }
    }

    fn add_point(&mut self, pos: (f32, f32)) {
        /*let pos = self.transform_cursor_pos(pos);
        if let Some(latest_selection) = self.children.last_mut() {
            latest_selection.add_point(pos);
        }*/
    }

    fn update_cursor(&mut self, pos: (f32, f32)) {
        /*let pos = self.transform_cursor_pos(pos);
        if let Some(latest_selection) = self.children.last_mut() {
            latest_selection.update_cursor(pos);
        }*/
    }

    fn transform_cursor_pos(&self, pos: (f32, f32)) -> (f32, f32) {
        let (scale_pos, scale) = self.matrix_to_scale();
        let pos = (pos.0 - scale_pos.0, pos.1 - scale_pos.1);
        (pos.0 / scale, pos.1 / scale)
    }

    fn matrix_to_scale(&self) -> ((f32, f32), f32) {
        let matrix = &self.world_matrix.data;

        let scale = matrix[0][0];
        let vec = (matrix[3][0], matrix[3][1]);
        ((vec), scale)
    }

    fn add_selection(&mut self) {
        //self.children.push(Box::new(UiImageSelection::new()));
        //self.selection = Some((self.children.len() - 1) as u32);
    }

    fn render(&self, frame: &glium::Frame, context: &ShaderContext) {
        let uniforms = uniform! {
            world_matrix: self.world_matrix.data
        };

        frame.draw(
            self.quad.0,
            self.quad.1,
            &context.tex_shader,
            &uniforms,
            Default::default(),
        );
    }
}

/*impl UiElementInner for UiImageEditor {
    fn on_mouse_event(&mut self, pos: (f32, f32), event: MouseEvent) -> bool {
        match event {
            MouseEvent::Scroll(s) => self.scale_image(pos, s as f32),
            MouseEvent::LeftClick => self.add_point(pos),
            MouseEvent::Movement => self.update_cursor(pos),
            MouseEvent::RightClick => self.add_selection(),
            _ => (),
        }
        true
    }

    fn render(&self, context: &mut ShaderContext) {
        if context
            .tex_shader
            .set_matrix("world\x00", context.get_matrix())
        {
            self.image.bind(&mut context.tex_shader);
            self.quad.render();
        }
    }

    fn set_position(&mut self, pos: (f32, f32)) {
        self.pos = pos;
        self.world_matrix = Matrix::translate(pos.0, pos.1, 0.0) * self.world_matrix;
    }

    fn get_bounding_box(&self) -> super::BoundingRect {
        let (x, y) = self.pos;
        let (width, height) = self.size;
        let rect = BoundingRect {
            left: x - width,
            top: y + height,
            width: width * 2.0,
            height: height * 2.0,
        };
        rect
    }

    fn get_world_matrix(&self) -> &Matrix {
        &self.world_matrix
    }

    fn get_children<'b>(&'b self) -> Box<dyn Iterator<Item = &dyn UiElement> + 'b> {
        Box::new(self.children.iter().map(|child| &**child as &dyn UiElement))
    }

    fn get_children_mut<'b>(&'b mut self) -> Box<dyn Iterator<Item = &mut dyn UiElement> + 'b> {
        Box::new(
            self.children
                .iter_mut()
                .map(|child| &mut **child as &mut dyn UiElement),
        )
    }
}
*/
