extern crate image;

use self::image::{GenericImage, DynamicImage, ImageBuffer, Pixel, Rgba, Rgb};
use gl::types::*;
use gl;
use std::rc::Rc;
use draw::{Drawable,Drawer};
use std::os::raw::c_void;
use std::mem;

/// Texture structure
#[derive(Debug,Clone)]
pub struct Texture {
	pub id: u32,
	width: u32,
	height: u32,
}

impl Texture {

	/// Create new texture from file path
	pub fn new(path_to_file: &str) -> Texture {
		let img = image::open(path_to_file).unwrap();
		let mut id = 0;
		let mut size = (0, 0);

		unsafe {
			gl::GenTextures(1, &mut id);
			gl::BindTexture(gl::TEXTURE_2D, id);
			Texture::set_texture_parameter();
		}
		match img {
			DynamicImage::ImageRgba8(data) => unsafe {
				size.0 = data.width();
				size.1 = data.height();
				gl::TexImage2D(
					gl::TEXTURE_2D,
					0,
					gl::RGBA as i32,
					data.width() as i32,
					data.height() as i32,
					0,
					gl::RGBA,
					gl::UNSIGNED_BYTE,
					mem::transmute(&data.into_raw()[0])
				);
            },
			DynamicImage::ImageRgb8(data) => unsafe {
				size.0 = data.width();
				size.1 = data.height();
				gl::TexImage2D(
					gl::TEXTURE_2D,
					0,
					gl::RGB as i32,
					data.width() as i32,
					data.height() as i32,
					0,
					gl::RGB,
					gl::UNSIGNED_BYTE,
					mem::transmute(&data.into_raw()[0])
				);
			},
			_ => println!("Error while loading !"),
		}
		unsafe {
			gl::GenerateMipmap(gl::TEXTURE_2D);
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
		Texture {
			id: id,
			width: size.0,
			height: size.1,
		}
	}

	/// Simple getter for width
	pub fn get_width(&self) -> u32 {
		self.width
	}

	/// Simple getter for height
	pub fn get_height(&self) -> u32 {
		self.height
	}

    pub fn active(&self, num: i32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + num as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

	unsafe fn set_texture_parameter() {
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
	}
}