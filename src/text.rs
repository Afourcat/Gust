//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  text.rs
//  module:
//! text render utils

use self::Vector;
use texture::Texture;
use draw::{Drawable,Drawer,Context,Movable};
use ::{Point,Vector};
use nalgebra;
use std::{
    rc::Rc,
    path::Path,
};

extern crate freetype as ft;

use self::ft::{
    library::Library,
    face::Face,
    FtResult
};


pub struct FontHandler(Library);

impl FontHandler {
    pub fn new() -> FontHandler {
        FontHandler(Library::init().unwrap())
    }

    pub fn create(&self, path: String) -> Font {
        let face = self.0.new_face(path).unwrap();
        Font::from();
    }
}

struct GraphicChar {
    texture: Texture,
    sizes: Vector<i32>,
    bearing: Vector<i32>,
    advance: u32
}

pub struct Font {
    font: Hashmap<char,GraphicChar>
}

impl From<Face> for FontHandler {
    
}

pub struct Text {
    font: Rc<Font>,
    content: String,
}

impl Text {
    pub fn new(font: &Rc<Font>) -> Text {
        Text {
            font: Rc::clone(font),
            content: String::new(),
        }
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
        unimplemented!();
    }

    fn draw_with_context<T: Drawer>(&self, target: &mut T, context: &mut Context) {
        unimplemented!();
    }

    fn set_texture(&mut self, texture: &Rc<Texture>) {
        unimplemented!();
    }
}
