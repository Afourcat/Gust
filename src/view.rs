//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  view.rs
//  module:
//! View system.
//! A view is a point of view on a 2D scene. It's like a camera filming paper.
//! A view should be defined by sizes and left and down.
//! ```
//! ...
//!
//! let view = View::new(1920, 1080, 0, 0);
//!
//! ```

use nalgebra;
use nalgebra::{Matrix4,Vector3};
use ::{Point,Vector};

/// Rect define a rectangle with down/left coord and width/height
#[derive(Debug,Clone,PartialEq)]
pub struct Rect<T: nalgebra::Scalar> {
    bottom: T,
    left: T,
    width: T,
    height: T,
}

impl<T: nalgebra::Scalar> Rect<T> {
    pub fn new(left: T, bottom: T, width: T, height: T) -> Rect<T> {
        Rect {
            bottom: bottom,
            left: left,
            width: width,
            height: height,
        }
    }
}

/// A View is a 2D camera
#[derive(Debug,Clone,PartialEq)]
pub struct View {
    projection: Matrix4<f32>,
    rect: Rect<usize>,
    center: Point<usize>,
}

impl View {

    /// Create a new View from a center point and a Rect
    pub fn new(center: Point<usize>, rect: Rect<usize>) -> View {
        View {
            projection: Matrix4::new_orthographic(
                rect.left as f32,
                rect.width as f32,
                rect.bottom as f32,
                rect.height as f32,
                -1.0, 1.0
            ),
            rect: rect,
            center: center,
        }
    }

    /// Reset the rect if you don't want to you can use (set_sizes)[]
    pub fn reset(&mut self, rect: Rect<usize>) {
        if self.rect == rect {
            println!("Rect: reset with the same Rect<usize>. {:?}", rect);
            return;
        }
        self.projection = Matrix4::new_orthographic(
                rect.left as f32,
                rect.width as f32,
                rect.bottom as f32,
                rect.height as f32,
                -1.0, 1.0
        );
        self.rect = rect;
    }

    /// Set center of the view (usefull for game like 2D Zelda-Like)
    pub fn set_center(&mut self, center: Point<usize>) {
        self.center = center;
    }

    /// Set the size of the rect
    pub fn set_sizes(&mut self, sizes: Vector<usize>) {
        self.rect.width = sizes.x;
        self.rect.height = sizes.y;
    }

    pub fn get_projection(&self) -> &Matrix4<f32> {
        &self.projection
    }

    pub fn translate(&self, offset: Vector<usize>) {
        &self.projection.append_translation(
            &Vector3::new(offset.x as f32, offset.y as f32, 0.0)
        );
    }
}

impl From<Rect<usize>> for View {

    fn from(rect: Rect<usize>) -> View {
        let mut proj = Matrix4::new_orthographic(
                rect.left as f32,
                rect.width as f32,
                rect.bottom as f32,
                rect.height as f32,
                -1.0, 1.0
        );

        // FUCKING NALGEBRA
        proj[5] *= -1.0;
        proj[13] = 0.0;
        View {
            center: Vector::new(rect.width / 2, rect.height / 2),
            projection: proj,
            rect: rect,
        }
    }
}
