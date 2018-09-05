//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  view.rs
//  module:
//! View system.
//! A view is a point of view on a 2D scene. It's like a camera filming paper.
//! A view should be defined by sizes and left and down.
//! ```no_run
//! use gust::view::View;
//! use gust::window::Window;
//! use gust::rect::Rect;
//!
//! let window = Window::new(1920, 1080, "Example View");
//! let view = View::from(Rect::new(500.0, 500.0, 10.0, 10.0));
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
    sizes: Vector<f32>,
    center: Point<f32>,
    screen: Rect<f32>,
    zoom: f32,
    need_update: bool,
}

impl View {

    /// Create a new View from a center point and a Rect
    pub fn new(center: Point<f32>, rect: Rect<f32>) -> View {
        View {
            projection: Matrix4::new_orthographic(
                rect.left as f32,
                rect.width as f32,
                rect.bottom as f32,
                rect.height as f32,
                -1.0, 1.0
            ),
            zoom: 1.0,
            sizes: Vector::new(rect.width, rect.height),
            center: center,
            screen: Rect::new(1.0, 1.0, 1.0, 1.0),
            need_update: false,
        }
    }

    /// Reset the rect if you don't want to you can use (set_sizes)[]
    pub fn reset(&mut self, rect: Rect<f32>) {
        self.projection = Matrix4::new_orthographic(
                rect.left as f32,
                rect.width as f32,
                rect.bottom as f32,
                rect.height as f32,
                -1.0, 1.0
        );
        self.sizes = Vector::new(rect.width, rect.height);
        self.need_update = true;
    }

    /// Set center of the view (usefull for game like 2D Zelda-Like)
    pub fn set_center(&mut self, center: Point<f32>) {
        self.center = center;
        self.need_update = true;
    }

    /// Set the viewport of the view (the viewport is given as a float factor 0.5 / 1.0 / 0.2 etc)
    /// That way people can simply handle screen part.
    pub fn set_viewport(&mut self, viewport: Rect<f32>) {
        self.screen = viewport;
    }

    /// Set the size of the rect
    pub fn set_sizes(&mut self, sizes: Vector<f32>) {
        self.sizes.x = sizes.x;
        self.sizes.y = sizes.y;
    }

    pub fn get_projection(&self) -> &Matrix4<f32> {
        &self.projection
    }

    pub fn translate<T: nalgebra::Scalar + Into<f32>>(&mut self, offset: Vector<T>) {
        self.center.x += offset.x.into();
        self.center.y += offset.y.into();
        self.need_update = true;
    }

    pub fn update(&mut self) {
        if self.need_update {
            println!("{:?} {:?}", self.center, self.sizes);
            self.projection = Matrix4::new_orthographic(
                self.center.x - self.sizes.x / 2.0,
                self.sizes.x,
                self.center.y - self.sizes.y / 2.0,
                self.sizes.y,
                -1.0, 1.0
            );
            println!("{}", self.projection);
            apply_proj_correction(&mut self.projection);
            self.need_update = false;
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}

impl From<Rect<f32>> for View {

    fn from(rect: Rect<f32>) -> View {
        let mut proj = Matrix4::new_orthographic(
                rect.left as f32,
                rect.width as f32,
                rect.bottom as f32,
                rect.height as f32,
                -1.0, 1.0
        );

        // FUCKING NALGEBRA
        apply_proj_correction(&mut proj);
        println!("{}", proj);
        View {
            center: Vector::new(rect.width / 2.0, rect.height / 2.0),
            projection: proj,
            sizes: Vector::new(rect.width, rect.height),
            need_update: false,
            zoom: 1.0,
            screen: Rect::new(0.0, 0.0, 1.0, 1.0),
        }
    }
}

fn apply_proj_correction(proj: &mut Matrix4<f32>) {
    proj[5] *= -1.0;
    proj[13] *= -1.0;
    proj[2] *= -1.0;
}
