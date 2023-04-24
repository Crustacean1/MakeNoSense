use image::{DynamicImage, GenericImageView};

use crate::AppError;

impl From<glium::texture::TextureCreationError> for AppError {
    fn from(err: glium::texture::TextureCreationError) -> Self {
        AppError {
            error_msg: format!("Error while loading teture: {}", err.to_string()),
        }
    }
}

pub struct Image {
    texture: glium::texture::CompressedSrgbTexture2d,
    width: u32,
    height: u32,
}

impl Image {
    pub fn texture(&self) -> &glium::texture::CompressedSrgbTexture2d {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn from_file(display: &glium::Display, image: &DynamicImage) -> Result<Self, AppError> {
        let (width, height) = image.dimensions();
        let texture = image.clone().into_rgba8();
        let texture =
            glium::texture::RawImage2d::from_raw_rgba(texture.into_raw(), (width, height));

        let texture = glium::texture::CompressedSrgbTexture2d::new(display, texture)?;

        Ok(Image {
            texture,
            width,
            height,
        })
    }
}
