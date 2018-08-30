//! Module to handle drawable texture that are called Sprite

use texture::Texture;
use vertex_buffer::{VertexBuffer,Primitive};
use std::rc::Rc;
use draw::{Drawable,Drawer,Context,BlendMode};
use color::Color;
use vertex::Vertex;
use nalgebra::*;
use nalgebra;
use draw::{Movable};
use vertex::*;
use shader::Shader;

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
    vertice: Box<VertexBuffer>,
    rotation: Option<Matrix4<f32>>,
    texture: Option<Rc<Texture>>,
    model: Matrix4<f32>,
    auto_update: bool,
}

impl Sprite {
    /// Create a empty sprite
    /// It's not very useful but you can assign texture later
    pub fn new() -> Sprite {
        Sprite {
            pos: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            vertice: Box::new(
                VertexBuffer::new( Primitive::TrianglesStrip, VertexArray::new(vec! [
                    Vertex::default(),
                    Vertex::default(),
                    Vertex::default(),
                    Vertex::default(),
                ]))
            ),
            texture: None,
            model: Matrix4::identity(),
            rotation: None,
            auto_update: false,
        }
    }

    pub fn set_color(&mut self, color: &Color) {
        self.vertice[0].color = color.clone();
        self.vertice[1].color = color.clone();
        self.vertice[2].color = color.clone();
        self.vertice[3].color = color.clone();
    }
}

impl<'a> From<&'a Rc<Texture>> for Sprite {

    /// You can create sprite from texture (precisly Rc<Texture>)
    /// ```
    /// let personnage = Sprite::from(Rc::clone(&texture));
    /// ...
    /// window.clear();
    /// window.draw(&personnage);
    /// window.display();
    /// ...
    /// ```
    fn from(tex: &'a Rc<Texture>) -> Sprite {
        let width = tex.get_width() as f32;
        let height = tex.get_height() as f32;
        let pos = Vector2::new(0.0, 0.0);
        Sprite {
            pos: pos,
            scale: Vector2::new(1.0, 1.0),
            vertice: Box::new(VertexBuffer::new(Primitive::TrianglesStrip,
                VertexArray::new(vec![
                    Vertex::new(Vector2::new(0.0,      0.0), Vector2::new(0.0, 0.0), Color::white()),
                    Vertex::new(Vector2::new(0.0,   height), Vector2::new(0.0, 1.0), Color::white()),
                    Vertex::new(Vector2::new(width,    0.0), Vector2::new(1.0, 0.0), Color::white()),
                    Vertex::new(Vector2::new(width, height), Vector2::new(1.0, 1.0), Color::white()),
                ])
            )),
            texture: Some(Rc::clone(tex)),
            model: Matrix4::identity().append_translation(&Vector3::new(pos.x, pos.y, 0.0)),
            rotation: None,
            auto_update: false,
        }
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

    fn rotate<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T) {
        if let Some(mut rot) = self.rotation {
            rot *= Matrix4::from_euler_angles(
                0.0, 0.0, angle.into() * (3.14116 / 180.0));
        } else {
            self.set_rotation(angle);
        }
    }

    fn set_rotation<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T) {
        self.rotation = Some(Matrix4::from_euler_angles(
                0.0, 0.0, angle.into() * (3.14116 / 180.0)
        ));
    }

    fn get_rotation(&self) -> f32 {
        if let Some(rot) = self.rotation {
            rot[0]
        } else {
            0.0
        }
    }
}
//
/// Drawing trait for sprite sturct
impl Drawable for Sprite {
    fn draw<T: Drawer>(&self, window: &mut T) {
        self.draw_with_context(window, &mut Context::new(
                    if let Some(ref rc_texture) = self.texture {
                        Some(rc_texture.as_ref())
                    } else {
                        None
                    },
                    Shader::default(),
                    Some(Matrix4::<f32>::identity() * self.model),
                    BlendMode::Alpha
        ));
    }

    fn draw_with_context<'a, T: Drawer>
    (&self, window: &mut T, context: &'a mut Context) {
        self.vertice.draw_with_context(window, context);
    }

    fn update(&mut self) {
        self.model = Matrix4::<f32>::identity().append_translation(
            &Vector3::new(self.pos.x, self.pos.y, 0.0)
        );

        self.model.append_nonuniform_scaling(
            &Vector3::new(self.scale.x, self.scale.y, 0.0)    
        );

        if let Some(rotation) = self.rotation {
            self.model *= rotation;   
        }
    }

    fn set_texture(&mut self, texture: &Rc<Texture>) {
        self.texture = Some(Rc::clone(texture))
    }
}
