use std::os::raw::c_void;

use gl;
use gl::types::*;

// Vertex Attribute
pub struct VertexAttrib {
    index: GLuint,
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl VertexAttrib {
    pub fn new(
        index: u32,
        size: i32,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttrib {
        unsafe {
            gl::VertexAttribPointer(index, size, type_, normalized, stride, pointer);
            gl::EnableVertexAttribArray(index);
        }

        VertexAttrib { index }
    }

    pub fn enable(&self) {
        unsafe { gl::EnableVertexAttribArray(self.index) }
    }

    pub fn disable(&self) {
        unsafe { gl::DisableVertexAttribArray(self.index) }
    }
}
