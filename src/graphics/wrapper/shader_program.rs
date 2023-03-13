use std::{collections::HashMap, ffi::CString, fs::File, io::Read, ptr};

use gl::types::*;

// use cgmath::Matrix;

pub struct ShaderProgram {
    id: u32,
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

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        let uniform_name_cstring = CString::new(uniform_name).unwrap();
        let uniform_location =
            unsafe { gl::GetUniformLocation(self.id, uniform_name_cstring.as_ptr()) };
        if uniform_location < 0 {
            panic!("Cannot locate uniform: {}", uniform_name);
        } else {
            self.uniform_ids
                .insert(uniform_name.to_string(), uniform_location);
        }

        unsafe { gl::Uniform4f(uniform_location, v0, v1, v2, v3) }
    }

    // pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
    //     unsafe {
    //         gl::UniformMatrix4fv(
    //             self.uniform_ids[uniform_name],
    //             1,
    //             gl::FALSE,
    //             matrix.as_ptr(),
    //         )
    //     }
    // }
}
