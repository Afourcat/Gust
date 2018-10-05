//! This module is for texture handling
//! Importing, Loading, Pushing into OpenGl
//! I'm using image crate that is really useful


use image::{DynamicImage};
use image;
use gl;
use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use ::Vector;
use std::error::Error;

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
    rgb_mode: RgbMode
}

impl Texture {

//--------------------CONSTRUCTOR---------------------//

    /// Create an empty texture
    pub fn new() -> Texture {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id); };
        Texture {
            id: id,
            width: 0,
            height: 0,
            rgb_mode: RgbMode::RGBA
        }
    }

    /// Create a texture from a raw data pointer needed for Font handling
    pub unsafe fn from_data
    (data: *const c_void, mode: RgbMode, width: u32, height: u32) -> Texture {
        Texture {
            id: Self::create(data, mode.as_gl(), width as i32, height as i32),
            width: width as u32,
            height: height as u32,
            rgb_mode: mode
        }
    }

    /// Create an empty texture with a size
    pub fn from_size(sizes: Vector<u32>) -> Texture {
        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0, 0,
                gl::RGBA as i32,
                sizes.x as i32, sizes.y as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                vec![0; sizes.x as usize * sizes.y as usize * 4].as_ptr() as *const c_void
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        };
        Texture {
            id: id,
            width: sizes.x,
            height: sizes.y,
            rgb_mode: RgbMode::RGBA
        }
    }

    /// Create a texture from an image
    pub fn from_image(img: DynamicImage) -> Texture {
        let mut id = 0;
        let mut size = (0, 0);
        let mode;

        match img {
            DynamicImage::ImageRgba8(data) => unsafe {
                size.0 = data.width();
                size.1 = data.height();
                id = Self::create(
                    mem::transmute(&data.into_raw()[0]),
                    gl::RGBA,
                    size.0 as i32,
                    size.1 as i32
                );
                mode = RgbMode::RGBA;
            },
            DynamicImage::ImageRgb8(data) => unsafe {
                size.0 = data.width();
                size.1 = data.height();
                id = Self::create(
                    mem::transmute(&data.into_raw()[0]),
                    gl::RGB,
                    size.0 as i32,
                    size.1 as i32
                );
                mode = RgbMode::RGB;
            },
            _ => {
                println!("Error while loading !");
                mode = RgbMode::RGBA
            },
        }

        Texture {
            id: id,
            width: size.0,
            height: size.1,
            rgb_mode: mode
        }
    }

    /// Create new texture from file path
    pub fn from_path(path_to_file: &str) -> Result<Texture,TextureError> {
        if let Ok(img) = image::open(path_to_file) {
            Ok(Texture::from_image(img))
        } else {
            Err(TextureError::FileError)
        }
    }

    /// Create a texture with a
    fn create
    (data: *const c_void, rgb_mode: GLenum, width: i32, height: i32) -> u32 {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            Texture::default_param();
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

    /// Update a block of a texture with an offset and a size
    /// return TextureError if the sizes are not correct.
    pub fn update_block(
        &mut self,
        data: Vec<u8>,
        sizes: Vector<u32>,
        pos: Vector<u32>,
        rgb_mode: RgbMode
    ) -> Result<(), TextureError> {
        
        println!("SIZES {:?}", sizes);
        println!("POS {:?}", pos);
        println!("Mode {:?}", rgb_mode);
        println!("TEXTSIZES {:?}", Vector::new(self.width, self.height));

        // If sizes are fucked up return an error
        if pos.x + sizes.x >= self.width || pos.y + sizes.y >= self.height {
            Err(TextureError::UpdateSize)
        } else {

            // Bind the texture then give it to opengl
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.id);
                gl::TexSubImage2D(
                    gl::TEXTURE_2D,
                    0,
                    pos.x as i32,
                    pos.y as i32,
                    sizes.x as i32,
                    sizes.y as i32,
                    rgb_mode as u32,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const c_void
                    );
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
            Ok(())
        }
    }

    pub fn get_rawsize(&self) -> usize {
        match self.rgb_mode {
            RgbMode::RGBA => {
                (self.height * self.width * 4) as usize
            },
            RgbMode::RGB => {
                (self.height * self.width * 3) as usize
            }
            _ => { panic!("Da fuck !") }
        }
    }

    pub fn update_from_texture(&mut self, texture: &Texture) {
        let size = texture.get_rawsize();
        let data: Vec<u8> = Vec::with_capacity(size);

        if self.rgb_mode != texture.rgb_mode {
            panic!("Try to assemble texture with different rgb_mode");
        }
        unsafe {
            gl::GetTextureImage(
                self.id,
                0,
                self.rgb_mode.as_gl(),
                gl::UNSIGNED_BYTE,
                size as i32,
                data.as_ptr() as *mut c_void
            );
        }
        let h = self.height;
        let w = self.width;
        let mode = self.rgb_mode;
        self.update(
            data,
            Vector::new(
                w as i32,
                h as i32
            ),
            mode
        );
    }

    /// Update the data of the texture
    pub fn update
    (&mut self, data: Vec<u8>, sizes: Vector<i32>, mode: RgbMode) {

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                mode.clone() as i32,
                sizes.x,
                sizes.y,
                0,
                mode as u32,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        
        self.rgb_mode = mode;
        self.width = sizes.x as u32;
        self.height= sizes.y as u32;
    }

//--------------------------UTILS----------------------------//

    /// Repeat mode texture wrap
    pub fn repeat_mode(&self) {
        self.active(0);
        unsafe {
            gl::TexParameteri
            (gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);

            gl::TexParameteri
            (gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }
        self.unbind();
    }

    /// Linear mode for filter
    pub fn linear_mode(&self) {
        self.active(0);
        unsafe {
            gl::TexParameteri
                (gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri
                (gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        self.unbind();
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn active(&self, num: i32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + num as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    unsafe fn default_param() {
        gl::TexParameteri
            (gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri
            (gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri
            (gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri
            (gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    //-------------------------GETTER-----------------------//

    /// Simple getter for width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Simple getter for height
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Default for Texture {
    fn default() -> Texture {
        let mut id = 0;
        unsafe {
            let data = vec![255, 255, 255, 255];
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            Texture::default_param();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                1,
                1,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void
            );
        };

        Texture {
            id: id,
            width: 1,
            height: 1,
            rgb_mode: RgbMode::RGBA
        }
    }
}

/// Enum to wrap gl RGB modes
#[derive(PartialEq,Clone,Copy,Debug,Eq)]
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

#[derive(Debug)]
pub enum TextureError {
    UpdateSize,
    FileError
}

use std::fmt;

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextureError::UpdateSize => {
                write!(f, "Error while updating
texture: Sizes are not okay with this texture.")
            },
            TextureError::FileError => {
                write!(f, "Error while openning given file")
            }
        }
    }
}

impl Error for TextureError {
    fn cause(&self) -> Option<&Error> {
        None
    }
}
