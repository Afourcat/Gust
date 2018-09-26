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
use shader::{Shader,DEFAULT_SHADER};
use rect::Rect;

/// A sprite is a transformable
/// drawable sprite
/// > Display a sprite from a texture
/// ```Rust
/// use texture::Texture;
/// use sprite::Sprite;
/// 
/// let texture = Texture::new("assets/texture.jpg");
/// let sprite = Sprite::from_texture(Rc::clone(&texutre));
/// sprite.rotate(45.0);
/// sprite.set_position(Vector2::new(100.0, 200.0));
/// ```
/// > A sprite is just attributes for textures to become printable ...
#[derive(Debug,Clone,PartialEq)]
pub struct Sprite {
    pos: Vector2<f32>,
    scale: Vector2<f32>,
    rotation: f32,
    origin: Vector2<f32>,
    vertice: Box<VertexBuffer>,
    texture: Option<Rc<Texture>>,
    model: Matrix4<f32>,
    auto_update: bool,
    need_update: bool
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
            need_update: true,
            texture: None,
            origin: Vector2::new(0.0, 0.0),
            model: Matrix4::identity(),
            rotation: 0.0,
            auto_update: false,
        }
    }

    pub fn set_color(&mut self, color: &Color) {
        self.vertice[0].color = color.clone();
        self.vertice[1].color = color.clone();
        self.vertice[2].color = color.clone();
        self.vertice[3].color = color.clone();
        self.vertice.update();
    }

    pub fn get_sizes(&self) -> Vector2<usize> {
        if let Some(ref texture) = self.texture {
            Vector2::new(
                texture.width() as usize,
                texture.height() as usize
            )
        } else {
            Vector2::new(0, 0)
        }
    }

    pub fn set_origin_to_center(&mut self) -> Result<(),&'static str> {

        if let Some(_) = self.texture {
            let mut center = Vector2::new(0.0, 0.0);
            let sizes = self.get_sizes();
            center.x = sizes.x as f32 / 2.0;
            center.y = sizes.y as f32 / 2.0;
            self.set_origin(center);
            Ok(())
        } else {
            return Err("You should set a texture before !")
        }
    }
}

impl<'a> From<&'a Rc<Texture>> for Sprite {

    /// You can create sprite from texture (precisly Rc<Texture>)
    /// ```no_run
    /// use gust::texture::Texture;
    /// use gust::sprite::Sprite;
    /// use std::rc::Rc;
    ///
    /// let texture = Rc::new(Texture::new("My great texture"));
    /// let personnage = Sprite::from(&texture);
    /// ```
    fn from(tex: &'a Rc<Texture>) -> Sprite {
        let width = tex.width() as f32;
        let height = tex.height() as f32;
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
            need_update: true,
            model: Matrix4::identity().append_translation(&Vector3::new(pos.x, pos.y, 0.0)),
            rotation: 0.0,
            origin: Vector2::new(0.0, 0.0),
            auto_update: false,
        }
    }
}

impl Movable for Sprite {

    fn contain<T: nalgebra::Scalar + From<f32> + Into<f32>>(&self, vec: ::Point<T>) -> bool {
        let sizes = self.get_sizes();
        let vec: Vector2<f32> = Vector2::new(vec.x.into(), vec.y.into());

        let a = Rect::new(self.pos.x as f32,
                  self.pos.y as f32,
                  sizes.x as f32,
                  sizes.y as f32);
        println!("{:?}", a);
        a.contain(vec)
    }

    fn translate<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector2<T>) {
        self.pos.x += vec.x.into();
        self.pos.y += vec.y.into();
        self.need_update = true;
    }

    fn set_scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector2<T>) {
        self.scale.x = vec.x.into();
        self.scale.y = vec.y.into();
        self.need_update = true;
    }

    fn get_scale(&self) -> Vector2<f32> {
        self.scale
    }

    fn scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, factor: Vector2<T>) {
        self.scale.x += factor.x.into();
        self.scale.y += factor.y.into();
        self.need_update = true;
    }

    fn get_position(&self) -> Vector2<f32> {
        self.pos
    }

    fn set_position<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector2<T>) {
        self.pos.x = vec.x.into();
        self.pos.y = vec.y.into();
        self.need_update = true;
    }

    fn set_origin<T: nalgebra::Scalar + Into<f32>>(&mut self, origin: Vector2<T>) {
        self.origin.x = origin.x.into();
        self.origin.y = origin.y.into();
        self.need_update = true;
    }

    fn get_origin(&self) -> Vector2<f32> {
        self.origin
    }

    fn rotate<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T) {
        self.rotation += angle.into();
        self.need_update = true;
    }

    fn set_rotation<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T) {
        self.rotation = angle.into();
        self.need_update = true;
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
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
                    &*DEFAULT_SHADER,
                    Some(Matrix4::<f32>::identity() * self.model),
                    BlendMode::Alpha
        ));
    }

    fn draw_with_context<'a, T: Drawer>
    (&self, window: &mut T, context: &'a mut Context) {
        self.vertice.draw_with_context(window, context);
    }

    fn update(&mut self) {
        if self.need_update {
            //translate to position
            self.model = Matrix4::<f32>::identity().append_translation(
                &Vector3::new(
                    self.pos.x - self.origin.x,
                    self.pos.y - self.origin.y,
                    0.0,
                )
            );

            if self.origin.x != 0.0 && self.origin.y != 0.0 {
                self.model.append_translation_mut(
                    &Vector3::new(
                        self.origin.x,
                        self.origin.y,
                        0.0
                    )
                );
                self.model *= Matrix4::from_euler_angles(
                        0.0, 0.0, self.rotation * (3.14116 * 180.0)
                );
                self.model.prepend_translation_mut(
                    &Vector3::new(
                        -self.origin.x,
                        -self.origin.y,
                        0.0
                    )
                );
            } else {
                self.model *= Matrix4::from_euler_angles(
                    0.0, 0.0, self.rotation * (3.14116 * 180.0)
                );
            }

            self.model.append_nonuniform_scaling_mut(
                &Vector3::new(self.scale.x, self.scale.y, 0.0)    
            );
            self.need_update = false;
        }
    }

    fn set_texture(&mut self, texture: &Rc<Texture>) {
        self.texture = Some(Rc::clone(texture))
    }
}
