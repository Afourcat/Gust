use gl::types::*;
use gl;
use texture::Texture;
use object::{VertexBuffer,Primitive};
use std::rc::Rc;
use draw::{Drawable,Drawer};
use window::Window;
use shader::Shader;
use color::Color;
use vertex::Vertex;
use nalgebra::*;
use std::borrow::BorrowMut;

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
#[derive(Debug)]
pub struct Sprite {
    pos: Vector2<f32>,
    texture: Option<Rc<Texture>>,
    color: Color,
    vertice: Box<VertexBuffer>,
}

impl Sprite {
    /// Create a empty sprite
    pub fn new() -> Sprite {
        Sprite {
            pos: Vector2::new(0.0, 0.0),
            texture: None,
            color: Color::white(),
            vertice: Box::new(VertexBuffer::new_from_vertex_array(Primitive::TrianglesStrip, &[
                Vertex::default(),
                Vertex::default(),
                Vertex::default(),
                Vertex::default(),
            ])),
        }
    }

    fn convert_pos(pos: f32, screen: f32, offset: f32) -> f32 {
        (pos + offset) / screen
    }

    fn new_rect_vertex(pos: Vector2<f32>, x: f32, y: f32, color: Color) -> Vertex {
        Vertex::new(
            Vector2::new(
                Self::convert_pos(pos.x, super::WIDTH as f32, x),
                Self::convert_pos(pos.y, super::HEIGHT as f32, y)
            ),
            Vector2::new(1.0, 1.0),
            color,
        )
    }

    /// Create a new sprite from a texture
    pub fn from_texture(texture: Rc<Texture>) -> Sprite {
        let pos = Vector2::new(0.0, 0.0);

        let mut new = Sprite {
            pos: pos,
            texture: Some(Rc::clone(&texture)),
            color: Color::white(),
            vertice: Box::new(VertexBuffer::new_from_vertex_array(Primitive::TrianglesStrip, &[
                Sprite::new_rect_vertex(pos, 0.0, 0.0, Color::white()),
                Sprite::new_rect_vertex(pos, texture.get_width() as f32, 0.0, Color::white()),
                Sprite::new_rect_vertex(pos, 0.0, texture.get_height() as f32, Color::white()),
                Sprite::new_rect_vertex(pos, texture.get_width() as f32, texture.get_height() as f32, Color::white()),
            ])),
        };
        new.vertice.assign_texture(texture);
        new
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
        self.vertice.draw(window);
    }

    fn assign_texture(&mut self, texture: Rc<Texture>) {
        self.texture = Some(Rc::clone(&texture));
    }
}
