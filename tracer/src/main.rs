use application::Program;
use window_context::WindowContext;

extern crate glad_gl;
extern crate glfw;

mod application;
mod ui_element;
mod window_context;
mod intersector;
mod vec;
mod linked_node;
mod triangulator;

fn main() {
    let mut program = Program::build("Final Solution", 1200, 800);

    let mut context = match WindowContext::build(&mut program) {
        Ok(context) => context,
        Err(msg) => {
            panic!("Failed to start application:\n{}", msg);
        }
    };

    context.run();
}
