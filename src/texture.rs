//! This module is for texture handling
//! Importing, Loading, Pushing into OpenGl
//! I'm using image crate that is really useful


use image::{DynamicImage,ImageBuffer};
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
    (data: *mut c_void, mode: RgbMode, width: u32, height: u32) -> Texture {
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
        let mut ve: Vec<u8> = vec![255; sizes.x as usize * sizes.y as usize * 4];

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            Self::from_data(ve.as_mut_ptr() as *mut c_void, RgbMode::RGBA, sizes.x, sizes.y);
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

    pub fn to_file(&self, path: &str) -> Result<(),&'static str> {
        unsafe {
            let data = self.get_data();
            
            let image = if self.rgb_mode == RgbMode::RGBA {
                let image: image::RgbaImage = ImageBuffer::from_vec(self.width, self.height, data).unwrap();
                image.save(path).unwrap();
            } else {
                let image: image::RgbImage = ImageBuffer::from_vec(self.width, self.height, data).unwrap();
                image.save(path).unwrap();
            };
            Ok(())
        }
    }

    /// Create a texture with a
    fn create
    (data: *mut c_void, rgb_mode: GLenum, width: i32, height: i32) -> u32 {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexStorage2D(gl::TEXTURE_2D,
                             1,
                             if rgb_mode == gl::RGBA { gl::RGBA8 } else { gl::RGB8 },
                             width,
                             height
            );
            gl::TexSubImage2D(
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
        let pos = pos.into().unwrap_or(Vector::new(0, 0));
        let rgb_mode = rgb_mode.into().unwrap_or(self.rgb_mode);
        // If sizes are fucked up return an error
        if data.len() == 0 {
            Ok(())
        } else if pos.x + sizes.x > self.width || pos.y + sizes.y > self.height {
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
                    rgb_mode.as_gl(),
                    gl::UNSIGNED_BYTE,
                    data as *const _ as *const c_void
                    //mem::transmute(data.as_ptr())
                );
                let a = gl::GetError();
                match a {
                    gl::INVALID_ENUM => {println!("SALUT");},
                    gl::NO_ERROR => {println!("TRKL");},
                    _ => {println!("AUTRE");}
                }
                println!("{:?}", a);
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
            }
        }
    }

    /// Unsafe for now
    pub unsafe fn get_data(&self) -> Vec<u8> {
        let size = self.get_rawsize();
        let mut data: Vec<u8> = Vec::with_capacity(size);

        println!("size {}", size);
        if size == 0 { return Vec::new(); }
        if true {
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
            return data;
        } else {
            let mut old = 0;
            let mut new = 0;
            let data: &mut [u8] = &mut [0];

            gl::GenFramebuffers(1, &mut new);
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut old);
            gl::BindFramebuffer(gl::FRAMEBUFFER, new);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D, self.id, 0);
            gl::ReadPixels(0, 0, self.width as i32, self.height as i32, self.rgb_mode.as_gl(),
                gl::UNSIGNED_BYTE, mem::transmute(&mut data[0]));
            gl::DeleteFramebuffers(1, &new);
            gl::BindFramebuffer(gl::FRAMEBUFFER, old as u32);
            return Vec::from(data);
        }
    }

    /// Two version for know using framebuffer
    /// Will be another version created
    pub fn update_from_texture(&mut self, texture: &Texture, pos: Vector<u32>) {

        // TRYING FRAMEBUFFER
        if false {
            let mut read: i32 = 0;
            let mut draw: i32 = 0;

            unsafe {
                gl::GetIntegerv(gl::READ_FRAMEBUFFER_BINDING, &mut read);
                gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut draw);
            }

            let mut source_frame = 0;
            let mut dest_frame = 0;

            unsafe {
                gl::GenFramebuffers(1, &mut source_frame);
                gl::GenFramebuffers(1, &mut dest_frame);

                gl::BindFramebuffer(gl::READ_FRAMEBUFFER, source_frame);
                gl::FramebufferTexture2D(gl::READ_FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                                         gl::TEXTURE_2D, texture.id, 0);

                gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, dest_frame);
                gl::FramebufferTexture2D(gl::DRAW_FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                                         gl::TEXTURE_2D, self.id, 0);

                gl::BlitFramebuffer(
                    0, 0, texture.width as i32, texture.height as i32,
                    pos.x as i32, pos.y as i32,
                    (pos.x + texture.width) as i32, (pos.y + texture.height) as i32,
                    gl::COLOR_BUFFER_BIT, gl::NEAREST
                );

                gl::BindFramebuffer(gl::READ_FRAMEBUFFER, read as u32);
                gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, draw as u32);

                gl::BindTexture(gl::TEXTURE_2D, self.id);
                Texture::default_param();
                gl::BindTexture(gl::TEXTURE_2D, 0);

                gl::Flush();
            }
        } else {
            let size = texture.get_rawsize();
            let mut data: Vec<u8> = Vec::with_capacity(size);

            if self.rgb_mode != texture.rgb_mode {
                panic!("Try to assemble texture with different rgb_mode");
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
            let h = self.height;
            let w = self.width;
            let mode = self.rgb_mode;
            // Then update block
            self.update_block(data.as_slice(), Vector::new(w, h), pos, mode).unwrap();
        }
    }

    /// Update the data of the texture
    pub fn update<T>(&mut self, mut data: &[u8], mode: T) 
    where 
        T: Into<Option<RgbMode>>,
    {
        let mode = mode.into().unwrap_or(self.rgb_mode);
        let w = self.width;
        let h = self.height;

        self.update_block(data, Vector::new(w, h), Vector::new(0, 0), mode);
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
    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0); }
    }
    
    #[inline]
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
