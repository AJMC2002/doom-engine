use std::mem;

use gl::types::*;

// Buffer Object
pub trait BO<DataType> {
    fn new(usage: GLenum) -> Self;
    fn bind(&self);
    fn unbind(&self);
    fn store(&self, data: &[DataType]);
}

// Vertex Buffer Object
pub struct VBO {
    id: GLuint,
    usage: GLenum,
}

impl BO<f32> for VBO {
    fn new(usage: GLenum) -> VBO {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        VBO { id, usage }
    }

    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) }
    }

    fn store(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                self.usage,
            )
        }
    }
}

// Element Buffer Object
pub struct EBO {
    id: GLuint,
    usage: GLenum,
}

impl BO<i32> for EBO {
    fn new(usage: GLenum) -> EBO {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        EBO { id, usage }
    }

    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) }
    }

    fn store(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                self.usage,
            )
        }
    }
}
