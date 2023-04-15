use std::sync::mpsc::Receiver;

use glad_gl::gl;
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};

use crate::application::{Application, MouseEvent};

type Callback<T> = Box<dyn Fn(T) -> ()>;

pub enum MouseClick {
    Left(u32, u32),
    Right(u32, u32),
    Middle(u32, u32),
}

pub struct WindowContext<'a> {
    context: Glfw,
    window: Window,
    event_channel: Receiver<(f64, WindowEvent)>,
    application: &'a mut dyn Application,
}

impl<'a> WindowContext<'a> {
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
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);
        window.set_framebuffer_size_polling(true);
        window.make_current();

        gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

        unsafe {
            gl::PointSize(10.0);
            gl::Enable(gl::DEPTH_TEST);
        }

        Ok(WindowContext {
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
    }

    fn handle_events(&mut self) {
        glfw::flush_messages(&self.event_channel).for_each(|(_time, event)| {
            Self::handle_window_event(self.application, &mut self.window, event)
        });
    }

    fn handle_window_event(
        application: &mut dyn Application,
        window: &mut Window,
        event: WindowEvent,
    ) {
        let pos = window.get_cursor_pos();
        let resolution = application.get_resolution();
        let half_resolution = (resolution.0 as f32 * 0.5, resolution.1 as f32 * 0.5);
        let pos = (
            (pos.0 as f32 - half_resolution.0 as f32) / half_resolution.0,
            (half_resolution.1 as f32 - pos.1 as f32) / half_resolution.1,
        );

        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                application.handle_event((x as f32, y as f32), MouseEvent::Movement)
            }
            glfw::WindowEvent::MouseButton(button, action, _modifiers) => {
                if action == Action::Press {
                    match button {
                        glfw::MouseButtonLeft => {
                            application.handle_event(pos, MouseEvent::LeftClick);
                        }
                        glfw::MouseButtonRight => {
                            application.handle_event(pos, MouseEvent::RightClick);
                        }
                        _ => {}
                    }
                }
            }
            glfw::WindowEvent::Scroll(_, scroll) => {
                application.handle_event(pos, MouseEvent::Scroll(scroll as f32));
            }
            _ => (),
        }
    }
}
