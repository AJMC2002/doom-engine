use std::{collections::HashMap, ffi::CString, fs::File, io::Read, ptr};

use egui_glfw_gl::gl;
use egui_glfw_gl::gl::types::*;

use crate::maths::Vector;

use super::texture::Texture2D;

pub struct ShaderProgram {
    id: GLuint,
    uniform_ids: HashMap<String, GLint>,
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
                uniform_ids: HashMap::new(),
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

    pub fn uniform_4f(&mut self, name: &str, v: Vector) {
        assert!(v.len() == 4);
        let location = match self.uniform_ids.get(&name.to_string()) {
            Some(id) => *id,
            None => {
                let name_cstring = CString::new(name).unwrap();
                let loc = unsafe { gl::GetUniformLocation(self.id, name_cstring.as_ptr()) };
                if loc < 0 {
                    panic!("Cannot locate uniform: {}", name);
                } else {
                    self.uniform_ids.insert(name.to_string(), loc);
                }
                loc
            }
        };
        unsafe {
            gl::Uniform4f(location, v[0], v[1], v[2], v[3]);
        }
    }

    pub fn uniform_2dtex(&mut self, name: &str, tex: &Texture2D) {
        match self.uniform_ids.get(&name.to_string()) {
            Some(id) => *id,
            None => {
                let name_cstring = CString::new(name).unwrap();
                let loc = unsafe { gl::GetUniformLocation(self.id, name_cstring.as_ptr()) };
                if loc < 0 {
                    panic!("Cannot locate uniform: {}", name);
                } else {
                    self.uniform_ids.insert(name.to_string(), loc);
                }
                loc
            }
        };

        unsafe {
            tex.bind();
            gl::ActiveTexture(gl::TEXTURE0);
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
