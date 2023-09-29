use std::ffi::CStr;
use egui_glfw::EguiBackend;
use gl::types::*;
use glfw::{self, Action, Context, Key, WindowEvent};
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    ui: EguiBackend,
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

        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::Viewport::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Disable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);
            gl::Enable(gl::DEBUG_OUTPUT);
            // gl::Enable(gl::FRAMEBUFFER_SRGB);
            gl::DebugMessageCallback(Some(debug_callback), ptr::null());
        }

        let ui = EguiBackend::new(&mut window, &mut glfw);

        Window {
            glfw,
            window,
            ui,
            events,
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

    pub fn update(&mut self) {
        self.glfw.poll_events();
        self.process_events();
        self.process_errors();
        self.update_ui();
        self.window.swap_buffers();
    }

    pub fn events(&self) -> glfw::FlushedMessages<'_, (f64, WindowEvent)> {
        glfw::flush_messages(&self.events)
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            self.ui.handle_event(&event, &self.window);
            // println!("EVENT - {:?}", event);
            match event {
                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
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

    fn update_ui(&mut self) {
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

    pub fn ui_begin_frame(&mut self) {
        self.ui.begin_frame(&mut self.window, &mut self.glfw)
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
    unsafe {
        println!(
            "GL CALLBACK: {} type = {}, severity = {}, message = {}",
            match gltype {
                gl::DEBUG_TYPE_ERROR => "** GL ERROR **",
                _ => ""
            },
            CStr::from_ptr(gl::GetString(gltype) as _).to_str().expect("Failed to convert debug GL type to String"),
            CStr::from_ptr(gl::GetString(severity) as _).to_str().expect("Failed to convert debug severity to String"),
            CStr::from_ptr(message as _).to_str().expect("Failed to convert debug message to String")
        )
    }
    // println!(
    //     "OpenGL Debug Message: source={}, type={}, id={}, severity={}, message={}",
    //     source,
    //     gltype,
    //     id,
    //     severity,
    //     unsafe { std::ffi::CStr::from_ptr(message).to_str().unwrap() }
    // );
}
