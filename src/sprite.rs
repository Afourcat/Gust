//
//
//
//
// 

use gl::types::*;
use gl;
use texture::Texture;
use object::VertexBuffer;
use std::rc::Rc;
use drawable::Drawable;
use window::Window;

pub struct Sprite {
    id: i32,
    pos: (i32, i32),
    texture: Option<Rc<Texture>>,
}

impl Sprite {
    
    /// Create a empty sprite
    pub fn new() -> Sprite {
        Sprite {
            id: -1,
            pos: (0, 0),
            texture: None,
        }
    }

    /// Create a new sprite from a texture
    pub fn new_from_texture(texture: Rc<Texture>) -> Sprite {
        

        Sprite {
            id: -1,
            pos: (0, 0),
            texture: None,
        }
    }

    pub fn set_texture(&self, texture: Rc<Texture>) -> Result<(),&'static str> {
        if true {
            Err("Fuck texture cannot be loaded !!")
        } else {
            Ok(())   
        }
    }
}

impl Drawable for Sprite {
    fn draw(&self, window: &mut Window) {
        if let Some(ref a) = self.texture { unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.id as u32);
            }
        }
    }
}
