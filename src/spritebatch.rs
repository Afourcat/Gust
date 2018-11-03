//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  spritebatch.rs
//  module:
//! spritebatch test system

use texture::Texture;
use rect::Rect;
use draw::{Drawer, Drawable, Context, BlendMode};
use shader::BATCH_SHADER;
use super::Vector;
use vertex::Vertex;
use nalgebra::{Matrix4, Vector3};
use gl;
use gl::types::*;
use std::mem;
use std::ptr;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use color::Color;
use nalgebra::Vector4;

#[derive(Debug, Clone)]
/// SpriteData is a structure representing transformation on a texture to become a sprite.
pub struct SpriteData {
    pos: Vector<f32>,
    rotation: f32,
    model: Matrix4<f32>,
    need_update: bool,
}

impl SpriteData {
    pub fn new(pos: Vector<f32>) -> Self {
        SpriteData {
            pos,
            ..Self::default()
        }
    }
}

impl Default for SpriteData {
    fn default() -> SpriteData {
        SpriteData {
            pos: Vector::new(0.0, 0.0),
            rotation: 0.0,
            model: Matrix4::identity(),
            need_update: true,
        }
    }
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
#[derive(Clone, Debug)]
pub struct SpriteBatch {
    texture: Option<Arc<Texture>>,
    sprites: Vec<SpriteData>,
    vertice: Vec<Vertex>,
    textures: Vec<Rect<f32>>,
    gl_objects: (u32, u32),
    origin: Vector<f32>,
    len: usize
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

    fn update_vbo(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_objects.1);

            if self.len != self.vertice.len() / 4{
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (std::mem::size_of::<GLfloat>() * self.vertice.len() * 8) as GLsizeiptr,
                    self.vertice.as_ptr() as *const GLvoid,
                    gl::STATIC_DRAW
                );
                self.len = self.vertice.len() / 4;
            }
            gl::BufferSubData(
				gl::ARRAY_BUFFER,
                0,
				(std::mem::size_of::<GLfloat>() * self.vertice.len() * 8) as GLsizeiptr,
				self.vertice.as_ptr() as *const GLvoid,
			);
            self.update_vao();
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn update_vao(&mut self) {
        unsafe {
            gl::BindVertexArray(self.gl_objects.0);
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

            gl::BindVertexArray(0);
        }
    }

    fn fill_vbo(&mut self) {
        unsafe {
            gl::BindVertexArray(self.gl_objects.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_objects.1);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<GLfloat>() * self.vertice.len() * 8) as GLsizeiptr,
                self.vertice.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );
            self.update_vao();

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn add_sprites(&mut self, mut sprites: SpriteData) {
        sprites.need_update = true;
        self.sprites.push(sprites);
        let (w , h) = if let Some(ref texture) = self.texture {
            (texture.width() as f32, texture.height() as f32)
        } else {
            (0.0, 0.0)
        };

        self.vertice.extend_from_slice(&[
            Vertex::new(Vector::new(0.0, 0.0), Vector::new(0.0, 0.0), Color::white()),
            Vertex::new(Vector::new(0.0,   h), Vector::new(0.0, 1.0), Color::white()),
            Vertex::new(Vector::new(w,   0.0), Vector::new(1.0, 0.0), Color::white()),
            Vertex::new(Vector::new(w,     h), Vector::new(1.0, 1.0), Color::white()),
        ]);
    }
}

impl Drawable for SpriteBatch {
    fn draw<T: Drawer>(&self, target: &mut T) {
        let texture = if let Some(ref rc_texture) = self.texture {
            Some(rc_texture.as_ref())
        } else {
            None
        };

        let mut context = Context::new(
            texture,
            &*BATCH_SHADER,
            vec![
                ("projection".to_string(), target.projection())
            ],
            BlendMode::Alpha
        );

        self.setup_draw(&mut context);
        unsafe {
            gl::BindVertexArray(self.gl_objects.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_objects.1);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.vertice.len() as i32);

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
        use std::sync::mpsc;
        let (rec, sen) = mpsc::channel();
        let rex = Mutex::new(rec);
        {
            let sprites = &mut self.sprites;
            let vertices = Mutex::new(&mut self.vertice);

            
            // Iterate over each sprites and update it if it need it.
            sprites
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, mut elem)| {
                    if elem.need_update {
                        let vert = &mut vertices.lock().unwrap()[(i * 4)..(i * 4 + 4)];
                        self::update_sprite(&mut elem, Vector::new(0.0, 0.0), vert);
                        rex.lock().unwrap().send(true).unwrap();
                    }
                });
        }
        for _i in sen {
            println!("Update");
            self.update_vbo();
        }
    }
}

fn update_sprite(data: &mut SpriteData, origin: Vector<f32>, vertice: &mut [Vertex]) {
    data.model = Matrix4::identity().append_translation(
        &Vector3::new(data.pos.x - origin.x, data.pos.y - origin.y, 0.0)
    );

    data.model *= Matrix4::from_euler_angles(
        0.0, 0.0, data.rotation * (3.14116 * 180.0)
    );

    for vertex in vertice {
        let b = data.model * Vector4::new(vertex.pos.x, vertex.pos.y, 0.0, 1.0);
        vertex.pos = Vector::new(b.x, b.y);
    }

    if data.rotation > 360.0 {
        data.rotation = 0.0;
    }

    data.need_update = false;
}

impl Default for SpriteBatch {
    fn default() -> Self {
        SpriteBatch {
            texture: None,
            sprites: Vec::new(),
            vertice: Vec::new(),
            gl_objects: Self::create_vbo(),
            textures: Vec::new(),
            origin: Vector::new(0.0, 0.0),
            len: 0
        }
    }
}

impl From<&Arc<Texture>> for SpriteBatch {
    fn from(what: &Arc<Texture>) -> SpriteBatch {
        SpriteBatch {
            texture: Some(Arc::clone(what)),
            sprites: Vec::new(),
            vertice: Vec::new(),
            gl_objects: Self::create_vbo(),
            textures: Vec::new(),
            origin: Vector::new(0.0, 0.0),
            len: 0
        }
    }
}
