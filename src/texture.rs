//
//
//
//
//

extern crate image;

use self::image::{GenericImage, DynamicImage};
use gl::types::*;
use gl;

/// Texture structure
#[derive(Debug,Clone)]
pub struct Texture {
	id: u32,
	data: GenericImage,
}

impl Texture {

	/// Create new texture from file path
	pub fn new(path_to_file: &str) -> Texture {
		let img = image::open(path_to_file).unwrap();

		match img {
			DynamicImage {

			}
		}

		Texture {
			data: img,
		}
	}
}