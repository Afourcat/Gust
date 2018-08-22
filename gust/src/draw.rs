//! Every traits needed by drawable object
//!

use nalgebra::{Vector2};
use nalgebra;
use texture::Texture;
use shader::Shader;
use std::rc::Rc;
use vertex::Context;
use nalgebra::Matrix4;

lazy_static! {
	static ref PROJECTION: Matrix4<f32> = Matrix4::new_orthographic(0.0, 900.0, 0.0, 1600.0, -1.0, 1.0);
}

pub fn setup_draw(context: &mut Context) {
    context.apply_projection(&PROJECTION);
    context.apply_texture(0);
    //context.apply_blendmode();
    context.setup_shader();
}

/// Trait defining a drawer
pub trait Drawer {
	/// Function that draw on itself
	fn draw<T: Drawable>(&mut self, drawable: &T);

    /// Draw with context fonction if you want to define you own fonction
    fn draw_with_context<T: Drawable>(&mut self, drawable: &T, Context: &mut Context);

}

/// Trait that can be use to draw on window
pub trait Drawable {

    /// Draw the drawable structure, you need a Drawer(Where the struct will be draw)
    fn draw<T: Drawer>(&self, window: &mut T);

    /// Draw with a particular context
    fn draw_with_context<T: Drawer>(&self, window: &mut T, context: &mut Context);

    /// Assign a texture to a drawable
    fn set_texture(&mut self, texture: &Rc<Texture>);

    /// Update the openGL state of the drawable entity
    /// Should be call often so be carefull when implementing.
    fn update(&mut self);
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
