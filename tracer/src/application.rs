use std::io::Write;

use crate::ui_element::ui_root::UiRoot;
use glad_gl::gl;

#[derive(Debug)]
pub struct AppError {
    pub error_msg: String,
}

#[derive(Clone, Copy)]
pub enum MouseEvent {
    Movement,
    LeftClick,
    RightClick,
    Scroll(f32),
}

pub trait Application {
    fn on_init(&mut self) {}
    fn handle_event(&mut self, pos: (f32, f32), event: MouseEvent) {}

    fn get_title(&self) -> &'static str;
    fn get_resolution(&self) -> (u32, u32);
    fn render(&mut self) {}
}

pub struct Program {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    ui_root: Option<UiRoot>,
}

impl Program {
    pub fn build(title: &'static str, width: u32, height: u32) -> Self {
        std::io::stdout().flush().unwrap();
        Program {
            title,
            width,
            height,
            ui_root: None,
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

    fn render(&mut self) {
        if let Some(ui_root) = &mut self.ui_root {
            unsafe {
                gl::ClearColor(0.2, 0.2, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                ui_root.render();
            }
        }
    }

    fn on_init(&mut self) {
        self.ui_root = match UiRoot::build() {
            Ok(ui_root) => Some(ui_root),
            _ => panic!("Failed to initialize UI context"),
        }
    }

    fn handle_event(&mut self, pos: (f32, f32), event: MouseEvent) {
        if let Some(ui_root) = &mut self.ui_root {
            ui_root.handle(pos, event);
        }
    }
}
