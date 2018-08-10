//! Drawer trait define a struct that can draw a drawable
//!

use nalgebra::{Vector2,Rotation2};
use nalgebra;
use window::Window;
use texture::Texture;
use std::rc::Rc;

/// Trait defining a drawer
pub trait Drawer {
	/// Function that draw on itself
	fn draw<T>(&mut self, drawable: &T) where T: Drawable;

    fn activate_shader(&self);
}

/// Trait that can be use to draw on window
pub trait Drawable {
    fn draw<T: Drawer>(&self, window: &mut T) {
        println!("You forgot to implement draw of Drawable Trait");
    }

    fn assign_texture<'a>(&mut self, texture: Rc<Texture>) {
        println!("You forgot to implement assign_texture of Drawable Trait");
    }
}

pub trait Angle {
    fn to_degrees() -> f64;
    fn to_radian() -> f64;
}

/// Trait defining movable structures as sprite or higher
pub trait Movable {
    fn translate<T: nalgebra::Scalar>(&mut self, vec: Vector2<T>);

    fn rotate_from_rotation<T: nalgebra::Scalar>(&mut self, rot: Rotation2<T>);

    fn rotate_from_angle<T: nalgebra::Scalar, R: Angle>(&mut self, angle: R);

    fn scale<T: nalgebra::Scalar>(&mut self, factor: Vector2<T>);
}