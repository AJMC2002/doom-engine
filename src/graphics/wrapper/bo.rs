use std::mem;

use egui_glfw_gl::gl;
use egui_glfw_gl::gl::types::*;

// Buffer Object
pub trait BO<Ty> {
    fn new(usage: GLenum, data: Box<[Ty]>) -> Self;
    fn bind(&self);
    fn unbind(&self);
    //fn store(&self);
}

// Vertex Buffer Object
pub struct VBO {
    id: GLuint,
    // usage: GLenum,
    // data: Box<[f32]>,
}

impl BO<f32> for VBO {
    fn new(usage: GLenum, data: Box<[f32]>) -> VBO {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                data.as_ptr().cast(),
                usage,
            );
        }
        VBO { id }
    }

    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

// Element Buffer Object
pub struct EBO {
    id: GLuint,
    // usage: GLenum,
    // data: Box<[i32]>,
}

impl BO<i32> for EBO {
    fn new(usage: GLenum, data: Box<[i32]>) -> EBO {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                data.as_ptr().cast(),
                usage,
            );
        }
        EBO { id }
    }

    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) }
    }
}

impl Drop for EBO {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
