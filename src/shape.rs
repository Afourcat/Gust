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

#[derive(Debug,PartialEq,PartialOrd,Eq,Default)]
/// An enum that contain all possible shape design in gust
pub enum Shapes {
    Circle(u32, AShape),
    Square(u32, AShape),
    Polygon(u32, u32, AShape),
    Rectangle(u32, u32, AShape)
}

#[derive(Debug,PartialEq,Eq,PartialOrd,Default)]
/// A struct globing all datas in shapes
struct AShape {
    pos: Vector<f32>,
    scale: Vector<f32>,
    angle: f32
}

impl Drawable for Shapes {
    fn draw<T: Drawer>(&self, window: &mut T) {
        match self {
            Shapes::Circle(r, shape) => {
                println!("Circle {} {:?}", r, shape)
            },
            Shapes::Square(d, shape) => {
                println!("Square {} {:?}", d, shape)
            },
            Shapes::Rectangle(w, h) => {
                println!("Rectangle {} {} {:?}", w, h, shape)
            },
            Shapes::Polygon(r, nbr) => {
                println!("Polygon {} {} {:?}", r, nbr, shape)
            },
        }
    }

    fn draw_with_context<T: Drawer>(&self, window: &mut T, context: &mut Context) {
        
    }
}
