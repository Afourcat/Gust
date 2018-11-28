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
use nalgebra::Matrix4;
use ::{Point,Vector};
use rect::Rect;

/// A View is a 2D camera
#[derive(Debug,Clone,PartialEq)]
pub struct View {
    projection: Matrix4<f32>,
    sizes: Vector<f32>,
    pos: Vector<f32>,
    screen: Rect<f32>,
    angle: f32,
    zoom: f32,
    need_update: bool,
}

impl View {

    /// Create a new View from a pos point and a Rect
    pub fn new(pos: Point<f32>, rect: Rect<f32>) -> View {
        View {
            projection: Matrix4::new_ortho(
                rect.left as f32,
                rect.width as f32,
                rect.top as f32,
                rect.height as f32,
                -1.0, 1.0
            ),
            zoom: 1.0,
            angle: 0.0,
            sizes: Vector::new(rect.width, rect.height),
            pos,
            screen: Rect::new(1.0, 1.0, 1.0, 1.0),
            need_update: false,
        }
    }

    /// Reset the rect if you don't want to you can use (set_sizes)[]
    pub fn reset(&mut self, rect: Rect<f32>) {
        self.projection = Matrix4::new_orthographic(
                rect.left as f32,
                rect.width as f32,
                rect.top as f32,
                rect.height as f32,
                -1.0, 1.0
        );
        self.sizes = Vector::new(rect.width, rect.height);
        self.need_update = true;
    }

    /// Set pos of the view (usefull for game like 2D Zelda-Like)
    pub fn set_center(&mut self, pos: Point<f32>) {
        self.pos.x = pos.x - self.sizes.x / 2.0;
        self.pos.y = pos.y - self.sizes.y / 2.0;
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
        self.pos.x += offset.x.into();
        self.pos.y += offset.y.into();
        self.need_update = true;
    }

    pub fn update(&mut self) {
        if self.need_update {
            let width = self.sizes.x * self.zoom;
            let height = self.sizes.y * self.zoom;

            self.projection = Matrix4::new_ortho(
                self.pos.x,
                width + self.pos.x,
                height + self.pos.y,
                self.pos.y,
                -1.0, 1.0
            );
            self.need_update = false;
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        if zoom != self.zoom {
            self.zoom = zoom;
            self.need_update = true;
        }
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.zoom *= zoom;
        self.need_update = true;
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn projection(&self) -> Matrix4<f32> {
        self.projection
    }

    pub fn sizes(&self) -> Vector<f32> {
        self.sizes
    }

    pub fn postition(&self) -> Vector<f32> {
        self.pos
    }
}

impl From<Rect<f32>> for View {

    fn from(rect: Rect<f32>) -> View {
        let proj = Matrix4::new_ortho(
            rect.left,
            rect.width + rect.left,
            rect.height + rect.top,
            rect.top,
            -1.0, 1.0);
        View {
            pos: Vector::new(rect.left, rect.top),
            projection: proj,
            sizes: Vector::new(rect.width, rect.height),
            need_update: false,
            zoom: 1.0,
            angle: 0.0,
            screen: Rect::new(0.0, 0.0, 1.0, 1.0),
        }
    }
}

impl Ortho for Matrix4<f32> {}

trait Ortho {
    fn new_ortho(left: f32, right: f32, bottom:f32, top: f32, near: f32, far: f32) -> Matrix4<f32> {
        let width = right - left;
        let height = top - bottom;
        let a = 2.0 / width;
        let b = 2.0 / height;
        let c = -2.0 / (far - near);
        let tx = -(right + left) / width;
        let ty = -(top + bottom) / height;
        let tz = -(far + near) / far - near;

        Matrix4::new(
            a   , 0.0, 0.0,  tx,
            0.0 ,   b, 0.0,  ty,
            0.0 , 0.0,   c,  tz,
            0.0 , 0.0, 0.0, 1.0
        )
    }
}
