use std::{collections::HashMap, ffi::CString, fs::File, io::Read, ptr};

use gl::types::*;

use crate::maths::{Matrix, Vector};

use super::texture::Texture2D;

pub struct ShaderProgram {
    id: GLuint,
    location_cache: HashMap<String, GLint>,
}

impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let mut vertex_shader_file =
            File::open(vertex_shader_path).expect("Failed to open vertex shader file");
        let mut fragment_shader_file =
            File::open(fragment_shader_path).expect("Failed to open fragment shader file");

        let mut vertex_shader_src = String::new();
        let mut fragment_shader_src = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_src)
            .expect("Failed to read vertex shader");
        fragment_shader_file
            .read_to_string(&mut fragment_shader_src)
            .expect("Failed to read fragment shader");

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_src.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_src.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram {
                id,
                location_cache: HashMap::new(),
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    fn get_location(&mut self, name: &str) -> GLint {
        match self.location_cache.get(&name.to_string()) {
            Some(id) => *id,
            None => {
                let name_cstring = CString::new(name).unwrap();
                let loc = unsafe { gl::GetUniformLocation(self.id, name_cstring.as_ptr()) };
                if loc < 0 {
                    panic!("Cannot locate uniform: {}", name);
                } else {
                    self.location_cache.insert(name.to_string(), loc);
                }
                loc
            }
        }
    }

    pub fn uniform_3fv(&mut self, name: &str, v: Vector) {
        assert_eq!(v.len(), 3);
        unsafe {
            gl::Uniform3fv(self.get_location(name), 1, v.as_ptr());
        }
    }

    pub fn uniform_4f(&mut self, name: &str, v1: f32, v2: f32, v3: f32, v4: f32) {
        unsafe {
            gl::Uniform4f(self.get_location(name), v1, v2, v3, v4);
        }
    }

    pub fn uniform_4fv(&mut self, name: &str, v: Vector) {
        assert_eq!(v.len(), 4);
        unsafe {
            gl::Uniform4fv(self.get_location(name), 1, v.as_ptr());
        }
    }

    pub fn uniform_matrix_4fv(&mut self, name: &str, m: &Matrix) {
        assert_eq!(m.rows(), 4);
        assert_eq!(m.cols(), 4);
        unsafe {
            //Matrix is a row-major matrix type, therefore we need to use
            //transpose: gl::TRUE since OpenGL uses column-major matrices
            gl::UniformMatrix4fv(self.get_location(name), 1, gl::TRUE, m.as_ptr());
        }
    }

    pub fn uniform_2dtex(&mut self, name: &str, tex: &Texture2D) {
        self.get_location(name);
        unsafe {
            //que se joda el que hizo opengl
            gl::ActiveTexture(gl::TEXTURE0);
            tex.bind();
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
