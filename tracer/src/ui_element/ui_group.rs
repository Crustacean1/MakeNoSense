use crate::application::MouseEvent;

use super::{
    matrix::Matrix,
    mesh::Mesh,
    shader_context::ShaderContext,
    vertex::{Color, MeshGenerator, MeshType, VertexPC},
    BoundingRect, UiElement, UiElementInner,
};

pub struct UiGroup {
    children: Vec<Box<dyn UiElement>>,
    bounding_rect: BoundingRect,
    mesh: Mesh<VertexPC, 3>,
    world_matrix: Matrix,
}

impl UiElementInner for UiGroup {
    fn render(&self, context: &mut ShaderContext) {
        if context
            .col_shader
            .set_matrix("world\x00", context.get_matrix())
        {
            self.mesh.render();
        }
    }

    fn set_position(&mut self, (x, y): (f32, f32)) {
        self.bounding_rect.left = x;
        self.bounding_rect.top = y;
    }

    fn get_bounding_box(&self) -> BoundingRect {
        self.bounding_rect
    }

    fn on_mouse_event(&mut self, _pos: (f32, f32), event: MouseEvent) -> bool {
        true
    }

    fn get_world_matrix(&self) -> &Matrix {
        &self.world_matrix
    }

    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &dyn UiElement> + 'a> {
        Box::new(self.children.iter().map(|child| &**child))
    }

    fn get_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut dyn UiElement> + 'a> {
        Box::new(
            self.children
                .iter_mut()
                .map(|child| &mut **child as &mut dyn UiElement),
        )
    }
}

impl UiGroup {
    pub fn new(
        pos: (f32, f32),
        size: (f32, f32),
        level: u32,
        color: Color,
        children: Vec<Box<dyn UiElement>>,
    ) -> Box<dyn UiElement> {
        let (mut v_buffer, i_buffer) = VertexPC::quad(size.0, size.1);
        v_buffer
            .vertices
            .iter_mut()
            .for_each(|VertexPC { col, .. }| *col = color);

        let mesh = Mesh::build(v_buffer, i_buffer, MeshType::Triangles);

        Box::new(UiGroup {
            children,
            bounding_rect: BoundingRect {
                left: pos.0 - size.0,
                top: pos.1 + size.1,
                width: size.0 * 2.0,
                height: size.1 * 2.0,
            },
            mesh,
            world_matrix: Matrix::trans(pos.0, pos.1, level as f32 * -0.1),
        })
    }
}
