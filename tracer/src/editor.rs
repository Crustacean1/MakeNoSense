use ::image::DynamicImage;
use egui::Ui;
use glium::{
    glutin::{self, event_loop::ControlFlow},
    Surface,
};

use crate::{
    image_processor::{EditorEvent, ImageProcessor},
    AppError,
};

use self::{bounded_rect::BoundingBox, ui_root::UiRoot};

pub mod bounded_rect;
pub mod image_selection;
pub mod rendering_context;
pub mod shader;
pub mod ui_layer;
pub mod ui_root;
pub mod vertex;

mod image;
mod image_renderer;
mod mesh;

pub enum MouseEventAction {
    Pressed,
    Released,
}

pub enum MouseEvent {
    LeftClick(MouseEventAction),
    RightClick(MouseEventAction),
    Movement,
    Scroll(f32),
}

pub struct Editor {
    display: glium::Display,
    egui: egui_glium::EguiGlium,
    ui_root: UiRoot,
    image_processor: ImageProcessor,
    mouse_position: (i32, i32),
}

impl Editor {
    pub fn build(
        image_filename: &str,
        label_filename: &str,
        event_loop: &glutin::event_loop::EventLoop<()>,
    ) -> Result<Self, AppError> {
        let display = Self::create_display(&event_loop);

        let sidebar_width = 200;
        let egui = egui_glium::EguiGlium::new(&display, &event_loop);

        let (screen_width, screen_height) = display.get_framebuffer_dimensions();

        let image = Self::load_image(image_filename);

        let ui_root = match UiRoot::build(
            &image,
            BoundingBox::from_quad(
                (0, 0),
                (screen_width as i32 - sidebar_width, screen_height as i32),
            ),
            &display,
        ) {
            Ok(root) => root,
            Err(e) => {
                return Err(AppError {
                    error_msg: format!("Failed to create UI root: {}", e.error_msg),
                });
            }
        };

        let layer_types = [
            String::from("NonMaskingBackground"),
            String::from("MaskingBackground"),
            String::from("Animal"),
            String::from("NonMaskingForegroundAttention"),
        ];

        let image_resolution = (image.width(), image.height());
        let image_processor = ImageProcessor::new(image_filename, image_resolution, &layer_types);

        Ok(Editor {
            display,
            egui,
            ui_root,
            image_processor,
            mouse_position: (0, 0),
        })
    }

    fn load_image(filename: &str) -> DynamicImage {
        match ::image::open(filename) {
            Ok(img) => img,
            Err(_) => {
                panic!("Failed to open file: '{}'", filename);
            }
        }
    }

    pub fn main_loop<T>(&mut self, event: glutin::event::Event<T>, control_flow: &mut ControlFlow) {
        self.handle_glutin_event(event, control_flow);
    }

    fn create_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_resizable(true)
            .with_inner_size(glutin::dpi::LogicalSize {
                width: 1200.0,
                height: 800.0,
            })
            .with_title("Make No Sense");

        let context_builder = glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_stencil_buffer(0)
            .with_vsync(true);

        glium::Display::new(window_builder, context_builder, event_loop).unwrap()
    }

    fn handle_glutin_event<T>(
        &mut self,
        event: glutin::event::Event<T>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => self.render(control_flow),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => self.render(control_flow),
            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached {
                ..
            }) => {
                self.display.gl_window().window().request_redraw();
            }
            glutin::event::Event::WindowEvent { event, .. } => {
                self.handle_window_event(&event, control_flow);

                let event_response = self.egui.on_event(&event);

                if event_response.repaint {
                    self.display.gl_window().window().request_redraw();
                }
            }
            _ => (),
        }
    }

    fn handle_window_event(
        &mut self,
        event: &glutin::event::WindowEvent,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                let action = match state {
                    glutin::event::ElementState::Pressed => MouseEventAction::Pressed,
                    glutin::event::ElementState::Released => MouseEventAction::Released,
                };
                let event = match button {
                    glutin::event::MouseButton::Left => Some(MouseEvent::LeftClick(action)),
                    glutin::event::MouseButton::Right => Some(MouseEvent::RightClick(action)),
                    _ => None,
                };
                if let Some(event) = event {
                    self.ui_root.on_mouse_event(
                        &mut self.image_processor,
                        self.mouse_position,
                        event,
                    );
                }
            }
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x as i32, position.y as i32);
                self.ui_root.on_mouse_event(
                    &mut self.image_processor,
                    self.mouse_position,
                    MouseEvent::Movement,
                );
            }
            glutin::event::WindowEvent::MouseWheel { delta, .. } => {
                if let &glutin::event::MouseScrollDelta::LineDelta(_delta_x, delta_y) = delta {
                    self.ui_root.on_mouse_event(
                        &mut self.image_processor,
                        self.mouse_position,
                        MouseEvent::Scroll(delta_y),
                    );
                }
            }
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit
            }
            glutin::event::WindowEvent::Destroyed => {
                *control_flow = glutin::event_loop::ControlFlow::Exit
            }
            _ => {}
        }
    }

    fn render_egui(&mut self, control_flow: &mut ControlFlow) {
        let quit = false;

        let repaint_after = self.egui.run(&self.display, |egui_ctx| {
            egui::SidePanel::right("Layers")
                .exact_width(200.0)
                .show(egui_ctx, |ui| {
                    Self::render_egui_combo_box(&mut self.image_processor, ui);
                    if ui.button("Save").clicked() {
                        self.image_processor.handle_event(EditorEvent::Save);
                    }
                });
        });

        *control_flow = if quit {
            glutin::event_loop::ControlFlow::Exit
        } else if repaint_after.is_zero() {
            self.display.gl_window().window().request_redraw();
            glutin::event_loop::ControlFlow::Poll
        } else if let Some(repaint_after_instant) =
            std::time::Instant::now().checked_add(repaint_after)
        {
            glutin::event_loop::ControlFlow::WaitUntil(repaint_after_instant)
        } else {
            glutin::event_loop::ControlFlow::Wait
        };
    }

    fn render_egui_combo_box(image_processor: &mut ImageProcessor, ui: &mut Ui) {
        let option = image_processor
            .layer_types()
            .get(image_processor.selected_layer_type);

        let option = match option {
            Some(option) => &option.layer_type,
            None => "None",
        };

        let labels: Vec<_> = image_processor
            .layer_types()
            .iter()
            .map(|layer_type| layer_type.layer_type.clone())
            .collect();

        egui::ComboBox::from_label("Layer")
            .selected_text(option)
            .show_ui(ui, |ui| {
                labels.iter().enumerate().for_each(|(i, label)| {
                    ui.selectable_value(&mut image_processor.selected_layer_type, i, label);
                })
            });
    }

    fn render(&mut self, control_flow: &mut ControlFlow) {
        self.render_egui(control_flow);

        let mut target = self.display.draw();

        let color = egui::Rgba::from_rgb(0.2, 0.2, 0.2);
        target.clear_color(color[0], color[1], color[2], color[3]);

        self.ui_root.render(&mut self.image_processor, &mut target);
        self.egui.paint(&self.display, &mut target);

        target.finish().unwrap();
    }
}
