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
use draw::{Drawable,Drawer,Context,Movable};
use ::{Point,Vector};
use nalgebra;
use std::rc::Rc;
use font::Font;
use std::cell::RefCell;

extern crate freetype as ft;

#[derive(Debug)]
pub struct Text {
    font: Rc<RefCell<Font>>,
    content: String,
    actual_size: u32
}

impl Text {
    pub fn new(font: &Rc<RefCell<Font>>) -> Text {
        Text {
            font: Rc::clone(font),
            content: String::new(),
            actual_size: 14
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

    pub fn add_to_buffer(&mut self, char_info: CharInfo, pos: Vector<u32>) {
        unimplemented!();
    }

    pub fn draw_test<T: Drawer>(&mut self, _target: &T) {
        for elem in self.content.as_str().chars() {
            self.font
                .try_borrow_mut()
                .unwrap()
                .glyph(self.actual_size, elem as u32);
        }
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

    }

    fn draw<T: Drawer>(&self, _target: &mut T) {
    }

    fn draw_with_context<T: Drawer>
    (&self, _target: &mut T, _context: &mut Context) {
        unimplemented!();
    }

    fn set_texture(&mut self, _texture: &Rc<Texture>) {
        unimplemented!();
    }
}
