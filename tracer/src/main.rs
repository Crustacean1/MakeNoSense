use application::Program;
use window_context::WindowContext;

extern crate glad_gl;
extern crate glfw;

mod application;
mod image;
mod mesh;
mod shader;
mod vertex;
mod window_context;

fn main() {
    let mut program = Program::new("Final Solution", 800, 600);

    let mut context = match WindowContext::build(&mut program) {
        Ok(context) => context,
        Err(msg) => {
            panic!("Failed to start application:\n{}", msg);
        }
    };

    context.run();
}
