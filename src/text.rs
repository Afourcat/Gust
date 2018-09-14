//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  text.rs
//  module:
//! text render utils

use texture::Texture;
use draw::{Drawable,Drawer,Context,Movable};
use ::{Point,Vector};
use nalgebra;
use std::{
    sync::Mutex,
    rc::Rc,
};


struct Font {
    jsp: i32
}

struct Text {
    font: Font,
    content: String,
    size: f32,
}

impl Text {
    pub fn new() {

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

fn init_ft() {
    lazy_static! {
        static ref init: Mutex<bool> = Mutex::new(false);
    }

    let mut locked = init.lock().unwrap();

    if *locked == false {
        // init freetype
        *locked = true;
    }
}

