//! This module is for texture handling
//! Importing, Loading, Pushing into OpenGl
//! I'm using image crate that is really useful


use image::{DynamicImage,ImageBuffer};
use image;
use gl;
use gl::types::*;
use std::os::raw::c_void;
use ::Vector;
use std::error::Error;
use color::Color;
use std::path::Path;

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
#[derive(Debug,Clone,PartialEq,Eq)]
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
            id,
            width: 0,
            height: 0,
            rgb_mode: RgbMode::RGBA
        }
    }

    /// Create a texture from a raw data pointer needed for Font handling unsafe version of
    /// from slice
    pub unsafe fn from_data(data: *mut c_void, mode: RgbMode, width: u32, height: u32) -> Texture {
        Texture {
            id: Self::create(data, mode.as_gl(), width as i32, height as i32),
            width: width as u32,
            height: height as u32,
            rgb_mode: mode
        }
    }

    /// Create a texture from a slice
    pub fn from_slice(data: &mut [u8], mode: RgbMode, width: u32, height: u32) -> Texture {
        Texture {
            id: Self::create(
                data.as_mut_ptr() as *mut c_void, mode.as_gl(), width as i32, height as i32),
            width: width as u32,
            height: height as u32,
            rgb_mode: mode
        }
    }

    pub fn from_color(color: Color, sizes: Vector<u32>) -> Texture {
        let length: usize = (sizes.x * sizes.y * 4) as usize;
        let mut data: Vec<u8> = Vec::with_capacity(length);
        let mut i: usize = 0;
        let color_u8: (u8, u8, u8, u8) = color.into();

        while i < length {
            data.push(color_u8.0);
            data.push(color_u8.1);
            data.push(color_u8.2);
            data.push(color_u8.3);
            i += 4;
        }
        Self::from_slice(data.as_mut_slice(), RgbMode::RGBA, sizes.x, sizes.y)
    }

    /// Create an empty texture with a size
    pub fn from_size(sizes: Vector<u32>) -> Texture {
        let mut ve: Vec<u8> = vec![255; sizes.x as usize * sizes.y as usize * 4];
        Self::from_slice(ve.as_mut_slice(), RgbMode::RGBA, sizes.x, sizes.y)
    }

    /// Create a texture from an image
    pub fn from_image(img: DynamicImage) -> Result<Texture,TextureError> {
        let id;
        let mut size = (0, 0);
        let mode;

        match img {
            DynamicImage::ImageRgba8(data) => {
                size.0 = data.width();
                size.1 = data.height();
                id = Self::create(
                    &data.into_raw()[0] as *const u8 as *mut std::ffi::c_void,
                    gl::RGBA,
                    size.0 as i32,
                    size.1 as i32
                );
                mode = RgbMode::RGBA;
            },
            DynamicImage::ImageRgb8(data) => {
                size.0 = data.width();
                size.1 = data.height();
                id = Self::create(
                    &data.into_raw()[0] as *const u8 as *mut std::ffi::c_void,
                    gl::RGB,
                    size.0 as i32,
                    size.1 as i32
                );
                mode = RgbMode::RGB;
            },
            _ => {
                return Err(TextureError::ImageLoading);
            },
        }

        Ok(
        Texture {
            id,
            width: size.0,
            height: size.1,
            rgb_mode: mode
        })
    }

    /// Create new texture from file path
    pub fn from_path<P: AsRef<Path>>(path_to_file: P) -> Result<Texture,TextureError> {
        if let Ok(img) = image::open(path_to_file) {
            Texture::from_image(img)
        } else {
            Err(TextureError::FileError)
        }
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(),Box<Error>> {
        let data = self.get_data();

        match self.rgb_mode {
            RgbMode::RGBA => {
                let image: Option<image::RgbaImage>
                    = ImageBuffer::from_vec(self.width, self.height, data);

                if let Some(img) = image {
                    img.save(path)?;
                } else {
                    return Err(Box::new(TextureError::WriteFile));
                }
            },
            RgbMode::RGB => {
                let image: Option<image::RgbImage>
                    = ImageBuffer::from_vec(self.width, self.height, data);

                if let Some(img) = image {
                    img.save(path)?;
                } else {
                    return Err(Box::new(TextureError::WriteFile));
                }
            },
            _ => { unimplemented!() }
        }
        Ok(())
    }

    /// Create a texture with a
    fn create
    (data: *mut c_void, rgb_mode: GLenum, width: i32, height: i32) -> u32 {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexStorage2D(   // Create the storage
                gl::TEXTURE_2D,
                1,
                if rgb_mode == gl::RGBA { gl::RGBA8 } else { gl::RGB8 },
                width,
                height
            );
            gl::TexSubImage2D(  // Put pixel inside the storage
                gl::TEXTURE_2D,
                0,
                0,
                0,
                width,
                height,
                rgb_mode,
                gl::UNSIGNED_BYTE,
                data
            );
            Texture::default_param();
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        id
    }

    /// Update a block of a texture with an offset and a size
    /// return TextureError if the sizes are not correct.
    pub fn update_block<T, U>(
        &mut self,
        data: &[u8],
        sizes: Vector<u32>,
        pos: T,
        rgb_mode: U
    ) -> Result<(), TextureError> 
    where 
        T: Into<Option<Vector<u32>>>,
        U: Into<Option<RgbMode>>
    {
        let pos = pos.into().unwrap_or_else(|| Vector::new(0, 0));
        let rgb_mode = rgb_mode.into().unwrap_or(self.rgb_mode);
        // If sizes are fucked up return an error
        if data.is_empty() {
            Ok(())
        } else if pos.x + sizes.x > self.width || pos.y + sizes.y > self.height {
            Err(TextureError::UpdateSize(
                self.width, pos.x + sizes.x, self.height, pos.y + sizes.y))
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
                    rgb_mode.as_gl(),
                    gl::UNSIGNED_BYTE,
                    data as *const _ as *const c_void
                    //mem::transmute(data.as_ptr())
                );
                gl::BindTexture(gl::TEXTURE_2D, 0);
                gl::Flush();
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
            },
            RgbMode::RED => {
                (self.height * self.width) as usize
            }
        }
    }

    /// Get a Vec<u8> representing pixels of the texture
    pub fn get_data(&self) -> Vec<u8> {
        let size = self.get_rawsize();
        let mut data: Vec<u8> = Vec::with_capacity(size);

        if size == 0 || self.id == 0 {
            Vec::new()
        } else {
            unsafe {
                data.set_len(size);
                gl::BindTexture(gl::TEXTURE_2D, self.id);
                gl::GetTexImage(
                    gl::TEXTURE_2D,
                    0,
                    self.rgb_mode.as_gl(),
                    gl::UNSIGNED_BYTE,
                    data.as_mut_ptr() as *mut c_void
                    );
                gl::BindTexture(gl::TEXTURE_2D, 0);
            };
           data
        }
    }

    /// Nicely working efficiently legacy fb deleted
    pub fn update_from_texture
    (&mut self, texture: &Texture, pos: Vector<u32>) -> Result<(),TextureError>{
        let size = texture.get_rawsize();
        let mut data: Vec<u8> = Vec::with_capacity(size);

        if self.rgb_mode != texture.rgb_mode {
            return Err(TextureError::UpdateMode(self.rgb_mode, texture.rgb_mode));
        }
        unsafe {
            data.set_len(size);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                self.rgb_mode.as_gl(),
                gl::UNSIGNED_BYTE,
                data.as_mut_ptr() as *mut c_void
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        let h = texture.height;
        let w = texture.width;
        let mode = texture.rgb_mode;
        // Then update block
        self.update_block(data.as_slice(), Vector::new(w, h), pos, mode)
    }

    /// Update the data of the texture
    pub fn update<T>(&mut self, data: &[u8], mode: T) -> Result<(),TextureError>
    where
        T: Into<Option<RgbMode>>,
    {
        let mode = mode.into().unwrap_or(self.rgb_mode);
        let w = self.width;
        let h = self.height;

        self.update_block(data, Vector::new(w, h), Vector::new(0, 0), mode)?;
        Ok(())
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

    #[inline]
    /// Unbind the texture
    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0); }
    }

    #[inline]
    /// Active texture num
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

    /// Getter for color mode
    pub fn rgb_mode(&self) -> &RgbMode {
        &self.rgb_mode
    }

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
    /// Create a 1 white pixel texture
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
            id,
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
    RGB,
    RED
}

impl RgbMode {
    pub fn as_gl(self) -> GLenum {
        match self {
            RgbMode::RGBA => { gl::RGBA },
            RgbMode::RGB => { gl::RGB },
            RgbMode::RED => { gl::RED }
        }
    }
}

#[derive(Debug)]
pub enum TextureError {
    UpdateSize(u32, u32, u32, u32),
    UpdateMode(RgbMode, RgbMode),
    FileError,
    ImageLoading,
    WriteFile
}

use std::fmt;

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextureError::UpdateSize(x, new_x, y, new_y) => {
                write!(f, "
Error while updating texture: Sizes are not
okay with this texture. x: {}
< new_x: {}  | y: {} new_y: {}", x, new_x, y, new_y)
            },
            TextureError::FileError => {
                write!(f, "Error while openning given file.")
            },
            TextureError::ImageLoading => {
                write!(f, "Error While loading image.")
            },
            TextureError::WriteFile => {
                write!(f, "Error while writing texture to file.")
            },
            TextureError::UpdateMode(a, b) => {
                write!(f, "You were trying to update a {:?} with a {:?} Texture", a, b)
            }
        }
    }
}

impl Error for TextureError {
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            println!("Texture {} deleted", self.id);
            gl::DeleteTextures(1, &[self.id] as *const _);
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::Vector;
    use texture::Texture;
    use self::test::Bencher;
    use color::Color;
    use window::Window;
    use texture::RgbMode;

    #[bench]
    fn from_color(b: &mut Bencher) {
        let _ = Window::new(200, 200, "Loader");

        b.iter(|| {
            Texture::from_color(Color::new(1.0, 1.0, 1.0), Vector::new(100, 100));
        });
    }

    #[bench]
    fn from_slice(b: &mut Bencher) {
        let _ = Window::new(200, 200, "Loader");

        b.iter(|| {
            let mut slice = vec![255; 10000];

            Texture::from_slice(slice.as_mut_slice(), RgbMode::RGBA, 100, 100);
        });
    }

    #[bench]
    fn update_block(b: &mut Bencher) {
        let _ = Window::new(200, 200, "Loader");

        let mut text_host = Texture::from_color(Color::new(0.0, 1.0, 0.0), Vector::new(100, 100));
        let text_guest = Texture::from_color(Color::new(0.0, 0.0, 1.0), Vector::new(10, 10));

        b.iter(|| {
            text_host.update_block(
                text_guest.get_data().as_slice(),
                Vector::new(10, 10),
                Vector::new(10, 10),
                None).unwrap();
        });
    }
}
