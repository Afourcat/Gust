//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  text.rs
//  module:
//! text render utils
//! # How it works
//! When you create a font you create a face from freetype
//! This face is stocked inside the Font struct.
//! When you trigger glyph from Text component. The font look if the glyph from
//! this size and this letter exist. Else it loadIt from the Face and Add it to
//! a globaltexture linked to the font size.
//! This way it load only a rect from the same texture when you draw text.
//! # Example
//! ```no_run
//! use text::{Font,Text};
//! let font = Rc::new(Font::new("examples/fonts/Monaco.ttf"));
//! let text = Text::new(&font);
//! text.set_content("This is the text content");
//! ```
//! Text is drawable so you can use targer.draw(text);
//! after initialising it.
//!
use texture::Texture;
use font::{Font,CharInfo};
use draw::{Drawable,Drawer,Context,Movable,BlendMode};
use shader;
use ::{Point,Vector};
use nalgebra;
use std::rc::Rc;
use std::cell::RefCell;
use vertex_buffer::{VertexBuffer,Primitive};
use vertex::Vertex;
use color::Color;

extern crate freetype as ft;

#[derive(Debug)]
pub struct Text {
    font: Rc<RefCell<Font>>,
    content: String,
    actual_size: u32,
    vertex_buffer: VertexBuffer,
    need_update: bool
}

impl Text {

    pub fn dump_texture(&mut self) {
        // Get the texture                                                                          
        let font_ref = self.font.try_borrow().unwrap();                                             
        let texture = font_ref.texture(self.actual_size).unwrap(); 

        texture.to_file("test.png");
    }

	pub fn new(font: &Rc<RefCell<Font>>) -> Text {
        Text {
            font: Rc::clone(font),
            content: String::new(),
            actual_size: 14,
            vertex_buffer: VertexBuffer::default(),
            need_update: true
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn set_size(&mut self, size: u32) {
        self.actual_size = size;
    }

    pub fn size(&self) -> u32 {
        self.actual_size
    }

    pub fn add_to_buffer(&mut self, _char_info: CharInfo, _pos: Vector<u32>) {
        unimplemented!();
    }
}

impl Movable for Text {

    fn contain<T>(&self, _point: Point<T>) -> bool
    where
        T: nalgebra::Scalar + From<f32> + Into<f32> {
        true
    }

    fn translate<T>(&mut self, _offset: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32> {
        unimplemented!();
    }

    fn set_position<T>(&mut self, _pos: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32> {
        unimplemented!();
    }

    fn get_position(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }

    fn scale<T>(&mut self, _factor: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32> {
        unimplemented!();
    }

    fn set_scale<T>(&mut self, _vec: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32> {
        unimplemented!();
    }

    fn get_scale(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }

    fn rotate<T>(&mut self, _angle: T)
    where
        T: nalgebra::Scalar + Into<f32> {
        unimplemented!();
    }

    fn set_rotation<T>(&mut self, _angle: T)
    where
        T: nalgebra::Scalar + Into<f32> {
        unimplemented!();
    }

    fn get_rotation(&self) -> f32 {
        0.0
    }

    fn set_origin<T>(&mut self, _origin: Vector<T>)
    where
        T: nalgebra::Scalar + Into<f32> {
            unimplemented!();
    }

    fn get_origin(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }

}

impl Drawable for Text {
    fn update(&mut self) {
        

        if !self.need_update {
            return;
        }
        // For each elem create an entry
        for elem in self.content.as_str().chars() {
            let mut font_ref = self.font
                        .try_borrow_mut()
                        .unwrap();
            let elem = font_ref.glyph(self.actual_size, elem as u32);

            let padding = 0.0;

            let left   = elem.rect.left - padding;
            let top    = elem.rect.top - padding;
            let right  = elem.rect.left + elem.rect.width + padding;
            let bottom = elem.rect.top  + elem.rect.height + padding;
    
            let u1 = (elem.tex_coord.left - padding as u32) as f32;
            let v1 = (elem.tex_coord.top - padding as u32) as f32;
            let u2 = (elem.tex_coord.left + elem.tex_coord.width + padding as u32) as f32;
            let v2 = (elem.tex_coord.top  + elem.tex_coord.height + padding as u32) as f32;


            // Create vertex
            let vertices = vec![
                Vertex::new(Vector::new(left, top),     Vector::new(u1, v1), Color::white()),
                Vertex::new(Vector::new(right, top),    Vector::new(u2, v1), Color::white()),
                Vertex::new(Vector::new(left, bottom),  Vector::new(u1, v2), Color::white()),
                Vertex::new(Vector::new(left, bottom),  Vector::new(u1, v2), Color::white()),
                Vertex::new(Vector::new(right, top),    Vector::new(u2, v1), Color::white()),
                Vertex::new(Vector::new(right, bottom), Vector::new(u2, v2), Color::white()),
            ];
            self.vertex_buffer.append(vertices.as_slice());
        }
    }

    fn draw<T: Drawer>(&self, target: &mut T) {
    
        // If there is no text don't draw
        if self.content.is_empty() {
            return;
        }

        // Get the texture
        let font_ref = self.font.try_borrow().unwrap();
        let texture = font_ref.texture(self.actual_size).unwrap();

        let mut context = Context::new(
            Some(texture),
            &*shader::DEFAULT_SHADER,
            None,
            BlendMode::Alpha
        );

        self.draw_with_context(target, &mut context)
    }

    fn draw_with_context<T: Drawer> (&self, target: &mut T, context: &mut Context) {
        self.vertex_buffer.draw_with_context(target, context)
    }

    fn set_texture(&mut self, _texture: &Rc<Texture>) {
        unimplemented!();
    }
}
