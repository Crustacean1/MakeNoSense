use glium::{glutin::dpi::PhysicalSize, Frame, Program};

use crate::{image_processor::ImageProcessor, matrix::Matrix, vector::Vector3, AppError};

use super::{
    bounded_rect::BoundingRect, image_renderer::ImageRenderer, rendering_context::RenderingContext,
    shader::ShaderProgram, MouseEvent,
};

pub struct UiRoot {
    shaders: Vec<Program>,
    screen: BoundingRect,
    viewport: BoundingRect,
    aspect_matrix: Matrix,
    viewport_matrix: Matrix,

    image_editor: ImageRenderer,
}

impl UiRoot {
    pub fn build(
        filename: &str,
        viewport: BoundingRect,
        display: &glium::Display,
    ) -> Result<UiRoot, AppError> {
        let PhysicalSize { width, height } = display.gl_window().window().inner_size();
        let screen = BoundingRect {
            left: 0.0,
            top: 0.0,
            width: width as f32,
            height: height as f32,
        };

        let (aspect_matrix, viewport_matrix) = Self::get_viewport_martices(viewport, screen);

        let image_editor =
            ImageRenderer::new(filename, display, (viewport.width, viewport.height))?;

        let shaders = Self::build_shaders(display)?;

        let root = UiRoot {
            screen,
            viewport,
            shaders,
            viewport_matrix,
            aspect_matrix,
            image_editor,
        };

        Ok(root)
    }

    pub fn render(&mut self, image_processor: &ImageProcessor, frame: &mut Frame) {
        let mut context = RenderingContext::new(&self.shaders, frame);
        context.push(&self.aspect_matrix);
        context.push(&self.viewport_matrix);
        self.image_editor.render(image_processor, &mut context);
    }

    pub fn on_mouse_event(
        &mut self,
        image_processor: &mut ImageProcessor,
        pos: (f32, f32),
        event: MouseEvent,
    ) {
        //println!("Viewport: {:?}, pos: {:?}", self.viewport,pos);
        if self.viewport.contains(pos) {
            let cursor = Vector3::new(
                pos.0 - self.screen.width * 0.5,
                -(pos.1 - self.screen.height * 0.5),
                1.0,
            );
            let cursor = self.viewport_matrix.st_inverse() * cursor;
            let cursor = (cursor.x, cursor.y);
            self.image_editor
                .on_mouse_event(cursor, event, image_processor);
        }
    }

    fn build_shaders(display: &glium::Display) -> Result<Vec<Program>, AppError> {
        let col_shader = ShaderProgram::build(
            display,
            "tracer/shaders/col_shader.vs",
            "tracer/shaders/col_shader.fs",
        )?;

        let tex_shader = ShaderProgram::build(
            display,
            "tracer/shaders/tex_shader.vs",
            "tracer/shaders/tex_shader.fs",
        )?;
        Ok(vec![col_shader, tex_shader])
    }

    fn get_viewport_martices(viewport: BoundingRect, screen: BoundingRect) -> (Matrix, Matrix) {
        let (screen_width, screen_height) = (screen.width * 0.5, screen.height * 0.5);

        let screen_center = (screen_width, screen_height);
        let viewport_center = (
            viewport.left + viewport.width * 0.5,
            viewport.top + viewport.height * 0.5,
        );

        let translation = (
            viewport_center.0 - screen_center.0,
            viewport_center.1 - screen_center.1,
        );

        (
            Matrix::scale((1.0 / screen_width, 1.0 / screen_height)),
            Matrix::translate(Vector3::new(translation.0, translation.1, 1.0)),
        )
    }
}
