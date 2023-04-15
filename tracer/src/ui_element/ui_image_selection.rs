use crate::application::MouseEvent;

use super::{
    matrix::Matrix,
    mesh::Mesh,
    shader_context::ShaderContext,
    vertex::{Color, IndexBuffer, MeshType, Position, VertexBuffer, VertexPC},
    BoundingRect, UiElement, UiElementInner,
};

pub struct UiImageSelection {
    points: Mesh<VertexPC, 1>,
    children: Vec<Box<dyn UiElement>>,
    world_matrix: Matrix,
}

impl UiImageSelection {
    pub fn new() -> Self {
        let (vertices, indices) = (VertexBuffer::new(vec![]), IndexBuffer::new(vec![]));
        let points = Mesh::build(vertices, indices, MeshType::Points);
        UiImageSelection {
            points,
            children: vec![],
            world_matrix: Matrix::trans(0.0, 0.0, -0.1),
        }
    }

    pub fn add_point(&mut self, (x, y): (f32, f32)) {
        self.points.v_buffer.vertices.push(VertexPC {
            pos: Position(x, y, 0.0),
            col: Color(1.0, 0.4, 0.2),
        });

        let v_count = self.points.v_buffer.vertices.len();

        self.points.i_buffer.add_point([v_count as u32]);
        self.points.load();
    }
}

impl UiElementInner for UiImageSelection {
    fn on_mouse_event(&mut self, pos: (f32, f32), event: MouseEvent) -> bool {
        false
    }

    fn render(&self, context: &mut ShaderContext) {
        if context
            .col_shader
            .set_matrix("world\x00", context.get_matrix())
        {
            self.points.render();
        }
    }

    fn get_world_matrix(&self) -> &Matrix {
        &self.world_matrix
    }

    fn get_bounding_box(&self) -> super::BoundingRect {
        let vertices = &self.points.v_buffer.vertices;
        let x_axis = vertices.iter().map(|v| v.pos.0);
        let y_axis = vertices.iter().map(|v| v.pos.1);

        let left = x_axis
            .clone()
            .reduce(|x1, x2| if x1 < x1 { x1 } else { x2 })
            .unwrap_or(0.0);

        let right = x_axis
            .clone()
            .reduce(|x1, x2| if x1 > x1 { x1 } else { x2 })
            .unwrap_or(0.0);

        let top = y_axis
            .clone()
            .reduce(|x1, x2| if x1 > x1 { x1 } else { x2 })
            .unwrap_or(0.0);

        let bottom = y_axis
            .clone()
            .reduce(|x1, x2| if x1 < x1 { x1 } else { x2 })
            .unwrap_or(0.0);

        BoundingRect {
            left,
            top,
            width: right - left,
            height: top - bottom,
        }
    }

    fn set_position(&mut self, _pos: (f32, f32)) {
        panic!("God left me unfinished");
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
