use gl::types::*;
use gl;
use texture::Texture;
use object::VertexBuffer;
use std::rc::Rc;
use drawable::Drawable;
use window::Window;
use shader::Shader;
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
pub struct Sprite {
    id: i32,
    pos: (i32, i32),
    angle: f64,
    texture: Option<Texture>,
    applying: Matrix2<f64>,
    shader: Shader,
}

impl Sprite {
    /// Create a empty sprite
    pub fn new() -> Sprite {
        Sprite {
            id: -1,
            pos: (0, 0),
            angle: 0.0,
            texture: None,
            applying: Matrix2::identity(),
            shader: Shader::default(),
        }
    }

    /// Create a new sprite from a texture
    pub fn from_texture(texture: &Texture) -> Sprite {
        Sprite {
            id: -1,
            pos: (0, 0),
            angle: 0.0,
            texture: Some(texture.clone()),
            applying: Matrix2::identity(),
            shader: Shader::default(),
        }
    }

    /// Rotate the sprite
    pub fn rotate(&mut self, rot: Rotation2<f64>) {
        //
    }
}

/// Drawing trait for sprite sturct
impl Drawable for Sprite {
    fn draw(&self, window: &mut Window) {
        if let Some(ref a) = self.texture { unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.id as u32);
            }
        }
    }

    fn assign_texture(&mut self, texture: &Texture) {
        self.texture = Some(texture.clone());
    }
}
