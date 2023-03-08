use doom_engine::graphics::window::Window;

fn main() {
    let mut window = Window::new(1080, 720, "Hello, Window!");

    window.init_gl();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.5, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT)
        }
        window.update();
    }
}
