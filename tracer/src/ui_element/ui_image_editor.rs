use crate::{
    application::{AppError, MouseEvent},
    ui_element::vertex::Color,
};

use super::{
    image::Image,
    matrix::Matrix,
    mesh::Mesh,
    shader_context::ShaderContext,
    ui_image_selection::UiImageSelection,
    vertex::{MeshGenerator, MeshType, VertexPT},
    BoundingRect, EditMode, UiElement, UiElementInner,
};

pub struct UiImageEditor<'a> {
    image: Image,
    quad: Mesh<VertexPT, 3>,
    children: Vec<Box<UiImageSelection<'a>>>,
    world_matrix: Matrix,
    screen_resolution: (f32, f32),
    size: (f32, f32),
    pos: (f32, f32),
    sensitivity: f32,
    edit_mode: EditMode,
    selection: Option<u32>,
}

impl<'a> UiImageEditor<'a> {
    pub fn new(
        (x, y): (f32, f32),
        (width, height): (f32, f32),
        screen_resolution: (f32, f32),
    ) -> UiImageEditor<'a> {
        let (vertices, indices) = VertexPT::quad(width, height);
        let quad = Mesh::build(vertices, indices, MeshType::Triangles);

        UiImageEditor {
            image: Image::from_color(Color(0.9, 0.5, 0.1, 1.0)),
            screen_resolution,
            quad,
            pos: (x, y),
            size: (width, height),
            children: vec![],
            world_matrix: Matrix::translate(x, y, 0.0),
            sensitivity: 0.1,
            edit_mode: EditMode::Preview,
            selection: None,
        }
    }

    pub fn load_image(mut self, filename: &str) -> Result<UiImageEditor<'a>, AppError> {
        let image = Image::from_file(filename)?;

        self.update_resolution((image.width() as f32, image.height() as f32));
        self.image = image;

        Ok(self)
    }

    fn update_resolution(&mut self, img_resolution: (f32, f32)) {
        let axis_scaling = (
            self.screen_resolution.0 as f32 / img_resolution.0,
            self.screen_resolution.1 as f32 / img_resolution.1,
        );

        let maximal_scaling = f32::min(axis_scaling.0, axis_scaling.1);

        println!("resolution:  {} {} ", img_resolution.0, img_resolution.1);

        let (vertices, _) = VertexPT::quad(
            self.size.0 * img_resolution.0 * maximal_scaling,
            self.size.1 * img_resolution.1 * maximal_scaling,
        );
        self.quad.v_buffer = vertices;

        self.quad.load();
    }

    fn scale_image(&mut self, (x, y): (f32, f32), factor: f32) {
        let scale = self.world_matrix.data[0][0];
        let factor = 1.0 + factor * self.sensitivity;

        if (scale > 0.2 || factor > 1.0) && (scale < 50.0 || factor < 1.0) {
            let mat = Matrix::translate(-x, -y, 0.0)
                * Matrix::scale(factor)
                * Matrix::translate(x, y, 0.0);
            self.world_matrix = self.world_matrix * mat;
        }
    }

    fn add_point(&mut self, pos: (f32, f32)) {
        let pos = self.transform_cursor_pos(pos);
        if let Some(latest_selection) = self.children.last_mut() {
            latest_selection.add_point(pos);
        }
    }

    fn update_cursor(&mut self, pos: (f32, f32)) {
        let pos = self.transform_cursor_pos(pos);
        if let Some(latest_selection) = self.children.last_mut() {
            latest_selection.update_cursor(pos);
        }
    }

    fn transform_cursor_pos(&self, pos: (f32, f32)) -> (f32, f32) {
        let (scale_pos, scale) = self.matrix_to_scale();
        let pos = (pos.0 - scale_pos.0, pos.1 - scale_pos.1);
        (pos.0 / scale, pos.1 / scale)
    }

    fn matrix_to_scale(&self) -> ((f32, f32), f32) {
        let matrix = &self.world_matrix.data;

        let scale = matrix[0][0];
        let vec = (matrix[3][0], matrix[3][1]);
        ((vec), scale)
    }

    fn add_selection<'b: 'a>(&'b mut self) {
        self.children
            .push(Box::new(UiImageSelection::<'b>::new(&self.edit_mode)));
        self.selection = Some((self.children.len() - 1) as u32);
    }
}

impl<'a> UiElementInner for UiImageEditor<'a> {
    fn on_mouse_event(&mut self, pos: (f32, f32), event: MouseEvent) -> bool {
        match event {
            MouseEvent::Scroll(s) => self.scale_image(pos, s as f32),
            MouseEvent::LeftClick => self.add_point(pos),
            MouseEvent::Movement => self.update_cursor(pos),
            MouseEvent::RightClick => self.add_selection(),
            _ => (),
        }
        true
    }

    fn render(&self, context: &mut ShaderContext) {
        if context
            .tex_shader
            .set_matrix("world\x00", context.get_matrix())
        {
            self.image.bind(&mut context.tex_shader);
            self.quad.render();
        }
    }

    fn set_position(&mut self, pos: (f32, f32)) {
        self.pos = pos;
        self.world_matrix = Matrix::translate(pos.0, pos.1, 0.0) * self.world_matrix;
    }

    fn get_bounding_box(&self) -> super::BoundingRect {
        let (x, y) = self.pos;
        let (width, height) = self.size;
        let rect = BoundingRect {
            left: x - width,
            top: y + height,
            width: width * 2.0,
            height: height * 2.0,
        };
        rect
    }

    fn get_world_matrix(&self) -> &Matrix {
        &self.world_matrix
    }

    fn get_children<'b>(&'b self) -> Box<dyn Iterator<Item = &dyn UiElement> + 'b> {
        Box::new(self.children.iter().map(|child| &**child as &dyn UiElement))
    }

    fn get_children_mut<'b>(&'b mut self) -> Box<dyn Iterator<Item = &mut dyn UiElement> + 'b> {
        Box::new(
            self.children
                .iter_mut()
                .map(|child| &mut **child as &mut dyn UiElement),
        )
    }
}
