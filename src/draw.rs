//! Every traits needed by drawable object
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
pub trait Movable<T: nalgebra::Scalar> {
    fn set_position(&mut self, vec: Vector2<T>);

    fn translate(&mut self, vec: Vector2<T>);

    fn set_scale(&mut self, vec: Vector2<T>);

    fn scale(&mut self, factor: Vector2<T>);
}