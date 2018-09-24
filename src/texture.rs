//! This module is for texture handling
//! Importing, Loading, Pushing into OpenGl
//! I'm using image crate that is really useful

extern crate image;

use self::image::{DynamicImage};
use gl;
use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use ::Vector;

/// # Texture structure
/// A texture is an id inside openGL that can contain a array of byte
/// this array can be spreaded to drawable object
/// ```no_run
/// use gust::window::Window;
/// use gust::sprite::Sprite;
/// use gust::texture::Texture;
/// use std::rc::Rc;
///
/// let window = Window::new(1080, 1920, "Test");
/// let leave = Rc::new(Texture::new("path/to/test"));
///	let sprite = Sprite::from(&leave);
/// ```
#[derive(Debug,Clone,PartialEq,Copy,Eq)]
pub struct Texture {
	pub id: u32,
	width: u32,
	height: u32,
}

impl Texture {

    /// Create a texture from a raw data pointer needed for Font handling
    pub fn from_data(data: *const c_void, rgb_mode: RgbMode, width: i32, height: i32) -> Texture {
        Texture {
            id: Self::create_texture(data, rgb_mode.as_gl(), width, height),
            width: width as u32,
            height: height as u32,
        }
    }

    fn create_texture(data: *const c_void, rgb_mode: GLenum, width: i32, height: i32) -> u32 {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            Texture::set_texture_parameter();
            gl::TexImage2D(
                gl::TEXTURE_2D,
		    	0,
		    	rgb_mode as i32,
		    	width,
		    	height,
		    	0,
		    	rgb_mode,
		    	gl::UNSIGNED_BYTE,
		    	data
		    );
			gl::GenerateMipmap(gl::TEXTURE_2D);
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
        id
    }

	/// Create new texture from file path
	pub fn new(path_to_file: &str) -> Texture {
		let img = image::open(path_to_file).unwrap();
		let mut id = 0;
		let mut size = (0, 0);
		
		match img {
			DynamicImage::ImageRgba8(data) => unsafe {
                size.0 = data.width();
                size.1 = data.height();
				id = Self::create_texture(
                    mem::transmute(&data.into_raw()[0]),
                    gl::RGBA,
                    size.0 as i32,
                    size.1 as i32
                );
            },
			DynamicImage::ImageRgb8(data) => unsafe {
                size.0 = data.width();
                size.1 = data.height();
                id = Self::create_texture(
                    mem::transmute(&data.into_raw()[0]),
                    gl::RGBA,
                    size.0 as i32,
                    size.1 as i32
                );
			},
			_ => println!("Error while loading !"),
		}
		
		Texture {
			id: id,
			width: size.0,
			height: size.1
		}
	}

    pub fn empty() -> Texture {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id); };
        Texture {
            id: id,
            width: 0,
            height: 0
        }
    }

    pub fn update(&mut self, data: Vec<u8>, sizes: Vector<i32>, rgb_mode: RgbMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
		    	0,
		    	rgb_mode.clone() as i32,
		    	sizes.x,
		    	sizes.y,
		    	0,
		    	rgb_mode as u32,
		    	gl::UNSIGNED_BYTE,
		    	data.as_ptr() as *const c_void
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
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

impl Default for Texture {
    fn default() -> Texture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);

            let data = vec![255, 255, 255, 255];

            gl::BindTexture(gl::TEXTURE_2D, id);

            Texture::set_texture_parameter();

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32,
                        1, 1, 0, gl::RGBA,
                        gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
        };

        Texture {
            id: id,
            width: 1,
            height: 1
        }
    }
}

/// Enum to wrap gl RGB modes
#[derive(Clone,Copy,Debug)]
pub enum RgbMode {
    RGBA,
    RGB
}

impl RgbMode {
    pub fn as_gl(&self) -> GLenum {
        match self {
            RgbMode::RGBA => { gl::RGBA },
            RgbMode::RGB => { gl::RGB },
        }
    }
}
