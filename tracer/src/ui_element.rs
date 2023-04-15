use std::cell::RefMut;

use crate::application::MouseEvent;

use self::{matrix::Matrix, shader::ShaderProgram, shader_context::ShaderContext};

pub mod shader;
pub mod shader_context;
pub mod ui_root;

mod image;
mod matrix;
mod mesh;
mod ui_group;
mod ui_image_editor;
mod ui_image_selection;
mod vertex;

#[derive(Clone, Copy)]
pub struct BoundingRect {
    top: f32,
    left: f32,
    width: f32,
    height: f32,
}

impl BoundingRect {
    pub fn contains(&self, pos: (f32, f32)) -> bool {
        pos.0 > self.left
            && pos.1 < self.top
            && pos.0 - self.left < self.width
            && self.top - pos.1 < self.height
    }
}

trait UiElementInner {
    fn on_mouse_event(&mut self, pos: (f32, f32), event: MouseEvent) -> bool;

    fn get_world_matrix(&self) -> &Matrix;
    fn render(&self, context: &mut ShaderContext);

    fn set_position(&mut self, pos: (f32, f32));

    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn UiElement> + 'a>;
    fn get_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut dyn UiElement> + 'a>;

    fn get_bounding_box(&self) -> BoundingRect;
}

pub trait UiElement {
    fn handle_mouse_event(&mut self, mouse: (f32, f32), event: MouseEvent) -> bool;

    fn render(&self, context: &mut ShaderContext);
    fn set_position(&mut self, pos: (f32, f32));

    fn get_bounding_box(&self) -> BoundingRect;
}

impl<T> UiElement for T
where
    T: UiElementInner,
{
    fn handle_mouse_event(&mut self, pos: (f32, f32), event: MouseEvent) -> bool {
        if self.get_bounding_box().contains(pos) {
            let event_handled = self
                .get_children_mut()
                .any(|child| child.handle_mouse_event(pos, event));
            event_handled || self.on_mouse_event(pos, event)
        } else {
            false
        }
    }

    fn render(&self, context: &mut ShaderContext) {
        context.push(self.get_world_matrix());

        self.render(context);
        self.get_children().for_each(|child| child.render(context));

        context.pop();
    }

    fn set_position(&mut self, pos: (f32, f32)) {
        self.set_position(pos);
    }

    fn get_bounding_box(&self) -> BoundingRect {
        self.get_bounding_box()
    }
}
