use std::os::raw::c_void;

use gl::types::*;

// Vertex Attribute
pub struct VertexAttrib {
    index: GLuint,
}

impl VertexAttrib {
    pub fn new(
        index: u32,
        size: i32,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttrib {
        unsafe { gl::VertexAttribPointer(index, size, type_, normalized, stride, pointer) }

        VertexAttrib { index }
    }

    pub fn enable(&self) {
        unsafe { gl::EnableVertexAttribArray(self.index) }
    }

    pub fn disable(&self) {
        unsafe { gl::DisableVertexAttribArray(self.index) }
    }
}
