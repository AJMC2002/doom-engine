use egui_glfw_gl::gl;
use egui_glfw_gl::gl::types::*;

// Vertex Array Object
pub struct VAO {
    id: GLuint,
}

impl VAO {
    pub fn new() -> VAO {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }
        VAO { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

impl Default for VAO {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
