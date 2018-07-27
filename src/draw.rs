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

/// Trait defining movable structures as sprite or higher
pub trait Movable {
    fn move_it<T: nalgebra::Scalar>(&mut self, vec: Vector2<T>);

    fn rotate<T: nalgebra::Scalar>(&mut self, rot: Rotation2<T>);
}