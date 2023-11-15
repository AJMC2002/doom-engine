use std::{mem::size_of, ptr};

use gl::types::{GLfloat, GLsizei};
use obj::Obj;

use crate::maths::{Matrix, Vector};

use super::{
    camera::Camera,
    wrapper::{ShaderProgram, Texture2D, VertexAttrib, BO, EBO, VAO, VBO},
};

pub struct Cube<'a> {
    translation: Matrix,
    rotation: Matrix,
    scaling: Matrix,
    vao: VAO,
    vbo: VBO,
    ebo: EBO,
    attrib: VertexAttrib,
    texture: Option<&'a Texture2D>,
}

impl<'a> Cube<'a> {
    pub fn new(
        model_matrices: Option<(Matrix, Matrix, Matrix)>,
        texture: Option<&'a Texture2D>,
        tex_coords: Option<Vec<f32>>,
    ) -> Self {
        let mut vertex_data = vec![
            0.5, 0.5, 0.5, //0
            0.5, 0.5, -0.5, //1
            0.5, -0.5, 0.5, //2
            0.5, -0.5, -0.5, //3
            -0.5, 0.5, 0.5, //4
            -0.5, 0.5, -0.5, //5
            -0.5, -0.5, 0.5, //6
            -0.5, -0.5, -0.5, //7
        ];

        // match texture {
        //     Some(texture) => {
        //         let tex_coords = tex_coords.unwrap_or(vec![
        //             1.0, 1.0, //0
        //             0.0, 1.0, //1
        //             1.0, 0.0, //2
        //             0.0, 0.0, //3
        //             0.0, 1.0, //4
        //             1.0, 1.0, //5
        //             0.0, 0.0, //6
        //             1.0, 0.0, //7
        //         ]);
        //     }
        //     None => (),
        // }

        let vao = VAO::new();
        let vbo: VBO = BO::new(gl::STATIC_DRAW, vertex_data);
        let ebo: EBO = BO::new(
            gl::STATIC_DRAW,
            vec![
                4, 5, 1, 4, 1, 0, //Top
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

        let (t, r, s) = model_matrices.unwrap_or((
            Matrix::identity(4),
            Matrix::identity(4),
            Matrix::identity(4),
        ));

        Cube {
            translation: t,
            rotation: r,
            scaling: s,
            vao,
            vbo,
            ebo,
            attrib,
            texture,
        }
    }

    pub fn set_pos(&mut self, pos: Vector) {
        self.translation = Matrix::translation(pos)
    }

    pub fn translate(&mut self, v: Vector) {
        self.translation = &self.translation * Matrix::translation(v);
    }

    pub fn model(&self) -> Matrix {
        &self.scaling * &self.rotation * &self.translation
    }

    pub fn pos(&self) -> Vector {
        let mut p = self.translation.clone().col(3);
        p.pop();
        p.into()
    }

    pub fn draw(&mut self, camera: &Camera, shader: &mut ShaderProgram) {
        shader.bind();
        self.vao.bind();
        self.vbo.bind();
        self.ebo.bind();
        self.attrib.enable();
        shader.uniform_matrix_4fv("proj", &camera.proj());
        shader.uniform_matrix_4fv("view", &camera.view());
        shader.uniform_matrix_4fv("model", &self.model());
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
