use std::error::Error;

use image::ImageReader;

pub struct Texture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub nr_channels: i32,
}

impl Texture {
    pub fn new(path: &str, filter: gl::types::GLenum) -> Result<Self, Box<dyn Error>> {
        let image = ImageReader::open(path)?.decode()?;

        let mut id: u32 = 0;

        // load texture from stbi
        unsafe {
            // manipulate the texture
            gl::GenTextures(1, &mut id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                filter as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                filter,
                gl::UNSIGNED_BYTE,
                image.as_bytes().as_ptr().cast(),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Texture {
            id,
            width: image.width(),
            height: image.height(),
            nr_channels: 0,
        })
    }
}
