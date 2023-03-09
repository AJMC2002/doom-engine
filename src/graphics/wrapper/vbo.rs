use std::{ffi::c_void, mem};

// # Vertex Buffer Object
// Stores vertex data
pub struct VBO {
    id: gl::types::GLuint,
    type_: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl VBO {
    pub fn new(type_: gl::types::GLenum, usage: gl::types::GLenum) -> VBO {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        VBO { id, type_, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.type_, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(self.type_, 0) }
    }

    pub fn store_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.type_,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            )
        }
    }
}
