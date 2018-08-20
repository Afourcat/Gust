//! Module to handle drawable texture that are called Sprite

use texture::Texture;
use object::{VertexBuffer,Primitive};
use std::rc::Rc;
use draw::{Drawable,Drawer};
use color::Color;
use vertex::Vertex;
use nalgebra::*;
use nalgebra;
use draw::{Movable};

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
    model: Matrix4<f32>
}

impl Sprite {
    /// Create a empty sprite
    /// It's not very useful but you can assign texture later
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
            model: Matrix4::identity(),
        }
    }

    #[inline]
    /// Convert pos from screen to offset
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

    /// Set a new color
    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    /// Get texture color
    pub fn get_color(&self) -> Color {
        self.color
    }

}

impl From<Rc<Texture>> for Sprite {

    /// You can create sprite from texture (precisly Rc<Texture>)
    /// ```
    /// let personnage = Sprite::from(Rc::clone(&texture));
    /// ...
    /// window.clear();
    /// window.draw(&personnage);
    /// window.display();
    /// ...
    /// ```
    fn from(tex: Rc<Texture>) -> Sprite {
        let pos = Vector2::new(100.0, 100.0);
        let width = tex.get_width() as f32;
        let height = tex.get_height() as f32;
        let mut new = Sprite {
            pos: pos,
            scale: Vector2::new(1.0, 1.0),
            color: Color::white(),
            vertice: Box::new(VertexBuffer::new_from_vertex_array(Primitive::TrianglesStrip, &[
                Vertex::new(Vector2::new(0.0,      0.0), Vector2::new(0.0, 0.0), Color::white()),
                Vertex::new(Vector2::new(0.0,   height), Vector2::new(0.0, 1.0), Color::white()),
                Vertex::new(Vector2::new(width,    0.0), Vector2::new(1.0, 0.0), Color::white()),
                Vertex::new(Vector2::new(width, height), Vector2::new(1.0, 1.0), Color::white()),
//                Sprite::new_rect_vertex(pos,   0.0,    0.0, Vector2::new(0.0, 1.0), Color::red()),          //  botom left    .__________.
//                Sprite::new_rect_vertex(pos,   0.0, height, Vector2::new(0.0, 0.0), Color::green()),        //  top left      |          |
//                Sprite::new_rect_vertex(pos, width,    0.0, Vector2::new(1.0, 1.0), Color::blue()),         //  bottom right  |          |
//                Sprite::new_rect_vertex(pos, width, height, Vector2::new(1.0, 0.0), Color::red()),          //  top right     .__________.
            ])),
            model: Matrix4::identity(),
        };

        new.model.append_translation(&Vector3::new(pos.x, pos.y, 0.0));
        new.vertice.assign_texture(tex);
        new
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
        window.activate_shader();
        window.get_shader().uniform_mat4f("model", self.model);
        self.vertice.draw(window);
    }

    fn update(&mut self) {
        unimplemented!();
    }

    fn assign_texture<'a>(&mut self, texture: Rc<Texture>) {
        self.vertice.assign_texture(texture);
    }
}
