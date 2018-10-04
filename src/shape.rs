//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  shape.rs
//  module:
//! shape

use draw::{Movable,Drawable};
use color::Color;
use super::Vector;

/// A Shape is a drawable and transformable object
pub trait Shape: draw::Drawable, draw::Movable {
    fn color(&self) -> &Color;

    fn color_mut(&mut self) -> &mut Color;

    fn set_color(&mut self, color: Color) -> Color {
        self.color_mut() = color;
    }
}

pub struct Circle {
    rayon: u32,
    color: Color,
    pos: Vector<f32>,
    scale: Vector<f32>,
    angle: f32,
}

impl Drawable for Circle {
    fn draw<T: Drawer>(&self, window: &mut T) {

    }

    fn draw_with_context<T: Drawer>(&self, window: &mut T, context: &mut Context)
}
