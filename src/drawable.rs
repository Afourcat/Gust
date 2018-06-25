/*
** Alexandre Fourcat 2018
** drawable
** Description:
** define Drawable trait
*/

use nalgebra::{Vector2};
use window::Window;

/// Trait that can be use to draw on window
pub trait Drawable {
    fn draw(&self, window: &mut Window) {
        println!("Test");
    }
}