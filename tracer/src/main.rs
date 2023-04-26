extern crate glium;

use std::{
    env::{self, Args},
    error::Error,
    fmt::{self, Display},
    process,
};

use editor::Editor;
use glium::glutin;

mod editor;
mod image_processor;
mod matrix;
mod triangulator;
mod vector;

#[derive(Debug, Clone)]
pub struct AppError {
    pub error_msg: String,
}

impl Display for AppError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        println!("Application encountered an error: {}", self.error_msg);
        Ok(())
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        "Application Error"
    }
}

fn main() {
    let mut args = env::args();
    match get_args(&mut args) {
        Some((image_filename, label_filename)) => {
            let event_loop = glutin::event_loop::EventLoopBuilder::with_user_event().build();

            let mut editor = match Editor::build(&image_filename, &label_filename, &event_loop) {
                Ok(editor) => editor,
                Err(error) => {
                    eprintln!("Failed to start application: {}", error.to_string());
                    process::exit(1);
                }
            };

            event_loop.run(move |event, _, control_flow| editor.main_loop(event, control_flow));
        }
        None => {
            println!("Invalid arguments\nUSAGE: ./tracer [image_file] [label_file]");
        }
    }
}

fn get_args(args: &mut Args) -> Option<(String, String)> {
    args.next()?;
    Some((args.next()?, args.next()?))
}
