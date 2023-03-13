use std::{os::raw::c_void, path::Path};

use gl::types::*;
use image::{GenericImageView, Pixel};

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new() -> Texture {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id) }
        Texture { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) }
    }

    pub fn set_params(&self) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }

    pub fn load_img(&self, img_path: &str) {
        let img = image::open(&Path::new(img_path)).expect("Failed to load texture");
        let (width, height) = img.dimensions();
        let mut pixels: Vec<u8> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let channels = pixel.channels();
                pixels.extend_from_slice(&channels);
            }
        }

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &pixels[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
}
