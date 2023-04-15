use crate::application::{AppError, MouseEvent};

use super::{
    shader_context::ShaderContext, ui_group::UiGroup, ui_image_editor::UiImageEditor,
    vertex::Color, UiElement,
};

pub struct UiRoot {
    context: ShaderContext,
    toolbox: Box<dyn UiElement>,
    image_editor: Box<dyn UiElement>,
}

impl UiRoot {
    pub fn build() -> Result<Self, AppError> {
        let selection = UiGroup::new((0.05, 0.125), (0.10, 0.25), 3, Color(0.0, 0.2, 0.9), vec![]);

        let toolbox = UiGroup::new(
            (0.75, 0.0),
            (0.20, 0.5),
            2,
            Color(0.1, 0.3, 0.7),
            vec![selection],
        );

        let image_editor = UiImageEditor::new((0.0, 0.0), (0.99, 0.99), (1200, 800));
        let image_editor = Box::new(image_editor.load_image("tracer/images/ratings.png")?);

        Ok(UiRoot {
            context: ShaderContext::build().expect("Failed to compile shaders"),
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
