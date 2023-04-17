use crate::AppError;

use super::{shader_context::ShaderContext, ui_image_editor::UiImageEditor};

pub struct UiRoot {
    context: ShaderContext,
    image_editor: UiImageEditor,
}

impl UiRoot {
    pub fn build(filename: &str, display: &glium::Display) -> Result<UiRoot, AppError> {
        let mut image_editor = UiImageEditor::new(filename, display, (0.0, 0.0), (0.99, 0.99))?;

        Ok(UiRoot {
            context: ShaderContext::build(display).expect("Failed to compile shaders"),
            image_editor,
        })
    }

    pub fn render(&mut self) {
        //self.image_editor.render(&mut self.context);
    }
}
