/*
** Alexandre Fourcat 2018
** drawable
** Description:
** define Drawable trait
*/

use nalgebra::{Vector2};
use window::Window;
use texture::Texture;

/// Trait that can be use to draw on window
pub trait Drawable {
    fn draw(&self, window: &mut Window) {
        println!("You forgot to implement draw of Drawable Trait");
    }

    fn assign_texture(&mut self, texture: &Texture) {
        println!("You forgot to implement assign_texture of Drawable Trait");
    }
}
