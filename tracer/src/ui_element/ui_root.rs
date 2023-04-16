use crate::{
    application::{AppError, MouseEvent},
    vec::Vec2,
};

use super::{
    matrix::Matrix, shader_context::ShaderContext, ui_dropdown::Dropdown, ui_group::UiGroup,
    ui_image_editor::UiImageEditor, vertex::Color, UiElement,
};

pub struct UiRoot {
    context: ShaderContext,
    toolbox: Box<dyn UiElement>,
    image_editor: Box<dyn UiElement>,
}

impl UiRoot {
    pub fn build() -> Result<Self, AppError> {
        let dropdown = Dropdown::new(
            Vec2::new((0.0, 0.125)),
            Vec2::new((0.15, 0.04)),
            vec!["jp2", "gmd"],
        );

        let toolbox = UiGroup::new(
            Vec2::new((1.25, 0.0)),
            Vec2::new((0.25, 0.5)),
            2,
            Color(0.1, 0.3, 0.7, 1.0),
            vec![Box::new(dropdown)],
        );

        let image_editor = UiImageEditor::new((0.0, 0.0), (0.99, 0.99), (1.2 / 0.8, 1.0));
        let image_editor = Box::new(image_editor.load_image("tracer/images/boomer.jpg")?);

        Ok(UiRoot {
            context: ShaderContext::build((1200, 800)).expect("Failed to compile shaders"),
            toolbox,
            image_editor,
        })
    }

    pub fn render(&mut self) {
        self.image_editor.render(&mut self.context);
        self.toolbox.render(&mut self.context);
    }

    pub fn handle(&mut self, pos: (f32, f32), event: MouseEvent) {
        if !self.toolbox.handle_mouse_event(pos, event) {
            self.image_editor.handle_mouse_event(pos, event);
        }
    }
}
