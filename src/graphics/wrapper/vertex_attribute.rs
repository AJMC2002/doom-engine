use std::ffi::c_void;

/// # Vertex Attribute
/// Discribes vertex data
pub struct VertexAttribute {
    index: gl::types::GLuint,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        type_: gl::types::GLenum,
        normalized: gl::types::GLboolean,
        stride: gl::types::GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute {
        unsafe { gl::VertexAttribPointer(index, size, type_, normalized, stride, pointer) }

        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe { gl::EnableVertexAttribArray(self.index) }
    }

    pub fn disable(&self) {
        unsafe { gl::DisableVertexAttribArray(self.index) }
    }
}
