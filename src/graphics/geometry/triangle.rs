use gl::types::*;
use std::{mem, os::raw::c_void, ptr};

use crate::{
    graphics::{wrapper::*, Context},
    maths::*,
};

pub trait Vertex {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

impl Vertex for Vector {
    fn x(&self) -> f32 {
        assert!(!self.is_empty());
        self[0]
    }
    fn y(&self) -> f32 {
        assert!(self.len() >= 2);
        self[1]
    }
    fn z(&self) -> f32 {
        assert!(self.len() >= 3);
        self[2]
    }
}

pub struct Triangle<'a> {
    model: Matrix,
    texture: Option<&'a Texture2D>,
    shader: &'a mut ShaderProgram,
}

impl<'a> Triangle<'a> {
    pub fn new(
        model: Matrix,
        texture: Option<&'a Texture2D>,
        shader: &'a mut ShaderProgram,
    ) -> Self {
        assert_eq!(model.cols(), 4);
        assert_eq!(model.rows(), 4);
        Triangle {
            model,
            texture,
            shader,
        }
    }
    pub fn draw(&mut self, context: &Context) {
        self.shader.bind();
        self.shader.uniform_matrix_4fv("proj", &context.projection);
        self.shader.uniform_matrix_4fv("view", &context.view);
        self.shader.uniform_matrix_4fv("model", &self.model);
        if let Some(texture) = self.texture {
            texture.bind();
            self.shader.uniform_2dtex("tex", texture)
        }
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        if let Some(texture) = self.texture {
            texture.unbind();
        }
        self.shader.unbind();
    }
}

pub struct TrianglePool<'a> {
    vao: VAO,
    attribs: Vec<VertexAttrib>,
    pool: Vec<Triangle<'a>>,
}

impl<'a> TrianglePool<'a> {
    pub fn new(
        base_data: Vec<f32>,
        attribs_sizes: Vec<i32>,
        shader: &'a mut ShaderProgram,
    ) -> Self {
        let vao = VAO::new();
        let vbo: VBO = BO::new(gl::STATIC_DRAW, base_data);
        let stride = attribs_sizes.iter().sum::<i32>() * mem::size_of::<GLfloat>() as GLsizei;
        let mut cur_data_index = 0;
        let attribs = attribs_sizes
            .iter()
            .enumerate()
            .map(|(idx, attrib_size)| {
                let attrib = VertexAttrib::new(
                    idx as u32,
                    *attrib_size,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    match cur_data_index {
                        0 => ptr::null(),
                        _ => (cur_data_index * mem::size_of::<GLfloat>()) as *const c_void,
                    },
                );
                cur_data_index += *attrib_size as usize;
                attrib
            })
            .collect::<Vec<VertexAttrib>>();
        let pool = vec![Triangle::new(Matrix::model_default(), None, shader)];

        vbo.unbind();
        vao.unbind();
        attribs.iter().for_each(|attrib| attrib.disable());

        TrianglePool { vao, attribs, pool }
    }
    pub fn draw(&mut self, context: &Context) {
        self.vao.bind();
        self.attribs.iter().for_each(|attrib| attrib.enable());
        self.pool
            .iter_mut()
            .for_each(|triangle| triangle.draw(context));
        self.attribs.iter().for_each(|attrib| attrib.disable());
        self.vao.unbind();
    }
}
