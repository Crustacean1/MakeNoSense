use std::sync::mpsc::Receiver;

use glad_gl::gl;
use glfw::{ffi::GLFWwindow, Action, Context, Glfw, Key, Window, WindowEvent};

use crate::application::Application;

type Callback<T> = Box<dyn Fn(T) -> ()>;

pub enum MouseClick {
    Left(u32, u32),
    Right(u32, u32),
    Middle(u32, u32),
}

pub struct GraphicsContext<'a> {
    context: Glfw,
    window: Window,
    event_channel: Receiver<(f64, WindowEvent)>,
    application: &'a mut dyn Application,
}

impl<'a> GraphicsContext<'a> {
    pub fn build(application: &'a mut dyn Application) -> Result<Self, &'static str> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW context");
        let title = application.get_title();
        let (width, height) = application.get_resolution();

        let Some((mut window, events)) = glfw.create_window(
            width,
            height,
            title,
            glfw::WindowMode::Windowed,
        ) else {
            return Err("Failed to initialize GLFW window");
        };

        window.set_key_polling(true);
        window.make_current();

        gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

        Ok(GraphicsContext {
            context: glfw,
            window,
            event_channel: events,
            application,
        })
    }

    pub fn run(&mut self) {
        self.application.on_init();

        while !self.window.should_close() {
            self.context.poll_events();
            self.handle_events();
            self.application.render();
            self.window.swap_buffers();
        }

        self.application.on_exit();
    }

    fn handle_events(&mut self) {
        glfw::flush_messages(&self.event_channel)
            .for_each(|(_time, event)| Self::handle_window_event(self.application, event));
    }

    fn handle_window_event(application: &mut dyn Application, event: WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {}
            glfw::WindowEvent::Pos(x, y) => application.on_mouse(x, y),
            _ => (),
        }
    }
}
