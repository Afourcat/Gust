//! Module to handle drawable texture that are called Sprite

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
use nalgebra;
use draw::{Movable};
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
#[derive(Debug,Clone,PartialEq)]
pub struct Sprite {
    pos: Vector2<f32>,
    scale: Vector2<f32>,
    color: Color,
    vertice: Box<VertexBuffer>,
}

impl Sprite {
    /// Create a empty sprite
    pub fn new() -> Sprite {
        Sprite {
            pos: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
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

    fn new_rect_vertex(pos: Vector2<f32>, x: f32, y: f32, text_coord: Vector2<f32> , color: Color) -> Vertex {
        let posy = Self::convert_pos(pos.y, super::HEIGHT as f32, y);
        let posx = Self::convert_pos(pos.x, super::WIDTH as f32, x);
        Vertex::new(
            Vector2::new(
                posx,
                posy
            ),
            text_coord,
            color,
        )
    }

    /// Create a new sprite from a texture
    pub fn from_texture(texture: Rc<Texture>) -> Sprite {
        let pos = Vector2::new(-250.0, -250.0);
        let width = texture.get_width() as f32;
        let height = texture.get_height() as f32;

        let mut new = Sprite {
            pos: pos,
            scale: Vector2::new(1.0, 1.0),
            color: Color::white(),
            vertice: Box::new(VertexBuffer::new_from_vertex_array(Primitive::TrianglesStrip, &[
                Sprite::new_rect_vertex(pos,   0.0,    0.0, Vector2::new(0.0, 0.0), Color::white()),
                Sprite::new_rect_vertex(pos,   0.0, height, Vector2::new(0.0, -1.0), Color::red()),
                Sprite::new_rect_vertex(pos, width,    0.0, Vector2::new(-1.0, 0.0), Color::blue()),
                Sprite::new_rect_vertex(pos, width, height, Vector2::new(-1.0, -1.0), Color::green()),
            ])),
        };
        new.vertice.assign_texture(texture);
        new
    }

    /// Set a new color for the sprite
    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

}

impl Movable for Sprite {

    fn translate<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector2<T>) {
        self.pos.x += vec.x.into();
        self.pos.y += vec.y.into();
    }

    fn set_scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector2<T>) {
        self.scale.x = vec.x.into();
        self.scale.y = vec.y.into();
    }

    fn get_scale(&self) -> Vector2<f32> {
        self.scale
    }

    fn scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, factor: Vector2<T>) {
        self.scale.x += factor.x.into();
        self.scale.y += factor.y.into();
    }

    fn get_position(&self) -> Vector2<f32> {
        self.pos
    }

    fn set_position<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector2<T>) {
        self.pos.x = vec.x.into();
        self.pos.y = vec.y.into();
    }
}
//
/// Drawing trait for sprite sturct
impl Drawable for Sprite {
    fn draw<T: Drawer>(&self, window: &mut T) {
        self.vertice.draw(window);
    }

    fn assign_texture<'a>(&mut self, texture: Rc<Texture>) {
        self.vertice.assign_texture(texture);
    }
}
