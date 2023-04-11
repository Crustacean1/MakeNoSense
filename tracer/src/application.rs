//TODO: delete shaders after use
use glad_gl::gl;

use crate::{
    mesh::{Mesh, MeshType},
    shader::ShaderProgram,
    vertex::{Color, MeshGenerator, VertexPC},
};

pub trait Application {
    fn on_init(&mut self) {}
    fn on_click(&mut self, x: f32, y: f32, key: u32) {}
    fn on_mouse(&mut self, x: f64, y: f64) {}
    fn on_key_down(&mut self, code: u32) {}
    fn on_key_up(&mut self, code: u32) {}
    fn on_exit(&mut self) {}

    fn get_title(&self) -> &'static str;
    fn get_resolution(&self) -> (u32, u32);
    fn render(&self) {}
}

struct GraphicsContext {
    ui_shader: ShaderProgram,
    ui_root: Mesh<VertexPC>,
}

impl GraphicsContext {
    fn build() -> Self {
        let ui_shader = match ShaderProgram::build(
            "tracer/shaders/ui_shader.vs",
            "tracer/shaders/ui_shader.fs",
        ) {
            Ok(shader) => shader,
            Err(error) => panic!("Failed to compile ui shader:\n{}", error.error_msg),
        };

        let (vertices, indices) = VertexPC::quad(0.5, Color(0.0, 0.5, 0.0));
        let ui_quad = Mesh::build(vertices, indices, MeshType::Triangles);

        GraphicsContext {
            ui_shader,
            ui_root: ui_quad,
        }
    }
}

pub struct Program {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    time: u32,
    graphics: Option<GraphicsContext>,
}

impl Program {
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
        Program {
            time: 0,
            title,
            width,
            height,
            graphics: None,
        }
    }
}

impl Application for Program {
    fn get_title(&self) -> &'static str {
        self.title
    }

    fn get_resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.9, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            if let Some(context) = self.graphics.as_ref() {
                context.ui_shader.use_program();
                context.ui_root.render();
            }
        }
    }

    fn on_init(&mut self) {
        println!("Compiling shaders...");
        self.graphics = Some(GraphicsContext::build());
    }

    fn on_click(&mut self, x: f32, y: f32, key: u32) {}
}
