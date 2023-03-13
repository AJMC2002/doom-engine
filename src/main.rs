use std::{mem, ptr};

use gl::types::*;

use doom_engine::graphics::{
    window::Window,
    wrapper::{
        bo::{BO, EBO, VBO},
        shader_program::ShaderProgram,
        vao::VAO,
        vertex_attribute::VertexAttribute,
    },
};

const WIDTH: u32 = 1080;

const HEIGHT: u32 = 720;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Doom Engine");
    window.init_gl();

    let mut shader_program = ShaderProgram::new("shaders/basic/basic.vs", "shaders/basic/basic.fs");

    let vertices = [
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];

    let indices = [
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    let vao: VAO = VAO::new();
    let vbo: VBO = BO::new(gl::STATIC_DRAW);
    let ebo: EBO = BO::new(gl::STATIC_DRAW);

    vao.bind();

    vbo.bind();
    vbo.store(&vertices);

    ebo.bind();
    ebo.store(&indices);

    let vertex_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    vertex_attribute.enable();

    vbo.unbind();
    vao.unbind();

    // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let t = window.get_time() as f32;
            let color = (t.sin() / 2.0) + 0.5;

            shader_program.bind();
            shader_program.create_uniform("globalColor", 1.0-color, color, 0.0, 1.0);
            vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            window.update();
        }
    }
}
