use std::{
    mem::{self, size_of},
    os::raw::c_void,
    ptr,
};

use gl::types::{GLfloat, GLsizei};
use obj::Obj;

use crate::maths::Matrix;

use super::{
    camera::Camera,
    wrapper::{shader_program, ShaderProgram, Texture2D, VertexAttrib, BO, EBO, VAO, VBO},
};

pub struct Cube<'a> {
    model: Matrix,
    vao: VAO,
    vbo: VBO,
    ebo: EBO,
    attrib:VertexAttrib,
    texture: Option<&'a Texture2D>,
}

impl<'a> Cube<'a> {
    pub fn new(
        model: Option<Matrix>,
        texture: Option<&'a Texture2D>,
        tex_coords: Option<Vec<f32>>,
    ) -> Self {
        let mut vertex_data = vec![
            1.0, 1.0, 1.0, //0
            1.0, 1.0, -1.0, //1
            1.0, -1.0, 1.0, //2
            1.0, -1.0, -1.0, //3
            -1.0, 1.0, 1.0, //4
            -1.0, 1.0, -1.0, //5
            -1.0, -1.0, 1.0, //6
            -1.0, -1.0, -1.0, //7
        ];

        match texture {
            Some(texture) => {
                let tex_coords = tex_coords.unwrap_or(vec![
                    1.0, 1.0, //0
                    0.0, 1.0, //1
                    1.0, 0.0, //2
                    0.0, 0.0, //3
                    0.0, 1.0, //4
                    1.0, 1.0, //5
                    0.0, 0.0, //6
                    1.0, 0.0, //7
                ]);
            }
            None => (),
        }

        let vao = VAO::new();
        let vbo: VBO = BO::new(gl::STATIC_DRAW, vertex_data);
        let ebo: EBO = BO::new(
            gl::STATIC_DRAW,
            vec![
                4, 5, 6, 4, 1, 0, //Top
                7, 6, 2, 7, 2, 3, //Bottom
                7, 5, 4, 7, 4, 6, //Left
                2, 0, 1, 2, 1, 3, //Right
                6, 4, 0, 6, 0, 2, //Front
                3, 1, 5, 3, 5, 7, //Back
            ],
        );
        let attrib = VertexAttrib::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        attrib.disable();
        ebo.unbind();
        vbo.unbind();
        vao.unbind();

        Cube {
            model: model.unwrap_or(Matrix::model_default()),
            vao,
            vbo,
            ebo,
            attrib,
            texture,
        }
    }

    pub fn draw(&mut self, camera: &Camera, shader: &mut ShaderProgram) {
        shader.bind();
        self.vao.bind();
        self.vbo.bind();
        self.ebo.bind();
        self.attrib.enable();
        shader.uniform_matrix_4fv("proj", &camera.proj());
        shader.uniform_matrix_4fv("view", &camera.view());
        shader.uniform_matrix_4fv("model", &self.model);
        unsafe { gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, ptr::null()) }
        self.attrib.disable();
        self.ebo.unbind();
        self.vbo.unbind();
        self.vao.unbind();
        shader.unbind();
    }
}

pub struct Mesh {
    obj: Obj,
    textures: Vec<Texture2D>,
}

impl Mesh {
    pub fn new(obj: Obj, textures: Vec<Texture2D>) -> Self {
        Self { obj, textures }
    }
}
