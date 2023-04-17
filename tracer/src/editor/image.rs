use glad_gl::gl;

use image::EncodableLayout;

use crate::AppError;

use super::shader::ShaderProgram;

pub enum ImageFormat {
    Rgb,
    Rgba,
}

impl ImageFormat {
    fn get_type(&self) -> u32 {
        match self {
            Self::Rgb => gl::RGB,
            Self::Rgba => gl::RGBA,
        }
    }
}

pub struct Image {
    tex_buffer: u32,
    width: u32,
    height: u32,
}

impl Image {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn from_file(filename: &str) -> Result<Self, AppError> {
        println!("Starting loading");
        let img = match image::open(filename) {
            Ok(img) => img,
            Err(_) => {
                return Err(AppError {
                    error_msg: format!("Failed to open file: '{}'", filename),
                });
            }
        };
        println!("File loaded");

        let img = img.into_rgba8();

        let tex_buffer = Self::load_texture(
            img.as_bytes().as_ptr(),
            img.width(),
            img.height(),
            ImageFormat::Rgba,
        );

        Ok(Image {
            tex_buffer,
            width: img.width(),
            height: img.height(),
        })
    }

    pub fn from_color(color: [f32; 4]) -> Self {
        let img: [u8; 4] = [
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
            (color[3] * 255.0) as u8,
        ];

        let tex_buffer = Self::load_texture(img.as_ptr(), 1, 1, ImageFormat::Rgb);

        Image {
            tex_buffer,
            width: 1,
            height: 1,
        }
    }

    pub fn bind(&self, shader: &mut ShaderProgram) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_buffer);
        }
        shader.set_tex("texture0\x00", self.tex_buffer);
    }

    fn load_texture(ptr: *const u8, width: u32, height: u32, image_format: ImageFormat) -> u32 {
        println!("Loading texture");
        unsafe {
            let mut tex_buffer = 0;
            gl::GenTextures(1, &mut tex_buffer);
            gl::BindTexture(gl::TEXTURE_2D, tex_buffer);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::REPEAT as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                image_format.get_type() as i32,
                width as i32,
                height as i32,
                0,
                image_format.get_type(),
                gl::UNSIGNED_BYTE,
                ptr as *const _,
            );
            tex_buffer
        }
    }
}
