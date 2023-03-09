// Vertex Array Object
pub struct VAO {
    id: gl::types::GLuint,
}

impl VAO {
    pub fn new() -> VAO {
        let mut id = 0;
        unsafe { gl::GenVertexArrays(1, &mut id) }
        VAO { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}
