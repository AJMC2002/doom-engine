use std::{mem, os::raw::c_void, ptr};

use gl::types::*;

use doom_engine::graphics::{wrapper::*, Window};

static WIDTH: u32 = 1080;

static HEIGHT: u32 = 720;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Doom Engine");
    window.init_gl();

    let mut shader_program = ShaderProgram::new(
        "resources/shaders/basic/basic.vs",
        "resources/shaders/basic/basic.fs",
    );

    let texture: Texture2D = Texture2D::new("resources/textures/cat.jpg");

    //All Buffer Objects are binded and the data is stored on creation
    let _vao: VAO = VAO::new();
    let _vbo: VBO = BO::new(
        gl::STATIC_DRAW,
        Box::new([
            // positions [3] // tex [2]
            -0.5, 0.5, 0.0, 0.0, 1.0, // top right
            0.5, 0.5, 0.0, 1., 1., // bottom right
            -0.5, -0.5, 0.0, 0., 0., // bottom left
            0.5, -0.5, 0.0, 1., 0., // top left
        ]),
    );
    let _ebo: EBO = BO::new(
        gl::STATIC_DRAW,
        Box::new([
            2, 1, 0, // first Triangle
            3, 2, 1, // second Triangle
        ]),
    );

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

    let _pos_attrib = VertexAttrib::new(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    let _tex_attrib = VertexAttrib::new(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );

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
            shader_program.create_4f_uniform("globalColor", 1.0 - color, color, color.powi(2), 1.0);
            shader_program.create_2dtex_uniform("myTex", &texture);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        error_logger();
        window.update();
    }

    _vao.unbind();
    _vbo.unbind();
    _ebo.unbind();
    _pos_attrib.disable();
    _tex_attrib.disable();
}

fn error_logger() {
    unsafe {
        let mut e: GLenum;
        while {
            e = gl::GetError();
            e != gl::NO_ERROR
        } {
            println!("Error {:?}", e as GLenum)
        }
    }
}
