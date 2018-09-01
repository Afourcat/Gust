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
use rect::Rect;

/// A View is a 2D camera
#[derive(Debug,Clone,PartialEq)]
pub struct View {
    projection: Matrix4<f32>,
    rect: Rect<usize>,
    center: Point<usize>,
    need_update: bool,
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
            need_update: false,
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

    pub fn set_viewport(&mut self, viewport: Rect<f32>) {
        self.rect *= viewport;
    }

    /// Set the size of the rect
    pub fn set_sizes(&mut self, sizes: Vector<usize>) {
        self.rect.width = sizes.x;
        self.rect.height = sizes.y;
    }

    pub fn get_projection(&self) -> &Matrix4<f32> {
        &self.projection
    }

    pub fn translate<T: nalgebra::Scalar + Into<usize>>(&mut self, offset: Vector<T>) {
        self.center.x += offset.x.into();
        self.center.y += offset.y.into();
    }

    pub fn update(&mut self) {
        if self.need_update {
            self.projection = Matrix4::new_orthographic(
                (self.rect.left as f32) - (self.center.x as f32),
                (self.rect.width as f32) - (self.center.y as f32),
                self.rect.bottom as f32,
                self.rect.height as f32,
                -1.0, 1.0
            );
            apply_proj_correction(&mut self.projection);
        }
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
        apply_proj_correction(&mut proj);
        View {
            center: Vector::new(rect.width / 2, rect.height / 2),
            projection: proj,
            rect: rect,
            need_update: false,
        }
    }
}

fn apply_proj_correction(proj: &mut Matrix4<f32>) {
    proj[5] *= -1.0;
    proj[13] = 0.0;
}
