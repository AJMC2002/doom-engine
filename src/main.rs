use std::{mem, ptr};

use doom_engine::graphics::{window::Window, wrapper};

const WIDTH: u32 = 1080;

const HEIGHT: u32 = 720;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Hello, Window!");

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // l
        0.5, -0.5, 0.0, // r
        0.0, 0.5, 0.0, // t
    ];

    window.init_gl();

    let vao = wrapper::vao::VAO::new();
    vao.bind();

    let vbo = wrapper::vbo::VBO::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_data(&vertices);

    let position_attribute = wrapper::vertex_attribute::VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
        ptr::null(),
    );
    position_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.4, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3)
        }
        window.update();
    }
}
