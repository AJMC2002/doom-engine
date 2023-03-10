use std::{mem, os::raw::c_void};

// Buffer Object
pub trait BO<DataType> {
    fn new(usage: gl::types::GLenum) -> Self;
    fn bind(&self);
    fn unbind(&self);
    fn store(&self, data: &[DataType]);
}

// Vertex Buffer Object
pub struct VBO {
    id: gl::types::GLuint,
    usage: gl::types::GLenum,
}

impl BO<f32> for VBO {
    fn new(usage: gl::types::GLenum) -> VBO {
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
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            )
        }
    }
}

// Element Buffer Object
pub struct EBO {
    id: gl::types::GLuint,
    usage: gl::types::GLenum,
}

impl BO<i32> for EBO {
    fn new(usage: gl::types::GLenum) -> EBO {
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
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            )
        }
    }
}
