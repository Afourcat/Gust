//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  rect.rs
//  module:
//! Rectangle struct and all fonction

use nalgebra::Scalar;
use std::ops::{Add, Div, Mul, MulAssign, Sub};

/// Rect define a rectangle with down/left coord and width/height
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect<T: Scalar> {
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}

impl<T: Scalar + Add<Output = T>> Add<Rect<T>> for Rect<T> {
    type Output = Rect<T>;

    fn add(self, rhs: Rect<T>) -> Self::Output {
        Rect {
            top: self.top + rhs.top,
            left: self.left + rhs.left,
            height: self.width + rhs.width,
            width: self.height + rhs.height,
        }
    }
}

impl<T: Scalar + Div<Output = T>> Div<Rect<T>> for Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: Rect<T>) -> Self::Output {
        Rect {
            top: self.top / rhs.top,
            left: self.left / rhs.left,
            height: self.width / rhs.width,
            width: self.height / rhs.height,
        }
    }
}

impl<T: Scalar + Sub<Output = T>> Sub<Rect<T>> for Rect<T> {
    type Output = Rect<T>;

    fn sub(self, rhs: Rect<T>) -> Self::Output {
        Rect {
            top: self.top - rhs.top,
            left: self.left - rhs.left,
            height: self.width - rhs.width,
            width: self.height - rhs.height,
        }
    }
}

impl<T: Scalar + Mul<Output = T>> Mul<Rect<T>> for Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: Rect<T>) -> Self::Output {
        Rect {
            top: self.top * rhs.top,
            left: self.left * rhs.left,
            height: self.width * rhs.width,
            width: self.height * rhs.height,
        }
    }
}

impl<T: Scalar + MulAssign<T>> MulAssign<Rect<T>> for Rect<T> {
    fn mul_assign(&mut self, rhs: Rect<T>) {
        self.left *= rhs.left;
        self.top *= rhs.top;
        self.width *= rhs.width;
        self.height *= rhs.height;
    }
}

//------------------
//
//              Rect impl
//
//------------------

impl<T> Rect<T>
where
    T: Scalar + Add<Output = T> + PartialOrd,
{
    pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
        Rect {
            top,
            left,
            width,
            height,
        }
    }

    pub fn contain(&self, point: super::Point<T>) -> bool {
        point.x > self.left
            && point.x < self.left + self.width
            && point.y > self.top
            && point.y < self.top + self.height
    }
}

impl From<Rect<u32>> for Rect<f32> {
    fn from(this: Rect<u32>) -> Rect<f32> {
        Rect {
            top: this.top as f32,
            left: this.left as f32,
            width: this.width as f32,
            height: this.height as f32,
        }
    }
}

impl<T> Default for Rect<T>
where
    T: Default + Scalar,
{
    fn default() -> Rect<T> {
        Rect {
            top: T::default(),
            left: T::default(),
            width: T::default(),
            height: T::default(),
        }
    }
}
