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
use rect::Rect;
use texture::Texture;
use draw::{Drawable,Drawer,Context,Movable};
use ::{Point,Vector};
use nalgebra;
use std::{
    rc::Rc,
    path::Path,
    collections::HashMap,
};
use font::Font;

extern crate freetype as ft;

pub struct Text {
    font: Rc<Font>,
    content: String,
    actual_size: u32
}

impl Text {
    pub fn new(font: &Rc<Font>) -> Text {
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
}

impl Movable for Text {
    fn contain<T: nalgebra::Scalar + From<f32> + Into<f32>>(&self, point: Point<T>) -> bool {
        true
    }

    fn translate<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, offset: Vector<T>) {

    }

    fn set_position<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, pos: Vector<T>) {

    }

    fn get_position(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }

    fn scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, factor: Vector<T>) {

    }

    fn set_scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector<T>) {

    }

    fn get_scale(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }

    fn rotate<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T) { 

    }

    fn set_rotation<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T) {

    }

    fn get_rotation(&self) -> f32 {
        0.0
    }

    fn set_origin<T: nalgebra::Scalar + Into<f32>>(&mut self, origin: Vector<T>) {

    }

    fn get_origin(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }
}

impl Drawable for Text {
    fn update(&mut self) {

    }

    fn draw<T: Drawer>(&self, target: &mut T) {
    }

    fn draw_with_context<T: Drawer>(&self, target: &mut T, context: &mut Context) {
        unimplemented!();
    }

    fn set_texture(&mut self, texture: &Rc<Texture>) {
        unimplemented!();
    }
}
