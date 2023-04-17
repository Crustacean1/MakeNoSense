/*use crate::{application::MouseEvent, triangulator::Triangulator, vec::Vec2};

use super::{
    shader_context::ShaderContext,
    vertex::{Color, IndexBuffer, MeshGenerator, MeshType, Position, VertexBuffer, VertexPC},
    BoundingRect, UiElement, UiElementInner,
};

pub struct UiImageSelection {
    selection_area: Mesh<VertexPC, 3>,
    selection_point: Mesh<VertexPC, 3>,

    children: Vec<Box<dyn UiElement>>,
    world_matrix: Matrix,
    triangulator: Triangulator,
}

impl UiImageSelection {
    pub fn new() -> Self {
        let (vertices, indices) = (VertexBuffer::new(vec![]), IndexBuffer::new(vec![]));
        let selection_area = Mesh::build(vertices, indices, MeshType::Triangles);
        let (vertices, indices) = VertexPC::ring(0.01, 0.015, 20);
        let selection_point = Mesh::build(vertices, indices, MeshType::Triangles);

        UiImageSelection {
            selection_area,
            selection_point,
            children: vec![],
            world_matrix: Matrix::translate(0.0, 0.0, 0.0),
            triangulator: Triangulator::new(),
        }
    }

    pub fn update_cursor(&mut self, (x, y): (f32, f32)) {
        self.triangulator.update(Vec2::new((x, y)));
        self.update_mesh();
    }

    pub fn add_point(&mut self, (x, y): (f32, f32)) {
        self.triangulator.add(Vec2::new((x, y)));
        self.update_mesh();
    }

    fn update_mesh(&mut self) {
        match self.triangulator.triangulate() {
            Some((vertices, indices)) => {
                self.selection_area.v_buffer.vertices = vertices
                    .iter()
                    .map(|Vec2 { x, y }| VertexPC {
                        pos: Position(*x, *y),
                        col: Color(0.2, 0.5, 0.9, 0.5),
                    })
                    .collect();
                self.selection_area.i_buffer.indices = indices;
                self.selection_area.load();
            }
            None => {}
        }
    }
}

impl UiElementInner for UiImageSelection {
    fn on_mouse_event(&mut self, _pos: (f32, f32), _event: MouseEvent) -> bool {
        false
    }

    fn render(&self, context: &mut ShaderContext) {
        if context
            .col_shader
            .set_matrix("world\x00", context.get_matrix())
        {
            self.selection_area.render();

            let world_mat = *context.get_matrix();
            let aspect_mat = context.get_aspect_matrix();

            self.triangulator
                .get_points()
                .iter()
                .rev()
                .skip(1)
                .for_each(|point| {
                    let translation = world_mat * *point;
                    let mat = *aspect_mat * Matrix::translate(translation.x, translation.y, 0.0);

                    context.col_shader.set_matrix("world\x00", &mat);
                    self.selection_point.render();
                });
        }
    }

    fn get_world_matrix(&self) -> &Matrix {
        &self.world_matrix
    }

    fn get_bounding_box(&self) -> super::BoundingRect {
        let vertices = &self.selection_area.v_buffer.vertices;
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

    fn get_children<'b>(&'b self) -> Box<dyn Iterator<Item = &dyn UiElement> + 'b> {
        Box::new(self.children.iter().map(|child| &**child))
    }

    fn get_children_mut<'b>(&'b mut self) -> Box<dyn Iterator<Item = &mut dyn UiElement> + 'b> {
        Box::new(
            self.children
                .iter_mut()
                .map(|child| &mut **child as &mut dyn UiElement),
        )
    }
}
*/