use gl::types::*;
use image::GenericImageView;

#[derive(Debug)]
pub struct Texture2D {
    id: GLuint,
}

impl Texture2D {
    pub fn new(img_path: &str) -> Texture2D {
        let img = image::open(img_path).unwrap().flipv();
        let (width, height) = img.dimensions();
        let data = img.to_rgba8().into_raw();

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, width as _, height as _);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                width as _,
                height as _,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // gl::TexImage2D(
            //     gl::TEXTURE_2D,
            //     0,
            //     gl::RGBA as i32,
            //     width as i32,
            //     height as i32,
            //     0,
            //     gl::RGBA,
            //     gl::UNSIGNED_BYTE,
            //     data.as_ptr() as *const GLvoid,
            // );
        }

        Texture2D { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
