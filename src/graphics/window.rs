use egui_glfw::EguiBackend;
use gl::types::*;
use glfw::{self, Action, Context, Key, WindowEvent};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

use super::camera::Camera;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    ui: EguiBackend,
    camera: Camera,
    events: Receiver<(f64, WindowEvent)>,
    last_pos: (f64, f64),
    last_frame: f64,
    time_delta: f64,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_scroll_polling(true);
        window.set_char_polling(true);

        let last_pos = (width as f64 / 2.0, height as f64 / 2.0);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_cursor_pos(last_pos.0, last_pos.1);
        window.set_sticky_keys(true);

        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        gl::Viewport::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        unsafe {
            gl::Disable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);
            gl::Enable(gl::DEBUG_OUTPUT);
            // gl::Enable(gl::FRAMEBUFFER_SRGB);
            gl::DebugMessageCallback(Some(debug_callback), ptr::null());
        }

        let ui = EguiBackend::new(&mut window, &mut glfw);
        let mut camera = Camera::default();
        camera.set_aspect(width as f32 / height as f32);

        Window {
            glfw,
            window,
            ui,
            camera,
            events,
            last_pos,
            last_frame: 0.0,
            time_delta: 0.0,
        }
    }

    pub fn glfw_handle(&self) -> &glfw::Glfw {
        &self.glfw
    }

    pub fn glfw_handle_mut(&mut self) -> &mut glfw::Glfw {
        &mut self.glfw
    }

    pub fn window_handle(&self) -> &glfw::Window {
        &self.window
    }

    pub fn window_handle_mut(&mut self) -> &mut glfw::Window {
        &mut self.window
    }

    pub fn ui_handle(&self) -> &EguiBackend {
        &self.ui
    }

    pub fn camera_handle(&self) -> &Camera {
        &self.camera
    }

    pub fn time_delta(&self) -> f64 {
        self.time_delta
    }

    pub fn begin_ui(&mut self) {
        self.ui.begin_frame(&self.window, &mut self.glfw)
    }

    pub fn end_ui(&mut self) {
        let (w, h) = self.window.get_framebuffer_size();
        let output = self.ui.end_frame((w as _, h as _));
        if !output.platform_output.copied_text.is_empty() {
            match copypasta_ext::try_context() {
                Some(mut context) => context
                    .set_contents(output.platform_output.copied_text)
                    .unwrap(),
                None => {
                    eprintln!("enable to gather context for clipboard");
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();
        self.process_events();
        self.process_errors();
        self.window.swap_buffers();

        let cur_frame = self.glfw.get_time();
        self.time_delta = cur_frame - self.last_frame;
        self.last_frame = cur_frame;
        self.camera.update_pos(self.time_delta, &self.window);
    }

    pub fn events(&self) -> glfw::FlushedMessages<'_, (f64, WindowEvent)> {
        glfw::flush_messages(&self.events)
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            self.ui.handle_event(&event, &self.window);

            match event {
                WindowEvent::CursorPos(x, y) => {
                    let (last_x, last_y) = self.last_pos;
                    self.camera.cursor_pos_callback(x - last_x, last_y - y);
                    self.last_pos = (x, y);
                }
                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                WindowEvent::Scroll(x_offset, y_offset) => {
                    self.camera.scroll_callback(x_offset, y_offset)
                }
                //egui bindings
                glfw::WindowEvent::Key(
                    glfw::Key::X,
                    _,
                    glfw::Action::Press,
                    glfw::Modifiers::Control,
                ) => {
                    self.ui.push_event(egui::Event::Cut);
                }
                glfw::WindowEvent::Key(
                    glfw::Key::C,
                    _,
                    glfw::Action::Press,
                    glfw::Modifiers::Control,
                ) => {
                    self.ui.push_event(egui::Event::Copy);
                }
                glfw::WindowEvent::Key(
                    glfw::Key::V,
                    _,
                    glfw::Action::Press,
                    glfw::Modifiers::Control,
                ) => {
                    let text = match copypasta_ext::try_context() {
                        Some(mut context) => Some(context.get_contents().unwrap()),
                        None => {
                            eprintln!("enable to gather context for clipboard");
                            None
                        }
                    };
                    if let Some(text) = text {
                        self.ui.push_event(egui::Event::Text(text));
                    }
                }
                _ => (),
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

extern "system" fn debug_callback(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    println!(
        "OpenGL Debug Message: source={}, type={}, id={}, severity={}, message={}",
        source,
        gltype,
        id,
        severity,
        unsafe { CStr::from_ptr(message).to_str().unwrap() }
    );
}
