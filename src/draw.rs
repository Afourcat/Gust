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
	fn draw<T: Drawable>(&mut self, drawable: &T);

    /// Active shader of the drawer
    fn activate_shader(&self);
}

/// Trait that can be use to draw on window
pub trait Drawable {

    /// Draw the drawable structure, you need a Drawer(Where the struct will be draw)
    fn draw<T: Drawer>(&self, window: &mut T) {
        println!("You forgot to implement draw of Drawable Trait");
    }

    /// Assign a texture to a drawable
    fn assign_texture<'a>(&mut self, texture: Rc<Texture>) {
        println!("You forgot to implement assign_texture of Drawable Trait");
    }
}

/// Trait defining movable structures as sprite or higher
pub trait Movable {
    /// Move the sprite off the offset
    fn translate<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, offset: Vector2<T>);

    /// Set position of the sprite
    fn set_position<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, pos: Vector2<T>);

    /// Get current position
    fn get_position(&self) -> Vector2<f32>;

    /// Scale the sprite from a factor
    fn scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, factor: Vector2<T>);

    /// Set the scale of the sprite
    fn set_scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector2<T>);

    /// Get the current scale
    fn get_scale(&self) -> Vector2<f32>;
}