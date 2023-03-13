use std::{mem, os::raw::c_void, ptr};

use gl::types::*;

use doom_engine::graphics::{
    window::Window,
    wrapper::{
        bo::{BO, EBO, VBO},
        shader_program::ShaderProgram,
        texture::Texture,
        vao::VAO,
        vertex_attrib::VertexAttrib,
    },
};

const WIDTH: u32 = 1080;

const HEIGHT: u32 = 720;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Doom Engine");
    window.init_gl();

    let mut shader_program = ShaderProgram::new(
        "resources/shaders/basic/basic.vs",
        "resources/shaders/basic/basic.fs",
    );

    let texture: Texture = Texture::new();
    texture.bind();
    texture.set_params();
    texture.load_img("resources/textures/pog.jpg");

    let vertices = [
        // pos [3], colors [3], tex coords [2]
        -1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, // top right
        1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom right
        -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, // top left
    ];

    let indices = [
        0, 1, 3, // first triangle
        0, 2, 3, // second triangle
    ];

    let vao: VAO = VAO::new();
    let vbo: VBO = BO::new(gl::STATIC_DRAW);
    let ebo: EBO = BO::new(gl::STATIC_DRAW);

    vao.bind();

    vbo.bind();
    vbo.store(&vertices);

    ebo.bind();
    ebo.store(&indices);

    let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

    let pos_attrib = VertexAttrib::new(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    let color_attrib = VertexAttrib::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    let tex_attrib = VertexAttrib::new(
        2,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (6 * mem::size_of::<GLfloat>()) as *const c_void,
    );

    pos_attrib.enable();
    color_attrib.enable();
    tex_attrib.enable();

    vbo.unbind();
    vao.unbind();

    // unsafe {
    //     gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    // }

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // let t = window.get_time() as f32;
            // let color = (t.sin() / 2.0) + 0.5;
            shader_program.bind();
            // shader_program.create_uniform("globalColor", 1.0 - color, color, 0.0, 1.0);
            texture.bind();
            vao.bind();

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.update();
    }
}
