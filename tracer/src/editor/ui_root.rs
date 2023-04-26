use glium::{glutin::dpi::PhysicalSize, Frame, Program};
use image::DynamicImage;

use crate::{image_processor::ImageProcessor, matrix::Matrix, vector::Vector3, AppError};

use super::{
    bounded_rect::BoundingBox, image_renderer::ImageRenderer, rendering_context::RenderingContext,
    shader::ShaderProgram, MouseEvent,
};

pub struct UiRoot {
    shaders: Vec<Program>,
    screen: BoundingBox<i32>,
    viewport: BoundingBox<i32>,
    aspect_matrix: Matrix,
    viewport_matrix: Matrix,

    image_editor: ImageRenderer,
}

impl UiRoot {
    pub fn build(
        image: &DynamicImage,
        viewport: BoundingBox<i32>,
        display: &glium::Display,
    ) -> Result<UiRoot, AppError> {
        let PhysicalSize { width, height } = display.gl_window().window().inner_size();
        let screen = BoundingBox::new(0, 0, width as i32, height as i32);

        let (aspect_matrix, viewport_matrix) = Self::get_viewport_martices(viewport, screen);

        let image_editor = ImageRenderer::new(
            image,
            display,
            (viewport.width() as i32, viewport.height() as i32),
        )?;

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
        self.image_editor
            .render(image_processor, &mut context)
            .unwrap();
    }

    pub fn on_mouse_event(
        &mut self,
        image_processor: &mut ImageProcessor,
        pos: (i32, i32),
        event: MouseEvent,
    ) {
        if self.viewport.contains(pos) {
            let center = (
                (self.screen.left + self.screen.right) as f32 / 2.0,
                (self.screen.top + self.screen.bottom) as f32 / 2.0,
            );

            let cursor = Vector3::new(pos.0 as f32 - center.0, center.1 - pos.1 as f32, 1.0);
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

    fn get_viewport_martices(
        viewport: BoundingBox<i32>,
        screen: BoundingBox<i32>,
    ) -> (Matrix, Matrix) {
        let (screen_half_width, screen_half_height) = (
            (screen.right - screen.left) as f32 / 2.0,
            (screen.bottom - screen.top) as f32 / 2.0,
        );

        let viewport_center = (
            (viewport.left + viewport.right) as f32 / 2.0,
            (viewport.top + viewport.bottom) as f32 / 2.0,
        );

        let screen_center = (
            (screen.left + screen.right) as f32 / 2.0,
            (screen.top + screen.bottom) as f32 / 2.0,
        );

        let translation = (
            viewport_center.0 - screen_center.0,
            viewport_center.1 - screen_center.1,
        );

        (
            Matrix::scale((
                1.0 / screen_half_width as f32,
                1.0 / screen_half_height as f32,
            )),
            Matrix::translate(Vector3::new(
                translation.0 as f32,
                translation.1 as f32,
                1.0,
            )),
        )
    }
}
