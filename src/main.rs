use std::{mem, ptr};

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

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Hello, Window!");
    window.init_gl();

    let shader_program = ShaderProgram::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

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

    let vao = VAO::new();
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
        3 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
        ptr::null(),
    );
    vertex_attribute.enable();

    vbo.unbind();
    vao.unbind();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.5, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.bind();
            vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null())
        }
        window.update();
    }
}
