//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  rect.rs
//  module:
//! Rectangle struct and all fonction

use nalgebra;
use std::ops::{Add,Mul,Div,Sub,MulAssign};

/// Rect define a rectangle with down/left coord and width/height
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Rect<T: nalgebra::Scalar> {
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}

impl<T: nalgebra::Scalar + Add<Output=T>> Add<Rect<T>> for Rect<T> {
    type Output = Rect<T>;

    fn add(self, rhs: Rect<T>) ->  Self::Output {
        Rect {
            top: self.top + rhs.top,
            left:   self.left + rhs.left,
            height: self.width + rhs.width,
            width:  self.height + rhs.height
        }
    }
}

impl<T: nalgebra::Scalar + Div<Output=T>> Div<Rect<T>> for Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: Rect<T>) -> Self::Output {
        Rect {
            top: self.top / rhs.top,
            left:   self.left / rhs.left,
            height: self.width / rhs.width,
            width:  self.height / rhs.height
        }
    }
}

impl<T: nalgebra::Scalar + Sub<Output=T>> Sub<Rect<T>> for Rect<T> {
    type Output = Rect<T>;

    fn sub(self, rhs: Rect<T>) -> Self::Output {
        Rect {
            top: self.top - rhs.top,
            left:   self.left - rhs.left,
            height: self.width - rhs.width,
            width:  self.height - rhs.height
        }
    }
}

impl<T: nalgebra::Scalar + Mul<Output=T>> Mul<Rect<T>> for Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: Rect<T>) -> Self::Output {
        Rect {
            top: self.top * rhs.top,
            left:   self.left * rhs.left,
            height: self.width * rhs.width,
            width:  self.height * rhs.height
        }
    }
}

impl<T: nalgebra::Scalar + MulAssign<T>> MulAssign<Rect<T>> for Rect<T> {
    fn mul_assign(&mut self, rhs: Rect<T>) {
        self.left *= rhs.left;
        self.top *= rhs.top;
        self.width *= rhs.width;
        self.height *= rhs.height;
    }
}

impl<T: nalgebra::Scalar> Rect<T> {
    pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
        Rect {
            top: top,
            left: left,
            width: width,
            height: height,
        }
    }
}

impl Into<Rect<f32>> for Rect<usize> {
    fn into(self) -> Rect<f32> {
        Rect {
            top: self.top as f32,
            left: self.left as f32,
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}


