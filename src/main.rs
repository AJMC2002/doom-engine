use std::{mem, os::raw::c_void, ptr};

use gl::types::*;

use doom_engine::{
    graphics::{window::Window, wrapper::*},
    maths::*,
    vector,
};

static WIDTH: u32 = 1080;

static HEIGHT: u32 = 720;

#[test]
fn vector_sum() {
    let a = Vector::from_vec(vec![0.5, 0.2, 0.2]);
    let b = Vector::from_vec(vec![0.5, 0.0, 0.2]);
    assert_eq!(&a + b, Vector::from_vec(vec![1.0, 0.2, 0.4]));
}

#[test]
fn vector_mult() {
    let a = vector![0.5, 0.2, 0.2];
    let b = Vector::from_vec(vec![0.5, 0.0, 0.2]);
    assert_eq!(&a * &2.0, vector![1.0, 0.4, 0.4]);
}

#[test]
fn dot_prod() {
    let a = Vector::from_vec(vec![0.5, 0.2, 0.2]);
    let b = Vector::from_vec(vec![0.5, 0.0, 0.2]);
    assert_eq!(a * b, 0.29);
}

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Doom Engine");
    window.init_gl();

    let mut shader_program = ShaderProgram::new(
        "resources/shaders/basic/basic.vs",
        "resources/shaders/basic/basic.fs",
    );

    let texture: Texture2D = Texture2D::new("resources/textures/pog.jpg");

    let vertices = [
        // positions [3] // tex [2]
        0.7, 0.3, 0.0, 1.0, 1.0, // top right
        0.3, -0.4, 0.0, 0.5, 0.0, // bottom right
        -0.3, -0.4, 0.0, 1.0, 1.0, // bottom left
        -0.5, 0.5, 0.0, 0.4, 1.0, // top left
    ];

    let indices = [
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
    ];

    let vao: VAO = VAO::new();
    let vbo: VBO = BO::new(gl::STATIC_DRAW);
    let ebo: EBO = BO::new(gl::STATIC_DRAW);

    vao.bind();

    vbo.bind();
    vbo.store(&vertices);

    ebo.bind();
    ebo.store(&indices);

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

    let pos_attrib = VertexAttrib::new(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    let tex_attrib = VertexAttrib::new(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );

    pos_attrib.enable();
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

            let t = window.get_time() as f32;
            let color = (t.sin() / 2.0) + 0.5;
            shader_program.bind();
            shader_program.create_4f_uniform("globalColor", 1.0 - color, color, 0.0, 1.0);
            shader_program.create_2dtex_uniform("myTex", &texture);
            vao.bind();

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.update();
    }
}
