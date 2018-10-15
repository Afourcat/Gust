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
//! The text is made from Text.cpp of SFML

use texture::Texture;
use font::{Font,CharInfo};
use draw::{Drawable,Drawer,Context,Movable,BlendMode};
use shader;
use ::{Point,Vector};
use nalgebra;
use std::{error::Error, rc::Rc};
use std::cell::RefCell;
use vertex_buffer::VertexBuffer;
use vertex::Vertex;
use color::Color;

extern crate freetype as ft;

#[derive(Debug)]
pub struct Text {
    font: Rc<RefCell<Font>>,
    content: String,
    actual_size: u32,
    vertex_buffer: VertexBuffer,
    need_update: bool,
    pos: Vector<f32>
}

impl Text {

    pub fn dump_texture(&mut self) -> Result<(),Box<Error>>{
        // Get the texture
        let font_ref = self.font.try_borrow().unwrap();
        let texture = font_ref.texture(self.actual_size).unwrap();

        texture.to_file("test.png")?;
        Ok(())
    }

	pub fn new(font: &Rc<RefCell<Font>>) -> Text {
        Text {
            font: Rc::clone(font),
            content: String::new(),
            actual_size: 14,
            vertex_buffer: VertexBuffer::default(),
            need_update: true,
            pos: Vector::new(0.0, 0.0)
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut String {
        &mut self.content
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

    fn set_position<T>(&mut self, pos: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32> {
        self.pos.x = pos.x.into();
        self.pos.y = pos.y.into();
        self.need_update = true;
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
        // Si l'update n'est pas necessaire
        if !self.need_update { return; }

        // Relative position
        let mut pos = self.pos;

        // Get reference to the font that is a reference counter
        let mut font_ref = self.font
                        .try_borrow_mut()
                        .unwrap();

        // Get the whitespace x size
        let whitespace = font_ref.glyph(self.actual_size, 0x20_u32).advance;

        // Setup padding
        let padding = 0.0;

        // Clear the buffer of the data
        self.vertex_buffer.clear();

        // Iter of character of the content to create a geometry for each one of them
        for charr in self.content.as_str().chars() {

            // If the char is a special one
            if charr == '\n' {
                pos.y += 20.0;
                continue;
            } else if charr == '\r' {
                continue;
            } else if charr == '\t' {
                pos.x += 4.0 * whitespace;
                continue;
            } else if charr == ' ' {
                pos.x += whitespace;
                continue;
            }

            // Get the glyph from the the font
            let char_info = font_ref.glyph(self.actual_size, charr as u32);

            // get vertices from char_info
            let vertices = get_vertice_letter(&char_info, &pos, padding);

            // append vertice to vertex_buffer
            self.vertex_buffer.append(&vertices);

            // x position of the character
            pos.x += char_info.advance as f32;
        }
        // Update final buffer
        self.vertex_buffer.update();

        // Set to false the boolean that contral this function
        self.need_update = false;
    }

    fn draw<T: Drawer>(&self, target: &mut T) {
        // If there is no text don't draw
        if self.content.is_empty() { return }

        // Get the texture
        let font_ref = self.font.try_borrow().unwrap();
        let texture = font_ref.texture(self.actual_size).unwrap();

        // Create a new context with the Texture of the font
        let mut context = Context::new(
            Some(texture),
            &*shader::DEFAULT_SHADER,
            None,
            BlendMode::Alpha
        );

        // Draw the vertex_buffer with context
        self.draw_with_context(target, &mut context)
    }

    fn draw_with_context<T: Drawer> (&self, target: &mut T, context: &mut Context) {
        self.vertex_buffer.draw_with_context(target, context)
    }

    fn set_texture(&mut self, _texture: &Rc<Texture>) {
        unimplemented!();
    }
}

fn get_vertice_letter(char_info: &CharInfo, pos: &Vector<f32>, padding: f32) -> [Vertex; 6] {
    let x = pos.x;
    let y = pos.y;

    // Set geometry for 1 character
    let left   = char_info.rect.left - padding;
    let top    = char_info.rect.top - padding;
    let right  = char_info.rect.left + char_info.rect.width + padding;
    let bottom = char_info.rect.top  + char_info.rect.height + padding;

    // Set texture coord for each character
    let u1 = ((char_info.tex_coord.left - padding as u32) as f32) / 128.0;
    let v1 = ((char_info.tex_coord.top - padding as u32) as f32) / 128.0;
    let u2 = ((char_info.tex_coord.left + char_info.tex_coord.width + padding as u32) as f32) / 128.0;
    let v2 = ((char_info.tex_coord.top  + char_info.tex_coord.height + padding as u32) as f32) / 128.0;

    [
        Vertex::new(Vector::new(x + left,   y + top),       Vector::new(u1, v1), Color::white()),
        Vertex::new(Vector::new(x + left,   y + bottom),    Vector::new(u1, v2), Color::white()),
        Vertex::new(Vector::new(x + right,  y + bottom),    Vector::new(u2, v2), Color::white()),
        Vertex::new(Vector::new(x + left,   y + top),       Vector::new(u1, v1), Color::white()),
        Vertex::new(Vector::new(x + right,   y + bottom),    Vector::new(u2, v2), Color::white()),
        Vertex::new(Vector::new(x + right,  y + top),       Vector::new(u2, v1), Color::white()),
    ]
}
