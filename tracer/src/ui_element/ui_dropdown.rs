use crate::vec::Vec2;

use super::{
    matrix::Matrix,
    mesh::Mesh,
    vertex::{IndexBuffer, MeshGenerator, MeshType, VertexPC},
    BoundingRect, UiElement, UiElementInner,
};

pub struct Dropdown {
    options: Vec<&'static str>,
    world_matrix: Matrix,
    bound_rect: BoundingRect,
    children: Vec<Box<dyn UiElement>>,

    button: Mesh<VertexPC, 3>,
    //option_buttons: Mesh<VertexPC, 3>,
}

impl Dropdown {
    pub fn new(pos: Vec2, size: Vec2, options: Vec<&'static str>) -> Self {
        let (vertices, indices) = VertexPC::quad(size.x, size.y);
        let button = Mesh::build(vertices, indices, MeshType::Triangles);

        Dropdown {
            options,
            world_matrix: Matrix::ident(),
            bound_rect: BoundingRect::new(pos, size),
            children: vec![],
            button,
        }
    }
}

impl UiElementInner for Dropdown {
    fn on_mouse_event(&mut self, pos: (f32, f32), event: crate::application::MouseEvent) -> bool {
        false
    }

    fn get_world_matrix(&self) -> &super::matrix::Matrix {
        &self.world_matrix
    }

    fn render(&self, context: &mut super::shader_context::ShaderContext) {
        self.button.render();
    }

    fn set_position(&mut self, pos: (f32, f32)) {}

    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn super::UiElement> + 'a> {
        Box::new(self.children.iter().map(|child| &**child as &dyn UiElement))
    }

    fn get_children_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &mut dyn super::UiElement> + 'a> {
        Box::new(
            self.children
                .iter_mut()
                .map(|child| &mut **child as &mut dyn UiElement),
        )
    }

    fn get_bounding_box(&self) -> super::BoundingRect {
        self.bound_rect
    }
}
