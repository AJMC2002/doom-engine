use glfw::{Action, Context, Key, WindowEvent};
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

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        Window {
            glfw,
            window,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        self.window.make_current();
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        gl::Viewport::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
        self.process_events();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }
}
