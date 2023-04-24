#[derive(Clone, Copy, Debug)]
pub struct BoundingRect {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
}

impl BoundingRect {
    pub fn from_centered_quad((x, y): (f32, f32), (width, height): (f32, f32)) -> Self {
        BoundingRect {
            top: x + height,
            left: y - width,
            width: 2.0 * width,
            height: 2.0 * height,
        }
    }

    pub fn from_quad((left, top): (f32, f32), (width, height): (f32, f32)) -> Self {
        BoundingRect {
            top,
            left,
            width,
            height,
        }
    }

    pub fn contains(&self, pos: (f32, f32)) -> bool {
        pos.0 > self.left
            && pos.1 > self.top
            && pos.0 - self.left < self.width
            && pos.1 - self.top < self.height
    }
}

/*mod image;
mod mesh;
mod ui_dropdown;
mod ui_group;

pub enum EditMode {
    Edit,
    Preview,
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
*/
