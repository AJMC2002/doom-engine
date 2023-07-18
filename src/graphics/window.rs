use egui_glfw_gl::gl::types::GLenum;
use egui_glfw_gl::glfw;
use egui_glfw_gl::glfw::{Action, Context, Key, WindowEvent};
use egui_glfw_gl::{gl, EguiInputState};
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        window.set_char_polling(true);
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);

        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::Viewport::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            glfw,
            window,
            events,
        }
    }

    pub fn window_handle(&self) -> &glfw::Window {
        &self.window
    }

    pub fn window_handle_mut(&mut self) -> &mut glfw::Window {
        &mut self.window
    }

    pub fn glfw_handle(&self) -> &glfw::Glfw {
        &self.glfw
    }

    pub fn glfw_handle_mut(&mut self) -> &mut glfw::Glfw {
        &mut self.glfw
    }

    pub fn update(&mut self, egui_input_state: &mut EguiInputState) {
        self.window.swap_buffers();
        self.glfw.poll_events();
        self.process_events(egui_input_state);
        self.process_errors();
    }

    pub fn events(&self) -> glfw::FlushedMessages<'_, (f64, WindowEvent)> {
        glfw::flush_messages(&self.events)
    }

    fn process_events(&mut self, egui_input_state: &mut EguiInputState) {
        for (_, event) in glfw::flush_messages(&self.events) {
            println!("EVENT - {:?}", event);
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => egui_glfw_gl::handle_event(event, egui_input_state),
            }
        }
    }

    fn process_errors(&self) {
        unsafe {
            let mut e: GLenum;
            while {
                e = gl::GetError();
                e != gl::NO_ERROR
            } {
                println!("ERROR - {:?}", e as GLenum)
            }
        }
    }
}
