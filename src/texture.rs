// Alexanre Fourcat 2018
// Rust_gl 
//
//
//

extern crate image;

use self::image::{GenericImage, DynamicImage, ImageBuffer, Pixel};
use gl::types::*;
use gl;
use std::rc::Rc;

/// Texture structure
#[derive(Debug,Clone)]
pub struct Texture {
	pub id: u32,
}

impl Texture {

	/// Create new texture from file path
	pub fn new(path_to_file: &str) -> Texture {
		let img = image::open(path_to_file).unwrap();
		let mut id = 0;

		unsafe {
			gl::GenTextures(1, &mut id);
			gl::BindTexture(gl::TEXTURE_2D, id);
			Texture::setTextureParameter();
		}
		match img {
			DynamicImage::ImageRgba8(_) => Texture::insert_texture(img, gl::RGBA),
			DynamicImage::ImageRgb8(_) => Texture::insert_texture(img, gl::RGB),
			_ => println!("Error while loading !"),
		}
		unsafe {
			gl::GenerateMipmap(gl::TEXTURE_2D);
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
		Texture {
			id: id,
		}
	}

    pub fn active(&mut self, num: i32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + num as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

	fn insert_texture(img: DynamicImage, gl_type: GLenum) {
		unsafe {
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl_type as i32,
				img.dimensions().0 as i32,
				img.dimensions().1 as i32,
				0,
				gl_type,
				gl::UNSIGNED_BYTE,
				img.raw_pixels().as_ptr() as *const _
			);
		}
	}

	unsafe fn setTextureParameter() {
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
	}
}
