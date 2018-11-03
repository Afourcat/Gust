//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  spritebatch.rs
//  module:
//! spritebatch test system

use std::rc::Rc;
use texture::Texture;
use rect::Rect;
use draw::{Drawer, Drawable, Context, BlendMode};
use shader::DEFAULT_SHADER;
use super::Vector;
use vertex::Vertex;
use nalgebra::{Matrix4, Vector3};
use gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::iter::{Extend, IntoIterator};

#[derive(Debug, Default, Clone)]
/// A Sprite data without vbo and texture
/// Represented only by it's transformation.
/// It's represented under the SAO representation to be efficient for cpu caching
pub struct SpritesData {
    pos: Vec<Vector<f32>>,
    rotation: Vec<f32>,
    vertices: Vec<[Vertex; 4]>,
    model: Vec<Matrix4<f32>>,
    need_update: Vec<bool>
}

#[derive(Debug, Clone)]
pub struct SpriteData {
    pos: Vector<f32>,
    rotation: f32,
    vertices: [Vertex; 4],
    model: Matrix4<f32>,
    need_update: bool

}

/// SpriteBatch is a datastructure that handle all sprites that have the same texture.
/// And make only 1 drawCall to draw them all. this way you can highly optimise data sended to
/// GPU.
/// You should use it in system where there is a lot's of sprite that should be drawn with the same
/// shaders and the same texture.
/// SpriteBatch is a kind of collection that implement some iterator traits.
/// ```no_run
///     let texture = Rc::new(Texture::from_path("path_to_texture"));
///     let mut batch = SpriteBatch::from(&texture);
///
///     SpriteData
/// ```
/// The idea behind SpriteBatch is to limit draw calls. Even if your sprites havn't the same texture
/// can pack textures. And give your Vertex text_coord the actual texture coordinate that you want to be drawn.
#[derive(Default, Clone, Debug)]
pub struct SpriteBatch {
    texture: Option<Rc<Texture>>,
    sprites: SpritesData,
    textures: Vec<Rect<f32>>,
    gl_objects: (u32, u32),
}

// For maximum efficiency we will not use the previously implemented abstraction of VertexBuffer
impl SpriteBatch {

    /// Create a new empty spriteBatch
    pub fn new() -> SpriteBatch {
        SpriteBatch::default()
    }

    fn create_vbo() -> (u32, u32) {
        let (mut vao, mut vbo) = (0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);

            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(0);
        }
        (vao, vbo)
    }

    fn fill_vbo(&self) {
        unsafe {
            gl::BindVertexArray(self.gl_objects.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_objects.1);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<GLfloat>() * self.sprites.need_update.len() * 8) as GLsizeiptr,
                self.sprites.vertices.as_ptr() as *const _ as *const std::ffi::c_void,
                gl::STATIC_DRAW
            );

            gl::BindVertexArray(self.gl_objects.1);

            // position (of each vertex)
            gl::VertexAttribPointer(
                            0,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            (8 * mem::size_of::<GLfloat>()) as GLsizei,
                            ptr::null()
            );
            gl::EnableVertexAttribArray(0);
            // texture coord (of each vertex)
            gl::VertexAttribPointer(
                            1,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            (8 * mem::size_of::<GLfloat>()) as GLsizei,
                            (2 * mem::size_of::<GLfloat>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);
            // color (of each vertex)
            gl::VertexAttribPointer(
                            2,
                            3,
                            gl::FLOAT,
                            gl::FALSE,
                            (8 * mem::size_of::<GLfloat>()) as GLsizei,
                            (4 * mem::size_of::<GLfloat>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn add_sprites(&mut self, sprites: SpritesData) {
        self.sprites.pos.extend_from_slice(&sprites.pos);
        self.sprites.rotation.extend_from_slice(&sprites.rotation);
        self.sprites.vertices.extend_from_slice(&sprites.vertices);
        self.sprites.model.extend_from_slice(&sprites.model);
        self.sprites.need_update.extend_from_slice(&sprites.need_update);
    }

    pub fn add_sprite_from(&mut self, sprites: &[SpriteData]) {
        for sprite in sprites {
            self.sprites.pos.push(sprite.pos);
            self.sprites.vertices.push(sprite.vertices);
            self.sprites.rotation.push(sprite.rotation);
            self.sprites.model.push(sprite.model);
            self.sprites.need_update.push(sprite.need_update);
        }
    }

    fn update_sprite(
        pos: &Vector<f32>,
        rotation: &mut f32,
        vertice: &mut [Vertex; 4],
        model: &mut Matrix4<f32>,
        origin: Vector<f32>,
    ) {
        *model = Matrix4::identity().append_translation(
            &Vector3::new(pos.x - origin.x, pos.y - origin.y, 0.0)
        );

        *model *= Matrix4::from_euler_angles(
            0.0, 0.0, *rotation * (3.14116 * 180.0)
        );

        for vertex in vertice {
            let a = *model * nalgebra::Vector4::new(vertex.pos.x, vertex.pos.y, 0.0, 0.0);
            vertex.pos = Vector::new(a.x, a.y);
        }

        if *rotation > 360.0 {
            *rotation = 0.0;
        }
    }
}

//impl IntoIterator for SpritesData {
//    type Item = SpriteData;
//    type IntoIter = Iterator;
//
//    fn into_iter(self) -> Self::IntoIter {
//    }
//}
//
//impl Extend<SpritesData> for SpriteBatch {
//    fn extend<T: IntoIterator<Item=SpriteData>>(&mut self, iter: T) {
//        for elem in iter {
//            self.sprites.pos.append(elem.pos);
//            self.sprites.need_update.append(elem.need_update);
//            self.sprites.rotation.append(elem.rotation);
//            self.sprites.vertices.append(elem.vertices);
//        }
//    }
//}

impl Drawable for SpriteBatch {
    fn draw<T: Drawer>(&self, target: &mut T) {
        let texture = if let Some(ref rc_texture) = self.texture {
            Some(rc_texture.as_ref())
        } else {
            None
        };

        let mut context = Context::new(
            texture,
            &*DEFAULT_SHADER,
            vec![
                ("projection".to_string(), &target.projection())
            ],
            BlendMode::Alpha
        );

        self.setup_draw(&mut context);
        unsafe {
            gl::BindVertexArray(self.gl_objects.1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_objects.0);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.sprites.need_update.len() as i32);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn draw_mut<T: Drawer>(&mut self, target: &mut T) {
        self.update();
        self.draw(target);
    }

    fn draw_with_context(&self, context: &mut Context) {
        unimplemented!("?");
    }

    fn update(&mut self) {
        for i in 0..self.sprites.need_update.len() {
            if self.sprites.need_update[i] {
                Self::update_sprite(
                    &self.sprites.pos[i],
                    &mut self.sprites.rotation[i],
                    &mut self.sprites.vertices[i],
                    &mut self.sprites.model[i],
                    Vector::new(0.0, 0.0),
                );
            }
        }
    }
}

impl From<&Rc<Texture>> for SpriteBatch {
    fn from(what: &Rc<Texture>) -> SpriteBatch {
        SpriteBatch {
            texture: Some(Rc::clone(what)),
            sprites: SpritesData::default(),
            gl_objects: Self::create_vbo(),
            textures: Vec::new()
        }
    }
}
