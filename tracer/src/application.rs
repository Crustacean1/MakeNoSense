use glad_gl::gl;

use crate::shader::ShaderProgram;

pub trait Application {
    fn on_init(&mut self) {}
    fn on_click(&mut self, x: i32, y: i32, key: u32) {}
    fn on_mouse(&mut self, x: i32, y: i32) {}
    fn on_key_down(&mut self, code: u32) {}
    fn on_key_up(&mut self, code: u32) {}
    fn on_exit(&mut self) {}

    fn get_title(&self) -> &'static str;
    fn get_resolution(&self) -> (u32, u32);
    fn render(&self) {}
}

pub struct Program {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
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
            gl::ClearColor(0.7, 0.9, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn on_init(&mut self) {
        println!("Compiling shaders...");
        let ui_shader = match ShaderProgram::build(
            "tracer/shaders/ui_shader.vs",
            "tracer/shaders/ui_shader.fs",
        ) {
            Ok(shader) => shader,
            Err(error) => panic!("Failed to compile ui shader:\n{}", error.error_msg),
        };


    }
}
