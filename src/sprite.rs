use gl::types::*;
use gl;
use texture::Texture;
use object::VertexBuffer;
use std::rc::Rc;
use draw::{Drawable,Drawer};
use window::Window;
use shader::Shader;
use color::Color;
use vertex::Vertex;
use nalgebra::*;

/// A sprite is a transformable
/// drawable sprite
/// > Display a sprite from a texture
/// ```Rust
/// let texture = Texture::new("assets/texture.jpg");
/// let sprite = Sprite::from_texture(Rc::clone(&texutre));
/// sprite.rotate(
///     Rotation2::new(
///         Vector2::new(0.1,-0.1,0.0)));
/// window.clear();
/// window.draw(sprite);
/// window.display();
/// ```
/// > A sprite is just attributes for textures to become printable ...
pub struct Sprite {
    id: i32,
    pos: (i32, i32),
    texture: Option<Rc<Texture>>,
    color: Color,
    vertices: [Vertex; 4],
}

impl Sprite {
    /// Create a empty sprite
    pub fn new() -> Sprite {
        Sprite {
            id: -1,
            pos: (0, 0),
            texture: None,
            color: Color::white(),
            vertices: [
                Vertex::from_texture(Vector2::new(1.0, 1.0), Vector2::new(1.0, 1.0)),
                Vertex::from_texture(Vector2::new(1.0, 1.0), Vector2::new(1.0, 1.0)),
                Vertex::from_texture(Vector2::new(1.0, 1.0), Vector2::new(1.0, 1.0)),
                Vertex::from_texture(Vector2::new(1.0, 1.0), Vector2::new(1.0, 1.0))
            ],
        }
    }

    /// Create a new sprite from a texture
    pub fn from_texture(texture: Rc<Texture>) -> Sprite {
        Sprite {
            id: -1,
            pos: (0, 0),
            texture: Some(Rc::clone(&texture)),
            color: Color::white(),
            vertices: [
                Vertex::default(),
                Vertex::default(),
                Vertex::default(),
                Vertex::default(),
            ],

        }
    }

    /// Set a new color for the sprite
    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    pub fn set_texture(&mut self, new_texture: Rc<Texture>) {
        self.texture = Some(Rc::clone(&new_texture));
    }

    /// Rotate the sprite
    pub fn rotate(&mut self, rot: Rotation2<f64>) {
        //
    }

}

/// Drawing trait for sprite sturct
impl Drawable for Sprite {
    fn draw<T: Drawer>(&self, window: &mut T) {
        if let Some(ref a) = self.texture { unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.id as u32);
            }
        }
    }

    fn assign_texture(&mut self, texture: Rc<Texture>) {
        self.texture = Some(Rc::clone(&texture));
    }
}
